use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DmsS3EndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    add_column_name: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    add_trailing_padding_character: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_folder: Option<PrimField<String>>,
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    canned_acl_for_objects: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_inserts_and_updates: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_inserts_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_max_batch_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_min_file_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdc_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_no_sup_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_null_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    csv_row_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_page_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_partition_delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_partition_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_partition_sequence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_partition_timezone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detach_target_on_lob_lookup_failure_parquet: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dict_page_size_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_statistics: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_mode: Option<PrimField<String>>,
    endpoint_id: PrimField<String>,
    endpoint_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_bucket_owner: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_table_definition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    glue_catalog_generation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_header_rows: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_op_for_full_load: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_file_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parquet_timestamp_in_millisecond: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parquet_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_transactions: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rfc_4180: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_group_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption_kms_key_id: Option<PrimField<String>>,
    service_access_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_column_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_csv_no_sup_value: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_task_start_time_for_full_load_timestamp: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DmsS3EndpointTimeoutsEl>,
}

struct DmsS3Endpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DmsS3EndpointData>,
}

#[derive(Clone)]
pub struct DmsS3Endpoint(Rc<DmsS3Endpoint_>);

impl DmsS3Endpoint {
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

    #[doc = "Set the field `add_column_name`.\n"]
    pub fn set_add_column_name(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().add_column_name = Some(v.into());
        self
    }

    #[doc = "Set the field `add_trailing_padding_character`.\n"]
    pub fn set_add_trailing_padding_character(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().add_trailing_padding_character = Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_folder`.\n"]
    pub fn set_bucket_folder(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bucket_folder = Some(v.into());
        self
    }

    #[doc = "Set the field `canned_acl_for_objects`.\n"]
    pub fn set_canned_acl_for_objects(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().canned_acl_for_objects = Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_inserts_and_updates`.\n"]
    pub fn set_cdc_inserts_and_updates(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cdc_inserts_and_updates = Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_inserts_only`.\n"]
    pub fn set_cdc_inserts_only(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cdc_inserts_only = Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_max_batch_interval`.\n"]
    pub fn set_cdc_max_batch_interval(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cdc_max_batch_interval = Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_min_file_size`.\n"]
    pub fn set_cdc_min_file_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cdc_min_file_size = Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_path`.\n"]
    pub fn set_cdc_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cdc_path = Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_arn`.\n"]
    pub fn set_certificate_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().certificate_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `compression_type`.\n"]
    pub fn set_compression_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compression_type = Some(v.into());
        self
    }

    #[doc = "Set the field `csv_delimiter`.\n"]
    pub fn set_csv_delimiter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().csv_delimiter = Some(v.into());
        self
    }

    #[doc = "Set the field `csv_no_sup_value`.\n"]
    pub fn set_csv_no_sup_value(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().csv_no_sup_value = Some(v.into());
        self
    }

    #[doc = "Set the field `csv_null_value`.\n"]
    pub fn set_csv_null_value(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().csv_null_value = Some(v.into());
        self
    }

    #[doc = "Set the field `csv_row_delimiter`.\n"]
    pub fn set_csv_row_delimiter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().csv_row_delimiter = Some(v.into());
        self
    }

    #[doc = "Set the field `data_format`.\n"]
    pub fn set_data_format(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_format = Some(v.into());
        self
    }

    #[doc = "Set the field `data_page_size`.\n"]
    pub fn set_data_page_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().data_page_size = Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_delimiter`.\n"]
    pub fn set_date_partition_delimiter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().date_partition_delimiter = Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_enabled`.\n"]
    pub fn set_date_partition_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().date_partition_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_sequence`.\n"]
    pub fn set_date_partition_sequence(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().date_partition_sequence = Some(v.into());
        self
    }

    #[doc = "Set the field `date_partition_timezone`.\n"]
    pub fn set_date_partition_timezone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().date_partition_timezone = Some(v.into());
        self
    }

    #[doc = "Set the field `detach_target_on_lob_lookup_failure_parquet`.\n"]
    pub fn set_detach_target_on_lob_lookup_failure_parquet(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().detach_target_on_lob_lookup_failure_parquet = Some(v.into());
        self
    }

    #[doc = "Set the field `dict_page_size_limit`.\n"]
    pub fn set_dict_page_size_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().dict_page_size_limit = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_statistics`.\n"]
    pub fn set_enable_statistics(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_statistics = Some(v.into());
        self
    }

    #[doc = "Set the field `encoding_type`.\n"]
    pub fn set_encoding_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().encoding_type = Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_mode`.\n"]
    pub fn set_encryption_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().encryption_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = Some(v.into());
        self
    }

    #[doc = "Set the field `external_table_definition`.\n"]
    pub fn set_external_table_definition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().external_table_definition = Some(v.into());
        self
    }

