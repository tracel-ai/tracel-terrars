use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueCatalogTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retention: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view_expanded_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view_original_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_table_format_input: Option<Vec<GlueCatalogTableOpenTableFormatInputEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_index: Option<Vec<GlueCatalogTablePartitionIndexEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    partition_keys: Option<Vec<GlueCatalogTablePartitionKeysEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_descriptor: Option<Vec<GlueCatalogTableStorageDescriptorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_table: Option<Vec<GlueCatalogTableTargetTableEl>>,
    dynamic: GlueCatalogTableDynamic,
}

struct GlueCatalogTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueCatalogTableData>,
}

#[derive(Clone)]
pub struct GlueCatalogTable(Rc<GlueCatalogTable_>);

impl GlueCatalogTable {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().owner = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `retention`.\n"]
    pub fn set_retention(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention = Some(v.into());
        self
    }

    #[doc = "Set the field `table_type`.\n"]
    pub fn set_table_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().table_type = Some(v.into());
        self
    }

    #[doc = "Set the field `view_expanded_text`.\n"]
    pub fn set_view_expanded_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().view_expanded_text = Some(v.into());
        self
    }

    #[doc = "Set the field `view_original_text`.\n"]
    pub fn set_view_original_text(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().view_original_text = Some(v.into());
        self
    }

    #[doc = "Set the field `open_table_format_input`.\n"]
    pub fn set_open_table_format_input(
        self,
        v: impl Into<BlockAssignable<GlueCatalogTableOpenTableFormatInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().open_table_format_input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.open_table_format_input = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `partition_index`.\n"]
    pub fn set_partition_index(self, v: impl Into<BlockAssignable<GlueCatalogTablePartitionIndexEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partition_index = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partition_index = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `partition_keys`.\n"]
    pub fn set_partition_keys(self, v: impl Into<BlockAssignable<GlueCatalogTablePartitionKeysEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().partition_keys = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.partition_keys = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_descriptor`.\n"]
    pub fn set_storage_descriptor(self, v: impl Into<BlockAssignable<GlueCatalogTableStorageDescriptorEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_descriptor = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_descriptor = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_table`.\n"]
    pub fn set_target_table(self, v: impl Into<BlockAssignable<GlueCatalogTableTargetTableEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_table = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_table = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_type` after provisioning.\n"]
    pub fn table_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_expanded_text` after provisioning.\n"]
    pub fn view_expanded_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.view_expanded_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_original_text` after provisioning.\n"]
    pub fn view_original_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.view_original_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_table_format_input` after provisioning.\n"]
    pub fn open_table_format_input(&self) -> ListRef<GlueCatalogTableOpenTableFormatInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.open_table_format_input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> ListRef<GlueCatalogTablePartitionIndexElRef> {
        ListRef::new(self.shared().clone(), format!("{}.partition_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> ListRef<GlueCatalogTablePartitionKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> ListRef<GlueCatalogTableStorageDescriptorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> ListRef<GlueCatalogTableTargetTableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_table", self.extract_ref()))
    }
}

impl Referable for GlueCatalogTable {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for GlueCatalogTable { }

impl ToListMappable for GlueCatalogTable {
    type O = ListRef<GlueCatalogTableRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GlueCatalogTable_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_catalog_table".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueCatalogTable {
    pub tf_id: String,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildGlueCatalogTable {
    pub fn build(self, stack: &mut Stack) -> GlueCatalogTable {
        let out = GlueCatalogTable(Rc::new(GlueCatalogTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueCatalogTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog_id: core::default::Default::default(),
                database_name: self.database_name,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                owner: core::default::Default::default(),
                parameters: core::default::Default::default(),
                region: core::default::Default::default(),
                retention: core::default::Default::default(),
                table_type: core::default::Default::default(),
                view_expanded_text: core::default::Default::default(),
                view_original_text: core::default::Default::default(),
                open_table_format_input: core::default::Default::default(),
                partition_index: core::default::Default::default(),
                partition_keys: core::default::Default::default(),
                storage_descriptor: core::default::Default::default(),
                target_table: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueCatalogTableRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl GlueCatalogTableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_type` after provisioning.\n"]
    pub fn table_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_expanded_text` after provisioning.\n"]
    pub fn view_expanded_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.view_expanded_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_original_text` after provisioning.\n"]
    pub fn view_original_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.view_original_text", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `open_table_format_input` after provisioning.\n"]
    pub fn open_table_format_input(&self) -> ListRef<GlueCatalogTableOpenTableFormatInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.open_table_format_input", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> ListRef<GlueCatalogTablePartitionIndexElRef> {
        ListRef::new(self.shared().clone(), format!("{}.partition_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> ListRef<GlueCatalogTablePartitionKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.partition_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> ListRef<GlueCatalogTableStorageDescriptorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_descriptor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> ListRef<GlueCatalogTableTargetTableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target_table", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableOpenTableFormatInputElIcebergInputEl {
    metadata_operation: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl GlueCatalogTableOpenTableFormatInputElIcebergInputEl {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for GlueCatalogTableOpenTableFormatInputElIcebergInputEl {
    type O = BlockAssignable<GlueCatalogTableOpenTableFormatInputElIcebergInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOpenTableFormatInputElIcebergInputEl {
    #[doc = ""]
    pub metadata_operation: PrimField<String>,
}

impl BuildGlueCatalogTableOpenTableFormatInputElIcebergInputEl {
    pub fn build(self) -> GlueCatalogTableOpenTableFormatInputElIcebergInputEl {
        GlueCatalogTableOpenTableFormatInputElIcebergInputEl {
            metadata_operation: self.metadata_operation,
            version: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableOpenTableFormatInputElIcebergInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableOpenTableFormatInputElIcebergInputElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableOpenTableFormatInputElIcebergInputElRef {
        GlueCatalogTableOpenTableFormatInputElIcebergInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOpenTableFormatInputElIcebergInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata_operation` after provisioning.\n"]
    pub fn metadata_operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_operation", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableOpenTableFormatInputElDynamic {
    iceberg_input: Option<DynamicBlock<GlueCatalogTableOpenTableFormatInputElIcebergInputEl>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTableOpenTableFormatInputEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_input: Option<Vec<GlueCatalogTableOpenTableFormatInputElIcebergInputEl>>,
    dynamic: GlueCatalogTableOpenTableFormatInputElDynamic,
}

impl GlueCatalogTableOpenTableFormatInputEl {
    #[doc = "Set the field `iceberg_input`.\n"]
    pub fn set_iceberg_input(
        mut self,
        v: impl Into<BlockAssignable<GlueCatalogTableOpenTableFormatInputElIcebergInputEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.iceberg_input = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.iceberg_input = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GlueCatalogTableOpenTableFormatInputEl {
    type O = BlockAssignable<GlueCatalogTableOpenTableFormatInputEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableOpenTableFormatInputEl {}

impl BuildGlueCatalogTableOpenTableFormatInputEl {
    pub fn build(self) -> GlueCatalogTableOpenTableFormatInputEl {
        GlueCatalogTableOpenTableFormatInputEl {
            iceberg_input: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GlueCatalogTableOpenTableFormatInputElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableOpenTableFormatInputElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableOpenTableFormatInputElRef {
        GlueCatalogTableOpenTableFormatInputElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableOpenTableFormatInputElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iceberg_input` after provisioning.\n"]
    pub fn iceberg_input(&self) -> ListRef<GlueCatalogTableOpenTableFormatInputElIcebergInputElRef> {
        ListRef::new(self.shared().clone(), format!("{}.iceberg_input", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTablePartitionIndexEl {
    index_name: PrimField<String>,
    keys: ListField<PrimField<String>>,
}

impl GlueCatalogTablePartitionIndexEl { }

impl ToListMappable for GlueCatalogTablePartitionIndexEl {
    type O = BlockAssignable<GlueCatalogTablePartitionIndexEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTablePartitionIndexEl {
    #[doc = ""]
    pub index_name: PrimField<String>,
    #[doc = ""]
    pub keys: ListField<PrimField<String>>,
}

impl BuildGlueCatalogTablePartitionIndexEl {
    pub fn build(self) -> GlueCatalogTablePartitionIndexEl {
        GlueCatalogTablePartitionIndexEl {
            index_name: self.index_name,
            keys: self.keys,
        }
    }
}

pub struct GlueCatalogTablePartitionIndexElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTablePartitionIndexElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTablePartitionIndexElRef {
        GlueCatalogTablePartitionIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTablePartitionIndexElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `index_name` after provisioning.\n"]
    pub fn index_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }

    #[doc = "Get a reference to the value of field `index_status` after provisioning.\n"]
    pub fn index_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.index_status", self.base))
    }

    #[doc = "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.keys", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTablePartitionKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl GlueCatalogTablePartitionKeysEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for GlueCatalogTablePartitionKeysEl {
    type O = BlockAssignable<GlueCatalogTablePartitionKeysEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTablePartitionKeysEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildGlueCatalogTablePartitionKeysEl {
    pub fn build(self) -> GlueCatalogTablePartitionKeysEl {
        GlueCatalogTablePartitionKeysEl {
            comment: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTablePartitionKeysElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTablePartitionKeysElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTablePartitionKeysElRef {
        GlueCatalogTablePartitionKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTablePartitionKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorElColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl GlueCatalogTableStorageDescriptorElColumnsEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for GlueCatalogTableStorageDescriptorElColumnsEl {
    type O = BlockAssignable<GlueCatalogTableStorageDescriptorElColumnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElColumnsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildGlueCatalogTableStorageDescriptorElColumnsEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElColumnsEl {
        GlueCatalogTableStorageDescriptorElColumnsEl {
            comment: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElColumnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableStorageDescriptorElColumnsElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableStorageDescriptorElColumnsElRef {
        GlueCatalogTableStorageDescriptorElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_name: Option<PrimField<String>>,
}

impl GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    #[doc = "Set the field `registry_name`.\n"]
    pub fn set_registry_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.registry_name = Some(v.into());
        self
    }

    #[doc = "Set the field `schema_arn`.\n"]
    pub fn set_schema_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `schema_name`.\n"]
    pub fn set_schema_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_name = Some(v.into());
        self
    }
}

impl ToListMappable for GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    type O = BlockAssignable<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {}

impl BuildGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
        GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
            registry_name: core::default::Default::default(),
            schema_arn: core::default::Default::default(),
            schema_name: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
        GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `registry_name` after provisioning.\n"]
    pub fn registry_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registry_name", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_arn` after provisioning.\n"]
    pub fn schema_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_name` after provisioning.\n"]
    pub fn schema_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableStorageDescriptorElSchemaReferenceElDynamic {
    schema_id: Option<DynamicBlock<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_version_id: Option<PrimField<String>>,
    schema_version_number: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_id: Option<Vec<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
    dynamic: GlueCatalogTableStorageDescriptorElSchemaReferenceElDynamic,
}

impl GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    #[doc = "Set the field `schema_version_id`.\n"]
    pub fn set_schema_version_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_version_id = Some(v.into());
        self
    }

    #[doc = "Set the field `schema_id`.\n"]
    pub fn set_schema_id(
        mut self,
        v: impl Into<BlockAssignable<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema_id = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_id = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    type O = BlockAssignable<GlueCatalogTableStorageDescriptorElSchemaReferenceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    #[doc = ""]
    pub schema_version_number: PrimField<f64>,
}

impl BuildGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
        GlueCatalogTableStorageDescriptorElSchemaReferenceEl {
            schema_version_id: core::default::Default::default(),
            schema_version_number: self.schema_version_number,
            schema_id: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
        GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema_version_id` after provisioning.\n"]
    pub fn schema_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema_version_id", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_version_number` after provisioning.\n"]
    pub fn schema_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema_version_number", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_id` after provisioning.\n"]
    pub fn schema_id(&self) -> ListRef<GlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema_id", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorElSerDeInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serialization_library: Option<PrimField<String>>,
}

impl GlueCatalogTableStorageDescriptorElSerDeInfoEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `serialization_library`.\n"]
    pub fn set_serialization_library(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.serialization_library = Some(v.into());
        self
    }
}

impl ToListMappable for GlueCatalogTableStorageDescriptorElSerDeInfoEl {
    type O = BlockAssignable<GlueCatalogTableStorageDescriptorElSerDeInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSerDeInfoEl {}

impl BuildGlueCatalogTableStorageDescriptorElSerDeInfoEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSerDeInfoEl {
        GlueCatalogTableStorageDescriptorElSerDeInfoEl {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
        GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSerDeInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `serialization_library` after provisioning.\n"]
    pub fn serialization_library(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serialization_library", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorElSkewedInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_column_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_column_value_location_maps: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_column_values: Option<ListField<PrimField<String>>>,
}

impl GlueCatalogTableStorageDescriptorElSkewedInfoEl {
    #[doc = "Set the field `skewed_column_names`.\n"]
    pub fn set_skewed_column_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.skewed_column_names = Some(v.into());
        self
    }

    #[doc = "Set the field `skewed_column_value_location_maps`.\n"]
    pub fn set_skewed_column_value_location_maps(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.skewed_column_value_location_maps = Some(v.into());
        self
    }

    #[doc = "Set the field `skewed_column_values`.\n"]
    pub fn set_skewed_column_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.skewed_column_values = Some(v.into());
        self
    }
}

impl ToListMappable for GlueCatalogTableStorageDescriptorElSkewedInfoEl {
    type O = BlockAssignable<GlueCatalogTableStorageDescriptorElSkewedInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSkewedInfoEl {}

impl BuildGlueCatalogTableStorageDescriptorElSkewedInfoEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSkewedInfoEl {
        GlueCatalogTableStorageDescriptorElSkewedInfoEl {
            skewed_column_names: core::default::Default::default(),
            skewed_column_value_location_maps: core::default::Default::default(),
            skewed_column_values: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
        GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSkewedInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `skewed_column_names` after provisioning.\n"]
    pub fn skewed_column_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.skewed_column_names", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_column_value_location_maps` after provisioning.\n"]
    pub fn skewed_column_value_location_maps(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.skewed_column_value_location_maps", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_column_values` after provisioning.\n"]
    pub fn skewed_column_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.skewed_column_values", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorElSortColumnsEl {
    column: PrimField<String>,
    sort_order: PrimField<f64>,
}

impl GlueCatalogTableStorageDescriptorElSortColumnsEl { }

impl ToListMappable for GlueCatalogTableStorageDescriptorElSortColumnsEl {
    type O = BlockAssignable<GlueCatalogTableStorageDescriptorElSortColumnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorElSortColumnsEl {
    #[doc = ""]
    pub column: PrimField<String>,
    #[doc = ""]
    pub sort_order: PrimField<f64>,
}

impl BuildGlueCatalogTableStorageDescriptorElSortColumnsEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorElSortColumnsEl {
        GlueCatalogTableStorageDescriptorElSortColumnsEl {
            column: self.column,
            sort_order: self.sort_order,
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElSortColumnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableStorageDescriptorElSortColumnsElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableStorageDescriptorElSortColumnsElRef {
        GlueCatalogTableStorageDescriptorElSortColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElSortColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\n"]
    pub fn column(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_order` after provisioning.\n"]
    pub fn sort_order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sort_order", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableStorageDescriptorElDynamic {
    columns: Option<DynamicBlock<GlueCatalogTableStorageDescriptorElColumnsEl>>,
    schema_reference: Option<DynamicBlock<GlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
    ser_de_info: Option<DynamicBlock<GlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
    skewed_info: Option<DynamicBlock<GlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
    sort_columns: Option<DynamicBlock<GlueCatalogTableStorageDescriptorElSortColumnsEl>>,
}

#[derive(Serialize)]
pub struct GlueCatalogTableStorageDescriptorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_columns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compressed: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_buckets: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stored_as_sub_directories: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<Vec<GlueCatalogTableStorageDescriptorElColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_reference: Option<Vec<GlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ser_de_info: Option<Vec<GlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_info: Option<Vec<GlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_columns: Option<Vec<GlueCatalogTableStorageDescriptorElSortColumnsEl>>,
    dynamic: GlueCatalogTableStorageDescriptorElDynamic,
}

impl GlueCatalogTableStorageDescriptorEl {
    #[doc = "Set the field `additional_locations`.\n"]
    pub fn set_additional_locations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.additional_locations = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_columns`.\n"]
    pub fn set_bucket_columns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.bucket_columns = Some(v.into());
        self
    }

    #[doc = "Set the field `compressed`.\n"]
    pub fn set_compressed(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.compressed = Some(v.into());
        self
    }

    #[doc = "Set the field `input_format`.\n"]
    pub fn set_input_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_format = Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_buckets`.\n"]
    pub fn set_number_of_buckets(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.number_of_buckets = Some(v.into());
        self
    }

    #[doc = "Set the field `output_format`.\n"]
    pub fn set_output_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_format = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `stored_as_sub_directories`.\n"]
    pub fn set_stored_as_sub_directories(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.stored_as_sub_directories = Some(v.into());
        self
    }

    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(mut self, v: impl Into<BlockAssignable<GlueCatalogTableStorageDescriptorElColumnsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.columns = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_reference`.\n"]
    pub fn set_schema_reference(
        mut self,
        v: impl Into<BlockAssignable<GlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.schema_reference = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_reference = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ser_de_info`.\n"]
    pub fn set_ser_de_info(
        mut self,
        v: impl Into<BlockAssignable<GlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ser_de_info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ser_de_info = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `skewed_info`.\n"]
    pub fn set_skewed_info(
        mut self,
        v: impl Into<BlockAssignable<GlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.skewed_info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.skewed_info = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sort_columns`.\n"]
    pub fn set_sort_columns(
        mut self,
        v: impl Into<BlockAssignable<GlueCatalogTableStorageDescriptorElSortColumnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sort_columns = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sort_columns = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for GlueCatalogTableStorageDescriptorEl {
    type O = BlockAssignable<GlueCatalogTableStorageDescriptorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableStorageDescriptorEl {}

impl BuildGlueCatalogTableStorageDescriptorEl {
    pub fn build(self) -> GlueCatalogTableStorageDescriptorEl {
        GlueCatalogTableStorageDescriptorEl {
            additional_locations: core::default::Default::default(),
            bucket_columns: core::default::Default::default(),
            compressed: core::default::Default::default(),
            input_format: core::default::Default::default(),
            location: core::default::Default::default(),
            number_of_buckets: core::default::Default::default(),
            output_format: core::default::Default::default(),
            parameters: core::default::Default::default(),
            stored_as_sub_directories: core::default::Default::default(),
            columns: core::default::Default::default(),
            schema_reference: core::default::Default::default(),
            ser_de_info: core::default::Default::default(),
            skewed_info: core::default::Default::default(),
            sort_columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GlueCatalogTableStorageDescriptorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableStorageDescriptorElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableStorageDescriptorElRef {
        GlueCatalogTableStorageDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableStorageDescriptorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_locations` after provisioning.\n"]
    pub fn additional_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.additional_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_columns` after provisioning.\n"]
    pub fn bucket_columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.bucket_columns", self.base))
    }

    #[doc = "Get a reference to the value of field `compressed` after provisioning.\n"]
    pub fn compressed(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.compressed", self.base))
    }

    #[doc = "Get a reference to the value of field `input_format` after provisioning.\n"]
    pub fn input_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_buckets` after provisioning.\n"]
    pub fn number_of_buckets(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_format", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_as_sub_directories` after provisioning.\n"]
    pub fn stored_as_sub_directories(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.stored_as_sub_directories", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> ListRef<GlueCatalogTableStorageDescriptorElColumnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_reference` after provisioning.\n"]
    pub fn schema_reference(&self) -> ListRef<GlueCatalogTableStorageDescriptorElSchemaReferenceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `ser_de_info` after provisioning.\n"]
    pub fn ser_de_info(&self) -> ListRef<GlueCatalogTableStorageDescriptorElSerDeInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ser_de_info", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_info` after provisioning.\n"]
    pub fn skewed_info(&self) -> ListRef<GlueCatalogTableStorageDescriptorElSkewedInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.skewed_info", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_columns` after provisioning.\n"]
    pub fn sort_columns(&self) -> ListRef<GlueCatalogTableStorageDescriptorElSortColumnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort_columns", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueCatalogTableTargetTableEl {
    catalog_id: PrimField<String>,
    database_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

impl GlueCatalogTableTargetTableEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}

impl ToListMappable for GlueCatalogTableTargetTableEl {
    type O = BlockAssignable<GlueCatalogTableTargetTableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueCatalogTableTargetTableEl {
    #[doc = ""]
    pub catalog_id: PrimField<String>,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildGlueCatalogTableTargetTableEl {
    pub fn build(self) -> GlueCatalogTableTargetTableEl {
        GlueCatalogTableTargetTableEl {
            catalog_id: self.catalog_id,
            database_name: self.database_name,
            name: self.name,
            region: core::default::Default::default(),
        }
    }
}

pub struct GlueCatalogTableTargetTableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueCatalogTableTargetTableElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogTableTargetTableElRef {
        GlueCatalogTableTargetTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueCatalogTableTargetTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueCatalogTableDynamic {
    open_table_format_input: Option<DynamicBlock<GlueCatalogTableOpenTableFormatInputEl>>,
    partition_index: Option<DynamicBlock<GlueCatalogTablePartitionIndexEl>>,
    partition_keys: Option<DynamicBlock<GlueCatalogTablePartitionKeysEl>>,
    storage_descriptor: Option<DynamicBlock<GlueCatalogTableStorageDescriptorEl>>,
    target_table: Option<DynamicBlock<GlueCatalogTableTargetTableEl>>,
}
