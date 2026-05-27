use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BiglakeIcebergTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    catalog: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    name: PrimField<String>,
    namespace: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_spec: Option<Vec<BiglakeIcebergTablePartitionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<Vec<BiglakeIcebergTableSchemaEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BiglakeIcebergTableTimeoutsEl>,
    dynamic: BiglakeIcebergTableDynamic,
}
struct BiglakeIcebergTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BiglakeIcebergTableData>,
}
#[derive(Clone)]
pub struct BiglakeIcebergTable(Rc<BiglakeIcebergTable_>);
impl BiglakeIcebergTable {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe location of the table."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nUser-defined properties for the table."]
    pub fn set_properties(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().properties = Some(v.into());
        self
    }
    #[doc = "Set the field `partition_spec`.\n"]
    pub fn set_partition_spec(
        self,
        v: impl Into<BlockAssignable<BiglakeIcebergTablePartitionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partition_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partition_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(self, v: impl Into<BlockAssignable<BiglakeIcebergTableSchemaEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BiglakeIcebergTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `catalog` after provisioning.\nThe name of the IcebergCatalog."]
    pub fn catalog(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the table."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the table."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe parent namespace of the table."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nUser-defined properties for the table."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partition_spec` after provisioning.\n"]
    pub fn partition_spec(&self) -> ListRef<BiglakeIcebergTablePartitionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<BiglakeIcebergTableSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeIcebergTableTimeoutsElRef {
        BiglakeIcebergTableTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BiglakeIcebergTable {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BiglakeIcebergTable {}
impl ToListMappable for BiglakeIcebergTable {
    type O = ListRef<BiglakeIcebergTableRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BiglakeIcebergTable_ {
    fn extract_resource_type(&self) -> String {
        "google_biglake_iceberg_table".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBiglakeIcebergTable {
    pub tf_id: String,
    #[doc = "The name of the IcebergCatalog."]
    pub catalog: PrimField<String>,
    #[doc = "The name of the table."]
    pub name: PrimField<String>,
    #[doc = "The parent namespace of the table."]
    pub namespace: PrimField<String>,
}
impl BuildBiglakeIcebergTable {
    pub fn build(self, stack: &mut Stack) -> BiglakeIcebergTable {
        let out = BiglakeIcebergTable(Rc::new(BiglakeIcebergTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BiglakeIcebergTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog: self.catalog,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                name: self.name,
                namespace: self.namespace,
                project: core::default::Default::default(),
                properties: core::default::Default::default(),
                partition_spec: core::default::Default::default(),
                schema: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BiglakeIcebergTableRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeIcebergTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BiglakeIcebergTableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog` after provisioning.\nThe name of the IcebergCatalog."]
    pub fn catalog(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the table."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the table."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nThe parent namespace of the table."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nUser-defined properties for the table."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partition_spec` after provisioning.\n"]
    pub fn partition_spec(&self) -> ListRef<BiglakeIcebergTablePartitionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<BiglakeIcebergTableSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeIcebergTableTimeoutsElRef {
        BiglakeIcebergTableTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BiglakeIcebergTablePartitionSpecElFieldsEl {
    name: PrimField<String>,
    source_id: PrimField<f64>,
    transform: PrimField<String>,
}
impl BiglakeIcebergTablePartitionSpecElFieldsEl {}
impl ToListMappable for BiglakeIcebergTablePartitionSpecElFieldsEl {
    type O = BlockAssignable<BiglakeIcebergTablePartitionSpecElFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeIcebergTablePartitionSpecElFieldsEl {
    #[doc = "The name of the partition field."]
    pub name: PrimField<String>,
    #[doc = "The source field ID for the partition field."]
    pub source_id: PrimField<f64>,
    #[doc = "The transform to apply to the source field."]
    pub transform: PrimField<String>,
}
impl BuildBiglakeIcebergTablePartitionSpecElFieldsEl {
    pub fn build(self) -> BiglakeIcebergTablePartitionSpecElFieldsEl {
        BiglakeIcebergTablePartitionSpecElFieldsEl {
            name: self.name,
            source_id: self.source_id,
            transform: self.transform,
        }
    }
}
pub struct BiglakeIcebergTablePartitionSpecElFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeIcebergTablePartitionSpecElFieldsElRef {
    fn new(shared: StackShared, base: String) -> BiglakeIcebergTablePartitionSpecElFieldsElRef {
        BiglakeIcebergTablePartitionSpecElFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeIcebergTablePartitionSpecElFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field_id` after provisioning.\nThe unique identifier of the partition field."]
    pub fn field_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the partition field."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `source_id` after provisioning.\nThe source field ID for the partition field."]
    pub fn source_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }
    #[doc = "Get a reference to the value of field `transform` after provisioning.\nThe transform to apply to the source field."]
    pub fn transform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transform", self.base))
    }
}
#[derive(Serialize, Default)]
struct BiglakeIcebergTablePartitionSpecElDynamic {
    fields: Option<DynamicBlock<BiglakeIcebergTablePartitionSpecElFieldsEl>>,
}
#[derive(Serialize)]
pub struct BiglakeIcebergTablePartitionSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<BiglakeIcebergTablePartitionSpecElFieldsEl>>,
    dynamic: BiglakeIcebergTablePartitionSpecElDynamic,
}
impl BiglakeIcebergTablePartitionSpecEl {
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v: impl Into<BlockAssignable<BiglakeIcebergTablePartitionSpecElFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BiglakeIcebergTablePartitionSpecEl {
    type O = BlockAssignable<BiglakeIcebergTablePartitionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeIcebergTablePartitionSpecEl {}
impl BuildBiglakeIcebergTablePartitionSpecEl {
    pub fn build(self) -> BiglakeIcebergTablePartitionSpecEl {
        BiglakeIcebergTablePartitionSpecEl {
            fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BiglakeIcebergTablePartitionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeIcebergTablePartitionSpecElRef {
    fn new(shared: StackShared, base: String) -> BiglakeIcebergTablePartitionSpecElRef {
        BiglakeIcebergTablePartitionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeIcebergTablePartitionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `spec_id` after provisioning.\nThe unique identifier of the partition spec."]
    pub fn spec_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.spec_id", self.base))
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> ListRef<BiglakeIcebergTablePartitionSpecElFieldsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}
#[derive(Serialize)]
pub struct BiglakeIcebergTableSchemaElFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    doc: Option<PrimField<String>>,
    id: PrimField<f64>,
    name: PrimField<String>,
    required: PrimField<bool>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl BiglakeIcebergTableSchemaElFieldsEl {
    #[doc = "Set the field `doc`.\nA description of the field."]
    pub fn set_doc(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.doc = Some(v.into());
        self
    }
}
impl ToListMappable for BiglakeIcebergTableSchemaElFieldsEl {
    type O = BlockAssignable<BiglakeIcebergTableSchemaElFieldsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeIcebergTableSchemaElFieldsEl {
    #[doc = "The unique identifier of the field."]
    pub id: PrimField<f64>,
    #[doc = "The name of the field."]
    pub name: PrimField<String>,
    #[doc = "Whether the field is required."]
    pub required: PrimField<bool>,
    #[doc = "The type of the field."]
    pub type_: PrimField<String>,
}
impl BuildBiglakeIcebergTableSchemaElFieldsEl {
    pub fn build(self) -> BiglakeIcebergTableSchemaElFieldsEl {
        BiglakeIcebergTableSchemaElFieldsEl {
            doc: core::default::Default::default(),
            id: self.id,
            name: self.name,
            required: self.required,
            type_: self.type_,
        }
    }
}
pub struct BiglakeIcebergTableSchemaElFieldsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeIcebergTableSchemaElFieldsElRef {
    fn new(shared: StackShared, base: String) -> BiglakeIcebergTableSchemaElFieldsElRef {
        BiglakeIcebergTableSchemaElFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeIcebergTableSchemaElFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `doc` after provisioning.\nA description of the field."]
    pub fn doc(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.doc", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the field."]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the field."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\nWhether the field is required."]
    pub fn required(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the field."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct BiglakeIcebergTableSchemaElDynamic {
    fields: Option<DynamicBlock<BiglakeIcebergTableSchemaElFieldsEl>>,
}
#[derive(Serialize)]
pub struct BiglakeIcebergTableSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier_field_ids: Option<ListField<PrimField<f64>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<BiglakeIcebergTableSchemaElFieldsEl>>,
    dynamic: BiglakeIcebergTableSchemaElDynamic,
}
impl BiglakeIcebergTableSchemaEl {
    #[doc = "Set the field `identifier_field_ids`.\nThe field IDs that make up the identifier for the table."]
    pub fn set_identifier_field_ids(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.identifier_field_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type of the schema."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v: impl Into<BlockAssignable<BiglakeIcebergTableSchemaElFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fields = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BiglakeIcebergTableSchemaEl {
    type O = BlockAssignable<BiglakeIcebergTableSchemaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeIcebergTableSchemaEl {}
impl BuildBiglakeIcebergTableSchemaEl {
    pub fn build(self) -> BiglakeIcebergTableSchemaEl {
        BiglakeIcebergTableSchemaEl {
            identifier_field_ids: core::default::Default::default(),
            type_: core::default::Default::default(),
            fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BiglakeIcebergTableSchemaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeIcebergTableSchemaElRef {
    fn new(shared: StackShared, base: String) -> BiglakeIcebergTableSchemaElRef {
        BiglakeIcebergTableSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeIcebergTableSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identifier_field_ids` after provisioning.\nThe field IDs that make up the identifier for the table."]
    pub fn identifier_field_ids(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identifier_field_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schema_id` after provisioning.\nThe unique identifier of the schema."]
    pub fn schema_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema_id", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the schema."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> ListRef<BiglakeIcebergTableSchemaElFieldsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}
#[derive(Serialize)]
pub struct BiglakeIcebergTableTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BiglakeIcebergTableTimeoutsEl {
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
impl ToListMappable for BiglakeIcebergTableTimeoutsEl {
    type O = BlockAssignable<BiglakeIcebergTableTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeIcebergTableTimeoutsEl {}
impl BuildBiglakeIcebergTableTimeoutsEl {
    pub fn build(self) -> BiglakeIcebergTableTimeoutsEl {
        BiglakeIcebergTableTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BiglakeIcebergTableTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeIcebergTableTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BiglakeIcebergTableTimeoutsElRef {
        BiglakeIcebergTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeIcebergTableTimeoutsElRef {
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
struct BiglakeIcebergTableDynamic {
    partition_spec: Option<DynamicBlock<BiglakeIcebergTablePartitionSpecEl>>,
    schema: Option<DynamicBlock<BiglakeIcebergTableSchemaEl>>,
}
