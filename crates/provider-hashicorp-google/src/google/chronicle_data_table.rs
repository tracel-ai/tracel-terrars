use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ChronicleDataTableData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_table_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    description: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_time_to_live: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_info: Option<Vec<ChronicleDataTableColumnInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope_info: Option<Vec<ChronicleDataTableScopeInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ChronicleDataTableTimeoutsEl>,
    dynamic: ChronicleDataTableDynamic,
}
struct ChronicleDataTable_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ChronicleDataTableData>,
}
#[derive(Clone)]
pub struct ChronicleDataTable(Rc<ChronicleDataTable_>);
impl ChronicleDataTable {
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
    #[doc = "Set the field `deletion_policy`.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_data_table.html.markdown for specifics"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `row_time_to_live`.\nUser-provided TTL of the data table."]
    pub fn set_row_time_to_live(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().row_time_to_live = Some(v.into());
        self
    }
    #[doc = "Set the field `column_info`.\n"]
    pub fn set_column_info(
        self,
        v: impl Into<BlockAssignable<ChronicleDataTableColumnInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().column_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.column_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scope_info`.\n"]
    pub fn set_scope_info(
        self,
        v: impl Into<BlockAssignable<ChronicleDataTableScopeInfoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scope_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scope_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ChronicleDataTableTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `approximate_row_count` after provisioning.\nThe count of rows in the data table."]
    pub fn approximate_row_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approximate_row_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTable create time"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_table_id` after provisioning.\nThe ID to use for the data table. This is also the display name for\nthe data table. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256."]
    pub fn data_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_table_uuid` after provisioning.\nData table unique id"]
    pub fn data_table_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_table_uuid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_data_table.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-provided description of the data table."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe unique display name of the data table."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the data table\nFormat:\n\"{project}/locations/{region}/instances/{instance}/dataTables/{data_table}\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_time_to_live` after provisioning.\nUser-provided TTL of the data table."]
    pub fn row_time_to_live(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.row_time_to_live", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_time_to_live_update_time` after provisioning.\nLast update time of the TTL of the data table."]
    pub fn row_time_to_live_update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.row_time_to_live_update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_associations_count` after provisioning.\nThe count of rules using the data table."]
    pub fn rule_associations_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_associations_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\nThe resource names for the associated Rules that use this\ndata table. Format:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}.\n{rule} here refers to the rule id."]
    pub fn rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_source` after provisioning.\nPossible values:\nUSER\nRULE\nSEARCH"]
    pub fn update_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTable update time"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `column_info` after provisioning.\n"]
    pub fn column_info(&self) -> ListRef<ChronicleDataTableColumnInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope_info` after provisioning.\n"]
    pub fn scope_info(&self) -> ListRef<ChronicleDataTableScopeInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDataTableTimeoutsElRef {
        ChronicleDataTableTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ChronicleDataTable {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ChronicleDataTable {}
impl ToListMappable for ChronicleDataTable {
    type O = ListRef<ChronicleDataTableRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ChronicleDataTable_ {
    fn extract_resource_type(&self) -> String {
        "google_chronicle_data_table".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildChronicleDataTable {
    pub tf_id: String,
    #[doc = "The ID to use for the data table. This is also the display name for\nthe data table. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256."]
    pub data_table_id: PrimField<String>,
    #[doc = "A user-provided description of the data table."]
    pub description: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub instance: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildChronicleDataTable {
    pub fn build(self, stack: &mut Stack) -> ChronicleDataTable {
        let out = ChronicleDataTable(Rc::new(ChronicleDataTable_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ChronicleDataTableData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_table_id: self.data_table_id,
                deletion_policy: core::default::Default::default(),
                description: self.description,
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                row_time_to_live: core::default::Default::default(),
                column_info: core::default::Default::default(),
                scope_info: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ChronicleDataTableRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataTableRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ChronicleDataTableRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `approximate_row_count` after provisioning.\nThe count of rows in the data table."]
    pub fn approximate_row_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approximate_row_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTable create time"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_table_id` after provisioning.\nThe ID to use for the data table. This is also the display name for\nthe data table. It must satisfy the following requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256."]
    pub fn data_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_table_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_table_uuid` after provisioning.\nData table unique id"]
    pub fn data_table_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_table_uuid", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nThis field uses a custom implementation please refer to documentation under /hashicorp/terraform-provider-google-beta/website/docs/r/chronicle_data_table.html.markdown for specifics"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA user-provided description of the data table."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe unique display name of the data table."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the data table\nFormat:\n\"{project}/locations/{region}/instances/{instance}/dataTables/{data_table}\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_time_to_live` after provisioning.\nUser-provided TTL of the data table."]
    pub fn row_time_to_live(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.row_time_to_live", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_time_to_live_update_time` after provisioning.\nLast update time of the TTL of the data table."]
    pub fn row_time_to_live_update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.row_time_to_live_update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_associations_count` after provisioning.\nThe count of rules using the data table."]
    pub fn rule_associations_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_associations_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\nThe resource names for the associated Rules that use this\ndata table. Format:\nprojects/{project}/locations/{location}/instances/{instance}/rules/{rule}.\n{rule} here refers to the rule id."]
    pub fn rules(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_source` after provisioning.\nPossible values:\nUSER\nRULE\nSEARCH"]
    pub fn update_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTable update time"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `column_info` after provisioning.\n"]
    pub fn column_info(&self) -> ListRef<ChronicleDataTableColumnInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope_info` after provisioning.\n"]
    pub fn scope_info(&self) -> ListRef<ChronicleDataTableScopeInfoElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scope_info", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleDataTableTimeoutsElRef {
        ChronicleDataTableTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDataTableColumnInfoEl {
    column_index: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_column: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mapped_column_path: Option<PrimField<String>>,
    original_column: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repeated_values: Option<PrimField<bool>>,
}
impl ChronicleDataTableColumnInfoEl {
    #[doc = "Set the field `column_type`.\nColumn type can be STRING, CIDR (Ex- 10.1.1.0/24), REGEX\nPossible values:\nSTRING\nREGEX\nCIDR\nNUMBER Possible values: [\"STRING\", \"REGEX\", \"CIDR\", \"NUMBER\"]"]
    pub fn set_column_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_type = Some(v.into());
        self
    }
    #[doc = "Set the field `key_column`.\nWhether to include this column in the calculation of the row ID.\nIf no columns have key_column = true, all columns will be included in the\ncalculation of the row ID."]
    pub fn set_key_column(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.key_column = Some(v.into());
        self
    }
    #[doc = "Set the field `mapped_column_path`.\nEntity proto field path that the column is mapped to"]
    pub fn set_mapped_column_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mapped_column_path = Some(v.into());
        self
    }
    #[doc = "Set the field `repeated_values`.\nWhether the column is a repeated values column."]
    pub fn set_repeated_values(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.repeated_values = Some(v.into());
        self
    }
}
impl ToListMappable for ChronicleDataTableColumnInfoEl {
    type O = BlockAssignable<ChronicleDataTableColumnInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDataTableColumnInfoEl {
    #[doc = "Column Index. 0,1,2..."]
    pub column_index: PrimField<f64>,
    #[doc = "Original column name of the Data Table (present in the CSV header in case\nof creation of data tables using file uploads). It must satisfy the\nfollowing requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256"]
    pub original_column: PrimField<String>,
}
impl BuildChronicleDataTableColumnInfoEl {
    pub fn build(self) -> ChronicleDataTableColumnInfoEl {
        ChronicleDataTableColumnInfoEl {
            column_index: self.column_index,
            column_type: core::default::Default::default(),
            key_column: core::default::Default::default(),
            mapped_column_path: core::default::Default::default(),
            original_column: self.original_column,
            repeated_values: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDataTableColumnInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataTableColumnInfoElRef {
    fn new(shared: StackShared, base: String) -> ChronicleDataTableColumnInfoElRef {
        ChronicleDataTableColumnInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDataTableColumnInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_index` after provisioning.\nColumn Index. 0,1,2..."]
    pub fn column_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_index", self.base))
    }
    #[doc = "Get a reference to the value of field `column_type` after provisioning.\nColumn type can be STRING, CIDR (Ex- 10.1.1.0/24), REGEX\nPossible values:\nSTRING\nREGEX\nCIDR\nNUMBER Possible values: [\"STRING\", \"REGEX\", \"CIDR\", \"NUMBER\"]"]
    pub fn column_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_type", self.base))
    }
    #[doc = "Get a reference to the value of field `key_column` after provisioning.\nWhether to include this column in the calculation of the row ID.\nIf no columns have key_column = true, all columns will be included in the\ncalculation of the row ID."]
    pub fn key_column(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_column", self.base))
    }
    #[doc = "Get a reference to the value of field `mapped_column_path` after provisioning.\nEntity proto field path that the column is mapped to"]
    pub fn mapped_column_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mapped_column_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `original_column` after provisioning.\nOriginal column name of the Data Table (present in the CSV header in case\nof creation of data tables using file uploads). It must satisfy the\nfollowing requirements:\n- Starts with letter.\n- Contains only letters, numbers and underscore.\n- Must be unique and has length < 256"]
    pub fn original_column(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.original_column", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `repeated_values` after provisioning.\nWhether the column is a repeated values column."]
    pub fn repeated_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repeated_values", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDataTableScopeInfoEl {
    data_access_scopes: ListField<PrimField<String>>,
}
impl ChronicleDataTableScopeInfoEl {}
impl ToListMappable for ChronicleDataTableScopeInfoEl {
    type O = BlockAssignable<ChronicleDataTableScopeInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDataTableScopeInfoEl {
    #[doc = "Contains the list of scope names of the data table. If the list is empty,\nthe data table is treated as unscoped. The scope names should be\nfull resource names and should be of the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope_name}\""]
    pub data_access_scopes: ListField<PrimField<String>>,
}
impl BuildChronicleDataTableScopeInfoEl {
    pub fn build(self) -> ChronicleDataTableScopeInfoEl {
        ChronicleDataTableScopeInfoEl {
            data_access_scopes: self.data_access_scopes,
        }
    }
}
pub struct ChronicleDataTableScopeInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataTableScopeInfoElRef {
    fn new(shared: StackShared, base: String) -> ChronicleDataTableScopeInfoElRef {
        ChronicleDataTableScopeInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDataTableScopeInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_access_scopes` after provisioning.\nContains the list of scope names of the data table. If the list is empty,\nthe data table is treated as unscoped. The scope names should be\nfull resource names and should be of the format:\n\"projects/{project}/locations/{location}/instances/{instance}/dataAccessScopes/{scope_name}\""]
    pub fn data_access_scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_access_scopes", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ChronicleDataTableTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ChronicleDataTableTimeoutsEl {
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
impl ToListMappable for ChronicleDataTableTimeoutsEl {
    type O = BlockAssignable<ChronicleDataTableTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildChronicleDataTableTimeoutsEl {}
impl BuildChronicleDataTableTimeoutsEl {
    pub fn build(self) -> ChronicleDataTableTimeoutsEl {
        ChronicleDataTableTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ChronicleDataTableTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ChronicleDataTableTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ChronicleDataTableTimeoutsElRef {
        ChronicleDataTableTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ChronicleDataTableTimeoutsElRef {
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
struct ChronicleDataTableDynamic {
    column_info: Option<DynamicBlock<ChronicleDataTableColumnInfoEl>>,
    scope_info: Option<DynamicBlock<ChronicleDataTableScopeInfoEl>>,
}
