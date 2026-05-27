use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCatalogEntryData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    entry_group: PrimField<String>,
    entry_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linked_resource: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_specified_system: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_specified_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcs_fileset_spec: Option<Vec<DataCatalogEntryGcsFilesetSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataCatalogEntryTimeoutsEl>,
    dynamic: DataCatalogEntryDynamic,
}
struct DataCatalogEntry_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCatalogEntryData>,
}
#[derive(Clone)]
pub struct DataCatalogEntry(Rc<DataCatalogEntry_>);
impl DataCatalogEntry {
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
    #[doc = "Set the field `description`.\nEntry description, which can consist of several sentences or paragraphs that describe entry contents."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay information such as title and description. A short name to identify the entry,\nfor example, \"Analytics Data - Jan 2011\"."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `linked_resource`.\nThe resource this metadata entry refers to.\nFor Google Cloud Platform resources, linkedResource is the full name of the resource.\nFor example, the linkedResource for a table resource from BigQuery is:\n//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId\nOutput only when Entry is of type in the EntryType enum. For entries with userSpecifiedType,\nthis field is optional and defaults to an empty string."]
    pub fn set_linked_resource(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().linked_resource = Some(v.into());
        self
    }
    #[doc = "Set the field `schema`.\nSchema of the entry (e.g. BigQuery, GoogleSQL, Avro schema), as a json string. An entry might not have any schema\nattached to it. See\nhttps://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries#schema\nfor what fields this schema can contain."]
    pub fn set_schema(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schema = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThe type of the entry. Only used for Entries with types in the EntryType enum.\nCurrently, only FILESET enum value is allowed. All other entries created through Data Catalog must use userSpecifiedType. Possible values: [\"FILESET\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `user_specified_system`.\nThis field indicates the entry's source system that Data Catalog does not integrate with.\nuserSpecifiedSystem strings must begin with a letter or underscore and can only contain letters, numbers,\nand underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn set_user_specified_system(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_specified_system = Some(v.into());
        self
    }
    #[doc = "Set the field `user_specified_type`.\nEntry type if it does not fit any of the input-allowed values listed in EntryType enum above.\nWhen creating an entry, users should check the enum values first, if nothing matches the entry\nto be created, then provide a custom value, for example \"my_special_type\".\nuserSpecifiedType strings must begin with a letter or underscore and can only contain letters,\nnumbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn set_user_specified_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_specified_type = Some(v.into());
        self
    }
    #[doc = "Set the field `gcs_fileset_spec`.\n"]
    pub fn set_gcs_fileset_spec(
        self,
        v: impl Into<BlockAssignable<DataCatalogEntryGcsFilesetSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gcs_fileset_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gcs_fileset_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataCatalogEntryTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bigquery_date_sharded_spec` after provisioning.\nSpecification for a group of BigQuery tables with name pattern [prefix]YYYYMMDD.\nContext: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding."]
    pub fn bigquery_date_sharded_spec(
        &self,
    ) -> ListRef<DataCatalogEntryBigqueryDateShardedSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_date_sharded_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_table_spec` after provisioning.\nSpecification that applies to a BigQuery table. This is only valid on entries of type TABLE."]
    pub fn bigquery_table_spec(&self) -> ListRef<DataCatalogEntryBigqueryTableSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_table_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nEntry description, which can consist of several sentences or paragraphs that describe entry contents."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay information such as title and description. A short name to identify the entry,\nfor example, \"Analytics Data - Jan 2011\"."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_group` after provisioning.\nThe name of the entry group this entry is in."]
    pub fn entry_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\nThe id of the entry to create."]
    pub fn entry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `integrated_system` after provisioning.\nThis field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub."]
    pub fn integrated_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integrated_system", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_resource` after provisioning.\nThe resource this metadata entry refers to.\nFor Google Cloud Platform resources, linkedResource is the full name of the resource.\nFor example, the linkedResource for a table resource from BigQuery is:\n//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId\nOutput only when Entry is of type in the EntryType enum. For entries with userSpecifiedType,\nthis field is optional and defaults to an empty string."]
    pub fn linked_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.linked_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe Data Catalog resource name of the entry in URL format.\nExample: projects/{project_id}/locations/{location}/entryGroups/{entryGroupId}/entries/{entryId}.\nNote that this Entry and its child resources may not actually be stored in the location in this name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema of the entry (e.g. BigQuery, GoogleSQL, Avro schema), as a json string. An entry might not have any schema\nattached to it. See\nhttps://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries#schema\nfor what fields this schema can contain."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the entry. Only used for Entries with types in the EntryType enum.\nCurrently, only FILESET enum value is allowed. All other entries created through Data Catalog must use userSpecifiedType. Possible values: [\"FILESET\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_specified_system` after provisioning.\nThis field indicates the entry's source system that Data Catalog does not integrate with.\nuserSpecifiedSystem strings must begin with a letter or underscore and can only contain letters, numbers,\nand underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn user_specified_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_specified_system", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_specified_type` after provisioning.\nEntry type if it does not fit any of the input-allowed values listed in EntryType enum above.\nWhen creating an entry, users should check the enum values first, if nothing matches the entry\nto be created, then provide a custom value, for example \"my_special_type\".\nuserSpecifiedType strings must begin with a letter or underscore and can only contain letters,\nnumbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn user_specified_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_specified_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_fileset_spec` after provisioning.\n"]
    pub fn gcs_fileset_spec(&self) -> ListRef<DataCatalogEntryGcsFilesetSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_fileset_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogEntryTimeoutsElRef {
        DataCatalogEntryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataCatalogEntry {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataCatalogEntry {}
impl ToListMappable for DataCatalogEntry {
    type O = ListRef<DataCatalogEntryRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataCatalogEntry_ {
    fn extract_resource_type(&self) -> String {
        "google_data_catalog_entry".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCatalogEntry {
    pub tf_id: String,
    #[doc = "The name of the entry group this entry is in."]
    pub entry_group: PrimField<String>,
    #[doc = "The id of the entry to create."]
    pub entry_id: PrimField<String>,
}
impl BuildDataCatalogEntry {
    pub fn build(self, stack: &mut Stack) -> DataCatalogEntry {
        let out = DataCatalogEntry(Rc::new(DataCatalogEntry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCatalogEntryData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                entry_group: self.entry_group,
                entry_id: self.entry_id,
                id: core::default::Default::default(),
                linked_resource: core::default::Default::default(),
                schema: core::default::Default::default(),
                type_: core::default::Default::default(),
                user_specified_system: core::default::Default::default(),
                user_specified_type: core::default::Default::default(),
                gcs_fileset_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataCatalogEntryRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogEntryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCatalogEntryRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bigquery_date_sharded_spec` after provisioning.\nSpecification for a group of BigQuery tables with name pattern [prefix]YYYYMMDD.\nContext: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding."]
    pub fn bigquery_date_sharded_spec(
        &self,
    ) -> ListRef<DataCatalogEntryBigqueryDateShardedSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_date_sharded_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `bigquery_table_spec` after provisioning.\nSpecification that applies to a BigQuery table. This is only valid on entries of type TABLE."]
    pub fn bigquery_table_spec(&self) -> ListRef<DataCatalogEntryBigqueryTableSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bigquery_table_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nEntry description, which can consist of several sentences or paragraphs that describe entry contents."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay information such as title and description. A short name to identify the entry,\nfor example, \"Analytics Data - Jan 2011\"."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_group` after provisioning.\nThe name of the entry group this entry is in."]
    pub fn entry_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `entry_id` after provisioning.\nThe id of the entry to create."]
    pub fn entry_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.entry_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `integrated_system` after provisioning.\nThis field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub."]
    pub fn integrated_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integrated_system", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `linked_resource` after provisioning.\nThe resource this metadata entry refers to.\nFor Google Cloud Platform resources, linkedResource is the full name of the resource.\nFor example, the linkedResource for a table resource from BigQuery is:\n//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId\nOutput only when Entry is of type in the EntryType enum. For entries with userSpecifiedType,\nthis field is optional and defaults to an empty string."]
    pub fn linked_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.linked_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe Data Catalog resource name of the entry in URL format.\nExample: projects/{project_id}/locations/{location}/entryGroups/{entryGroupId}/entries/{entryId}.\nNote that this Entry and its child resources may not actually be stored in the location in this name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema of the entry (e.g. BigQuery, GoogleSQL, Avro schema), as a json string. An entry might not have any schema\nattached to it. See\nhttps://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries#schema\nfor what fields this schema can contain."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the entry. Only used for Entries with types in the EntryType enum.\nCurrently, only FILESET enum value is allowed. All other entries created through Data Catalog must use userSpecifiedType. Possible values: [\"FILESET\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_specified_system` after provisioning.\nThis field indicates the entry's source system that Data Catalog does not integrate with.\nuserSpecifiedSystem strings must begin with a letter or underscore and can only contain letters, numbers,\nand underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn user_specified_system(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_specified_system", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `user_specified_type` after provisioning.\nEntry type if it does not fit any of the input-allowed values listed in EntryType enum above.\nWhen creating an entry, users should check the enum values first, if nothing matches the entry\nto be created, then provide a custom value, for example \"my_special_type\".\nuserSpecifiedType strings must begin with a letter or underscore and can only contain letters,\nnumbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
    pub fn user_specified_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_specified_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gcs_fileset_spec` after provisioning.\n"]
    pub fn gcs_fileset_spec(&self) -> ListRef<DataCatalogEntryGcsFilesetSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gcs_fileset_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogEntryTimeoutsElRef {
        DataCatalogEntryTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCatalogEntryBigqueryDateShardedSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dataset: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shard_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_prefix: Option<PrimField<String>>,
}
impl DataCatalogEntryBigqueryDateShardedSpecEl {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dataset = Some(v.into());
        self
    }
    #[doc = "Set the field `shard_count`.\n"]
    pub fn set_shard_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.shard_count = Some(v.into());
        self
    }
    #[doc = "Set the field `table_prefix`.\n"]
    pub fn set_table_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_prefix = Some(v.into());
        self
    }
}
impl ToListMappable for DataCatalogEntryBigqueryDateShardedSpecEl {
    type O = BlockAssignable<DataCatalogEntryBigqueryDateShardedSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogEntryBigqueryDateShardedSpecEl {}
impl BuildDataCatalogEntryBigqueryDateShardedSpecEl {
    pub fn build(self) -> DataCatalogEntryBigqueryDateShardedSpecEl {
        DataCatalogEntryBigqueryDateShardedSpecEl {
            dataset: core::default::Default::default(),
            shard_count: core::default::Default::default(),
            table_prefix: core::default::Default::default(),
        }
    }
}
pub struct DataCatalogEntryBigqueryDateShardedSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogEntryBigqueryDateShardedSpecElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogEntryBigqueryDateShardedSpecElRef {
        DataCatalogEntryBigqueryDateShardedSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogEntryBigqueryDateShardedSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
    #[doc = "Get a reference to the value of field `shard_count` after provisioning.\n"]
    pub fn shard_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.shard_count", self.base))
    }
    #[doc = "Get a reference to the value of field `table_prefix` after provisioning.\n"]
    pub fn table_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_prefix", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCatalogEntryBigqueryTableSpecElTableSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grouped_entry: Option<PrimField<String>>,
}
impl DataCatalogEntryBigqueryTableSpecElTableSpecEl {
    #[doc = "Set the field `grouped_entry`.\n"]
    pub fn set_grouped_entry(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.grouped_entry = Some(v.into());
        self
    }
}
impl ToListMappable for DataCatalogEntryBigqueryTableSpecElTableSpecEl {
    type O = BlockAssignable<DataCatalogEntryBigqueryTableSpecElTableSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogEntryBigqueryTableSpecElTableSpecEl {}
impl BuildDataCatalogEntryBigqueryTableSpecElTableSpecEl {
    pub fn build(self) -> DataCatalogEntryBigqueryTableSpecElTableSpecEl {
        DataCatalogEntryBigqueryTableSpecElTableSpecEl {
            grouped_entry: core::default::Default::default(),
        }
    }
}
pub struct DataCatalogEntryBigqueryTableSpecElTableSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogEntryBigqueryTableSpecElTableSpecElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogEntryBigqueryTableSpecElTableSpecElRef {
        DataCatalogEntryBigqueryTableSpecElTableSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogEntryBigqueryTableSpecElTableSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `grouped_entry` after provisioning.\n"]
    pub fn grouped_entry(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grouped_entry", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCatalogEntryBigqueryTableSpecElViewSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    view_query: Option<PrimField<String>>,
}
impl DataCatalogEntryBigqueryTableSpecElViewSpecEl {
    #[doc = "Set the field `view_query`.\n"]
    pub fn set_view_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.view_query = Some(v.into());
        self
    }
}
impl ToListMappable for DataCatalogEntryBigqueryTableSpecElViewSpecEl {
    type O = BlockAssignable<DataCatalogEntryBigqueryTableSpecElViewSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogEntryBigqueryTableSpecElViewSpecEl {}
impl BuildDataCatalogEntryBigqueryTableSpecElViewSpecEl {
    pub fn build(self) -> DataCatalogEntryBigqueryTableSpecElViewSpecEl {
        DataCatalogEntryBigqueryTableSpecElViewSpecEl {
            view_query: core::default::Default::default(),
        }
    }
}
pub struct DataCatalogEntryBigqueryTableSpecElViewSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogEntryBigqueryTableSpecElViewSpecElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogEntryBigqueryTableSpecElViewSpecElRef {
        DataCatalogEntryBigqueryTableSpecElViewSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogEntryBigqueryTableSpecElViewSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `view_query` after provisioning.\n"]
    pub fn view_query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.view_query", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCatalogEntryBigqueryTableSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    table_source_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_spec: Option<ListField<DataCatalogEntryBigqueryTableSpecElTableSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view_spec: Option<ListField<DataCatalogEntryBigqueryTableSpecElViewSpecEl>>,
}
impl DataCatalogEntryBigqueryTableSpecEl {
    #[doc = "Set the field `table_source_type`.\n"]
    pub fn set_table_source_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_source_type = Some(v.into());
        self
    }
    #[doc = "Set the field `table_spec`.\n"]
    pub fn set_table_spec(
        mut self,
        v: impl Into<ListField<DataCatalogEntryBigqueryTableSpecElTableSpecEl>>,
    ) -> Self {
        self.table_spec = Some(v.into());
        self
    }
    #[doc = "Set the field `view_spec`.\n"]
    pub fn set_view_spec(
        mut self,
        v: impl Into<ListField<DataCatalogEntryBigqueryTableSpecElViewSpecEl>>,
    ) -> Self {
        self.view_spec = Some(v.into());
        self
    }
}
impl ToListMappable for DataCatalogEntryBigqueryTableSpecEl {
    type O = BlockAssignable<DataCatalogEntryBigqueryTableSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogEntryBigqueryTableSpecEl {}
impl BuildDataCatalogEntryBigqueryTableSpecEl {
    pub fn build(self) -> DataCatalogEntryBigqueryTableSpecEl {
        DataCatalogEntryBigqueryTableSpecEl {
            table_source_type: core::default::Default::default(),
            table_spec: core::default::Default::default(),
            view_spec: core::default::Default::default(),
        }
    }
}
pub struct DataCatalogEntryBigqueryTableSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogEntryBigqueryTableSpecElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogEntryBigqueryTableSpecElRef {
        DataCatalogEntryBigqueryTableSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogEntryBigqueryTableSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `table_source_type` after provisioning.\n"]
    pub fn table_source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_source_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `table_spec` after provisioning.\n"]
    pub fn table_spec(&self) -> ListRef<DataCatalogEntryBigqueryTableSpecElTableSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.table_spec", self.base))
    }
    #[doc = "Get a reference to the value of field `view_spec` after provisioning.\n"]
    pub fn view_spec(&self) -> ListRef<DataCatalogEntryBigqueryTableSpecElViewSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.view_spec", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    file_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_bytes: Option<PrimField<f64>>,
}
impl DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsEl {
    #[doc = "Set the field `file_path`.\n"]
    pub fn set_file_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_path = Some(v.into());
        self
    }
    #[doc = "Set the field `size_bytes`.\n"]
    pub fn set_size_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size_bytes = Some(v.into());
        self
    }
}
impl ToListMappable for DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsEl {
    type O = BlockAssignable<DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsEl {}
impl BuildDataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsEl {
    pub fn build(self) -> DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsEl {
        DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsEl {
            file_path: core::default::Default::default(),
            size_bytes: core::default::Default::default(),
        }
    }
}
pub struct DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsElRef {
        DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_path` after provisioning.\n"]
    pub fn file_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_path", self.base))
    }
    #[doc = "Get a reference to the value of field `size_bytes` after provisioning.\n"]
    pub fn size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_bytes", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCatalogEntryGcsFilesetSpecEl {
    file_patterns: ListField<PrimField<String>>,
}
impl DataCatalogEntryGcsFilesetSpecEl {}
impl ToListMappable for DataCatalogEntryGcsFilesetSpecEl {
    type O = BlockAssignable<DataCatalogEntryGcsFilesetSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogEntryGcsFilesetSpecEl {
    #[doc = "Patterns to identify a set of files in Google Cloud Storage.\nSee [Cloud Storage documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames)\nfor more information. Note that bucket wildcards are currently not supported. Examples of valid filePatterns:\n\n* gs://bucket_name/dir/*: matches all files within bucket_name/dir directory.\n* gs://bucket_name/dir/**: matches all files in bucket_name/dir spanning all subdirectories.\n* gs://bucket_name/file*: matches files prefixed by file in bucket_name\n* gs://bucket_name/??.txt: matches files with two characters followed by .txt in bucket_name\n* gs://bucket_name/[aeiou].txt: matches files that contain a single vowel character followed by .txt in bucket_name\n* gs://bucket_name/[a-m].txt: matches files that contain a, b, ... or m followed by .txt in bucket_name\n* gs://bucket_name/a/*/b: matches all files in bucket_name that match a/*/b pattern, such as a/c/b, a/d/b\n* gs://another_bucket/a.txt: matches gs://another_bucket/a.txt"]
    pub file_patterns: ListField<PrimField<String>>,
}
impl BuildDataCatalogEntryGcsFilesetSpecEl {
    pub fn build(self) -> DataCatalogEntryGcsFilesetSpecEl {
        DataCatalogEntryGcsFilesetSpecEl {
            file_patterns: self.file_patterns,
        }
    }
}
pub struct DataCatalogEntryGcsFilesetSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogEntryGcsFilesetSpecElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogEntryGcsFilesetSpecElRef {
        DataCatalogEntryGcsFilesetSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogEntryGcsFilesetSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_patterns` after provisioning.\nPatterns to identify a set of files in Google Cloud Storage.\nSee [Cloud Storage documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames)\nfor more information. Note that bucket wildcards are currently not supported. Examples of valid filePatterns:\n\n* gs://bucket_name/dir/*: matches all files within bucket_name/dir directory.\n* gs://bucket_name/dir/**: matches all files in bucket_name/dir spanning all subdirectories.\n* gs://bucket_name/file*: matches files prefixed by file in bucket_name\n* gs://bucket_name/??.txt: matches files with two characters followed by .txt in bucket_name\n* gs://bucket_name/[aeiou].txt: matches files that contain a single vowel character followed by .txt in bucket_name\n* gs://bucket_name/[a-m].txt: matches files that contain a, b, ... or m followed by .txt in bucket_name\n* gs://bucket_name/a/*/b: matches all files in bucket_name that match a/*/b pattern, such as a/c/b, a/d/b\n* gs://another_bucket/a.txt: matches gs://another_bucket/a.txt"]
    pub fn file_patterns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.file_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sample_gcs_file_specs` after provisioning.\nSample files contained in this fileset, not all files contained in this fileset are represented here."]
    pub fn sample_gcs_file_specs(
        &self,
    ) -> ListRef<DataCatalogEntryGcsFilesetSpecElSampleGcsFileSpecsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sample_gcs_file_specs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCatalogEntryTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataCatalogEntryTimeoutsEl {
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
impl ToListMappable for DataCatalogEntryTimeoutsEl {
    type O = BlockAssignable<DataCatalogEntryTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCatalogEntryTimeoutsEl {}
impl BuildDataCatalogEntryTimeoutsEl {
    pub fn build(self) -> DataCatalogEntryTimeoutsEl {
        DataCatalogEntryTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataCatalogEntryTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCatalogEntryTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataCatalogEntryTimeoutsElRef {
        DataCatalogEntryTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCatalogEntryTimeoutsElRef {
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
struct DataCatalogEntryDynamic {
    gcs_fileset_spec: Option<DynamicBlock<DataCatalogEntryGcsFilesetSpecEl>>,
}
