use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBigqueryTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    dataset_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    table_id: PrimField<String>,
}
struct DataBigqueryTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBigqueryTableData>,
}
#[derive(Clone)]
pub struct DataBigqueryTable(Rc<DataBigqueryTable_>);
impl DataBigqueryTable {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `biglake_configuration` after provisioning.\nSpecifies the configuration of a BigLake managed table."]
    pub fn biglake_configuration(&self) -> ListRef<DataBigqueryTableBiglakeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.biglake_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `clustering` after provisioning.\nSpecifies column names to use for data clustering. Up to four top-level columns are allowed, and should be specified in descending priority order."]
    pub fn clustering(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.clustering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this table was created, in milliseconds since the epoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the instance. When the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the table will fail. When the field is set to false, deleting the table is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe field description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\nSpecifies how the table should be encrypted. If left blank, the table will be encrypted with a Google-managed key; that process is transparent to the user."]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<DataBigqueryTableEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\nThe time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
    pub fn expiration_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_catalog_table_options` after provisioning.\nOptions defining open source compatible table."]
    pub fn external_catalog_table_options(
        &self,
    ) -> ListRef<DataBigqueryTableExternalCatalogTableOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_catalog_table_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_data_configuration` after provisioning.\nDescribes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
    pub fn external_data_configuration(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_data_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the table."]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.friendly_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_schema_columns` after provisioning.\n(Output-only) A list of autogenerated schema fields."]
    pub fn generated_schema_columns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_schema_columns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_auto_generated_schema` after provisioning.\nWhether Terraform will prevent implicitly added columns in schema from showing diff."]
    pub fn ignore_auto_generated_schema(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_auto_generated_schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_schema_changes` after provisioning.\nMention which fields in schema are to be ignored"]
    pub fn ignore_schema_changes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ignore_schema_changes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA mapping of labels to assign to the resource.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this table was last modified, in milliseconds since the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `materialized_view` after provisioning.\nIf specified, configures this table as a materialized view."]
    pub fn materialized_view(&self) -> ListRef<DataBigqueryTableMaterializedViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.materialized_view", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_staleness` after provisioning.\nThe maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of [SQL IntervalValue type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type)."]
    pub fn max_staleness(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_staleness", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_bytes` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn num_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_long_term_bytes` after provisioning.\nThe number of bytes in the table that are considered \"long-term storage\"."]
    pub fn num_long_term_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_long_term_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_rows` after provisioning.\nThe number of rows of data in this table, excluding any data in the streaming buffer."]
    pub fn num_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_rows", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `range_partitioning` after provisioning.\nIf specified, configures range-based partitioning for this table."]
    pub fn range_partitioning(&self) -> ListRef<DataBigqueryTableRangePartitioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.range_partitioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_partition_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\nThe tags attached to this table. Tag keys are globally unique. Tag key is expected to be in the namespaced format, for example \"123456789012/environment\" where 123456789012 is the ID of the parent organization or project resource for this tag key. Tag value is expected to be the short name, for example \"Production\"."]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nA JSON schema for the table."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_foreign_type_info` after provisioning.\nSpecifies metadata of the foreign data type definition in field schema."]
    pub fn schema_foreign_type_info(&self) -> ListRef<DataBigqueryTableSchemaForeignTypeInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_foreign_type_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_constraints` after provisioning.\nDefines the primary key and foreign keys."]
    pub fn table_constraints(&self) -> ListRef<DataBigqueryTableTableConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_constraints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nA unique ID for the resource. Changing this forces a new resource to be created."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_metadata_view` after provisioning.\nView sets the optional parameter \"view\": Specifies the view that determines which table information is returned. By default, basic table information and storage statistics (STORAGE_STATS) are returned. Possible values: TABLE_METADATA_VIEW_UNSPECIFIED, BASIC, STORAGE_STATS, FULL"]
    pub fn table_metadata_view(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_metadata_view", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_replication_info` after provisioning.\nReplication info of a table created using \"AS REPLICA\" DDL like: \"CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv\"."]
    pub fn table_replication_info(&self) -> ListRef<DataBigqueryTableTableReplicationInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_replication_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_partitioning` after provisioning.\nIf specified, configures time-based partitioning for this table."]
    pub fn time_partitioning(&self) -> ListRef<DataBigqueryTableTimePartitioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.time_partitioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nDescribes the table type."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view` after provisioning.\nIf specified, configures this table as a view."]
    pub fn view(&self) -> ListRef<DataBigqueryTableViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.view", self.extract_ref()),
        )
    }
}
impl Referable for DataBigqueryTable {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBigqueryTable {}
impl ToListMappable for DataBigqueryTable {
    type O = ListRef<DataBigqueryTableRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBigqueryTable_ {
    fn extract_datasource_type(&self) -> String {
        "google_bigquery_table".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBigqueryTable {
    pub tf_id: String,
    #[doc = "The dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub dataset_id: PrimField<String>,
    #[doc = "A unique ID for the resource. Changing this forces a new resource to be created."]
    pub table_id: PrimField<String>,
}
impl BuildDataBigqueryTable {
    pub fn build(self, stack: &mut Stack) -> DataBigqueryTable {
        let out = DataBigqueryTable(Rc::new(DataBigqueryTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBigqueryTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                dataset_id: self.dataset_id,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                table_id: self.table_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBigqueryTableRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBigqueryTableRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `biglake_configuration` after provisioning.\nSpecifies the configuration of a BigLake managed table."]
    pub fn biglake_configuration(&self) -> ListRef<DataBigqueryTableBiglakeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.biglake_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `clustering` after provisioning.\nSpecifies column names to use for data clustering. Up to four top-level columns are allowed, and should be specified in descending priority order."]
    pub fn clustering(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.clustering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this table was created, in milliseconds since the epoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID to create the table in. Changing this forces a new resource to be created."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the instance. When the field is set to true or unset in Terraform state, a terraform apply or terraform destroy that would delete the table will fail. When the field is set to false, deleting the table is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe field description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\nSpecifies how the table should be encrypted. If left blank, the table will be encrypted with a Google-managed key; that process is transparent to the user."]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<DataBigqueryTableEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration_time` after provisioning.\nThe time when this table expires, in milliseconds since the epoch. If not present, the table will persist indefinitely. Expired tables will be deleted and their storage reclaimed."]
    pub fn expiration_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_catalog_table_options` after provisioning.\nOptions defining open source compatible table."]
    pub fn external_catalog_table_options(
        &self,
    ) -> ListRef<DataBigqueryTableExternalCatalogTableOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_catalog_table_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_data_configuration` after provisioning.\nDescribes the data format, location, and other properties of a table stored outside of BigQuery. By defining these properties, the data source can then be queried as if it were a standard BigQuery table."]
    pub fn external_data_configuration(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_data_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the table."]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.friendly_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_schema_columns` after provisioning.\n(Output-only) A list of autogenerated schema fields."]
    pub fn generated_schema_columns(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_schema_columns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ignore_auto_generated_schema` after provisioning.\nWhether Terraform will prevent implicitly added columns in schema from showing diff."]
    pub fn ignore_auto_generated_schema(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_auto_generated_schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_schema_changes` after provisioning.\nMention which fields in schema are to be ignored"]
    pub fn ignore_schema_changes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ignore_schema_changes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nA mapping of labels to assign to the resource.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this table was last modified, in milliseconds since the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `materialized_view` after provisioning.\nIf specified, configures this table as a materialized view."]
    pub fn materialized_view(&self) -> ListRef<DataBigqueryTableMaterializedViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.materialized_view", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_staleness` after provisioning.\nThe maximum staleness of data that could be returned when the table (or stale MV) is queried. Staleness encoded as a string encoding of [SQL IntervalValue type](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#interval_type)."]
    pub fn max_staleness(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_staleness", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_bytes` after provisioning.\nThe geographic location where the table resides. This value is inherited from the dataset."]
    pub fn num_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_long_term_bytes` after provisioning.\nThe number of bytes in the table that are considered \"long-term storage\"."]
    pub fn num_long_term_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_long_term_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `num_rows` after provisioning.\nThe number of rows of data in this table, excluding any data in the streaming buffer."]
    pub fn num_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.num_rows", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `range_partitioning` after provisioning.\nIf specified, configures range-based partitioning for this table."]
    pub fn range_partitioning(&self) -> ListRef<DataBigqueryTableRangePartitioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.range_partitioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf set to true, queries over this table require a partition filter that can be used for partition elimination to be specified."]
    pub fn require_partition_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_partition_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\nThe tags attached to this table. Tag keys are globally unique. Tag key is expected to be in the namespaced format, for example \"123456789012/environment\" where 123456789012 is the ID of the parent organization or project resource for this tag key. Tag value is expected to be the short name, for example \"Production\"."]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nA JSON schema for the table."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_foreign_type_info` after provisioning.\nSpecifies metadata of the foreign data type definition in field schema."]
    pub fn schema_foreign_type_info(&self) -> ListRef<DataBigqueryTableSchemaForeignTypeInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_foreign_type_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_constraints` after provisioning.\nDefines the primary key and foreign keys."]
    pub fn table_constraints(&self) -> ListRef<DataBigqueryTableTableConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_constraints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nA unique ID for the resource. Changing this forces a new resource to be created."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_metadata_view` after provisioning.\nView sets the optional parameter \"view\": Specifies the view that determines which table information is returned. By default, basic table information and storage statistics (STORAGE_STATS) are returned. Possible values: TABLE_METADATA_VIEW_UNSPECIFIED, BASIC, STORAGE_STATS, FULL"]
    pub fn table_metadata_view(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_metadata_view", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_replication_info` after provisioning.\nReplication info of a table created using \"AS REPLICA\" DDL like: \"CREATE MATERIALIZED VIEW mv1 AS REPLICA OF src_mv\"."]
    pub fn table_replication_info(&self) -> ListRef<DataBigqueryTableTableReplicationInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_replication_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `time_partitioning` after provisioning.\nIf specified, configures time-based partitioning for this table."]
    pub fn time_partitioning(&self) -> ListRef<DataBigqueryTableTimePartitioningElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.time_partitioning", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nDescribes the table type."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view` after provisioning.\nIf specified, configures this table as a view."]
    pub fn view(&self) -> ListRef<DataBigqueryTableViewElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.view", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableBiglakeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_format: Option<PrimField<String>>,
}
impl DataBigqueryTableBiglakeConfigurationEl {
    #[doc = "Set the field `connection_id`.\n"]
    pub fn set_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `file_format`.\n"]
    pub fn set_file_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_format = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_uri`.\n"]
    pub fn set_storage_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `table_format`.\n"]
    pub fn set_table_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_format = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableBiglakeConfigurationEl {
    type O = BlockAssignable<DataBigqueryTableBiglakeConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableBiglakeConfigurationEl {}
impl BuildDataBigqueryTableBiglakeConfigurationEl {
    pub fn build(self) -> DataBigqueryTableBiglakeConfigurationEl {
        DataBigqueryTableBiglakeConfigurationEl {
            connection_id: core::default::Default::default(),
            file_format: core::default::Default::default(),
            storage_uri: core::default::Default::default(),
            table_format: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableBiglakeConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableBiglakeConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableBiglakeConfigurationElRef {
        DataBigqueryTableBiglakeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableBiglakeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_format` after provisioning.\n"]
    pub fn file_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_format", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_uri` after provisioning.\n"]
    pub fn storage_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `table_format` after provisioning.\n"]
    pub fn table_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_format", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_version: Option<PrimField<String>>,
}
impl DataBigqueryTableEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_version`.\n"]
    pub fn set_kms_key_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_version = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableEncryptionConfigurationEl {
    type O = BlockAssignable<DataBigqueryTableEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableEncryptionConfigurationEl {}
impl BuildDataBigqueryTableEncryptionConfigurationEl {
    pub fn build(self) -> DataBigqueryTableEncryptionConfigurationEl {
        DataBigqueryTableEncryptionConfigurationEl {
            kms_key_name: core::default::Default::default(),
            kms_key_version: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableEncryptionConfigurationElRef {
        DataBigqueryTableEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_version` after provisioning.\n"]
    pub fn kms_key_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serialization_library: Option<PrimField<String>>,
}
impl DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl {
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
impl ToListMappable
    for DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl
{
    type O = BlockAssignable<
        DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl {}
impl BuildDataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl {
    pub fn build(
        self,
    ) -> DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl {
        DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl {
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            serialization_library: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoElRef {
        DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoElRef {
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
pub struct DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serde_info: Option<
        ListField<DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl>,
    >,
}
impl DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl {
    #[doc = "Set the field `input_format`.\n"]
    pub fn set_input_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_format = Some(v.into());
        self
    }
    #[doc = "Set the field `location_uri`.\n"]
    pub fn set_location_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `output_format`.\n"]
    pub fn set_output_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_format = Some(v.into());
        self
    }
    #[doc = "Set the field `serde_info`.\n"]
    pub fn set_serde_info(
        mut self,
        v: impl Into<
            ListField<DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoEl>,
        >,
    ) -> Self {
        self.serde_info = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl {
    type O = BlockAssignable<DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl {}
impl BuildDataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl {
    pub fn build(self) -> DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl {
        DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl {
            input_format: core::default::Default::default(),
            location_uri: core::default::Default::default(),
            output_format: core::default::Default::default(),
            serde_info: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElRef {
        DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `input_format` after provisioning.\n"]
    pub fn input_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }
    #[doc = "Get a reference to the value of field `location_uri` after provisioning.\n"]
    pub fn location_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `output_format` after provisioning.\n"]
    pub fn output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `serde_info` after provisioning.\n"]
    pub fn serde_info(
        &self,
    ) -> ListRef<DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElSerdeInfoElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.serde_info", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalCatalogTableOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_descriptor:
        Option<ListField<DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl>>,
}
impl DataBigqueryTableExternalCatalogTableOptionsEl {
    #[doc = "Set the field `connection_id`.\n"]
    pub fn set_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_descriptor`.\n"]
    pub fn set_storage_descriptor(
        mut self,
        v: impl Into<ListField<DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorEl>>,
    ) -> Self {
        self.storage_descriptor = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalCatalogTableOptionsEl {
    type O = BlockAssignable<DataBigqueryTableExternalCatalogTableOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalCatalogTableOptionsEl {}
impl BuildDataBigqueryTableExternalCatalogTableOptionsEl {
    pub fn build(self) -> DataBigqueryTableExternalCatalogTableOptionsEl {
        DataBigqueryTableExternalCatalogTableOptionsEl {
            connection_id: core::default::Default::default(),
            parameters: core::default::Default::default(),
            storage_descriptor: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalCatalogTableOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalCatalogTableOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableExternalCatalogTableOptionsElRef {
        DataBigqueryTableExternalCatalogTableOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalCatalogTableOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(
        &self,
    ) -> ListRef<DataBigqueryTableExternalCatalogTableOptionsElStorageDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_descriptor", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationElAvroOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    use_avro_logical_types: Option<PrimField<bool>>,
}
impl DataBigqueryTableExternalDataConfigurationElAvroOptionsEl {
    #[doc = "Set the field `use_avro_logical_types`.\n"]
    pub fn set_use_avro_logical_types(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_avro_logical_types = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalDataConfigurationElAvroOptionsEl {
    type O = BlockAssignable<DataBigqueryTableExternalDataConfigurationElAvroOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationElAvroOptionsEl {}
impl BuildDataBigqueryTableExternalDataConfigurationElAvroOptionsEl {
    pub fn build(self) -> DataBigqueryTableExternalDataConfigurationElAvroOptionsEl {
        DataBigqueryTableExternalDataConfigurationElAvroOptionsEl {
            use_avro_logical_types: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElAvroOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalDataConfigurationElAvroOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalDataConfigurationElAvroOptionsElRef {
        DataBigqueryTableExternalDataConfigurationElAvroOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElAvroOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `use_avro_logical_types` after provisioning.\n"]
    pub fn use_avro_logical_types(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_avro_logical_types", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_read_latest: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualifier_encoded: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualifier_string: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl {
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `field_name`.\n"]
    pub fn set_field_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_name = Some(v.into());
        self
    }
    #[doc = "Set the field `only_read_latest`.\n"]
    pub fn set_only_read_latest(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.only_read_latest = Some(v.into());
        self
    }
    #[doc = "Set the field `qualifier_encoded`.\n"]
    pub fn set_qualifier_encoded(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.qualifier_encoded = Some(v.into());
        self
    }
    #[doc = "Set the field `qualifier_string`.\n"]
    pub fn set_qualifier_string(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.qualifier_string = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl
{
    type O = BlockAssignable<
        DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl
{}
impl BuildDataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl {
    pub fn build(
        self,
    ) -> DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl {
        DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl {
            encoding: core::default::Default::default(),
            field_name: core::default::Default::default(),
            only_read_latest: core::default::Default::default(),
            qualifier_encoded: core::default::Default::default(),
            qualifier_string: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnElRef
    {
        DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `field_name` after provisioning.\n"]
    pub fn field_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }
    #[doc = "Get a reference to the value of field `only_read_latest` after provisioning.\n"]
    pub fn only_read_latest(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.only_read_latest", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `qualifier_encoded` after provisioning.\n"]
    pub fn qualifier_encoded(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualifier_encoded", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `qualifier_string` after provisioning.\n"]
    pub fn qualifier_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualifier_string", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column: Option<
        ListField<
            DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    only_read_latest: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl {
    #[doc = "Set the field `column`.\n"]
    pub fn set_column(
        mut self,
        v: impl Into<
            ListField<
                DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnEl,
            >,
        >,
    ) -> Self {
        self.column = Some(v.into());
        self
    }
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `family_id`.\n"]
    pub fn set_family_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.family_id = Some(v.into());
        self
    }
    #[doc = "Set the field `only_read_latest`.\n"]
    pub fn set_only_read_latest(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.only_read_latest = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl
{
    type O = BlockAssignable<
        DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl {}
impl BuildDataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl {
    pub fn build(
        self,
    ) -> DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl {
        DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl {
            column: core::default::Default::default(),
            encoding: core::default::Default::default(),
            family_id: core::default::Default::default(),
            only_read_latest: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElRef {
        DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column` after provisioning.\n"]
    pub fn column(
        &self,
    ) -> ListRef<
        DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElColumnElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.column", self.base))
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `family_id` after provisioning.\n"]
    pub fn family_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family_id", self.base))
    }
    #[doc = "Get a reference to the value of field `only_read_latest` after provisioning.\n"]
    pub fn only_read_latest(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.only_read_latest", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationElBigtableOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_family: Option<
        ListField<DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_unspecified_column_families: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_column_families_as_json: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_rowkey_as_string: Option<PrimField<bool>>,
}
impl DataBigqueryTableExternalDataConfigurationElBigtableOptionsEl {
    #[doc = "Set the field `column_family`.\n"]
    pub fn set_column_family(
        mut self,
        v: impl Into<
            ListField<DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyEl>,
        >,
    ) -> Self {
        self.column_family = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_unspecified_column_families`.\n"]
    pub fn set_ignore_unspecified_column_families(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_unspecified_column_families = Some(v.into());
        self
    }
    #[doc = "Set the field `output_column_families_as_json`.\n"]
    pub fn set_output_column_families_as_json(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.output_column_families_as_json = Some(v.into());
        self
    }
    #[doc = "Set the field `read_rowkey_as_string`.\n"]
    pub fn set_read_rowkey_as_string(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.read_rowkey_as_string = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalDataConfigurationElBigtableOptionsEl {
    type O = BlockAssignable<DataBigqueryTableExternalDataConfigurationElBigtableOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationElBigtableOptionsEl {}
impl BuildDataBigqueryTableExternalDataConfigurationElBigtableOptionsEl {
    pub fn build(self) -> DataBigqueryTableExternalDataConfigurationElBigtableOptionsEl {
        DataBigqueryTableExternalDataConfigurationElBigtableOptionsEl {
            column_family: core::default::Default::default(),
            ignore_unspecified_column_families: core::default::Default::default(),
            output_column_families_as_json: core::default::Default::default(),
            read_rowkey_as_string: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElBigtableOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalDataConfigurationElBigtableOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalDataConfigurationElBigtableOptionsElRef {
        DataBigqueryTableExternalDataConfigurationElBigtableOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElBigtableOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_family` after provisioning.\n"]
    pub fn column_family(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElBigtableOptionsElColumnFamilyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_family", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_unspecified_column_families` after provisioning.\n"]
    pub fn ignore_unspecified_column_families(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_unspecified_column_families", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_column_families_as_json` after provisioning.\n"]
    pub fn output_column_families_as_json(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_column_families_as_json", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `read_rowkey_as_string` after provisioning.\n"]
    pub fn read_rowkey_as_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.read_rowkey_as_string", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationElCsvOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_jagged_rows: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_quoted_newlines: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_leading_rows: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_column_match: Option<PrimField<String>>,
}
impl DataBigqueryTableExternalDataConfigurationElCsvOptionsEl {
    #[doc = "Set the field `allow_jagged_rows`.\n"]
    pub fn set_allow_jagged_rows(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_jagged_rows = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_quoted_newlines`.\n"]
    pub fn set_allow_quoted_newlines(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_quoted_newlines = Some(v.into());
        self
    }
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
    #[doc = "Set the field `field_delimiter`.\n"]
    pub fn set_field_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_delimiter = Some(v.into());
        self
    }
    #[doc = "Set the field `quote`.\n"]
    pub fn set_quote(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quote = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_leading_rows`.\n"]
    pub fn set_skip_leading_rows(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.skip_leading_rows = Some(v.into());
        self
    }
    #[doc = "Set the field `source_column_match`.\n"]
    pub fn set_source_column_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_column_match = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalDataConfigurationElCsvOptionsEl {
    type O = BlockAssignable<DataBigqueryTableExternalDataConfigurationElCsvOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationElCsvOptionsEl {}
impl BuildDataBigqueryTableExternalDataConfigurationElCsvOptionsEl {
    pub fn build(self) -> DataBigqueryTableExternalDataConfigurationElCsvOptionsEl {
        DataBigqueryTableExternalDataConfigurationElCsvOptionsEl {
            allow_jagged_rows: core::default::Default::default(),
            allow_quoted_newlines: core::default::Default::default(),
            encoding: core::default::Default::default(),
            field_delimiter: core::default::Default::default(),
            quote: core::default::Default::default(),
            skip_leading_rows: core::default::Default::default(),
            source_column_match: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElCsvOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalDataConfigurationElCsvOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalDataConfigurationElCsvOptionsElRef {
        DataBigqueryTableExternalDataConfigurationElCsvOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElCsvOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_jagged_rows` after provisioning.\n"]
    pub fn allow_jagged_rows(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_jagged_rows", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_quoted_newlines` after provisioning.\n"]
    pub fn allow_quoted_newlines(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_quoted_newlines", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
    #[doc = "Get a reference to the value of field `field_delimiter` after provisioning.\n"]
    pub fn field_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field_delimiter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `quote` after provisioning.\n"]
    pub fn quote(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quote", self.base))
    }
    #[doc = "Get a reference to the value of field `skip_leading_rows` after provisioning.\n"]
    pub fn skip_leading_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_leading_rows", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_column_match` after provisioning.\n"]
    pub fn source_column_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_column_match", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_leading_rows: Option<PrimField<f64>>,
}
impl DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.range = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_leading_rows`.\n"]
    pub fn set_skip_leading_rows(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.skip_leading_rows = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
    type O = BlockAssignable<DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {}
impl BuildDataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
    pub fn build(self) -> DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
        DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl {
            range: core::default::Default::default(),
            skip_leading_rows: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
        DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }
    #[doc = "Get a reference to the value of field `skip_leading_rows` after provisioning.\n"]
    pub fn skip_leading_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_leading_rows", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_partition_filter: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_uri_prefix: Option<PrimField<String>>,
}
impl DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `require_partition_filter`.\n"]
    pub fn set_require_partition_filter(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_partition_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `source_uri_prefix`.\n"]
    pub fn set_source_uri_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_uri_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
    type O = BlockAssignable<DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {}
impl BuildDataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
    pub fn build(self) -> DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
        DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl {
            mode: core::default::Default::default(),
            require_partition_filter: core::default::Default::default(),
            source_uri_prefix: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
        DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `require_partition_filter` after provisioning.\n"]
    pub fn require_partition_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_partition_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_uri_prefix` after provisioning.\n"]
    pub fn source_uri_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_uri_prefix", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationElJsonOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
}
impl DataBigqueryTableExternalDataConfigurationElJsonOptionsEl {
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalDataConfigurationElJsonOptionsEl {
    type O = BlockAssignable<DataBigqueryTableExternalDataConfigurationElJsonOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationElJsonOptionsEl {}
impl BuildDataBigqueryTableExternalDataConfigurationElJsonOptionsEl {
    pub fn build(self) -> DataBigqueryTableExternalDataConfigurationElJsonOptionsEl {
        DataBigqueryTableExternalDataConfigurationElJsonOptionsEl {
            encoding: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElJsonOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalDataConfigurationElJsonOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalDataConfigurationElJsonOptionsElRef {
        DataBigqueryTableExternalDataConfigurationElJsonOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElJsonOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationElParquetOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_list_inference: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enum_as_string: Option<PrimField<bool>>,
}
impl DataBigqueryTableExternalDataConfigurationElParquetOptionsEl {
    #[doc = "Set the field `enable_list_inference`.\n"]
    pub fn set_enable_list_inference(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_list_inference = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_as_string`.\n"]
    pub fn set_enum_as_string(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enum_as_string = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalDataConfigurationElParquetOptionsEl {
    type O = BlockAssignable<DataBigqueryTableExternalDataConfigurationElParquetOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationElParquetOptionsEl {}
impl BuildDataBigqueryTableExternalDataConfigurationElParquetOptionsEl {
    pub fn build(self) -> DataBigqueryTableExternalDataConfigurationElParquetOptionsEl {
        DataBigqueryTableExternalDataConfigurationElParquetOptionsEl {
            enable_list_inference: core::default::Default::default(),
            enum_as_string: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElParquetOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalDataConfigurationElParquetOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableExternalDataConfigurationElParquetOptionsElRef {
        DataBigqueryTableExternalDataConfigurationElParquetOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElParquetOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_list_inference` after provisioning.\n"]
    pub fn enable_list_inference(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_list_inference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enum_as_string` after provisioning.\n"]
    pub fn enum_as_string(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enum_as_string", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableExternalDataConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autodetect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avro_options: Option<ListField<DataBigqueryTableExternalDataConfigurationElAvroOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigtable_options:
        Option<ListField<DataBigqueryTableExternalDataConfigurationElBigtableOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_options: Option<ListField<DataBigqueryTableExternalDataConfigurationElCsvOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decimal_target_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_set_spec_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_sheets_options:
        Option<ListField<DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hive_partitioning_options:
        Option<ListField<DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_unknown_values: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_extension: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_options: Option<ListField<DataBigqueryTableExternalDataConfigurationElJsonOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_bad_records: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_cache_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object_metadata: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parquet_options:
        Option<ListField<DataBigqueryTableExternalDataConfigurationElParquetOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_file_schema_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_uris: Option<ListField<PrimField<String>>>,
}
impl DataBigqueryTableExternalDataConfigurationEl {
    #[doc = "Set the field `autodetect`.\n"]
    pub fn set_autodetect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.autodetect = Some(v.into());
        self
    }
    #[doc = "Set the field `avro_options`.\n"]
    pub fn set_avro_options(
        mut self,
        v: impl Into<ListField<DataBigqueryTableExternalDataConfigurationElAvroOptionsEl>>,
    ) -> Self {
        self.avro_options = Some(v.into());
        self
    }
    #[doc = "Set the field `bigtable_options`.\n"]
    pub fn set_bigtable_options(
        mut self,
        v: impl Into<ListField<DataBigqueryTableExternalDataConfigurationElBigtableOptionsEl>>,
    ) -> Self {
        self.bigtable_options = Some(v.into());
        self
    }
    #[doc = "Set the field `compression`.\n"]
    pub fn set_compression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compression = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_id`.\n"]
    pub fn set_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `csv_options`.\n"]
    pub fn set_csv_options(
        mut self,
        v: impl Into<ListField<DataBigqueryTableExternalDataConfigurationElCsvOptionsEl>>,
    ) -> Self {
        self.csv_options = Some(v.into());
        self
    }
    #[doc = "Set the field `decimal_target_types`.\n"]
    pub fn set_decimal_target_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.decimal_target_types = Some(v.into());
        self
    }
    #[doc = "Set the field `file_set_spec_type`.\n"]
    pub fn set_file_set_spec_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_set_spec_type = Some(v.into());
        self
    }
    #[doc = "Set the field `google_sheets_options`.\n"]
    pub fn set_google_sheets_options(
        mut self,
        v: impl Into<ListField<DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsEl>>,
    ) -> Self {
        self.google_sheets_options = Some(v.into());
        self
    }
    #[doc = "Set the field `hive_partitioning_options`.\n"]
    pub fn set_hive_partitioning_options(
        mut self,
        v: impl Into<ListField<DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsEl>>,
    ) -> Self {
        self.hive_partitioning_options = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_unknown_values`.\n"]
    pub fn set_ignore_unknown_values(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_unknown_values = Some(v.into());
        self
    }
    #[doc = "Set the field `json_extension`.\n"]
    pub fn set_json_extension(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json_extension = Some(v.into());
        self
    }
    #[doc = "Set the field `json_options`.\n"]
    pub fn set_json_options(
        mut self,
        v: impl Into<ListField<DataBigqueryTableExternalDataConfigurationElJsonOptionsEl>>,
    ) -> Self {
        self.json_options = Some(v.into());
        self
    }
    #[doc = "Set the field `max_bad_records`.\n"]
    pub fn set_max_bad_records(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_bad_records = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_cache_mode`.\n"]
    pub fn set_metadata_cache_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata_cache_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `object_metadata`.\n"]
    pub fn set_object_metadata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object_metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `parquet_options`.\n"]
    pub fn set_parquet_options(
        mut self,
        v: impl Into<ListField<DataBigqueryTableExternalDataConfigurationElParquetOptionsEl>>,
    ) -> Self {
        self.parquet_options = Some(v.into());
        self
    }
    #[doc = "Set the field `reference_file_schema_uri`.\n"]
    pub fn set_reference_file_schema_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reference_file_schema_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema = Some(v.into());
        self
    }
    #[doc = "Set the field `source_format`.\n"]
    pub fn set_source_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_format = Some(v.into());
        self
    }
    #[doc = "Set the field `source_uris`.\n"]
    pub fn set_source_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.source_uris = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableExternalDataConfigurationEl {
    type O = BlockAssignable<DataBigqueryTableExternalDataConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableExternalDataConfigurationEl {}
impl BuildDataBigqueryTableExternalDataConfigurationEl {
    pub fn build(self) -> DataBigqueryTableExternalDataConfigurationEl {
        DataBigqueryTableExternalDataConfigurationEl {
            autodetect: core::default::Default::default(),
            avro_options: core::default::Default::default(),
            bigtable_options: core::default::Default::default(),
            compression: core::default::Default::default(),
            connection_id: core::default::Default::default(),
            csv_options: core::default::Default::default(),
            decimal_target_types: core::default::Default::default(),
            file_set_spec_type: core::default::Default::default(),
            google_sheets_options: core::default::Default::default(),
            hive_partitioning_options: core::default::Default::default(),
            ignore_unknown_values: core::default::Default::default(),
            json_extension: core::default::Default::default(),
            json_options: core::default::Default::default(),
            max_bad_records: core::default::Default::default(),
            metadata_cache_mode: core::default::Default::default(),
            object_metadata: core::default::Default::default(),
            parquet_options: core::default::Default::default(),
            reference_file_schema_uri: core::default::Default::default(),
            schema: core::default::Default::default(),
            source_format: core::default::Default::default(),
            source_uris: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableExternalDataConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableExternalDataConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableExternalDataConfigurationElRef {
        DataBigqueryTableExternalDataConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableExternalDataConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `autodetect` after provisioning.\n"]
    pub fn autodetect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.autodetect", self.base))
    }
    #[doc = "Get a reference to the value of field `avro_options` after provisioning.\n"]
    pub fn avro_options(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElAvroOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.avro_options", self.base))
    }
    #[doc = "Get a reference to the value of field `bigtable_options` after provisioning.\n"]
    pub fn bigtable_options(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElBigtableOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigtable_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }
    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `csv_options` after provisioning.\n"]
    pub fn csv_options(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElCsvOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.csv_options", self.base))
    }
    #[doc = "Get a reference to the value of field `decimal_target_types` after provisioning.\n"]
    pub fn decimal_target_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.decimal_target_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_set_spec_type` after provisioning.\n"]
    pub fn file_set_spec_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_set_spec_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_sheets_options` after provisioning.\n"]
    pub fn google_sheets_options(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElGoogleSheetsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_sheets_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hive_partitioning_options` after provisioning.\n"]
    pub fn hive_partitioning_options(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElHivePartitioningOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hive_partitioning_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_unknown_values` after provisioning.\n"]
    pub fn ignore_unknown_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_unknown_values", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `json_extension` after provisioning.\n"]
    pub fn json_extension(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.json_extension", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `json_options` after provisioning.\n"]
    pub fn json_options(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElJsonOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json_options", self.base))
    }
    #[doc = "Get a reference to the value of field `max_bad_records` after provisioning.\n"]
    pub fn max_bad_records(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_bad_records", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_cache_mode` after provisioning.\n"]
    pub fn metadata_cache_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_cache_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `object_metadata` after provisioning.\n"]
    pub fn object_metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.object_metadata", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parquet_options` after provisioning.\n"]
    pub fn parquet_options(
        &self,
    ) -> ListRef<DataBigqueryTableExternalDataConfigurationElParquetOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parquet_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reference_file_schema_uri` after provisioning.\n"]
    pub fn reference_file_schema_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reference_file_schema_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `source_format` after provisioning.\n"]
    pub fn source_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_format", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_uris` after provisioning.\n"]
    pub fn source_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.source_uris", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableMaterializedViewEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_non_incremental_definition: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_refresh: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_interval_ms: Option<PrimField<f64>>,
}
impl DataBigqueryTableMaterializedViewEl {
    #[doc = "Set the field `allow_non_incremental_definition`.\n"]
    pub fn set_allow_non_incremental_definition(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_non_incremental_definition = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_refresh`.\n"]
    pub fn set_enable_refresh(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_refresh = Some(v.into());
        self
    }
    #[doc = "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query = Some(v.into());
        self
    }
    #[doc = "Set the field `refresh_interval_ms`.\n"]
    pub fn set_refresh_interval_ms(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.refresh_interval_ms = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableMaterializedViewEl {
    type O = BlockAssignable<DataBigqueryTableMaterializedViewEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableMaterializedViewEl {}
impl BuildDataBigqueryTableMaterializedViewEl {
    pub fn build(self) -> DataBigqueryTableMaterializedViewEl {
        DataBigqueryTableMaterializedViewEl {
            allow_non_incremental_definition: core::default::Default::default(),
            enable_refresh: core::default::Default::default(),
            query: core::default::Default::default(),
            refresh_interval_ms: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableMaterializedViewElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableMaterializedViewElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableMaterializedViewElRef {
        DataBigqueryTableMaterializedViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableMaterializedViewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_non_incremental_definition` after provisioning.\n"]
    pub fn allow_non_incremental_definition(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_non_incremental_definition", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_refresh` after provisioning.\n"]
    pub fn enable_refresh(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_refresh", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
    #[doc = "Get a reference to the value of field `refresh_interval_ms` after provisioning.\n"]
    pub fn refresh_interval_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.refresh_interval_ms", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableRangePartitioningElRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<f64>>,
}
impl DataBigqueryTableRangePartitioningElRangeEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.end = Some(v.into());
        self
    }
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableRangePartitioningElRangeEl {
    type O = BlockAssignable<DataBigqueryTableRangePartitioningElRangeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableRangePartitioningElRangeEl {}
impl BuildDataBigqueryTableRangePartitioningElRangeEl {
    pub fn build(self) -> DataBigqueryTableRangePartitioningElRangeEl {
        DataBigqueryTableRangePartitioningElRangeEl {
            end: core::default::Default::default(),
            interval: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableRangePartitioningElRangeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableRangePartitioningElRangeElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableRangePartitioningElRangeElRef {
        DataBigqueryTableRangePartitioningElRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableRangePartitioningElRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableRangePartitioningEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<ListField<DataBigqueryTableRangePartitioningElRangeEl>>,
}
impl DataBigqueryTableRangePartitioningEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl Into<ListField<DataBigqueryTableRangePartitioningElRangeEl>>,
    ) -> Self {
        self.range = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableRangePartitioningEl {
    type O = BlockAssignable<DataBigqueryTableRangePartitioningEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableRangePartitioningEl {}
impl BuildDataBigqueryTableRangePartitioningEl {
    pub fn build(self) -> DataBigqueryTableRangePartitioningEl {
        DataBigqueryTableRangePartitioningEl {
            field: core::default::Default::default(),
            range: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableRangePartitioningElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableRangePartitioningElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableRangePartitioningElRef {
        DataBigqueryTableRangePartitioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableRangePartitioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> ListRef<DataBigqueryTableRangePartitioningElRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableSchemaForeignTypeInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    type_system: Option<PrimField<String>>,
}
impl DataBigqueryTableSchemaForeignTypeInfoEl {
    #[doc = "Set the field `type_system`.\n"]
    pub fn set_type_system(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_system = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableSchemaForeignTypeInfoEl {
    type O = BlockAssignable<DataBigqueryTableSchemaForeignTypeInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableSchemaForeignTypeInfoEl {}
impl BuildDataBigqueryTableSchemaForeignTypeInfoEl {
    pub fn build(self) -> DataBigqueryTableSchemaForeignTypeInfoEl {
        DataBigqueryTableSchemaForeignTypeInfoEl {
            type_system: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableSchemaForeignTypeInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableSchemaForeignTypeInfoElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableSchemaForeignTypeInfoElRef {
        DataBigqueryTableSchemaForeignTypeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableSchemaForeignTypeInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_system` after provisioning.\n"]
    pub fn type_system(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type_system", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    referenced_column: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referencing_column: Option<PrimField<String>>,
}
impl DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
    #[doc = "Set the field `referenced_column`.\n"]
    pub fn set_referenced_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.referenced_column = Some(v.into());
        self
    }
    #[doc = "Set the field `referencing_column`.\n"]
    pub fn set_referencing_column(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.referencing_column = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
    type O = BlockAssignable<DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {}
impl BuildDataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
    pub fn build(self) -> DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
        DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl {
            referenced_column: core::default::Default::default(),
            referencing_column: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
        DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `referenced_column` after provisioning.\n"]
    pub fn referenced_column(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.referenced_column", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `referencing_column` after provisioning.\n"]
    pub fn referencing_column(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.referencing_column", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_id: Option<PrimField<String>>,
}
impl DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
    #[doc = "Set the field `dataset_id`.\n"]
    pub fn set_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `table_id`.\n"]
    pub fn set_table_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
    type O = BlockAssignable<DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {}
impl BuildDataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
    pub fn build(self) -> DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
        DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
        DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\n"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\n"]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableTableConstraintsElForeignKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_references:
        Option<ListField<DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referenced_table:
        Option<ListField<DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl>>,
}
impl DataBigqueryTableTableConstraintsElForeignKeysEl {
    #[doc = "Set the field `column_references`.\n"]
    pub fn set_column_references(
        mut self,
        v: impl Into<ListField<DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesEl>>,
    ) -> Self {
        self.column_references = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `referenced_table`.\n"]
    pub fn set_referenced_table(
        mut self,
        v: impl Into<ListField<DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableEl>>,
    ) -> Self {
        self.referenced_table = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableTableConstraintsElForeignKeysEl {
    type O = BlockAssignable<DataBigqueryTableTableConstraintsElForeignKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableTableConstraintsElForeignKeysEl {}
impl BuildDataBigqueryTableTableConstraintsElForeignKeysEl {
    pub fn build(self) -> DataBigqueryTableTableConstraintsElForeignKeysEl {
        DataBigqueryTableTableConstraintsElForeignKeysEl {
            column_references: core::default::Default::default(),
            name: core::default::Default::default(),
            referenced_table: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableTableConstraintsElForeignKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableTableConstraintsElForeignKeysElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableTableConstraintsElForeignKeysElRef {
        DataBigqueryTableTableConstraintsElForeignKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableTableConstraintsElForeignKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_references` after provisioning.\n"]
    pub fn column_references(
        &self,
    ) -> ListRef<DataBigqueryTableTableConstraintsElForeignKeysElColumnReferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_references", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `referenced_table` after provisioning.\n"]
    pub fn referenced_table(
        &self,
    ) -> ListRef<DataBigqueryTableTableConstraintsElForeignKeysElReferencedTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.referenced_table", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableTableConstraintsElPrimaryKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<ListField<PrimField<String>>>,
}
impl DataBigqueryTableTableConstraintsElPrimaryKeyEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.columns = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableTableConstraintsElPrimaryKeyEl {
    type O = BlockAssignable<DataBigqueryTableTableConstraintsElPrimaryKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableTableConstraintsElPrimaryKeyEl {}
impl BuildDataBigqueryTableTableConstraintsElPrimaryKeyEl {
    pub fn build(self) -> DataBigqueryTableTableConstraintsElPrimaryKeyEl {
        DataBigqueryTableTableConstraintsElPrimaryKeyEl {
            columns: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableTableConstraintsElPrimaryKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableTableConstraintsElPrimaryKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBigqueryTableTableConstraintsElPrimaryKeyElRef {
        DataBigqueryTableTableConstraintsElPrimaryKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableTableConstraintsElPrimaryKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableTableConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    foreign_keys: Option<ListField<DataBigqueryTableTableConstraintsElForeignKeysEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key: Option<ListField<DataBigqueryTableTableConstraintsElPrimaryKeyEl>>,
}
impl DataBigqueryTableTableConstraintsEl {
    #[doc = "Set the field `foreign_keys`.\n"]
    pub fn set_foreign_keys(
        mut self,
        v: impl Into<ListField<DataBigqueryTableTableConstraintsElForeignKeysEl>>,
    ) -> Self {
        self.foreign_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `primary_key`.\n"]
    pub fn set_primary_key(
        mut self,
        v: impl Into<ListField<DataBigqueryTableTableConstraintsElPrimaryKeyEl>>,
    ) -> Self {
        self.primary_key = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableTableConstraintsEl {
    type O = BlockAssignable<DataBigqueryTableTableConstraintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableTableConstraintsEl {}
impl BuildDataBigqueryTableTableConstraintsEl {
    pub fn build(self) -> DataBigqueryTableTableConstraintsEl {
        DataBigqueryTableTableConstraintsEl {
            foreign_keys: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableTableConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableTableConstraintsElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableTableConstraintsElRef {
        DataBigqueryTableTableConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableTableConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `foreign_keys` after provisioning.\n"]
    pub fn foreign_keys(&self) -> ListRef<DataBigqueryTableTableConstraintsElForeignKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.foreign_keys", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_key` after provisioning.\n"]
    pub fn primary_key(&self) -> ListRef<DataBigqueryTableTableConstraintsElPrimaryKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableTableReplicationInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    replication_interval_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_dataset_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_table_id: Option<PrimField<String>>,
}
impl DataBigqueryTableTableReplicationInfoEl {
    #[doc = "Set the field `replication_interval_ms`.\n"]
    pub fn set_replication_interval_ms(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.replication_interval_ms = Some(v.into());
        self
    }
    #[doc = "Set the field `source_dataset_id`.\n"]
    pub fn set_source_dataset_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_dataset_id = Some(v.into());
        self
    }
    #[doc = "Set the field `source_project_id`.\n"]
    pub fn set_source_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `source_table_id`.\n"]
    pub fn set_source_table_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_table_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableTableReplicationInfoEl {
    type O = BlockAssignable<DataBigqueryTableTableReplicationInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableTableReplicationInfoEl {}
impl BuildDataBigqueryTableTableReplicationInfoEl {
    pub fn build(self) -> DataBigqueryTableTableReplicationInfoEl {
        DataBigqueryTableTableReplicationInfoEl {
            replication_interval_ms: core::default::Default::default(),
            source_dataset_id: core::default::Default::default(),
            source_project_id: core::default::Default::default(),
            source_table_id: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableTableReplicationInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableTableReplicationInfoElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableTableReplicationInfoElRef {
        DataBigqueryTableTableReplicationInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableTableReplicationInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `replication_interval_ms` after provisioning.\n"]
    pub fn replication_interval_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.replication_interval_ms", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_dataset_id` after provisioning.\n"]
    pub fn source_dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_dataset_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_project_id` after provisioning.\n"]
    pub fn source_project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_project_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_table_id` after provisioning.\n"]
    pub fn source_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_table_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableTimePartitioningEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_partition_filter: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataBigqueryTableTimePartitioningEl {
    #[doc = "Set the field `expiration_ms`.\n"]
    pub fn set_expiration_ms(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.expiration_ms = Some(v.into());
        self
    }
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field = Some(v.into());
        self
    }
    #[doc = "Set the field `require_partition_filter`.\n"]
    pub fn set_require_partition_filter(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_partition_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableTimePartitioningEl {
    type O = BlockAssignable<DataBigqueryTableTimePartitioningEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableTimePartitioningEl {}
impl BuildDataBigqueryTableTimePartitioningEl {
    pub fn build(self) -> DataBigqueryTableTimePartitioningEl {
        DataBigqueryTableTimePartitioningEl {
            expiration_ms: core::default::Default::default(),
            field: core::default::Default::default(),
            require_partition_filter: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableTimePartitioningElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableTimePartitioningElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableTimePartitioningElRef {
        DataBigqueryTableTimePartitioningElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableTimePartitioningElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expiration_ms` after provisioning.\n"]
    pub fn expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_ms", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `require_partition_filter` after provisioning.\n"]
    pub fn require_partition_filter(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_partition_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBigqueryTableViewEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_legacy_sql: Option<PrimField<bool>>,
}
impl DataBigqueryTableViewEl {
    #[doc = "Set the field `query`.\n"]
    pub fn set_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query = Some(v.into());
        self
    }
    #[doc = "Set the field `use_legacy_sql`.\n"]
    pub fn set_use_legacy_sql(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_legacy_sql = Some(v.into());
        self
    }
}
impl ToListMappable for DataBigqueryTableViewEl {
    type O = BlockAssignable<DataBigqueryTableViewEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBigqueryTableViewEl {}
impl BuildDataBigqueryTableViewEl {
    pub fn build(self) -> DataBigqueryTableViewEl {
        DataBigqueryTableViewEl {
            query: core::default::Default::default(),
            use_legacy_sql: core::default::Default::default(),
        }
    }
}
pub struct DataBigqueryTableViewElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBigqueryTableViewElRef {
    fn new(shared: StackShared, base: String) -> DataBigqueryTableViewElRef {
        DataBigqueryTableViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBigqueryTableViewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
    #[doc = "Get a reference to the value of field `use_legacy_sql` after provisioning.\n"]
    pub fn use_legacy_sql(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_legacy_sql", self.base),
        )
    }
}
