use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryDatasetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    dataset_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_collation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_partition_expiration_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_table_expiration_ms: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_contents_on_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    friendly_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_case_insensitive: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_time_travel_hours: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_billing_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access: Option<Vec<BigqueryDatasetAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_encryption_configuration: Option<Vec<BigqueryDatasetDefaultEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_catalog_dataset_options: Option<Vec<BigqueryDatasetExternalCatalogDatasetOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_dataset_reference: Option<Vec<BigqueryDatasetExternalDatasetReferenceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryDatasetTimeoutsEl>,
    dynamic: BigqueryDatasetDynamic,
}
struct BigqueryDataset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryDatasetData>,
}
#[derive(Clone)]
pub struct BigqueryDataset(Rc<BigqueryDataset_>);
impl BigqueryDataset {
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
    #[doc = "Set the field `default_collation`.\nDefines the default collation specification of future tables created\nin the dataset. If a table is created in this dataset without table-level\ndefault collation, then the table inherits the dataset default collation,\nwhich is applied to the string fields that do not have explicit collation\nspecified. A change to this field affects only tables created afterwards,\nand does not alter the existing tables.\n\nThe following values are supported:\n- 'und:ci': undetermined locale, case insensitive.\n- '': empty string. Default to case-sensitive behavior."]
    pub fn set_default_collation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_collation = Some(v.into());
        self
    }
    #[doc = "Set the field `default_partition_expiration_ms`.\nThe default partition expiration for all partitioned tables in\nthe dataset, in milliseconds.\nOnce this property is set, all newly-created partitioned tables in\nthe dataset will have an 'expirationMs' property in the 'timePartitioning'\nsettings set to this value, and changing the value will only\naffect new tables, not existing ones. The storage in a partition will\nhave an expiration time of its partition time plus this value.\nSetting this property overrides the use of 'defaultTableExpirationMs'\nfor partitioned tables: only one of 'defaultTableExpirationMs' and\n'defaultPartitionExpirationMs' will be used for any new partitioned\ntable. If you provide an explicit 'timePartitioning.expirationMs' when\ncreating or updating a partitioned table, that value takes precedence\nover the default partition expiration time indicated by this property."]
    pub fn set_default_partition_expiration_ms(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_partition_expiration_ms = Some(v.into());
        self
    }
    #[doc = "Set the field `default_table_expiration_ms`.\nThe default lifetime of all tables in the dataset, in milliseconds.\nThe minimum value is 3600000 milliseconds (one hour).\nOnce this property is set, all newly-created tables in the dataset\nwill have an 'expirationTime' property set to the creation time plus\nthe value in this property, and changing the value will only affect\nnew tables, not existing ones. When the 'expirationTime' for a given\ntable is reached, that table will be deleted automatically.\nIf a table's 'expirationTime' is modified or removed before the\ntable expires, or if you provide an explicit 'expirationTime' when\ncreating a table, that value takes precedence over the default\nexpiration time indicated by this property."]
    pub fn set_default_table_expiration_ms(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_table_expiration_ms = Some(v.into());
        self
    }
    #[doc = "Set the field `delete_contents_on_destroy`.\nIf set to 'true', delete all the tables in the\ndataset when destroying the resource; otherwise,\ndestroying the resource will fail if tables are present."]
    pub fn set_delete_contents_on_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_contents_on_destroy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA user-friendly description of the dataset"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `friendly_name`.\nA descriptive name for the dataset"]
    pub fn set_friendly_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().friendly_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `is_case_insensitive`.\nTRUE if the dataset and its table names are case-insensitive, otherwise FALSE.\nBy default, this is FALSE, which means the dataset and its table names are\ncase-sensitive. This field does not affect routine references."]
    pub fn set_is_case_insensitive(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_case_insensitive = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe geographic location where the dataset should reside.\nSee [official docs](https://cloud.google.com/bigquery/docs/dataset-locations).\nThere are two types of locations, regional or multi-regional. A regional\nlocation is a specific geographic place, such as Tokyo, and a multi-regional\nlocation is a large geographic area, such as the United States, that\ncontains at least two geographic places.\nThe default value is multi-regional location 'US'.\nChanging this forces a new resource to be created."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `max_time_travel_hours`.\nDefines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days)."]
    pub fn set_max_time_travel_hours(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().max_time_travel_hours = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_tags`.\nThe tags attached to this table. Tag keys are globally unique. Tag key is expected to be\nin the namespaced format, for example \"123456789012/environment\" where 123456789012 is the\nID of the parent organization or project resource for this tag key. Tag value is expected\nto be the short name, for example \"Production\". See [Tag definitions](https://cloud.google.com/iam/docs/tags-access-control#definitions)\nfor more details."]
    pub fn set_resource_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_billing_model`.\nSpecifies the storage billing model for the dataset.\nSet this flag value to LOGICAL to use logical bytes for storage billing,\nor to PHYSICAL to use physical bytes instead.\n\nLOGICAL is the default if this flag isn't specified."]
    pub fn set_storage_billing_model(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_billing_model = Some(v.into());
        self
    }
    #[doc = "Set the field `access`.\n"]
    pub fn set_access(self, v: impl Into<BlockAssignable<BigqueryDatasetAccessEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_encryption_configuration`.\n"]
    pub fn set_default_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<BigqueryDatasetDefaultEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .default_encryption_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `external_catalog_dataset_options`.\n"]
    pub fn set_external_catalog_dataset_options(
        self,
        v: impl Into<BlockAssignable<BigqueryDatasetExternalCatalogDatasetOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_catalog_dataset_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .external_catalog_dataset_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `external_dataset_reference`.\n"]
    pub fn set_external_dataset_reference(
        self,
        v: impl Into<BlockAssignable<BigqueryDatasetExternalDatasetReferenceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().external_dataset_reference = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.external_dataset_reference = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryDatasetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this dataset was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_collation` after provisioning.\nDefines the default collation specification of future tables created\nin the dataset. If a table is created in this dataset without table-level\ndefault collation, then the table inherits the dataset default collation,\nwhich is applied to the string fields that do not have explicit collation\nspecified. A change to this field affects only tables created afterwards,\nand does not alter the existing tables.\n\nThe following values are supported:\n- 'und:ci': undetermined locale, case insensitive.\n- '': empty string. Default to case-sensitive behavior."]
    pub fn default_collation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_collation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_partition_expiration_ms` after provisioning.\nThe default partition expiration for all partitioned tables in\nthe dataset, in milliseconds.\nOnce this property is set, all newly-created partitioned tables in\nthe dataset will have an 'expirationMs' property in the 'timePartitioning'\nsettings set to this value, and changing the value will only\naffect new tables, not existing ones. The storage in a partition will\nhave an expiration time of its partition time plus this value.\nSetting this property overrides the use of 'defaultTableExpirationMs'\nfor partitioned tables: only one of 'defaultTableExpirationMs' and\n'defaultPartitionExpirationMs' will be used for any new partitioned\ntable. If you provide an explicit 'timePartitioning.expirationMs' when\ncreating or updating a partitioned table, that value takes precedence\nover the default partition expiration time indicated by this property."]
    pub fn default_partition_expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_partition_expiration_ms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_table_expiration_ms` after provisioning.\nThe default lifetime of all tables in the dataset, in milliseconds.\nThe minimum value is 3600000 milliseconds (one hour).\nOnce this property is set, all newly-created tables in the dataset\nwill have an 'expirationTime' property set to the creation time plus\nthe value in this property, and changing the value will only affect\nnew tables, not existing ones. When the 'expirationTime' for a given\ntable is reached, that table will be deleted automatically.\nIf a table's 'expirationTime' is modified or removed before the\ntable expires, or if you provide an explicit 'expirationTime' when\ncreating a table, that value takes precedence over the default\nexpiration time indicated by this property."]
    pub fn default_table_expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_table_expiration_ms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_contents_on_destroy` after provisioning.\nIf set to 'true', delete all the tables in the\ndataset when destroying the resource; otherwise,\ndestroying the resource will fail if tables are present."]
    pub fn delete_contents_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_contents_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-friendly description of the dataset"]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the dataset"]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.friendly_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_case_insensitive` after provisioning.\nTRUE if the dataset and its table names are case-insensitive, otherwise FALSE.\nBy default, this is FALSE, which means the dataset and its table names are\ncase-sensitive. This field does not affect routine references."]
    pub fn is_case_insensitive(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_case_insensitive", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\nThe date when this dataset or any of its tables was last modified, in\nmilliseconds since the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee [official docs](https://cloud.google.com/bigquery/docs/dataset-locations).\nThere are two types of locations, regional or multi-regional. A regional\nlocation is a specific geographic place, such as Tokyo, and a multi-regional\nlocation is a large geographic area, such as the United States, that\ncontains at least two geographic places.\nThe default value is multi-regional location 'US'.\nChanging this forces a new resource to be created."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_time_travel_hours` after provisioning.\nDefines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days)."]
    pub fn max_time_travel_hours(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_time_travel_hours", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\nThe tags attached to this table. Tag keys are globally unique. Tag key is expected to be\nin the namespaced format, for example \"123456789012/environment\" where 123456789012 is the\nID of the parent organization or project resource for this tag key. Tag value is expected\nto be the short name, for example \"Production\". See [Tag definitions](https://cloud.google.com/iam/docs/tags-access-control#definitions)\nfor more details."]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_billing_model` after provisioning.\nSpecifies the storage billing model for the dataset.\nSet this flag value to LOGICAL to use logical bytes for storage billing,\nor to PHYSICAL to use physical bytes instead.\n\nLOGICAL is the default if this flag isn't specified."]
    pub fn storage_billing_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_billing_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_encryption_configuration` after provisioning.\n"]
    pub fn default_encryption_configuration(
        &self,
    ) -> ListRef<BigqueryDatasetDefaultEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_catalog_dataset_options` after provisioning.\n"]
    pub fn external_catalog_dataset_options(
        &self,
    ) -> ListRef<BigqueryDatasetExternalCatalogDatasetOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_catalog_dataset_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_dataset_reference` after provisioning.\n"]
    pub fn external_dataset_reference(
        &self,
    ) -> ListRef<BigqueryDatasetExternalDatasetReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_dataset_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatasetTimeoutsElRef {
        BigqueryDatasetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryDataset {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryDataset {}
impl ToListMappable for BigqueryDataset {
    type O = ListRef<BigqueryDatasetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryDataset_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_dataset".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryDataset {
    pub tf_id: String,
    #[doc = "A unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub dataset_id: PrimField<String>,
}
impl BuildBigqueryDataset {
    pub fn build(self, stack: &mut Stack) -> BigqueryDataset {
        let out = BigqueryDataset(Rc::new(BigqueryDataset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryDatasetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                dataset_id: self.dataset_id,
                default_collation: core::default::Default::default(),
                default_partition_expiration_ms: core::default::Default::default(),
                default_table_expiration_ms: core::default::Default::default(),
                delete_contents_on_destroy: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                friendly_name: core::default::Default::default(),
                id: core::default::Default::default(),
                is_case_insensitive: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                max_time_travel_hours: core::default::Default::default(),
                project: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
                storage_billing_model: core::default::Default::default(),
                access: core::default::Default::default(),
                default_encryption_configuration: core::default::Default::default(),
                external_catalog_dataset_options: core::default::Default::default(),
                external_dataset_reference: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryDatasetRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryDatasetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this dataset was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID\nmust contain only letters (a-z, A-Z), numbers (0-9), or\nunderscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_collation` after provisioning.\nDefines the default collation specification of future tables created\nin the dataset. If a table is created in this dataset without table-level\ndefault collation, then the table inherits the dataset default collation,\nwhich is applied to the string fields that do not have explicit collation\nspecified. A change to this field affects only tables created afterwards,\nand does not alter the existing tables.\n\nThe following values are supported:\n- 'und:ci': undetermined locale, case insensitive.\n- '': empty string. Default to case-sensitive behavior."]
    pub fn default_collation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_collation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_partition_expiration_ms` after provisioning.\nThe default partition expiration for all partitioned tables in\nthe dataset, in milliseconds.\nOnce this property is set, all newly-created partitioned tables in\nthe dataset will have an 'expirationMs' property in the 'timePartitioning'\nsettings set to this value, and changing the value will only\naffect new tables, not existing ones. The storage in a partition will\nhave an expiration time of its partition time plus this value.\nSetting this property overrides the use of 'defaultTableExpirationMs'\nfor partitioned tables: only one of 'defaultTableExpirationMs' and\n'defaultPartitionExpirationMs' will be used for any new partitioned\ntable. If you provide an explicit 'timePartitioning.expirationMs' when\ncreating or updating a partitioned table, that value takes precedence\nover the default partition expiration time indicated by this property."]
    pub fn default_partition_expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_partition_expiration_ms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_table_expiration_ms` after provisioning.\nThe default lifetime of all tables in the dataset, in milliseconds.\nThe minimum value is 3600000 milliseconds (one hour).\nOnce this property is set, all newly-created tables in the dataset\nwill have an 'expirationTime' property set to the creation time plus\nthe value in this property, and changing the value will only affect\nnew tables, not existing ones. When the 'expirationTime' for a given\ntable is reached, that table will be deleted automatically.\nIf a table's 'expirationTime' is modified or removed before the\ntable expires, or if you provide an explicit 'expirationTime' when\ncreating a table, that value takes precedence over the default\nexpiration time indicated by this property."]
    pub fn default_table_expiration_ms(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_table_expiration_ms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_contents_on_destroy` after provisioning.\nIf set to 'true', delete all the tables in the\ndataset when destroying the resource; otherwise,\ndestroying the resource will fail if tables are present."]
    pub fn delete_contents_on_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_contents_on_destroy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-friendly description of the dataset"]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nA hash of the resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the dataset"]
    pub fn friendly_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.friendly_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `is_case_insensitive` after provisioning.\nTRUE if the dataset and its table names are case-insensitive, otherwise FALSE.\nBy default, this is FALSE, which means the dataset and its table names are\ncase-sensitive. This field does not affect routine references."]
    pub fn is_case_insensitive(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_case_insensitive", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\nThe date when this dataset or any of its tables was last modified, in\nmilliseconds since the epoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee [official docs](https://cloud.google.com/bigquery/docs/dataset-locations).\nThere are two types of locations, regional or multi-regional. A regional\nlocation is a specific geographic place, such as Tokyo, and a multi-regional\nlocation is a large geographic area, such as the United States, that\ncontains at least two geographic places.\nThe default value is multi-regional location 'US'.\nChanging this forces a new resource to be created."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `max_time_travel_hours` after provisioning.\nDefines the time travel window in hours. The value can be from 48 to 168 hours (2 to 7 days)."]
    pub fn max_time_travel_hours(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_time_travel_hours", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_tags` after provisioning.\nThe tags attached to this table. Tag keys are globally unique. Tag key is expected to be\nin the namespaced format, for example \"123456789012/environment\" where 123456789012 is the\nID of the parent organization or project resource for this tag key. Tag value is expected\nto be the short name, for example \"Production\". See [Tag definitions](https://cloud.google.com/iam/docs/tags-access-control#definitions)\nfor more details."]
    pub fn resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `storage_billing_model` after provisioning.\nSpecifies the storage billing model for the dataset.\nSet this flag value to LOGICAL to use logical bytes for storage billing,\nor to PHYSICAL to use physical bytes instead.\n\nLOGICAL is the default if this flag isn't specified."]
    pub fn storage_billing_model(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_billing_model", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_encryption_configuration` after provisioning.\n"]
    pub fn default_encryption_configuration(
        &self,
    ) -> ListRef<BigqueryDatasetDefaultEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_catalog_dataset_options` after provisioning.\n"]
    pub fn external_catalog_dataset_options(
        &self,
    ) -> ListRef<BigqueryDatasetExternalCatalogDatasetOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_catalog_dataset_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_dataset_reference` after provisioning.\n"]
    pub fn external_dataset_reference(
        &self,
    ) -> ListRef<BigqueryDatasetExternalDatasetReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_dataset_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatasetTimeoutsElRef {
        BigqueryDatasetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessElConditionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl BigqueryDatasetAccessElConditionEl {
    #[doc = "Set the field `description`.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file\nname and a position in the file."]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryDatasetAccessElConditionEl {
    type O = BlockAssignable<BigqueryDatasetAccessElConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessElConditionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildBigqueryDatasetAccessElConditionEl {
    pub fn build(self) -> BigqueryDatasetAccessElConditionEl {
        BigqueryDatasetAccessElConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDatasetAccessElConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessElConditionElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessElConditionElRef {
        BigqueryDatasetAccessElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file\nname and a position in the file."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessElDatasetElDatasetEl {
    dataset_id: PrimField<String>,
    project_id: PrimField<String>,
}
impl BigqueryDatasetAccessElDatasetElDatasetEl {}
impl ToListMappable for BigqueryDatasetAccessElDatasetElDatasetEl {
    type O = BlockAssignable<BigqueryDatasetAccessElDatasetElDatasetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessElDatasetElDatasetEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: PrimField<String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: PrimField<String>,
}
impl BuildBigqueryDatasetAccessElDatasetElDatasetEl {
    pub fn build(self) -> BigqueryDatasetAccessElDatasetElDatasetEl {
        BigqueryDatasetAccessElDatasetElDatasetEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
        }
    }
}
pub struct BigqueryDatasetAccessElDatasetElDatasetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessElDatasetElDatasetElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessElDatasetElDatasetElRef {
        BigqueryDatasetAccessElDatasetElDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessElDatasetElDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryDatasetAccessElDatasetElDynamic {
    dataset: Option<DynamicBlock<BigqueryDatasetAccessElDatasetElDatasetEl>>,
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessElDatasetEl {
    target_types: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<Vec<BigqueryDatasetAccessElDatasetElDatasetEl>>,
    dynamic: BigqueryDatasetAccessElDatasetElDynamic,
}
impl BigqueryDatasetAccessElDatasetEl {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(
        mut self,
        v: impl Into<BlockAssignable<BigqueryDatasetAccessElDatasetElDatasetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryDatasetAccessElDatasetEl {
    type O = BlockAssignable<BigqueryDatasetAccessElDatasetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessElDatasetEl {
    #[doc = "Which resources in the dataset this entry applies to. Currently, only views are supported,\nbut additional target types may be added in the future. Possible values: VIEWS"]
    pub target_types: ListField<PrimField<String>>,
}
impl BuildBigqueryDatasetAccessElDatasetEl {
    pub fn build(self) -> BigqueryDatasetAccessElDatasetEl {
        BigqueryDatasetAccessElDatasetEl {
            target_types: self.target_types,
            dataset: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryDatasetAccessElDatasetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessElDatasetElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessElDatasetElRef {
        BigqueryDatasetAccessElDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessElDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_types` after provisioning.\nWhich resources in the dataset this entry applies to. Currently, only views are supported,\nbut additional target types may be added in the future. Possible values: VIEWS"]
    pub fn target_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.target_types", self.base))
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> ListRef<BigqueryDatasetAccessElDatasetElDatasetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessElRoutineEl {
    dataset_id: PrimField<String>,
    project_id: PrimField<String>,
    routine_id: PrimField<String>,
}
impl BigqueryDatasetAccessElRoutineEl {}
impl ToListMappable for BigqueryDatasetAccessElRoutineEl {
    type O = BlockAssignable<BigqueryDatasetAccessElRoutineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessElRoutineEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: PrimField<String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: PrimField<String>,
    #[doc = "The ID of the routine. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 256 characters."]
    pub routine_id: PrimField<String>,
}
impl BuildBigqueryDatasetAccessElRoutineEl {
    pub fn build(self) -> BigqueryDatasetAccessElRoutineEl {
        BigqueryDatasetAccessElRoutineEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            routine_id: self.routine_id,
        }
    }
}
pub struct BigqueryDatasetAccessElRoutineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessElRoutineElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessElRoutineElRef {
        BigqueryDatasetAccessElRoutineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessElRoutineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `routine_id` after provisioning.\nThe ID of the routine. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 256 characters."]
    pub fn routine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routine_id", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessElViewEl {
    dataset_id: PrimField<String>,
    project_id: PrimField<String>,
    table_id: PrimField<String>,
}
impl BigqueryDatasetAccessElViewEl {}
impl ToListMappable for BigqueryDatasetAccessElViewEl {
    type O = BlockAssignable<BigqueryDatasetAccessElViewEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessElViewEl {
    #[doc = "The ID of the dataset containing this table."]
    pub dataset_id: PrimField<String>,
    #[doc = "The ID of the project containing this table."]
    pub project_id: PrimField<String>,
    #[doc = "The ID of the table. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 1,024 characters."]
    pub table_id: PrimField<String>,
}
impl BuildBigqueryDatasetAccessElViewEl {
    pub fn build(self) -> BigqueryDatasetAccessElViewEl {
        BigqueryDatasetAccessElViewEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}
pub struct BigqueryDatasetAccessElViewElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessElViewElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessElViewElRef {
        BigqueryDatasetAccessElViewElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessElViewElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe ID of the table. The ID must contain only letters (a-z,\nA-Z), numbers (0-9), or underscores (_). The maximum length\nis 1,024 characters."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct BigqueryDatasetAccessElDynamic {
    condition: Option<DynamicBlock<BigqueryDatasetAccessElConditionEl>>,
    dataset: Option<DynamicBlock<BigqueryDatasetAccessElDatasetEl>>,
    routine: Option<DynamicBlock<BigqueryDatasetAccessElRoutineEl>>,
    view: Option<DynamicBlock<BigqueryDatasetAccessElViewEl>>,
}
#[derive(Serialize)]
pub struct BigqueryDatasetAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_by_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_member: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_by_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Vec<BigqueryDatasetAccessElConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<Vec<BigqueryDatasetAccessElDatasetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routine: Option<Vec<BigqueryDatasetAccessElRoutineEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view: Option<Vec<BigqueryDatasetAccessElViewEl>>,
    dynamic: BigqueryDatasetAccessElDynamic,
}
impl BigqueryDatasetAccessEl {
    #[doc = "Set the field `domain`.\nA domain to grant access to. Any users signed in with the\ndomain specified will be granted the specified access"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `group_by_email`.\nAn email address of a Google Group to grant access to."]
    pub fn set_group_by_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_by_email = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_member`.\nSome other type of member that appears in the IAM Policy but isn't a user,\ngroup, domain, or special group. For example: 'allUsers'"]
    pub fn set_iam_member(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.iam_member = Some(v.into());
        self
    }
    #[doc = "Set the field `role`.\nDescribes the rights granted to the user specified by the other\nmember of the access object. Basic, predefined, and custom roles\nare supported. Predefined roles that have equivalent basic roles\nare swapped by the API to their basic counterparts. See\n[official docs](https://cloud.google.com/bigquery/docs/access-control)."]
    pub fn set_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role = Some(v.into());
        self
    }
    #[doc = "Set the field `special_group`.\nA special group to grant access to. Possible values include:\n* 'projectOwners': Owners of the enclosing project.\n* 'projectReaders': Readers of the enclosing project.\n* 'projectWriters': Writers of the enclosing project.\n* 'allAuthenticatedUsers': All authenticated BigQuery users."]
    pub fn set_special_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.special_group = Some(v.into());
        self
    }
    #[doc = "Set the field `user_by_email`.\nAn email address of a user to grant access to. For example:\nfred@example.com"]
    pub fn set_user_by_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_by_email = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl Into<BlockAssignable<BigqueryDatasetAccessElConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(
        mut self,
        v: impl Into<BlockAssignable<BigqueryDatasetAccessElDatasetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dataset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `routine`.\n"]
    pub fn set_routine(
        mut self,
        v: impl Into<BlockAssignable<BigqueryDatasetAccessElRoutineEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.routine = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.routine = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `view`.\n"]
    pub fn set_view(
        mut self,
        v: impl Into<BlockAssignable<BigqueryDatasetAccessElViewEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.view = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.view = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BigqueryDatasetAccessEl {
    type O = BlockAssignable<BigqueryDatasetAccessEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetAccessEl {}
impl BuildBigqueryDatasetAccessEl {
    pub fn build(self) -> BigqueryDatasetAccessEl {
        BigqueryDatasetAccessEl {
            domain: core::default::Default::default(),
            group_by_email: core::default::Default::default(),
            iam_member: core::default::Default::default(),
            role: core::default::Default::default(),
            special_group: core::default::Default::default(),
            user_by_email: core::default::Default::default(),
            condition: core::default::Default::default(),
            dataset: core::default::Default::default(),
            routine: core::default::Default::default(),
            view: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BigqueryDatasetAccessElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetAccessElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetAccessElRef {
        BigqueryDatasetAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nA domain to grant access to. Any users signed in with the\ndomain specified will be granted the specified access"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `group_by_email` after provisioning.\nAn email address of a Google Group to grant access to."]
    pub fn group_by_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_by_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `iam_member` after provisioning.\nSome other type of member that appears in the IAM Policy but isn't a user,\ngroup, domain, or special group. For example: 'allUsers'"]
    pub fn iam_member(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.iam_member", self.base))
    }
    #[doc = "Get a reference to the value of field `role` after provisioning.\nDescribes the rights granted to the user specified by the other\nmember of the access object. Basic, predefined, and custom roles\nare supported. Predefined roles that have equivalent basic roles\nare swapped by the API to their basic counterparts. See\n[official docs](https://cloud.google.com/bigquery/docs/access-control)."]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
    #[doc = "Get a reference to the value of field `special_group` after provisioning.\nA special group to grant access to. Possible values include:\n* 'projectOwners': Owners of the enclosing project.\n* 'projectReaders': Readers of the enclosing project.\n* 'projectWriters': Writers of the enclosing project.\n* 'allAuthenticatedUsers': All authenticated BigQuery users."]
    pub fn special_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.special_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_by_email` after provisioning.\nAn email address of a user to grant access to. For example:\nfred@example.com"]
    pub fn user_by_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_by_email", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<BigqueryDatasetAccessElConditionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> ListRef<BigqueryDatasetAccessElDatasetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
    #[doc = "Get a reference to the value of field `routine` after provisioning.\n"]
    pub fn routine(&self) -> ListRef<BigqueryDatasetAccessElRoutineElRef> {
        ListRef::new(self.shared().clone(), format!("{}.routine", self.base))
    }
    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> ListRef<BigqueryDatasetAccessElViewElRef> {
        ListRef::new(self.shared().clone(), format!("{}.view", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetDefaultEncryptionConfigurationEl {
    kms_key_name: PrimField<String>,
}
impl BigqueryDatasetDefaultEncryptionConfigurationEl {}
impl ToListMappable for BigqueryDatasetDefaultEncryptionConfigurationEl {
    type O = BlockAssignable<BigqueryDatasetDefaultEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetDefaultEncryptionConfigurationEl {
    #[doc = "Describes the Cloud KMS encryption key that will be used to protect destination\nBigQuery table. The BigQuery Service Account associated with your project requires\naccess to this encryption key."]
    pub kms_key_name: PrimField<String>,
}
impl BuildBigqueryDatasetDefaultEncryptionConfigurationEl {
    pub fn build(self) -> BigqueryDatasetDefaultEncryptionConfigurationEl {
        BigqueryDatasetDefaultEncryptionConfigurationEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct BigqueryDatasetDefaultEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetDefaultEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryDatasetDefaultEncryptionConfigurationElRef {
        BigqueryDatasetDefaultEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetDefaultEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect destination\nBigQuery table. The BigQuery Service Account associated with your project requires\naccess to this encryption key."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetExternalCatalogDatasetOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_storage_location_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
}
impl BigqueryDatasetExternalCatalogDatasetOptionsEl {
    #[doc = "Set the field `default_storage_location_uri`.\nThe storage location URI for all tables in the dataset. Equivalent to hive metastore's\ndatabase locationUri. Maximum length of 1024 characters."]
    pub fn set_default_storage_location_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_storage_location_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\nA map of key value pairs defining the parameters and properties of the open source schema.\nMaximum size of 2Mib."]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryDatasetExternalCatalogDatasetOptionsEl {
    type O = BlockAssignable<BigqueryDatasetExternalCatalogDatasetOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetExternalCatalogDatasetOptionsEl {}
impl BuildBigqueryDatasetExternalCatalogDatasetOptionsEl {
    pub fn build(self) -> BigqueryDatasetExternalCatalogDatasetOptionsEl {
        BigqueryDatasetExternalCatalogDatasetOptionsEl {
            default_storage_location_uri: core::default::Default::default(),
            parameters: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDatasetExternalCatalogDatasetOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetExternalCatalogDatasetOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetExternalCatalogDatasetOptionsElRef {
        BigqueryDatasetExternalCatalogDatasetOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetExternalCatalogDatasetOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_storage_location_uri` after provisioning.\nThe storage location URI for all tables in the dataset. Equivalent to hive metastore's\ndatabase locationUri. Maximum length of 1024 characters."]
    pub fn default_storage_location_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_storage_location_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\nA map of key value pairs defining the parameters and properties of the open source schema.\nMaximum size of 2Mib."]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetExternalDatasetReferenceEl {
    connection: PrimField<String>,
    external_source: PrimField<String>,
}
impl BigqueryDatasetExternalDatasetReferenceEl {}
impl ToListMappable for BigqueryDatasetExternalDatasetReferenceEl {
    type O = BlockAssignable<BigqueryDatasetExternalDatasetReferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetExternalDatasetReferenceEl {
    #[doc = "The connection id that is used to access the externalSource.\nFormat: projects/{projectId}/locations/{locationId}/connections/{connectionId}"]
    pub connection: PrimField<String>,
    #[doc = "External source that backs this dataset."]
    pub external_source: PrimField<String>,
}
impl BuildBigqueryDatasetExternalDatasetReferenceEl {
    pub fn build(self) -> BigqueryDatasetExternalDatasetReferenceEl {
        BigqueryDatasetExternalDatasetReferenceEl {
            connection: self.connection,
            external_source: self.external_source,
        }
    }
}
pub struct BigqueryDatasetExternalDatasetReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetExternalDatasetReferenceElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetExternalDatasetReferenceElRef {
        BigqueryDatasetExternalDatasetReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetExternalDatasetReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection` after provisioning.\nThe connection id that is used to access the externalSource.\nFormat: projects/{projectId}/locations/{locationId}/connections/{connectionId}"]
    pub fn connection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection", self.base))
    }
    #[doc = "Get a reference to the value of field `external_source` after provisioning.\nExternal source that backs this dataset."]
    pub fn external_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.external_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryDatasetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryDatasetTimeoutsEl {
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
impl ToListMappable for BigqueryDatasetTimeoutsEl {
    type O = BlockAssignable<BigqueryDatasetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatasetTimeoutsEl {}
impl BuildBigqueryDatasetTimeoutsEl {
    pub fn build(self) -> BigqueryDatasetTimeoutsEl {
        BigqueryDatasetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDatasetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatasetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatasetTimeoutsElRef {
        BigqueryDatasetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatasetTimeoutsElRef {
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
struct BigqueryDatasetDynamic {
    access: Option<DynamicBlock<BigqueryDatasetAccessEl>>,
    default_encryption_configuration:
        Option<DynamicBlock<BigqueryDatasetDefaultEncryptionConfigurationEl>>,
    external_catalog_dataset_options:
        Option<DynamicBlock<BigqueryDatasetExternalCatalogDatasetOptionsEl>>,
    external_dataset_reference: Option<DynamicBlock<BigqueryDatasetExternalDatasetReferenceEl>>,
}
