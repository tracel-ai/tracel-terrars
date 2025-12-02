use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataGlueCatalogTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_as_of_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transaction_id: Option<PrimField<f64>>,
}
struct DataGlueCatalogTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGlueCatalogTableData>,
}
#[derive(Clone)]
pub struct DataGlueCatalogTable(Rc<DataGlueCatalogTable_>);
impl DataGlueCatalogTable {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Set the field `query_as_of_time`.\n"]
    pub fn set_query_as_of_time(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().query_as_of_time = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `transaction_id`.\n"]
    pub fn set_transaction_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().transaction_id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> ListRef<DataGlueCatalogTablePartitionIndexElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_index", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> ListRef<DataGlueCatalogTablePartitionKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_as_of_time` after provisioning.\n"]
    pub fn query_as_of_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_as_of_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> ListRef<DataGlueCatalogTableStorageDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_descriptor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_type` after provisioning.\n"]
    pub fn table_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> ListRef<DataGlueCatalogTableTargetTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transaction_id` after provisioning.\n"]
    pub fn transaction_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transaction_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view_expanded_text` after provisioning.\n"]
    pub fn view_expanded_text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_expanded_text", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view_original_text` after provisioning.\n"]
    pub fn view_original_text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_original_text", self.extract_ref()),
        )
    }
}
impl Referable for DataGlueCatalogTable {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataGlueCatalogTable {}
impl ToListMappable for DataGlueCatalogTable {
    type O = ListRef<DataGlueCatalogTableRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataGlueCatalogTable_ {
    fn extract_datasource_type(&self) -> String {
        "aws_glue_catalog_table".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataGlueCatalogTable {
    pub tf_id: String,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataGlueCatalogTable {
    pub fn build(self, stack: &mut Stack) -> DataGlueCatalogTable {
        let out = DataGlueCatalogTable(Rc::new(DataGlueCatalogTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGlueCatalogTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                catalog_id: core::default::Default::default(),
                database_name: self.database_name,
                id: core::default::Default::default(),
                name: self.name,
                query_as_of_time: core::default::Default::default(),
                region: core::default::Default::default(),
                transaction_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataGlueCatalogTableRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataGlueCatalogTableRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partition_index` after provisioning.\n"]
    pub fn partition_index(&self) -> ListRef<DataGlueCatalogTablePartitionIndexElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_index", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `partition_keys` after provisioning.\n"]
    pub fn partition_keys(&self) -> ListRef<DataGlueCatalogTablePartitionKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.partition_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_as_of_time` after provisioning.\n"]
    pub fn query_as_of_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_as_of_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.retention", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> ListRef<DataGlueCatalogTableStorageDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_descriptor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_type` after provisioning.\n"]
    pub fn table_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_table` after provisioning.\n"]
    pub fn target_table(&self) -> ListRef<DataGlueCatalogTableTargetTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transaction_id` after provisioning.\n"]
    pub fn transaction_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transaction_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view_expanded_text` after provisioning.\n"]
    pub fn view_expanded_text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_expanded_text", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view_original_text` after provisioning.\n"]
    pub fn view_original_text(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_original_text", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataGlueCatalogTablePartitionIndexEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    index_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keys: Option<ListField<PrimField<String>>>,
}
impl DataGlueCatalogTablePartitionIndexEl {
    #[doc = "Set the field `index_name`.\n"]
    pub fn set_index_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.index_name = Some(v.into());
        self
    }
    #[doc = "Set the field `index_status`.\n"]
    pub fn set_index_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.index_status = Some(v.into());
        self
    }
    #[doc = "Set the field `keys`.\n"]
    pub fn set_keys(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.keys = Some(v.into());
        self
    }
}
impl ToListMappable for DataGlueCatalogTablePartitionIndexEl {
    type O = BlockAssignable<DataGlueCatalogTablePartitionIndexEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTablePartitionIndexEl {}
impl BuildDataGlueCatalogTablePartitionIndexEl {
    pub fn build(self) -> DataGlueCatalogTablePartitionIndexEl {
        DataGlueCatalogTablePartitionIndexEl {
            index_name: core::default::Default::default(),
            index_status: core::default::Default::default(),
            keys: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTablePartitionIndexElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTablePartitionIndexElRef {
    fn new(shared: StackShared, base: String) -> DataGlueCatalogTablePartitionIndexElRef {
        DataGlueCatalogTablePartitionIndexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTablePartitionIndexElRef {
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
pub struct DataGlueCatalogTablePartitionKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataGlueCatalogTablePartitionKeysEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment = Some(v.into());
        self
    }
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
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataGlueCatalogTablePartitionKeysEl {
    type O = BlockAssignable<DataGlueCatalogTablePartitionKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTablePartitionKeysEl {}
impl BuildDataGlueCatalogTablePartitionKeysEl {
    pub fn build(self) -> DataGlueCatalogTablePartitionKeysEl {
        DataGlueCatalogTablePartitionKeysEl {
            comment: core::default::Default::default(),
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTablePartitionKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTablePartitionKeysElRef {
    fn new(shared: StackShared, base: String) -> DataGlueCatalogTablePartitionKeysElRef {
        DataGlueCatalogTablePartitionKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTablePartitionKeysElRef {
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
pub struct DataGlueCatalogTableStorageDescriptorElColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataGlueCatalogTableStorageDescriptorElColumnsEl {
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.comment = Some(v.into());
        self
    }
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
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataGlueCatalogTableStorageDescriptorElColumnsEl {
    type O = BlockAssignable<DataGlueCatalogTableStorageDescriptorElColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTableStorageDescriptorElColumnsEl {}
impl BuildDataGlueCatalogTableStorageDescriptorElColumnsEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElColumnsEl {
        DataGlueCatalogTableStorageDescriptorElColumnsEl {
            comment: core::default::Default::default(),
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTableStorageDescriptorElColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTableStorageDescriptorElColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlueCatalogTableStorageDescriptorElColumnsElRef {
        DataGlueCatalogTableStorageDescriptorElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTableStorageDescriptorElColumnsElRef {
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
pub struct DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    registry_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_name: Option<PrimField<String>>,
}
impl DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
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
impl ToListMappable for DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    type O = BlockAssignable<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {}
impl BuildDataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
        DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl {
            registry_name: core::default::Default::default(),
            schema_arn: core::default::Default::default(),
            schema_name: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
        DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `registry_name` after provisioning.\n"]
    pub fn registry_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registry_name", self.base),
        )
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
#[derive(Serialize)]
pub struct DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_id:
        Option<ListField<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_version_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_version_number: Option<PrimField<f64>>,
}
impl DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    #[doc = "Set the field `schema_id`.\n"]
    pub fn set_schema_id(
        mut self,
        v: impl Into<ListField<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdEl>>,
    ) -> Self {
        self.schema_id = Some(v.into());
        self
    }
    #[doc = "Set the field `schema_version_id`.\n"]
    pub fn set_schema_version_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema_version_id = Some(v.into());
        self
    }
    #[doc = "Set the field `schema_version_number`.\n"]
    pub fn set_schema_version_number(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.schema_version_number = Some(v.into());
        self
    }
}
impl ToListMappable for DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    type O = BlockAssignable<DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {}
impl BuildDataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
        DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl {
            schema_id: core::default::Default::default(),
            schema_version_id: core::default::Default::default(),
            schema_version_number: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
        DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `schema_id` after provisioning.\n"]
    pub fn schema_id(
        &self,
    ) -> ListRef<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElSchemaIdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema_id", self.base))
    }
    #[doc = "Get a reference to the value of field `schema_version_id` after provisioning.\n"]
    pub fn schema_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_version_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schema_version_number` after provisioning.\n"]
    pub fn schema_version_number(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema_version_number", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serialization_library: Option<PrimField<String>>,
}
impl DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
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
impl ToListMappable for DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
    type O = BlockAssignable<DataGlueCatalogTableStorageDescriptorElSerDeInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTableStorageDescriptorElSerDeInfoEl {}
impl BuildDataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
        DataGlueCatalogTableStorageDescriptorElSerDeInfoEl {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
        DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef {
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
pub struct DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_column_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_column_value_location_maps: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_column_values: Option<ListField<PrimField<String>>>,
}
impl DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
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
impl ToListMappable for DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
    type O = BlockAssignable<DataGlueCatalogTableStorageDescriptorElSkewedInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTableStorageDescriptorElSkewedInfoEl {}
impl BuildDataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
        DataGlueCatalogTableStorageDescriptorElSkewedInfoEl {
            skewed_column_names: core::default::Default::default(),
            skewed_column_value_location_maps: core::default::Default::default(),
            skewed_column_values: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
        DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef {
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
pub struct DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<PrimField<f64>>,
}
impl DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
    #[doc = "Set the field `column`.\n"]
    pub fn set_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column = Some(v.into());
        self
    }
    #[doc = "Set the field `sort_order`.\n"]
    pub fn set_sort_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
}
impl ToListMappable for DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
    type O = BlockAssignable<DataGlueCatalogTableStorageDescriptorElSortColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTableStorageDescriptorElSortColumnsEl {}
impl BuildDataGlueCatalogTableStorageDescriptorElSortColumnsEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
        DataGlueCatalogTableStorageDescriptorElSortColumnsEl {
            column: core::default::Default::default(),
            sort_order: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
        DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTableStorageDescriptorElSortColumnsElRef {
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
#[derive(Serialize)]
pub struct DataGlueCatalogTableStorageDescriptorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_locations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_columns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<ListField<DataGlueCatalogTableStorageDescriptorElColumnsEl>>,
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
    schema_reference: Option<ListField<DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ser_de_info: Option<ListField<DataGlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skewed_info: Option<ListField<DataGlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_columns: Option<ListField<DataGlueCatalogTableStorageDescriptorElSortColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stored_as_sub_directories: Option<PrimField<bool>>,
}
impl DataGlueCatalogTableStorageDescriptorEl {
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
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl Into<ListField<DataGlueCatalogTableStorageDescriptorElColumnsEl>>,
    ) -> Self {
        self.columns = Some(v.into());
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
    #[doc = "Set the field `schema_reference`.\n"]
    pub fn set_schema_reference(
        mut self,
        v: impl Into<ListField<DataGlueCatalogTableStorageDescriptorElSchemaReferenceEl>>,
    ) -> Self {
        self.schema_reference = Some(v.into());
        self
    }
    #[doc = "Set the field `ser_de_info`.\n"]
    pub fn set_ser_de_info(
        mut self,
        v: impl Into<ListField<DataGlueCatalogTableStorageDescriptorElSerDeInfoEl>>,
    ) -> Self {
        self.ser_de_info = Some(v.into());
        self
    }
    #[doc = "Set the field `skewed_info`.\n"]
    pub fn set_skewed_info(
        mut self,
        v: impl Into<ListField<DataGlueCatalogTableStorageDescriptorElSkewedInfoEl>>,
    ) -> Self {
        self.skewed_info = Some(v.into());
        self
    }
    #[doc = "Set the field `sort_columns`.\n"]
    pub fn set_sort_columns(
        mut self,
        v: impl Into<ListField<DataGlueCatalogTableStorageDescriptorElSortColumnsEl>>,
    ) -> Self {
        self.sort_columns = Some(v.into());
        self
    }
    #[doc = "Set the field `stored_as_sub_directories`.\n"]
    pub fn set_stored_as_sub_directories(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.stored_as_sub_directories = Some(v.into());
        self
    }
}
impl ToListMappable for DataGlueCatalogTableStorageDescriptorEl {
    type O = BlockAssignable<DataGlueCatalogTableStorageDescriptorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTableStorageDescriptorEl {}
impl BuildDataGlueCatalogTableStorageDescriptorEl {
    pub fn build(self) -> DataGlueCatalogTableStorageDescriptorEl {
        DataGlueCatalogTableStorageDescriptorEl {
            additional_locations: core::default::Default::default(),
            bucket_columns: core::default::Default::default(),
            columns: core::default::Default::default(),
            compressed: core::default::Default::default(),
            input_format: core::default::Default::default(),
            location: core::default::Default::default(),
            number_of_buckets: core::default::Default::default(),
            output_format: core::default::Default::default(),
            parameters: core::default::Default::default(),
            schema_reference: core::default::Default::default(),
            ser_de_info: core::default::Default::default(),
            skewed_info: core::default::Default::default(),
            sort_columns: core::default::Default::default(),
            stored_as_sub_directories: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTableStorageDescriptorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTableStorageDescriptorElRef {
    fn new(shared: StackShared, base: String) -> DataGlueCatalogTableStorageDescriptorElRef {
        DataGlueCatalogTableStorageDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTableStorageDescriptorElRef {
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
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> ListRef<DataGlueCatalogTableStorageDescriptorElColumnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
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
    #[doc = "Get a reference to the value of field `schema_reference` after provisioning.\n"]
    pub fn schema_reference(
        &self,
    ) -> ListRef<DataGlueCatalogTableStorageDescriptorElSchemaReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_reference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ser_de_info` after provisioning.\n"]
    pub fn ser_de_info(&self) -> ListRef<DataGlueCatalogTableStorageDescriptorElSerDeInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ser_de_info", self.base))
    }
    #[doc = "Get a reference to the value of field `skewed_info` after provisioning.\n"]
    pub fn skewed_info(&self) -> ListRef<DataGlueCatalogTableStorageDescriptorElSkewedInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.skewed_info", self.base))
    }
    #[doc = "Get a reference to the value of field `sort_columns` after provisioning.\n"]
    pub fn sort_columns(&self) -> ListRef<DataGlueCatalogTableStorageDescriptorElSortColumnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort_columns", self.base))
    }
    #[doc = "Get a reference to the value of field `stored_as_sub_directories` after provisioning.\n"]
    pub fn stored_as_sub_directories(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stored_as_sub_directories", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGlueCatalogTableTargetTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl DataGlueCatalogTableTargetTableEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_name = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for DataGlueCatalogTableTargetTableEl {
    type O = BlockAssignable<DataGlueCatalogTableTargetTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGlueCatalogTableTargetTableEl {}
impl BuildDataGlueCatalogTableTargetTableEl {
    pub fn build(self) -> DataGlueCatalogTableTargetTableEl {
        DataGlueCatalogTableTargetTableEl {
            catalog_id: core::default::Default::default(),
            database_name: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}
pub struct DataGlueCatalogTableTargetTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGlueCatalogTableTargetTableElRef {
    fn new(shared: StackShared, base: String) -> DataGlueCatalogTableTargetTableElRef {
        DataGlueCatalogTableTargetTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGlueCatalogTableTargetTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.base),
        )
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
