use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct QuicksightDataSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    data_set_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    import_mode: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_groups: Option<Vec<QuicksightDataSetColumnGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_level_permission_rules: Option<Vec<QuicksightDataSetColumnLevelPermissionRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_set_usage_configuration: Option<Vec<QuicksightDataSetDataSetUsageConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_folders: Option<Vec<QuicksightDataSetFieldFoldersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logical_table_map: Option<Vec<QuicksightDataSetLogicalTableMapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<QuicksightDataSetPermissionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_table_map: Option<Vec<QuicksightDataSetPhysicalTableMapEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_properties: Option<Vec<QuicksightDataSetRefreshPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_level_permission_data_set: Option<Vec<QuicksightDataSetRowLevelPermissionDataSetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_level_permission_tag_configuration:
        Option<Vec<QuicksightDataSetRowLevelPermissionTagConfigurationEl>>,
    dynamic: QuicksightDataSetDynamic,
}
struct QuicksightDataSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<QuicksightDataSetData>,
}
#[derive(Clone)]
pub struct QuicksightDataSet(Rc<QuicksightDataSet_>);
impl QuicksightDataSet {
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
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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
    #[doc = "Set the field `column_groups`.\n"]
    pub fn set_column_groups(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetColumnGroupsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().column_groups = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.column_groups = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `column_level_permission_rules`.\n"]
    pub fn set_column_level_permission_rules(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetColumnLevelPermissionRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().column_level_permission_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .column_level_permission_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_set_usage_configuration`.\n"]
    pub fn set_data_set_usage_configuration(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetDataSetUsageConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_set_usage_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .data_set_usage_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `field_folders`.\n"]
    pub fn set_field_folders(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetFieldFoldersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().field_folders = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.field_folders = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logical_table_map`.\n"]
    pub fn set_logical_table_map(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetLogicalTableMapEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logical_table_map = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logical_table_map = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetPermissionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permissions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permissions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `physical_table_map`.\n"]
    pub fn set_physical_table_map(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetPhysicalTableMapEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().physical_table_map = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.physical_table_map = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `refresh_properties`.\n"]
    pub fn set_refresh_properties(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetRefreshPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().refresh_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.refresh_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `row_level_permission_data_set`.\n"]
    pub fn set_row_level_permission_data_set(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetRowLevelPermissionDataSetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().row_level_permission_data_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .row_level_permission_data_set = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `row_level_permission_tag_configuration`.\n"]
    pub fn set_row_level_permission_tag_configuration(
        self,
        v: impl Into<BlockAssignable<QuicksightDataSetRowLevelPermissionTagConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .row_level_permission_tag_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .row_level_permission_tag_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_mode` after provisioning.\n"]
    pub fn import_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_columns` after provisioning.\n"]
    pub fn output_columns(&self) -> ListRef<QuicksightDataSetOutputColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_columns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `column_groups` after provisioning.\n"]
    pub fn column_groups(&self) -> ListRef<QuicksightDataSetColumnGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `column_level_permission_rules` after provisioning.\n"]
    pub fn column_level_permission_rules(
        &self,
    ) -> ListRef<QuicksightDataSetColumnLevelPermissionRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_level_permission_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_usage_configuration` after provisioning.\n"]
    pub fn data_set_usage_configuration(
        &self,
    ) -> ListRef<QuicksightDataSetDataSetUsageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_set_usage_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_properties` after provisioning.\n"]
    pub fn refresh_properties(&self) -> ListRef<QuicksightDataSetRefreshPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.refresh_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_level_permission_data_set` after provisioning.\n"]
    pub fn row_level_permission_data_set(
        &self,
    ) -> ListRef<QuicksightDataSetRowLevelPermissionDataSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.row_level_permission_data_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_level_permission_tag_configuration` after provisioning.\n"]
    pub fn row_level_permission_tag_configuration(
        &self,
    ) -> ListRef<QuicksightDataSetRowLevelPermissionTagConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.row_level_permission_tag_configuration",
                self.extract_ref()
            ),
        )
    }
}
impl Referable for QuicksightDataSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for QuicksightDataSet {}
impl ToListMappable for QuicksightDataSet {
    type O = ListRef<QuicksightDataSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for QuicksightDataSet_ {
    fn extract_resource_type(&self) -> String {
        "aws_quicksight_data_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildQuicksightDataSet {
    pub tf_id: String,
    #[doc = ""]
    pub data_set_id: PrimField<String>,
    #[doc = ""]
    pub import_mode: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildQuicksightDataSet {
    pub fn build(self, stack: &mut Stack) -> QuicksightDataSet {
        let out = QuicksightDataSet(Rc::new(QuicksightDataSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(QuicksightDataSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aws_account_id: core::default::Default::default(),
                data_set_id: self.data_set_id,
                id: core::default::Default::default(),
                import_mode: self.import_mode,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                column_groups: core::default::Default::default(),
                column_level_permission_rules: core::default::Default::default(),
                data_set_usage_configuration: core::default::Default::default(),
                field_folders: core::default::Default::default(),
                logical_table_map: core::default::Default::default(),
                permissions: core::default::Default::default(),
                physical_table_map: core::default::Default::default(),
                refresh_properties: core::default::Default::default(),
                row_level_permission_data_set: core::default::Default::default(),
                row_level_permission_tag_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct QuicksightDataSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl QuicksightDataSetRef {
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
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `import_mode` after provisioning.\n"]
    pub fn import_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_columns` after provisioning.\n"]
    pub fn output_columns(&self) -> ListRef<QuicksightDataSetOutputColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.output_columns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `column_groups` after provisioning.\n"]
    pub fn column_groups(&self) -> ListRef<QuicksightDataSetColumnGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `column_level_permission_rules` after provisioning.\n"]
    pub fn column_level_permission_rules(
        &self,
    ) -> ListRef<QuicksightDataSetColumnLevelPermissionRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_level_permission_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_usage_configuration` after provisioning.\n"]
    pub fn data_set_usage_configuration(
        &self,
    ) -> ListRef<QuicksightDataSetDataSetUsageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_set_usage_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `refresh_properties` after provisioning.\n"]
    pub fn refresh_properties(&self) -> ListRef<QuicksightDataSetRefreshPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.refresh_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_level_permission_data_set` after provisioning.\n"]
    pub fn row_level_permission_data_set(
        &self,
    ) -> ListRef<QuicksightDataSetRowLevelPermissionDataSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.row_level_permission_data_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_level_permission_tag_configuration` after provisioning.\n"]
    pub fn row_level_permission_tag_configuration(
        &self,
    ) -> ListRef<QuicksightDataSetRowLevelPermissionTagConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.row_level_permission_tag_configuration",
                self.extract_ref()
            ),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetOutputColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl QuicksightDataSetOutputColumnsEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSetOutputColumnsEl {
    type O = BlockAssignable<QuicksightDataSetOutputColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetOutputColumnsEl {}
impl BuildQuicksightDataSetOutputColumnsEl {
    pub fn build(self) -> QuicksightDataSetOutputColumnsEl {
        QuicksightDataSetOutputColumnsEl {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSetOutputColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetOutputColumnsElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetOutputColumnsElRef {
        QuicksightDataSetOutputColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetOutputColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
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
pub struct QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
    columns: ListField<PrimField<String>>,
    country_code: PrimField<String>,
    name: PrimField<String>,
}
impl QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {}
impl ToListMappable for QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
    type O = BlockAssignable<QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
    #[doc = ""]
    pub columns: ListField<PrimField<String>>,
    #[doc = ""]
    pub country_code: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
    pub fn build(self) -> QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
        QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
            columns: self.columns,
            country_code: self.country_code,
            name: self.name,
        }
    }
}
pub struct QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
        QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetColumnGroupsElDynamic {
    geo_spatial_column_group:
        Option<DynamicBlock<QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSetColumnGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    geo_spatial_column_group: Option<Vec<QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl>>,
    dynamic: QuicksightDataSetColumnGroupsElDynamic,
}
impl QuicksightDataSetColumnGroupsEl {
    #[doc = "Set the field `geo_spatial_column_group`.\n"]
    pub fn set_geo_spatial_column_group(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.geo_spatial_column_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_spatial_column_group = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetColumnGroupsEl {
    type O = BlockAssignable<QuicksightDataSetColumnGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetColumnGroupsEl {}
impl BuildQuicksightDataSetColumnGroupsEl {
    pub fn build(self) -> QuicksightDataSetColumnGroupsEl {
        QuicksightDataSetColumnGroupsEl {
            geo_spatial_column_group: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetColumnGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetColumnGroupsElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetColumnGroupsElRef {
        QuicksightDataSetColumnGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetColumnGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `geo_spatial_column_group` after provisioning.\n"]
    pub fn geo_spatial_column_group(
        &self,
    ) -> ListRef<QuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.geo_spatial_column_group", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetColumnLevelPermissionRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principals: Option<ListField<PrimField<String>>>,
}
impl QuicksightDataSetColumnLevelPermissionRulesEl {
    #[doc = "Set the field `column_names`.\n"]
    pub fn set_column_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.column_names = Some(v.into());
        self
    }
    #[doc = "Set the field `principals`.\n"]
    pub fn set_principals(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.principals = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSetColumnLevelPermissionRulesEl {
    type O = BlockAssignable<QuicksightDataSetColumnLevelPermissionRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetColumnLevelPermissionRulesEl {}
impl BuildQuicksightDataSetColumnLevelPermissionRulesEl {
    pub fn build(self) -> QuicksightDataSetColumnLevelPermissionRulesEl {
        QuicksightDataSetColumnLevelPermissionRulesEl {
            column_names: core::default::Default::default(),
            principals: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSetColumnLevelPermissionRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetColumnLevelPermissionRulesElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetColumnLevelPermissionRulesElRef {
        QuicksightDataSetColumnLevelPermissionRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetColumnLevelPermissionRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_names` after provisioning.\n"]
    pub fn column_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.column_names", self.base))
    }
    #[doc = "Get a reference to the value of field `principals` after provisioning.\n"]
    pub fn principals(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.principals", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetDataSetUsageConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_use_as_direct_query_source: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_use_as_imported_source: Option<PrimField<bool>>,
}
impl QuicksightDataSetDataSetUsageConfigurationEl {
    #[doc = "Set the field `disable_use_as_direct_query_source`.\n"]
    pub fn set_disable_use_as_direct_query_source(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_use_as_direct_query_source = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_use_as_imported_source`.\n"]
    pub fn set_disable_use_as_imported_source(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_use_as_imported_source = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSetDataSetUsageConfigurationEl {
    type O = BlockAssignable<QuicksightDataSetDataSetUsageConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetDataSetUsageConfigurationEl {}
impl BuildQuicksightDataSetDataSetUsageConfigurationEl {
    pub fn build(self) -> QuicksightDataSetDataSetUsageConfigurationEl {
        QuicksightDataSetDataSetUsageConfigurationEl {
            disable_use_as_direct_query_source: core::default::Default::default(),
            disable_use_as_imported_source: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSetDataSetUsageConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetDataSetUsageConfigurationElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetDataSetUsageConfigurationElRef {
        QuicksightDataSetDataSetUsageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetDataSetUsageConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_use_as_direct_query_source` after provisioning.\n"]
    pub fn disable_use_as_direct_query_source(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_use_as_direct_query_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_use_as_imported_source` after provisioning.\n"]
    pub fn disable_use_as_imported_source(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_use_as_imported_source", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetFieldFoldersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    field_folders_id: PrimField<String>,
}
impl QuicksightDataSetFieldFoldersEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.columns = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSetFieldFoldersEl {
    type O = BlockAssignable<QuicksightDataSetFieldFoldersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetFieldFoldersEl {
    #[doc = ""]
    pub field_folders_id: PrimField<String>,
}
impl BuildQuicksightDataSetFieldFoldersEl {
    pub fn build(self) -> QuicksightDataSetFieldFoldersEl {
        QuicksightDataSetFieldFoldersEl {
            columns: core::default::Default::default(),
            description: core::default::Default::default(),
            field_folders_id: self.field_folders_id,
        }
    }
}
pub struct QuicksightDataSetFieldFoldersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetFieldFoldersElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetFieldFoldersElRef {
        QuicksightDataSetFieldFoldersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetFieldFoldersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `field_folders_id` after provisioning.\n"]
    pub fn field_folders_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.field_folders_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
    column_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
    new_column_type: PrimField<String>,
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }
}
impl ToListMappable
    for QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl
{
    type O = BlockAssignable<
        QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
    #[doc = ""]
    pub column_name: PrimField<String>,
    #[doc = ""]
    pub new_column_type: PrimField<String>,
}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
    pub fn build(
        self,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
        QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
            column_name: self.column_name,
            format: core::default::Default::default(),
            new_column_type: self.new_column_type,
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
        QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
    #[doc = "Get a reference to the value of field `new_column_type` after provisioning.\n"]
    pub fn new_column_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.new_column_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl {
    column_id: PrimField<String>,
    column_name: PrimField<String>,
    expression: PrimField<String>,
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl {}
impl ToListMappable
    for QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl
{
    type O = BlockAssignable<
        QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl
{
    #[doc = ""]
    pub column_id: PrimField<String>,
    #[doc = ""]
    pub column_name: PrimField<String>,
    #[doc = ""]
    pub expression: PrimField<String>,
}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl {
    pub fn build(
        self,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl {
        QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl {
            column_id: self.column_id,
            column_name: self.column_name,
            expression: self.expression,
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef
    {
        QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_id` after provisioning.\n"]
    pub fn column_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_id", self.base))
    }
    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElDynamic {
    columns: Option<
        DynamicBlock<
            QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<
        Vec<QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl>,
    >,
    dynamic: QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElDynamic,
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl Into<
            BlockAssignable<
                QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl,
            >,
        >,
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
}
impl ToListMappable for QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
    type O =
        BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
    pub fn build(
        self,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
        QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
            columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
        QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> ListRef<
        QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
    condition_expression: PrimField<String>,
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {}
impl ToListMappable for QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
    type O = BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
    #[doc = ""]
    pub condition_expression: PrimField<String>,
}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
    pub fn build(self) -> QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
        QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
            condition_expression: self.condition_expression,
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
        QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition_expression` after provisioning.\n"]
    pub fn condition_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.condition_expression", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
    projected_columns: ListField<PrimField<String>>,
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {}
impl ToListMappable for QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
    type O = BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
    #[doc = ""]
    pub projected_columns: ListField<PrimField<String>>,
}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
    pub fn build(self) -> QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
        QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
            projected_columns: self.projected_columns,
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
        QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `projected_columns` after provisioning.\n"]
    pub fn projected_columns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.projected_columns", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
    column_name: PrimField<String>,
    new_column_name: PrimField<String>,
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {}
impl ToListMappable for QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
    type O =
        BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
    #[doc = ""]
    pub column_name: PrimField<String>,
    #[doc = ""]
    pub new_column_name: PrimField<String>,
}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
    pub fn build(
        self,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
        QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
            column_name: self.column_name,
            new_column_name: self.new_column_name,
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
        QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }
    #[doc = "Get a reference to the value of field `new_column_name` after provisioning.\n"]
    pub fn new_column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.new_column_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl
    QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl
{
    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl { type O = BlockAssignable < QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl
{}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl { pub fn build (self) -> QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl { QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl { text : core :: default :: Default :: default () , } } }
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef { fn new (shared : StackShared , base : String) -> QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef { QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef { shared : shared , base : base . to_string () , } } }
impl QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } }
#[derive(Serialize, Default)]
struct QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElDynamic { column_description : Option < DynamicBlock < QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl >> , }
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl { # [serde (skip_serializing_if = "Option::is_none")] column_geographic_role : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] column_description : Option < Vec < QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl > > , dynamic : QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElDynamic , }
impl QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {
    #[doc = "Set the field `column_geographic_role`.\n"]
    pub fn set_column_geographic_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_geographic_role = Some(v.into());
        self
    }
    #[doc = "Set the field `column_description`.\n"]
    pub fn set_column_description(
        mut self,
        v : impl Into < BlockAssignable < QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.column_description = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.column_description = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl
{
    type O = BlockAssignable<
        QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {
    pub fn build(
        self,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {
        QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {
            column_geographic_role: core::default::Default::default(),
            column_description: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
        QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_geographic_role` after provisioning.\n"]
    pub fn column_geographic_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.column_geographic_role", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `column_description` after provisioning.\n"]    pub fn column_description (& self) -> ListRef < QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_description", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElDynamic {
    tags: Option<
        DynamicBlock<QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl>,
    >,
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
    column_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl>>,
    dynamic: QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElDynamic,
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<
            BlockAssignable<
                QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tags = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
    type O =
        BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
    #[doc = ""]
    pub column_name: PrimField<String>,
}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
    pub fn build(self) -> QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
        QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
            column_name: self.column_name,
            tags: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
        QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
    column_name: PrimField<String>,
    tag_names: ListField<PrimField<String>>,
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {}
impl ToListMappable for QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
    type O =
        BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
    #[doc = ""]
    pub column_name: PrimField<String>,
    #[doc = ""]
    pub tag_names: ListField<PrimField<String>>,
}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
    pub fn build(self) -> QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
        QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
            column_name: self.column_name,
            tag_names: self.tag_names,
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
        QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_names` after provisioning.\n"]
    pub fn tag_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tag_names", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetLogicalTableMapElDataTransformsElDynamic {
    cast_column_type_operation: Option<
        DynamicBlock<QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl>,
    >,
    create_columns_operation: Option<
        DynamicBlock<QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl>,
    >,
    filter_operation:
        Option<DynamicBlock<QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl>>,
    project_operation:
        Option<DynamicBlock<QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl>>,
    rename_column_operation: Option<
        DynamicBlock<QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl>,
    >,
    tag_column_operation: Option<
        DynamicBlock<QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl>,
    >,
    untag_column_operation: Option<
        DynamicBlock<QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl>,
    >,
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElDataTransformsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cast_column_type_operation:
        Option<Vec<QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_columns_operation:
        Option<Vec<QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_operation:
        Option<Vec<QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_operation:
        Option<Vec<QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rename_column_operation:
        Option<Vec<QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_column_operation:
        Option<Vec<QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    untag_column_operation:
        Option<Vec<QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl>>,
    dynamic: QuicksightDataSetLogicalTableMapElDataTransformsElDynamic,
}
impl QuicksightDataSetLogicalTableMapElDataTransformsEl {
    #[doc = "Set the field `cast_column_type_operation`.\n"]
    pub fn set_cast_column_type_operation(
        mut self,
        v: impl Into<
            BlockAssignable<
                QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cast_column_type_operation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cast_column_type_operation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `create_columns_operation`.\n"]
    pub fn set_create_columns_operation(
        mut self,
        v: impl Into<
            BlockAssignable<
                QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.create_columns_operation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.create_columns_operation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `filter_operation`.\n"]
    pub fn set_filter_operation(
        mut self,
        v: impl Into<
            BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_operation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_operation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `project_operation`.\n"]
    pub fn set_project_operation(
        mut self,
        v: impl Into<
            BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.project_operation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.project_operation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rename_column_operation`.\n"]
    pub fn set_rename_column_operation(
        mut self,
        v: impl Into<
            BlockAssignable<
                QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rename_column_operation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rename_column_operation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tag_column_operation`.\n"]
    pub fn set_tag_column_operation(
        mut self,
        v: impl Into<
            BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_column_operation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_column_operation = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `untag_column_operation`.\n"]
    pub fn set_untag_column_operation(
        mut self,
        v: impl Into<
            BlockAssignable<
                QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.untag_column_operation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.untag_column_operation = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetLogicalTableMapElDataTransformsEl {
    type O = BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElDataTransformsEl {}
impl BuildQuicksightDataSetLogicalTableMapElDataTransformsEl {
    pub fn build(self) -> QuicksightDataSetLogicalTableMapElDataTransformsEl {
        QuicksightDataSetLogicalTableMapElDataTransformsEl {
            cast_column_type_operation: core::default::Default::default(),
            create_columns_operation: core::default::Default::default(),
            filter_operation: core::default::Default::default(),
            project_operation: core::default::Default::default(),
            rename_column_operation: core::default::Default::default(),
            tag_column_operation: core::default::Default::default(),
            untag_column_operation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElDataTransformsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElDataTransformsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElDataTransformsElRef {
        QuicksightDataSetLogicalTableMapElDataTransformsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElDataTransformsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cast_column_type_operation` after provisioning.\n"]
    pub fn cast_column_type_operation(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cast_column_type_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_columns_operation` after provisioning.\n"]
    pub fn create_columns_operation(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_columns_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_operation` after provisioning.\n"]
    pub fn filter_operation(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project_operation` after provisioning.\n"]
    pub fn project_operation(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.project_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rename_column_operation` after provisioning.\n"]
    pub fn rename_column_operation(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rename_column_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_column_operation` after provisioning.\n"]
    pub fn tag_column_operation(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_column_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `untag_column_operation` after provisioning.\n"]
    pub fn untag_column_operation(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.untag_column_operation", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_key: Option<PrimField<bool>>,
}
impl QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl {
    #[doc = "Set the field `unique_key`.\n"]
    pub fn set_unique_key(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_key = Some(v.into());
        self
    }
}
impl ToListMappable
    for QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl
{
    type O = BlockAssignable<
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl
{}
impl BuildQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl {
    pub fn build(
        self,
    ) -> QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl {
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl {
            unique_key: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef {
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unique_key` after provisioning.\n"]
    pub fn unique_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_key", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_key: Option<PrimField<bool>>,
}
impl QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl {
    #[doc = "Set the field `unique_key`.\n"]
    pub fn set_unique_key(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_key = Some(v.into());
        self
    }
}
impl ToListMappable
    for QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl
{
    type O = BlockAssignable<
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl
{}
impl BuildQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl {
    pub fn build(
        self,
    ) -> QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl {
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl {
            unique_key: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef
    {
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unique_key` after provisioning.\n"]
    pub fn unique_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_key", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElDynamic {
    left_join_key_properties: Option<
        DynamicBlock<
            QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl,
        >,
    >,
    right_join_key_properties: Option<
        DynamicBlock<
            QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
    left_operand: PrimField<String>,
    on_clause: PrimField<String>,
    right_operand: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    left_join_key_properties: Option<
        Vec<QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    right_join_key_properties: Option<
        Vec<QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl>,
    >,
    dynamic: QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElDynamic,
}
impl QuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
    #[doc = "Set the field `left_join_key_properties`.\n"]
    pub fn set_left_join_key_properties(
        mut self,
        v: impl Into<
            BlockAssignable<
                QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.left_join_key_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.left_join_key_properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `right_join_key_properties`.\n"]
    pub fn set_right_join_key_properties(
        mut self,
        v: impl Into<
            BlockAssignable<
                QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.right_join_key_properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.right_join_key_properties = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
    type O = BlockAssignable<QuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
    #[doc = ""]
    pub left_operand: PrimField<String>,
    #[doc = ""]
    pub on_clause: PrimField<String>,
    #[doc = ""]
    pub right_operand: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
    pub fn build(self) -> QuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
            left_operand: self.left_operand,
            on_clause: self.on_clause,
            right_operand: self.right_operand,
            type_: self.type_,
            left_join_key_properties: core::default::Default::default(),
            right_join_key_properties: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `left_operand` after provisioning.\n"]
    pub fn left_operand(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.left_operand", self.base))
    }
    #[doc = "Get a reference to the value of field `on_clause` after provisioning.\n"]
    pub fn on_clause(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_clause", self.base))
    }
    #[doc = "Get a reference to the value of field `right_operand` after provisioning.\n"]
    pub fn right_operand(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.right_operand", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `left_join_key_properties` after provisioning.\n"]
    pub fn left_join_key_properties(
        &self,
    ) -> ListRef<
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.left_join_key_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `right_join_key_properties` after provisioning.\n"]
    pub fn right_join_key_properties(
        &self,
    ) -> ListRef<
        QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.right_join_key_properties", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetLogicalTableMapElSourceElDynamic {
    join_instruction:
        Option<DynamicBlock<QuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapElSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_set_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_table_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    join_instruction: Option<Vec<QuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl>>,
    dynamic: QuicksightDataSetLogicalTableMapElSourceElDynamic,
}
impl QuicksightDataSetLogicalTableMapElSourceEl {
    #[doc = "Set the field `data_set_arn`.\n"]
    pub fn set_data_set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_set_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `physical_table_id`.\n"]
    pub fn set_physical_table_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.physical_table_id = Some(v.into());
        self
    }
    #[doc = "Set the field `join_instruction`.\n"]
    pub fn set_join_instruction(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.join_instruction = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.join_instruction = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetLogicalTableMapElSourceEl {
    type O = BlockAssignable<QuicksightDataSetLogicalTableMapElSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapElSourceEl {}
impl BuildQuicksightDataSetLogicalTableMapElSourceEl {
    pub fn build(self) -> QuicksightDataSetLogicalTableMapElSourceEl {
        QuicksightDataSetLogicalTableMapElSourceEl {
            data_set_arn: core::default::Default::default(),
            physical_table_id: core::default::Default::default(),
            join_instruction: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElSourceElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetLogicalTableMapElSourceElRef {
        QuicksightDataSetLogicalTableMapElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_set_arn` after provisioning.\n"]
    pub fn data_set_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_set_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `physical_table_id` after provisioning.\n"]
    pub fn physical_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.physical_table_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `join_instruction` after provisioning.\n"]
    pub fn join_instruction(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.join_instruction", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetLogicalTableMapElDynamic {
    data_transforms: Option<DynamicBlock<QuicksightDataSetLogicalTableMapElDataTransformsEl>>,
    source: Option<DynamicBlock<QuicksightDataSetLogicalTableMapElSourceEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSetLogicalTableMapEl {
    alias: PrimField<String>,
    logical_table_map_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_transforms: Option<Vec<QuicksightDataSetLogicalTableMapElDataTransformsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<QuicksightDataSetLogicalTableMapElSourceEl>>,
    dynamic: QuicksightDataSetLogicalTableMapElDynamic,
}
impl QuicksightDataSetLogicalTableMapEl {
    #[doc = "Set the field `data_transforms`.\n"]
    pub fn set_data_transforms(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetLogicalTableMapElDataTransformsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_transforms = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_transforms = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetLogicalTableMapElSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetLogicalTableMapEl {
    type O = BlockAssignable<QuicksightDataSetLogicalTableMapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetLogicalTableMapEl {
    #[doc = ""]
    pub alias: PrimField<String>,
    #[doc = ""]
    pub logical_table_map_id: PrimField<String>,
}
impl BuildQuicksightDataSetLogicalTableMapEl {
    pub fn build(self) -> QuicksightDataSetLogicalTableMapEl {
        QuicksightDataSetLogicalTableMapEl {
            alias: self.alias,
            logical_table_map_id: self.logical_table_map_id,
            data_transforms: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetLogicalTableMapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetLogicalTableMapElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetLogicalTableMapElRef {
        QuicksightDataSetLogicalTableMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetLogicalTableMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.base))
    }
    #[doc = "Get a reference to the value of field `logical_table_map_id` after provisioning.\n"]
    pub fn logical_table_map_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logical_table_map_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_transforms` after provisioning.\n"]
    pub fn data_transforms(
        &self,
    ) -> ListRef<QuicksightDataSetLogicalTableMapElDataTransformsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_transforms", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<QuicksightDataSetLogicalTableMapElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetPermissionsEl {
    actions: SetField<PrimField<String>>,
    principal: PrimField<String>,
}
impl QuicksightDataSetPermissionsEl {}
impl ToListMappable for QuicksightDataSetPermissionsEl {
    type O = BlockAssignable<QuicksightDataSetPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetPermissionsEl {
    #[doc = ""]
    pub actions: SetField<PrimField<String>>,
    #[doc = ""]
    pub principal: PrimField<String>,
}
impl BuildQuicksightDataSetPermissionsEl {
    pub fn build(self) -> QuicksightDataSetPermissionsEl {
        QuicksightDataSetPermissionsEl {
            actions: self.actions,
            principal: self.principal,
        }
    }
}
pub struct QuicksightDataSetPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetPermissionsElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetPermissionsElRef {
        QuicksightDataSetPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {}
impl ToListMappable for QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
    type O = BlockAssignable<QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
    pub fn build(self) -> QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
        QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
            name: self.name,
            type_: self.type_,
        }
    }
}
pub struct QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
        QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
#[derive(Serialize, Default)]
struct QuicksightDataSetPhysicalTableMapElCustomSqlElDynamic {
    columns: Option<DynamicBlock<QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSetPhysicalTableMapElCustomSqlEl {
    data_source_arn: PrimField<String>,
    name: PrimField<String>,
    sql_query: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<Vec<QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl>>,
    dynamic: QuicksightDataSetPhysicalTableMapElCustomSqlElDynamic,
}
impl QuicksightDataSetPhysicalTableMapElCustomSqlEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl>>,
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
}
impl ToListMappable for QuicksightDataSetPhysicalTableMapElCustomSqlEl {
    type O = BlockAssignable<QuicksightDataSetPhysicalTableMapElCustomSqlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetPhysicalTableMapElCustomSqlEl {
    #[doc = ""]
    pub data_source_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub sql_query: PrimField<String>,
}
impl BuildQuicksightDataSetPhysicalTableMapElCustomSqlEl {
    pub fn build(self) -> QuicksightDataSetPhysicalTableMapElCustomSqlEl {
        QuicksightDataSetPhysicalTableMapElCustomSqlEl {
            data_source_arn: self.data_source_arn,
            name: self.name,
            sql_query: self.sql_query,
            columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetPhysicalTableMapElCustomSqlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetPhysicalTableMapElCustomSqlElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetPhysicalTableMapElCustomSqlElRef {
        QuicksightDataSetPhysicalTableMapElCustomSqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetPhysicalTableMapElCustomSqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `sql_query` after provisioning.\n"]
    pub fn sql_query(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sql_query", self.base))
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> ListRef<QuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {}
impl ToListMappable for QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
    type O = BlockAssignable<QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
    pub fn build(self) -> QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
        QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
            name: self.name,
            type_: self.type_,
        }
    }
}
pub struct QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
        QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
#[derive(Serialize, Default)]
struct QuicksightDataSetPhysicalTableMapElRelationalTableElDynamic {
    input_columns:
        Option<DynamicBlock<QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSetPhysicalTableMapElRelationalTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog: Option<PrimField<String>>,
    data_source_arn: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_columns: Option<Vec<QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl>>,
    dynamic: QuicksightDataSetPhysicalTableMapElRelationalTableElDynamic,
}
impl QuicksightDataSetPhysicalTableMapElRelationalTableEl {
    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog = Some(v.into());
        self
    }
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema = Some(v.into());
        self
    }
    #[doc = "Set the field `input_columns`.\n"]
    pub fn set_input_columns(
        mut self,
        v: impl Into<
            BlockAssignable<QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetPhysicalTableMapElRelationalTableEl {
    type O = BlockAssignable<QuicksightDataSetPhysicalTableMapElRelationalTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetPhysicalTableMapElRelationalTableEl {
    #[doc = ""]
    pub data_source_arn: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildQuicksightDataSetPhysicalTableMapElRelationalTableEl {
    pub fn build(self) -> QuicksightDataSetPhysicalTableMapElRelationalTableEl {
        QuicksightDataSetPhysicalTableMapElRelationalTableEl {
            catalog: core::default::Default::default(),
            data_source_arn: self.data_source_arn,
            name: self.name,
            schema: core::default::Default::default(),
            input_columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetPhysicalTableMapElRelationalTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetPhysicalTableMapElRelationalTableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetPhysicalTableMapElRelationalTableElRef {
        QuicksightDataSetPhysicalTableMapElRelationalTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetPhysicalTableMapElRelationalTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.base))
    }
    #[doc = "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
    #[doc = "Get a reference to the value of field `input_columns` after provisioning.\n"]
    pub fn input_columns(
        &self,
    ) -> ListRef<QuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_columns", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {}
impl ToListMappable for QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
    type O = BlockAssignable<QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
    pub fn build(self) -> QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
        QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
            name: self.name,
            type_: self.type_,
        }
    }
}
pub struct QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
        QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
pub struct QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contains_header: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delimiter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_from_row: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_qualifier: Option<PrimField<String>>,
}
impl QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
    #[doc = "Set the field `contains_header`.\n"]
    pub fn set_contains_header(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.contains_header = Some(v.into());
        self
    }
    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delimiter = Some(v.into());
        self
    }
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }
    #[doc = "Set the field `start_from_row`.\n"]
    pub fn set_start_from_row(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.start_from_row = Some(v.into());
        self
    }
    #[doc = "Set the field `text_qualifier`.\n"]
    pub fn set_text_qualifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.text_qualifier = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
    type O = BlockAssignable<QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {}
impl BuildQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
    pub fn build(self) -> QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
        QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
            contains_header: core::default::Default::default(),
            delimiter: core::default::Default::default(),
            format: core::default::Default::default(),
            start_from_row: core::default::Default::default(),
            text_qualifier: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
        QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contains_header` after provisioning.\n"]
    pub fn contains_header(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contains_header", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
    #[doc = "Get a reference to the value of field `start_from_row` after provisioning.\n"]
    pub fn start_from_row(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_from_row", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `text_qualifier` after provisioning.\n"]
    pub fn text_qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.text_qualifier", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetPhysicalTableMapElS3SourceElDynamic {
    input_columns:
        Option<DynamicBlock<QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl>>,
    upload_settings:
        Option<DynamicBlock<QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSetPhysicalTableMapElS3SourceEl {
    data_source_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_columns: Option<Vec<QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_settings: Option<Vec<QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl>>,
    dynamic: QuicksightDataSetPhysicalTableMapElS3SourceElDynamic,
}
impl QuicksightDataSetPhysicalTableMapElS3SourceEl {
    #[doc = "Set the field `input_columns`.\n"]
    pub fn set_input_columns(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.input_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.input_columns = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `upload_settings`.\n"]
    pub fn set_upload_settings(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.upload_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.upload_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetPhysicalTableMapElS3SourceEl {
    type O = BlockAssignable<QuicksightDataSetPhysicalTableMapElS3SourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetPhysicalTableMapElS3SourceEl {
    #[doc = ""]
    pub data_source_arn: PrimField<String>,
}
impl BuildQuicksightDataSetPhysicalTableMapElS3SourceEl {
    pub fn build(self) -> QuicksightDataSetPhysicalTableMapElS3SourceEl {
        QuicksightDataSetPhysicalTableMapElS3SourceEl {
            data_source_arn: self.data_source_arn,
            input_columns: core::default::Default::default(),
            upload_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetPhysicalTableMapElS3SourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetPhysicalTableMapElS3SourceElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetPhysicalTableMapElS3SourceElRef {
        QuicksightDataSetPhysicalTableMapElS3SourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetPhysicalTableMapElS3SourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_source_arn` after provisioning.\n"]
    pub fn data_source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `input_columns` after provisioning.\n"]
    pub fn input_columns(
        &self,
    ) -> ListRef<QuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_columns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upload_settings` after provisioning.\n"]
    pub fn upload_settings(
        &self,
    ) -> ListRef<QuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upload_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetPhysicalTableMapElDynamic {
    custom_sql: Option<DynamicBlock<QuicksightDataSetPhysicalTableMapElCustomSqlEl>>,
    relational_table: Option<DynamicBlock<QuicksightDataSetPhysicalTableMapElRelationalTableEl>>,
    s3_source: Option<DynamicBlock<QuicksightDataSetPhysicalTableMapElS3SourceEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSetPhysicalTableMapEl {
    physical_table_map_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_sql: Option<Vec<QuicksightDataSetPhysicalTableMapElCustomSqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relational_table: Option<Vec<QuicksightDataSetPhysicalTableMapElRelationalTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_source: Option<Vec<QuicksightDataSetPhysicalTableMapElS3SourceEl>>,
    dynamic: QuicksightDataSetPhysicalTableMapElDynamic,
}
impl QuicksightDataSetPhysicalTableMapEl {
    #[doc = "Set the field `custom_sql`.\n"]
    pub fn set_custom_sql(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetPhysicalTableMapElCustomSqlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_sql = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_sql = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `relational_table`.\n"]
    pub fn set_relational_table(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetPhysicalTableMapElRelationalTableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.relational_table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.relational_table = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_source`.\n"]
    pub fn set_s3_source(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetPhysicalTableMapElS3SourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetPhysicalTableMapEl {
    type O = BlockAssignable<QuicksightDataSetPhysicalTableMapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetPhysicalTableMapEl {
    #[doc = ""]
    pub physical_table_map_id: PrimField<String>,
}
impl BuildQuicksightDataSetPhysicalTableMapEl {
    pub fn build(self) -> QuicksightDataSetPhysicalTableMapEl {
        QuicksightDataSetPhysicalTableMapEl {
            physical_table_map_id: self.physical_table_map_id,
            custom_sql: core::default::Default::default(),
            relational_table: core::default::Default::default(),
            s3_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetPhysicalTableMapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetPhysicalTableMapElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetPhysicalTableMapElRef {
        QuicksightDataSetPhysicalTableMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetPhysicalTableMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `physical_table_map_id` after provisioning.\n"]
    pub fn physical_table_map_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.physical_table_map_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_sql` after provisioning.\n"]
    pub fn custom_sql(&self) -> ListRef<QuicksightDataSetPhysicalTableMapElCustomSqlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_sql", self.base))
    }
    #[doc = "Get a reference to the value of field `relational_table` after provisioning.\n"]
    pub fn relational_table(
        &self,
    ) -> ListRef<QuicksightDataSetPhysicalTableMapElRelationalTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.relational_table", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_source` after provisioning.\n"]
    pub fn s3_source(&self) -> ListRef<QuicksightDataSetPhysicalTableMapElS3SourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_source", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl
{
    column_name: PrimField<String>,
    size: PrimField<f64>,
    size_unit: PrimField<String>,
}
impl
    QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl
{
}
impl ToListMappable for QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl { type O = BlockAssignable < QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildQuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl
{
    #[doc = ""]
    pub column_name: PrimField<String>,
    #[doc = ""]
    pub size: PrimField<f64>,
    #[doc = ""]
    pub size_unit: PrimField<String>,
}
impl BuildQuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl { pub fn build (self) -> QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl { QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl { column_name : self . column_name , size : self . size , size_unit : self . size_unit , } } }
pub struct QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowElRef { fn new (shared : StackShared , base : String) -> QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowElRef { QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowElRef { shared : shared , base : base . to_string () , } } }
impl QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `column_name` after provisioning.\n"] pub fn column_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.column_name" , self . base)) } # [doc = "Get a reference to the value of field `size` after provisioning.\n"] pub fn size (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.size" , self . base)) } # [doc = "Get a reference to the value of field `size_unit` after provisioning.\n"] pub fn size_unit (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.size_unit" , self . base)) } }
#[derive(Serialize, Default)]
struct QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElDynamic { lookback_window : Option < DynamicBlock < QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl >> , }
#[derive(Serialize)]
pub struct QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl { # [serde (skip_serializing_if = "Option::is_none")] lookback_window : Option < Vec < QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl > > , dynamic : QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElDynamic , }
impl QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl {
    #[doc = "Set the field `lookback_window`.\n"]
    pub fn set_lookback_window(
        mut self,
        v : impl Into < BlockAssignable < QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lookback_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lookback_window = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl
{
    type O = BlockAssignable<
        QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl {}
impl BuildQuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl {
    pub fn build(
        self,
    ) -> QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl {
        QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl {
            lookback_window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElRef {
        QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lookback_window` after provisioning.\n"]    pub fn lookback_window (& self) -> ListRef < QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElLookbackWindowElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.lookback_window", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetRefreshPropertiesElRefreshConfigurationElDynamic {
    incremental_refresh: Option<
        DynamicBlock<
            QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct QuicksightDataSetRefreshPropertiesElRefreshConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    incremental_refresh:
        Option<Vec<QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl>>,
    dynamic: QuicksightDataSetRefreshPropertiesElRefreshConfigurationElDynamic,
}
impl QuicksightDataSetRefreshPropertiesElRefreshConfigurationEl {
    #[doc = "Set the field `incremental_refresh`.\n"]
    pub fn set_incremental_refresh(
        mut self,
        v: impl Into<
            BlockAssignable<
                QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.incremental_refresh = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.incremental_refresh = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetRefreshPropertiesElRefreshConfigurationEl {
    type O = BlockAssignable<QuicksightDataSetRefreshPropertiesElRefreshConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetRefreshPropertiesElRefreshConfigurationEl {}
impl BuildQuicksightDataSetRefreshPropertiesElRefreshConfigurationEl {
    pub fn build(self) -> QuicksightDataSetRefreshPropertiesElRefreshConfigurationEl {
        QuicksightDataSetRefreshPropertiesElRefreshConfigurationEl {
            incremental_refresh: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetRefreshPropertiesElRefreshConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetRefreshPropertiesElRefreshConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetRefreshPropertiesElRefreshConfigurationElRef {
        QuicksightDataSetRefreshPropertiesElRefreshConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetRefreshPropertiesElRefreshConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `incremental_refresh` after provisioning.\n"]
    pub fn incremental_refresh(
        &self,
    ) -> ListRef<QuicksightDataSetRefreshPropertiesElRefreshConfigurationElIncrementalRefreshElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.incremental_refresh", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetRefreshPropertiesElDynamic {
    refresh_configuration:
        Option<DynamicBlock<QuicksightDataSetRefreshPropertiesElRefreshConfigurationEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSetRefreshPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_configuration: Option<Vec<QuicksightDataSetRefreshPropertiesElRefreshConfigurationEl>>,
    dynamic: QuicksightDataSetRefreshPropertiesElDynamic,
}
impl QuicksightDataSetRefreshPropertiesEl {
    #[doc = "Set the field `refresh_configuration`.\n"]
    pub fn set_refresh_configuration(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetRefreshPropertiesElRefreshConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.refresh_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.refresh_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetRefreshPropertiesEl {
    type O = BlockAssignable<QuicksightDataSetRefreshPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetRefreshPropertiesEl {}
impl BuildQuicksightDataSetRefreshPropertiesEl {
    pub fn build(self) -> QuicksightDataSetRefreshPropertiesEl {
        QuicksightDataSetRefreshPropertiesEl {
            refresh_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetRefreshPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetRefreshPropertiesElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetRefreshPropertiesElRef {
        QuicksightDataSetRefreshPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetRefreshPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `refresh_configuration` after provisioning.\n"]
    pub fn refresh_configuration(
        &self,
    ) -> ListRef<QuicksightDataSetRefreshPropertiesElRefreshConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.refresh_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetRowLevelPermissionDataSetEl {
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    permission_policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl QuicksightDataSetRowLevelPermissionDataSetEl {
    #[doc = "Set the field `format_version`.\n"]
    pub fn set_format_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format_version = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSetRowLevelPermissionDataSetEl {
    type O = BlockAssignable<QuicksightDataSetRowLevelPermissionDataSetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetRowLevelPermissionDataSetEl {
    #[doc = ""]
    pub arn: PrimField<String>,
    #[doc = ""]
    pub permission_policy: PrimField<String>,
}
impl BuildQuicksightDataSetRowLevelPermissionDataSetEl {
    pub fn build(self) -> QuicksightDataSetRowLevelPermissionDataSetEl {
        QuicksightDataSetRowLevelPermissionDataSetEl {
            arn: self.arn,
            format_version: core::default::Default::default(),
            namespace: core::default::Default::default(),
            permission_policy: self.permission_policy,
            status: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSetRowLevelPermissionDataSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetRowLevelPermissionDataSetElRef {
    fn new(shared: StackShared, base: String) -> QuicksightDataSetRowLevelPermissionDataSetElRef {
        QuicksightDataSetRowLevelPermissionDataSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetRowLevelPermissionDataSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `format_version` after provisioning.\n"]
    pub fn format_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.format_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `permission_policy` after provisioning.\n"]
    pub fn permission_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.permission_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
    column_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_all_value: Option<PrimField<String>>,
    tag_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_multi_value_delimiter: Option<PrimField<String>>,
}
impl QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
    #[doc = "Set the field `match_all_value`.\n"]
    pub fn set_match_all_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_all_value = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_multi_value_delimiter`.\n"]
    pub fn set_tag_multi_value_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_multi_value_delimiter = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
    type O = BlockAssignable<QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
    #[doc = ""]
    pub column_name: PrimField<String>,
    #[doc = ""]
    pub tag_key: PrimField<String>,
}
impl BuildQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
    pub fn build(self) -> QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
        QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
            column_name: self.column_name,
            match_all_value: core::default::Default::default(),
            tag_key: self.tag_key,
            tag_multi_value_delimiter: core::default::Default::default(),
        }
    }
}
pub struct QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
        QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_name` after provisioning.\n"]
    pub fn column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.column_name", self.base))
    }
    #[doc = "Get a reference to the value of field `match_all_value` after provisioning.\n"]
    pub fn match_all_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.match_all_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_key` after provisioning.\n"]
    pub fn tag_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag_key", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_multi_value_delimiter` after provisioning.\n"]
    pub fn tag_multi_value_delimiter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tag_multi_value_delimiter", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetRowLevelPermissionTagConfigurationElDynamic {
    tag_rules:
        Option<DynamicBlock<QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl>>,
}
#[derive(Serialize)]
pub struct QuicksightDataSetRowLevelPermissionTagConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_rules: Option<Vec<QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl>>,
    dynamic: QuicksightDataSetRowLevelPermissionTagConfigurationElDynamic,
}
impl QuicksightDataSetRowLevelPermissionTagConfigurationEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_rules`.\n"]
    pub fn set_tag_rules(
        mut self,
        v: impl Into<BlockAssignable<QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tag_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for QuicksightDataSetRowLevelPermissionTagConfigurationEl {
    type O = BlockAssignable<QuicksightDataSetRowLevelPermissionTagConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightDataSetRowLevelPermissionTagConfigurationEl {}
impl BuildQuicksightDataSetRowLevelPermissionTagConfigurationEl {
    pub fn build(self) -> QuicksightDataSetRowLevelPermissionTagConfigurationEl {
        QuicksightDataSetRowLevelPermissionTagConfigurationEl {
            status: core::default::Default::default(),
            tag_rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct QuicksightDataSetRowLevelPermissionTagConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightDataSetRowLevelPermissionTagConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> QuicksightDataSetRowLevelPermissionTagConfigurationElRef {
        QuicksightDataSetRowLevelPermissionTagConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightDataSetRowLevelPermissionTagConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `tag_rules` after provisioning.\n"]
    pub fn tag_rules(
        &self,
    ) -> ListRef<QuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tag_rules", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightDataSetDynamic {
    column_groups: Option<DynamicBlock<QuicksightDataSetColumnGroupsEl>>,
    column_level_permission_rules:
        Option<DynamicBlock<QuicksightDataSetColumnLevelPermissionRulesEl>>,
    data_set_usage_configuration:
        Option<DynamicBlock<QuicksightDataSetDataSetUsageConfigurationEl>>,
    field_folders: Option<DynamicBlock<QuicksightDataSetFieldFoldersEl>>,
    logical_table_map: Option<DynamicBlock<QuicksightDataSetLogicalTableMapEl>>,
    permissions: Option<DynamicBlock<QuicksightDataSetPermissionsEl>>,
    physical_table_map: Option<DynamicBlock<QuicksightDataSetPhysicalTableMapEl>>,
    refresh_properties: Option<DynamicBlock<QuicksightDataSetRefreshPropertiesEl>>,
    row_level_permission_data_set:
        Option<DynamicBlock<QuicksightDataSetRowLevelPermissionDataSetEl>>,
    row_level_permission_tag_configuration:
        Option<DynamicBlock<QuicksightDataSetRowLevelPermissionTagConfigurationEl>>,
}