    #[doc = "Set the field `glue_catalog_generation`.\n"]
    pub fn set_glue_catalog_generation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().glue_catalog_generation = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_header_rows`.\n"]
    pub fn set_ignore_header_rows(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ignore_header_rows = Some(v.into());
        self
    }

    #[doc = "Set the field `include_op_for_full_load`.\n"]
    pub fn set_include_op_for_full_load(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_op_for_full_load = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `max_file_size`.\n"]
    pub fn set_max_file_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_file_size = Some(v.into());
        self
    }

    #[doc = "Set the field `parquet_timestamp_in_millisecond`.\n"]
    pub fn set_parquet_timestamp_in_millisecond(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().parquet_timestamp_in_millisecond = Some(v.into());
        self
    }

    #[doc = "Set the field `parquet_version`.\n"]
    pub fn set_parquet_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parquet_version = Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_transactions`.\n"]
    pub fn set_preserve_transactions(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().preserve_transactions = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `rfc_4180`.\n"]
    pub fn set_rfc_4180(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().rfc_4180 = Some(v.into());
        self
    }

    #[doc = "Set the field `row_group_length`.\n"]
    pub fn set_row_group_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().row_group_length = Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_encryption_kms_key_id`.\n"]
    pub fn set_server_side_encryption_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().server_side_encryption_kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_mode`.\n"]
    pub fn set_ssl_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ssl_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp_column_name`.\n"]
    pub fn set_timestamp_column_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().timestamp_column_name = Some(v.into());
        self
    }

    #[doc = "Set the field `use_csv_no_sup_value`.\n"]
    pub fn set_use_csv_no_sup_value(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_csv_no_sup_value = Some(v.into());
        self
    }

    #[doc = "Set the field `use_task_start_time_for_full_load_timestamp`.\n"]
    pub fn set_use_task_start_time_for_full_load_timestamp(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_task_start_time_for_full_load_timestamp = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DmsS3EndpointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `add_column_name` after provisioning.\n"]
    pub fn add_column_name(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.add_column_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `add_trailing_padding_character` after provisioning.\n"]
    pub fn add_trailing_padding_character(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.add_trailing_padding_character", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_folder` after provisioning.\n"]
    pub fn bucket_folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `canned_acl_for_objects` after provisioning.\n"]
    pub fn canned_acl_for_objects(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.canned_acl_for_objects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_and_updates` after provisioning.\n"]
    pub fn cdc_inserts_and_updates(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_and_updates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_only` after provisioning.\n"]
    pub fn cdc_inserts_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_max_batch_interval` after provisioning.\n"]
    pub fn cdc_max_batch_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_max_batch_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_min_file_size` after provisioning.\n"]
    pub fn cdc_min_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_min_file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_path` after provisioning.\n"]
    pub fn cdc_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_delimiter` after provisioning.\n"]
    pub fn csv_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_no_sup_value` after provisioning.\n"]
    pub fn csv_no_sup_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_no_sup_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_null_value` after provisioning.\n"]
    pub fn csv_null_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_null_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_row_delimiter` after provisioning.\n"]
    pub fn csv_row_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_row_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_page_size` after provisioning.\n"]
    pub fn data_page_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_page_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_delimiter` after provisioning.\n"]
    pub fn date_partition_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_enabled` after provisioning.\n"]
    pub fn date_partition_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_sequence` after provisioning.\n"]
    pub fn date_partition_sequence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_sequence", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_timezone` after provisioning.\n"]
    pub fn date_partition_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detach_target_on_lob_lookup_failure_parquet` after provisioning.\n"]
    pub fn detach_target_on_lob_lookup_failure_parquet(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detach_target_on_lob_lookup_failure_parquet", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dict_page_size_limit` after provisioning.\n"]
    pub fn dict_page_size_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dict_page_size_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_statistics` after provisioning.\n"]
    pub fn enable_statistics(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encoding_type` after provisioning.\n"]
    pub fn encoding_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_display_name` after provisioning.\n"]
    pub fn engine_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_table_definition` after provisioning.\n"]
    pub fn external_table_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_table_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_catalog_generation` after provisioning.\n"]
    pub fn glue_catalog_generation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.glue_catalog_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_header_rows` after provisioning.\n"]
    pub fn ignore_header_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_header_rows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_op_for_full_load` after provisioning.\n"]
    pub fn include_op_for_full_load(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_op_for_full_load", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parquet_timestamp_in_millisecond` after provisioning.\n"]
    pub fn parquet_timestamp_in_millisecond(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.parquet_timestamp_in_millisecond", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parquet_version` after provisioning.\n"]
    pub fn parquet_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parquet_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_transactions` after provisioning.\n"]
    pub fn preserve_transactions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_transactions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rfc_4180` after provisioning.\n"]
    pub fn rfc_4180(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rfc_4180", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_group_length` after provisioning.\n"]
    pub fn row_group_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.row_group_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_kms_key_id` after provisioning.\n"]
    pub fn server_side_encryption_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_side_encryption_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timestamp_column_name` after provisioning.\n"]
    pub fn timestamp_column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timestamp_column_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_csv_no_sup_value` after provisioning.\n"]
    pub fn use_csv_no_sup_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_csv_no_sup_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_task_start_time_for_full_load_timestamp` after provisioning.\n"]
    pub fn use_task_start_time_for_full_load_timestamp(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_task_start_time_for_full_load_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsS3EndpointTimeoutsElRef {
        DmsS3EndpointTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DmsS3Endpoint {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DmsS3Endpoint { }

impl ToListMappable for DmsS3Endpoint {
    type O = ListRef<DmsS3EndpointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DmsS3Endpoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_dms_s3_endpoint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDmsS3Endpoint {
    pub tf_id: String,
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub endpoint_id: PrimField<String>,
    #[doc = ""]
    pub endpoint_type: PrimField<String>,
    #[doc = ""]
    pub service_access_role_arn: PrimField<String>,
}

impl BuildDmsS3Endpoint {
    pub fn build(self, stack: &mut Stack) -> DmsS3Endpoint {
        let out = DmsS3Endpoint(Rc::new(DmsS3Endpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DmsS3EndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                add_column_name: core::default::Default::default(),
                add_trailing_padding_character: core::default::Default::default(),
                bucket_folder: core::default::Default::default(),
                bucket_name: self.bucket_name,
                canned_acl_for_objects: core::default::Default::default(),
                cdc_inserts_and_updates: core::default::Default::default(),
                cdc_inserts_only: core::default::Default::default(),
                cdc_max_batch_interval: core::default::Default::default(),
                cdc_min_file_size: core::default::Default::default(),
                cdc_path: core::default::Default::default(),
                certificate_arn: core::default::Default::default(),
                compression_type: core::default::Default::default(),
                csv_delimiter: core::default::Default::default(),
                csv_no_sup_value: core::default::Default::default(),
                csv_null_value: core::default::Default::default(),
                csv_row_delimiter: core::default::Default::default(),
                data_format: core::default::Default::default(),
                data_page_size: core::default::Default::default(),
                date_partition_delimiter: core::default::Default::default(),
                date_partition_enabled: core::default::Default::default(),
                date_partition_sequence: core::default::Default::default(),
                date_partition_timezone: core::default::Default::default(),
                detach_target_on_lob_lookup_failure_parquet: core::default::Default::default(),
                dict_page_size_limit: core::default::Default::default(),
                enable_statistics: core::default::Default::default(),
                encoding_type: core::default::Default::default(),
                encryption_mode: core::default::Default::default(),
                endpoint_id: self.endpoint_id,
                endpoint_type: self.endpoint_type,
                expected_bucket_owner: core::default::Default::default(),
                external_table_definition: core::default::Default::default(),
                glue_catalog_generation: core::default::Default::default(),
                id: core::default::Default::default(),
                ignore_header_rows: core::default::Default::default(),
                include_op_for_full_load: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                max_file_size: core::default::Default::default(),
                parquet_timestamp_in_millisecond: core::default::Default::default(),
                parquet_version: core::default::Default::default(),
                preserve_transactions: core::default::Default::default(),
                region: core::default::Default::default(),
                rfc_4180: core::default::Default::default(),
                row_group_length: core::default::Default::default(),
                server_side_encryption_kms_key_id: core::default::Default::default(),
                service_access_role_arn: self.service_access_role_arn,
                ssl_mode: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timestamp_column_name: core::default::Default::default(),
                use_csv_no_sup_value: core::default::Default::default(),
                use_task_start_time_for_full_load_timestamp: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DmsS3EndpointRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsS3EndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DmsS3EndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add_column_name` after provisioning.\n"]
    pub fn add_column_name(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.add_column_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `add_trailing_padding_character` after provisioning.\n"]
    pub fn add_trailing_padding_character(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.add_trailing_padding_character", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_folder` after provisioning.\n"]
    pub fn bucket_folder(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `canned_acl_for_objects` after provisioning.\n"]
    pub fn canned_acl_for_objects(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.canned_acl_for_objects", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_and_updates` after provisioning.\n"]
    pub fn cdc_inserts_and_updates(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_and_updates", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_inserts_only` after provisioning.\n"]
    pub fn cdc_inserts_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_inserts_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_max_batch_interval` after provisioning.\n"]
    pub fn cdc_max_batch_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_max_batch_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_min_file_size` after provisioning.\n"]
    pub fn cdc_min_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_min_file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_path` after provisioning.\n"]
    pub fn cdc_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cdc_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_arn` after provisioning.\n"]
    pub fn certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compression_type` after provisioning.\n"]
    pub fn compression_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compression_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_delimiter` after provisioning.\n"]
    pub fn csv_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_no_sup_value` after provisioning.\n"]
    pub fn csv_no_sup_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_no_sup_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_null_value` after provisioning.\n"]
    pub fn csv_null_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_null_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `csv_row_delimiter` after provisioning.\n"]
    pub fn csv_row_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csv_row_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_format` after provisioning.\n"]
    pub fn data_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_page_size` after provisioning.\n"]
    pub fn data_page_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_page_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_delimiter` after provisioning.\n"]
    pub fn date_partition_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_delimiter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_enabled` after provisioning.\n"]
    pub fn date_partition_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_sequence` after provisioning.\n"]
    pub fn date_partition_sequence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_sequence", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `date_partition_timezone` after provisioning.\n"]
    pub fn date_partition_timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.date_partition_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detach_target_on_lob_lookup_failure_parquet` after provisioning.\n"]
    pub fn detach_target_on_lob_lookup_failure_parquet(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detach_target_on_lob_lookup_failure_parquet", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dict_page_size_limit` after provisioning.\n"]
    pub fn dict_page_size_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.dict_page_size_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_statistics` after provisioning.\n"]
    pub fn enable_statistics(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_statistics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encoding_type` after provisioning.\n"]
    pub fn encoding_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_mode` after provisioning.\n"]
    pub fn encryption_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_display_name` after provisioning.\n"]
    pub fn engine_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_table_definition` after provisioning.\n"]
    pub fn external_table_definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_table_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glue_catalog_generation` after provisioning.\n"]
    pub fn glue_catalog_generation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.glue_catalog_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignore_header_rows` after provisioning.\n"]
    pub fn ignore_header_rows(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_header_rows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_op_for_full_load` after provisioning.\n"]
    pub fn include_op_for_full_load(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_op_for_full_load", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_file_size` after provisioning.\n"]
    pub fn max_file_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_file_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parquet_timestamp_in_millisecond` after provisioning.\n"]
    pub fn parquet_timestamp_in_millisecond(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.parquet_timestamp_in_millisecond", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parquet_version` after provisioning.\n"]
    pub fn parquet_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parquet_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preserve_transactions` after provisioning.\n"]
    pub fn preserve_transactions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_transactions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rfc_4180` after provisioning.\n"]
    pub fn rfc_4180(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.rfc_4180", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `row_group_length` after provisioning.\n"]
    pub fn row_group_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.row_group_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_kms_key_id` after provisioning.\n"]
    pub fn server_side_encryption_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_side_encryption_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_access_role_arn` after provisioning.\n"]
    pub fn service_access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_access_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\n"]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timestamp_column_name` after provisioning.\n"]
    pub fn timestamp_column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timestamp_column_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_csv_no_sup_value` after provisioning.\n"]
    pub fn use_csv_no_sup_value(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_csv_no_sup_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_task_start_time_for_full_load_timestamp` after provisioning.\n"]
    pub fn use_task_start_time_for_full_load_timestamp(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_task_start_time_for_full_load_timestamp", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DmsS3EndpointTimeoutsElRef {
        DmsS3EndpointTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DmsS3EndpointTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl DmsS3EndpointTimeoutsEl {
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

impl ToListMappable for DmsS3EndpointTimeoutsEl {
    type O = BlockAssignable<DmsS3EndpointTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDmsS3EndpointTimeoutsEl {}

impl BuildDmsS3EndpointTimeoutsEl {
    pub fn build(self) -> DmsS3EndpointTimeoutsEl {
        DmsS3EndpointTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct DmsS3EndpointTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DmsS3EndpointTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DmsS3EndpointTimeoutsElRef {
        DmsS3EndpointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DmsS3EndpointTimeoutsElRef {
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
