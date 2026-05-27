use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BiglakeTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hive_options: Option<Vec<BiglakeTableHiveOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BiglakeTableTimeoutsEl>,
    dynamic: BiglakeTableDynamic,
}
struct BiglakeTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BiglakeTableData>,
}
#[derive(Clone)]
pub struct BiglakeTable(Rc<BiglakeTable_>);
impl BiglakeTable {
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
    #[doc = "Set the field `database`.\nThe id of the parent database."]
    pub fn set_database(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database = Some(v.into());
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
    #[doc = "Set the field `type_`.\nThe database type. Possible values: [\"HIVE\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `hive_options`.\n"]
    pub fn set_hive_options(
        self,
        v: impl Into<BlockAssignable<BiglakeTableHiveOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hive_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hive_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BiglakeTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time of the table. A timestamp in RFC3339 UTC\n\"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe id of the parent database."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. The deletion time of the table. Only set after the\ntable is deleted. A timestamp in RFC3339 UTC \"Zulu\" format, with\nnanosecond resolution and up to nine fractional digits. Examples:\n\"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe checksum of a table object computed by the server based on the value\nof other fields. It may be sent on update requests to ensure the client\nhas an up-to-date value before proceeding. It is only checked for update\ntable operations."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nOutput only. The time when this table is considered expired. Only set\nafter the table is deleted. A timestamp in RFC3339 UTC \"Zulu\" format,\nwith nanosecond resolution and up to nine fractional digits. Examples:\n\"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. The name of the Table. Format:\nprojects/{project_id_or_number}/locations/{locationId}/catalogs/{catalogId}/databases/{databaseId}/tables/{tableId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe database type. Possible values: [\"HIVE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last modification time of the table. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hive_options` after provisioning.\n"]
    pub fn hive_options(&self) -> ListRef<BiglakeTableHiveOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hive_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeTableTimeoutsElRef {
        BiglakeTableTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BiglakeTable {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BiglakeTable {}
impl ToListMappable for BiglakeTable {
    type O = ListRef<BiglakeTableRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BiglakeTable_ {
    fn extract_resource_type(&self) -> String {
        "google_biglake_table".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBiglakeTable {
    pub tf_id: String,
    #[doc = "Output only. The name of the Table. Format:\nprojects/{project_id_or_number}/locations/{locationId}/catalogs/{catalogId}/databases/{databaseId}/tables/{tableId}"]
    pub name: PrimField<String>,
}
impl BuildBiglakeTable {
    pub fn build(self, stack: &mut Stack) -> BiglakeTable {
        let out = BiglakeTable(Rc::new(BiglakeTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BiglakeTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                database: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                type_: core::default::Default::default(),
                hive_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BiglakeTableRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BiglakeTableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation time of the table. A timestamp in RFC3339 UTC\n\"Zulu\" format, with nanosecond resolution and up to nine fractional\ndigits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe id of the parent database."]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. The deletion time of the table. Only set after the\ntable is deleted. A timestamp in RFC3339 UTC \"Zulu\" format, with\nnanosecond resolution and up to nine fractional digits. Examples:\n\"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe checksum of a table object computed by the server based on the value\nof other fields. It may be sent on update requests to ensure the client\nhas an up-to-date value before proceeding. It is only checked for update\ntable operations."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nOutput only. The time when this table is considered expired. Only set\nafter the table is deleted. A timestamp in RFC3339 UTC \"Zulu\" format,\nwith nanosecond resolution and up to nine fractional digits. Examples:\n\"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. The name of the Table. Format:\nprojects/{project_id_or_number}/locations/{locationId}/catalogs/{catalogId}/databases/{databaseId}/tables/{tableId}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe database type. Possible values: [\"HIVE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last modification time of the table. A timestamp in\nRFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine\nfractional digits. Examples: \"2014-10-02T15:01:23Z\" and\n\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hive_options` after provisioning.\n"]
    pub fn hive_options(&self) -> ListRef<BiglakeTableHiveOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hive_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BiglakeTableTimeoutsElRef {
        BiglakeTableTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BiglakeTableHiveOptionsElStorageDescriptorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_format: Option<PrimField<String>>,
}
impl BiglakeTableHiveOptionsElStorageDescriptorEl {
    #[doc = "Set the field `input_format`.\nThe fully qualified Java class name of the input format."]
    pub fn set_input_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_format = Some(v.into());
        self
    }
    #[doc = "Set the field `location_uri`.\nCloud Storage folder URI where the table data is stored, starting with \"gs://\"."]
    pub fn set_location_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `output_format`.\nThe fully qualified Java class name of the output format."]
    pub fn set_output_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_format = Some(v.into());
        self
    }
}
impl ToListMappable for BiglakeTableHiveOptionsElStorageDescriptorEl {
    type O = BlockAssignable<BiglakeTableHiveOptionsElStorageDescriptorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeTableHiveOptionsElStorageDescriptorEl {}
impl BuildBiglakeTableHiveOptionsElStorageDescriptorEl {
    pub fn build(self) -> BiglakeTableHiveOptionsElStorageDescriptorEl {
        BiglakeTableHiveOptionsElStorageDescriptorEl {
            input_format: core::default::Default::default(),
            location_uri: core::default::Default::default(),
            output_format: core::default::Default::default(),
        }
    }
}
pub struct BiglakeTableHiveOptionsElStorageDescriptorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeTableHiveOptionsElStorageDescriptorElRef {
    fn new(shared: StackShared, base: String) -> BiglakeTableHiveOptionsElStorageDescriptorElRef {
        BiglakeTableHiveOptionsElStorageDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeTableHiveOptionsElStorageDescriptorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `input_format` after provisioning.\nThe fully qualified Java class name of the input format."]
    pub fn input_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_format", self.base))
    }
    #[doc = "Get a reference to the value of field `location_uri` after provisioning.\nCloud Storage folder URI where the table data is stored, starting with \"gs://\"."]
    pub fn location_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `output_format` after provisioning.\nThe fully qualified Java class name of the output format."]
    pub fn output_format(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_format", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BiglakeTableHiveOptionsElDynamic {
    storage_descriptor: Option<DynamicBlock<BiglakeTableHiveOptionsElStorageDescriptorEl>>,
}
#[derive(Serialize)]
pub struct BiglakeTableHiveOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_descriptor: Option<Vec<BiglakeTableHiveOptionsElStorageDescriptorEl>>,
    dynamic: BiglakeTableHiveOptionsElDynamic,
}
impl BiglakeTableHiveOptionsEl {
    #[doc = "Set the field `parameters`.\nStores user supplied Hive table parameters. An object containing a\nlist of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_parameters(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `table_type`.\nHive table type. For example, MANAGED_TABLE, EXTERNAL_TABLE."]
    pub fn set_table_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_type = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_descriptor`.\n"]
    pub fn set_storage_descriptor(
        mut self,
        v: impl Into<BlockAssignable<BiglakeTableHiveOptionsElStorageDescriptorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.storage_descriptor = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_descriptor = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BiglakeTableHiveOptionsEl {
    type O = BlockAssignable<BiglakeTableHiveOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeTableHiveOptionsEl {}
impl BuildBiglakeTableHiveOptionsEl {
    pub fn build(self) -> BiglakeTableHiveOptionsEl {
        BiglakeTableHiveOptionsEl {
            parameters: core::default::Default::default(),
            table_type: core::default::Default::default(),
            storage_descriptor: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BiglakeTableHiveOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeTableHiveOptionsElRef {
    fn new(shared: StackShared, base: String) -> BiglakeTableHiveOptionsElRef {
        BiglakeTableHiveOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeTableHiveOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\nStores user supplied Hive table parameters. An object containing a\nlist of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `table_type` after provisioning.\nHive table type. For example, MANAGED_TABLE, EXTERNAL_TABLE."]
    pub fn table_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_type", self.base))
    }
    #[doc = "Get a reference to the value of field `storage_descriptor` after provisioning.\n"]
    pub fn storage_descriptor(&self) -> ListRef<BiglakeTableHiveOptionsElStorageDescriptorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.storage_descriptor", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BiglakeTableTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BiglakeTableTimeoutsEl {
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
impl ToListMappable for BiglakeTableTimeoutsEl {
    type O = BlockAssignable<BiglakeTableTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBiglakeTableTimeoutsEl {}
impl BuildBiglakeTableTimeoutsEl {
    pub fn build(self) -> BiglakeTableTimeoutsEl {
        BiglakeTableTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BiglakeTableTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BiglakeTableTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BiglakeTableTimeoutsElRef {
        BiglakeTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BiglakeTableTimeoutsElRef {
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
struct BiglakeTableDynamic {
    hive_options: Option<DynamicBlock<BiglakeTableHiveOptionsEl>>,
}
