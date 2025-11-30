use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct GluePartitionData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    partition_values: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_descriptor: Option<Vec<GluePartitionStorageDescriptorEl>>,
    dynamic: GluePartitionDynamic,
}

struct GluePartition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GluePartitionData>,
}

#[derive(Clone)]
pub struct GluePartition(Rc<GluePartition_>);

impl GluePartition {
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

    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_descriptor`.\n"]
    pub fn set_storage_descriptor(
        self,
        v: impl Into<BlockAssignable<GluePartitionStorageDescriptorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_descriptor = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_descriptor = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_accessed_time` after provisioning.\n"]
    pub fn last_accessed_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_accessed_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_analyzed_time` after provisioning.\n"]
    pub fn last_analyzed_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_analyzed_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `partition_values` after provisioning.\n"]
    pub fn partition_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_values", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> ListRef<GluePartitionStorageDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_descriptor", self.extract_ref()),
        )
    }
}

impl Referable for GluePartition {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for GluePartition {}

impl ToListMappable for GluePartition {
    type O = ListRef<GluePartitionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GluePartition_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_partition".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGluePartition {
    pub tf_id: String,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub partition_values: ListField<PrimField<String>>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}

impl BuildGluePartition {
    pub fn build(self, stack: &mut Stack) -> GluePartition {
        let out = GluePartition(Rc::new(GluePartition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GluePartitionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog_id: core::default::Default::default(),
                database_name: self.database_name,
                id: core::default::Default::default(),
                parameters: core::default::Default::default(),
                partition_values: self.partition_values,
                region: core::default::Default::default(),
                table_name: self.table_name,
                storage_descriptor: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GluePartitionRef {
    shared: StackShared,
    base: String,
}

impl Ref for GluePartitionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl GluePartitionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_accessed_time` after provisioning.\n"]
    pub fn last_accessed_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_accessed_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `last_analyzed_time` after provisioning.\n"]
    pub fn last_analyzed_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_analyzed_time", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `partition_values` after provisioning.\n"]
    pub fn partition_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_values", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> ListRef<GluePartitionStorageDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_descriptor", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GluePartitionStorageDescriptorElColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl GluePartitionStorageDescriptorElColumnsEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for GluePartitionStorageDescriptorElColumnsEl {
    type O = BlockAssignable<GluePartitionStorageDescriptorElColumnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorElColumnsEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildGluePartitionStorageDescriptorElColumnsEl {
    pub fn build(self) -> GluePartitionStorageDescriptorElColumnsEl {
        GluePartitionStorageDescriptorElColumnsEl {
            comment: core::default::Default::default(),
            name: self.name,
            type_: core::default::Default::default(),
        }
    }
}

pub struct GluePartitionStorageDescriptorElColumnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GluePartitionStorageDescriptorElColumnsElRef {
    fn new(shared: StackShared, base: String) -> GluePartitionStorageDescriptorElColumnsElRef {
        GluePartitionStorageDescriptorElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElColumnsElRef {
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

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct GluePartitionStorageDescriptorElSerDeInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serialization_library: Option<PrimField<String>>,
}

impl GluePartitionStorageDescriptorElSerDeInfoEl {
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

impl ToListMappable for GluePartitionStorageDescriptorElSerDeInfoEl {
    type O = BlockAssignable<GluePartitionStorageDescriptorElSerDeInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorElSerDeInfoEl {}

impl BuildGluePartitionStorageDescriptorElSerDeInfoEl {
    pub fn build(self) -> GluePartitionStorageDescriptorElSerDeInfoEl {
        GluePartitionStorageDescriptorElSerDeInfoEl {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: core::default::Default::default(),
        }
    }
}

pub struct GluePartitionStorageDescriptorElSerDeInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GluePartitionStorageDescriptorElSerDeInfoElRef {
    fn new(shared: StackShared, base: String) -> GluePartitionStorageDescriptorElSerDeInfoElRef {
        GluePartitionStorageDescriptorElSerDeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElSerDeInfoElRef {
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serialization_library", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct GluePartitionStorageDescriptorElSkewedInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_column_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_column_value_location_maps: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_column_values: Option<ListField<PrimField<String>>>,
}

impl GluePartitionStorageDescriptorElSkewedInfoEl {
    #[doc = "Set the field `skewed_column_names`.\n"]
    pub fn set_skewed_column_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.skewed_column_names = Some(v.into());
        self
    }

    #[doc = "Set the field `skewed_column_value_location_maps`.\n"]
    pub fn set_skewed_column_value_location_maps(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.skewed_column_value_location_maps = Some(v.into());
        self
    }

    #[doc = "Set the field `skewed_column_values`.\n"]
    pub fn set_skewed_column_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.skewed_column_values = Some(v.into());
        self
    }
}

impl ToListMappable for GluePartitionStorageDescriptorElSkewedInfoEl {
    type O = BlockAssignable<GluePartitionStorageDescriptorElSkewedInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorElSkewedInfoEl {}

impl BuildGluePartitionStorageDescriptorElSkewedInfoEl {
    pub fn build(self) -> GluePartitionStorageDescriptorElSkewedInfoEl {
        GluePartitionStorageDescriptorElSkewedInfoEl {
            skewed_column_names: core::default::Default::default(),
            skewed_column_value_location_maps: core::default::Default::default(),
            skewed_column_values: core::default::Default::default(),
        }
    }
}

pub struct GluePartitionStorageDescriptorElSkewedInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GluePartitionStorageDescriptorElSkewedInfoElRef {
    fn new(shared: StackShared, base: String) -> GluePartitionStorageDescriptorElSkewedInfoElRef {
        GluePartitionStorageDescriptorElSkewedInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElSkewedInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `skewed_column_names` after provisioning.\n"]
    pub fn skewed_column_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.skewed_column_names", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `skewed_column_value_location_maps` after provisioning.\n"]
    pub fn skewed_column_value_location_maps(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.skewed_column_value_location_maps", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `skewed_column_values` after provisioning.\n"]
    pub fn skewed_column_values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.skewed_column_values", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct GluePartitionStorageDescriptorElSortColumnsEl {
    column: PrimField<String>,
    sort_order: PrimField<f64>,
}

impl GluePartitionStorageDescriptorElSortColumnsEl {}

impl ToListMappable for GluePartitionStorageDescriptorElSortColumnsEl {
    type O = BlockAssignable<GluePartitionStorageDescriptorElSortColumnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorElSortColumnsEl {
    #[doc = ""]
    pub column: PrimField<String>,
    #[doc = ""]
    pub sort_order: PrimField<f64>,
}

impl BuildGluePartitionStorageDescriptorElSortColumnsEl {
    pub fn build(self) -> GluePartitionStorageDescriptorElSortColumnsEl {
        GluePartitionStorageDescriptorElSortColumnsEl {
            column: self.column,
            sort_order: self.sort_order,
        }
    }
}

pub struct GluePartitionStorageDescriptorElSortColumnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GluePartitionStorageDescriptorElSortColumnsElRef {
    fn new(shared: StackShared, base: String) -> GluePartitionStorageDescriptorElSortColumnsElRef {
        GluePartitionStorageDescriptorElSortColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElSortColumnsElRef {
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
struct GluePartitionStorageDescriptorElDynamic {
    columns: Option<DynamicBlock<GluePartitionStorageDescriptorElColumnsEl>>,
    ser_de_info: Option<DynamicBlock<GluePartitionStorageDescriptorElSerDeInfoEl>>,
    skewed_info: Option<DynamicBlock<GluePartitionStorageDescriptorElSkewedInfoEl>>,
    sort_columns: Option<DynamicBlock<GluePartitionStorageDescriptorElSortColumnsEl>>,
}

#[derive(Serialize)]
pub struct GluePartitionStorageDescriptorEl {
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
    columns: Option<Vec<GluePartitionStorageDescriptorElColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ser_de_info: Option<Vec<GluePartitionStorageDescriptorElSerDeInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_info: Option<Vec<GluePartitionStorageDescriptorElSkewedInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_columns: Option<Vec<GluePartitionStorageDescriptorElSortColumnsEl>>,
    dynamic: GluePartitionStorageDescriptorElDynamic,
}

impl GluePartitionStorageDescriptorEl {
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
    pub fn set_columns(
        mut self,
        v: impl Into<BlockAssignable<GluePartitionStorageDescriptorElColumnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ser_de_info`.\n"]
    pub fn set_ser_de_info(
        mut self,
        v: impl Into<BlockAssignable<GluePartitionStorageDescriptorElSerDeInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ser_de_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ser_de_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `skewed_info`.\n"]
    pub fn set_skewed_info(
        mut self,
        v: impl Into<BlockAssignable<GluePartitionStorageDescriptorElSkewedInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.skewed_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.skewed_info = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sort_columns`.\n"]
    pub fn set_sort_columns(
        mut self,
        v: impl Into<BlockAssignable<GluePartitionStorageDescriptorElSortColumnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sort_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sort_columns = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for GluePartitionStorageDescriptorEl {
    type O = BlockAssignable<GluePartitionStorageDescriptorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGluePartitionStorageDescriptorEl {}

impl BuildGluePartitionStorageDescriptorEl {
    pub fn build(self) -> GluePartitionStorageDescriptorEl {
        GluePartitionStorageDescriptorEl {
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
            ser_de_info: core::default::Default::default(),
            skewed_info: core::default::Default::default(),
            sort_columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GluePartitionStorageDescriptorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GluePartitionStorageDescriptorElRef {
    fn new(shared: StackShared, base: String) -> GluePartitionStorageDescriptorElRef {
        GluePartitionStorageDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GluePartitionStorageDescriptorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_locations` after provisioning.\n"]
    pub fn additional_locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_locations", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `bucket_columns` after provisioning.\n"]
    pub fn bucket_columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bucket_columns", self.base),
        )
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
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_buckets", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_format", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_as_sub_directories` after provisioning.\n"]
    pub fn stored_as_sub_directories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stored_as_sub_directories", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> ListRef<GluePartitionStorageDescriptorElColumnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }

    #[doc = "Get a reference to the value of field `ser_de_info` after provisioning.\n"]
    pub fn ser_de_info(&self) -> ListRef<GluePartitionStorageDescriptorElSerDeInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ser_de_info", self.base))
    }

    #[doc = "Get a reference to the value of field `skewed_info` after provisioning.\n"]
    pub fn skewed_info(&self) -> ListRef<GluePartitionStorageDescriptorElSkewedInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.skewed_info", self.base))
    }

    #[doc = "Get a reference to the value of field `sort_columns` after provisioning.\n"]
    pub fn sort_columns(&self) -> ListRef<GluePartitionStorageDescriptorElSortColumnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct GluePartitionDynamic {
    storage_descriptor: Option<DynamicBlock<GluePartitionStorageDescriptorEl>>,
}
