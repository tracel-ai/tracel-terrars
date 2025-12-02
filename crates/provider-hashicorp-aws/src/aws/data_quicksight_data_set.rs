use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataQuicksightDataSetData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    data_set_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataQuicksightDataSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataQuicksightDataSetData>,
}
#[derive(Clone)]
pub struct DataQuicksightDataSet(Rc<DataQuicksightDataSet_>);
impl DataQuicksightDataSet {
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
    #[doc = "Get a reference to the value of field `column_groups` after provisioning.\n"]
    pub fn column_groups(&self) -> ListRef<DataQuicksightDataSetColumnGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `column_level_permission_rules` after provisioning.\n"]
    pub fn column_level_permission_rules(
        &self,
    ) -> ListRef<DataQuicksightDataSetColumnLevelPermissionRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_level_permission_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_usage_configuration` after provisioning.\n"]
    pub fn data_set_usage_configuration(
        &self,
    ) -> ListRef<DataQuicksightDataSetDataSetUsageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_set_usage_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `field_folders` after provisioning.\n"]
    pub fn field_folders(&self) -> SetRef<DataQuicksightDataSetFieldFoldersElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.field_folders", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `logical_table_map` after provisioning.\n"]
    pub fn logical_table_map(&self) -> SetRef<DataQuicksightDataSetLogicalTableMapElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.logical_table_map", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<DataQuicksightDataSetPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `physical_table_map` after provisioning.\n"]
    pub fn physical_table_map(&self) -> SetRef<DataQuicksightDataSetPhysicalTableMapElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.physical_table_map", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_level_permission_data_set` after provisioning.\n"]
    pub fn row_level_permission_data_set(
        &self,
    ) -> ListRef<DataQuicksightDataSetRowLevelPermissionDataSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.row_level_permission_data_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_level_permission_tag_configuration` after provisioning.\n"]
    pub fn row_level_permission_tag_configuration(
        &self,
    ) -> ListRef<DataQuicksightDataSetRowLevelPermissionTagConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.row_level_permission_tag_configuration",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
impl Referable for DataQuicksightDataSet {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataQuicksightDataSet {}
impl ToListMappable for DataQuicksightDataSet {
    type O = ListRef<DataQuicksightDataSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataQuicksightDataSet_ {
    fn extract_datasource_type(&self) -> String {
        "aws_quicksight_data_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataQuicksightDataSet {
    pub tf_id: String,
    #[doc = ""]
    pub data_set_id: PrimField<String>,
}
impl BuildDataQuicksightDataSet {
    pub fn build(self, stack: &mut Stack) -> DataQuicksightDataSet {
        let out = DataQuicksightDataSet(Rc::new(DataQuicksightDataSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataQuicksightDataSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                aws_account_id: core::default::Default::default(),
                data_set_id: self.data_set_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataQuicksightDataSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataQuicksightDataSetRef {
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
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `column_groups` after provisioning.\n"]
    pub fn column_groups(&self) -> ListRef<DataQuicksightDataSetColumnGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `column_level_permission_rules` after provisioning.\n"]
    pub fn column_level_permission_rules(
        &self,
    ) -> ListRef<DataQuicksightDataSetColumnLevelPermissionRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_level_permission_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_usage_configuration` after provisioning.\n"]
    pub fn data_set_usage_configuration(
        &self,
    ) -> ListRef<DataQuicksightDataSetDataSetUsageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_set_usage_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `field_folders` after provisioning.\n"]
    pub fn field_folders(&self) -> SetRef<DataQuicksightDataSetFieldFoldersElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.field_folders", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `logical_table_map` after provisioning.\n"]
    pub fn logical_table_map(&self) -> SetRef<DataQuicksightDataSetLogicalTableMapElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.logical_table_map", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<DataQuicksightDataSetPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `physical_table_map` after provisioning.\n"]
    pub fn physical_table_map(&self) -> SetRef<DataQuicksightDataSetPhysicalTableMapElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.physical_table_map", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_level_permission_data_set` after provisioning.\n"]
    pub fn row_level_permission_data_set(
        &self,
    ) -> ListRef<DataQuicksightDataSetRowLevelPermissionDataSetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.row_level_permission_data_set", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `row_level_permission_tag_configuration` after provisioning.\n"]
    pub fn row_level_permission_tag_configuration(
        &self,
    ) -> ListRef<DataQuicksightDataSetRowLevelPermissionTagConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.row_level_permission_tag_configuration",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.columns = Some(v.into());
        self
    }
    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country_code = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
    type O = BlockAssignable<DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {}
impl BuildDataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
    pub fn build(self) -> DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
        DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl {
            columns: core::default::Default::default(),
            country_code: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
        DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef {
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
#[derive(Serialize)]
pub struct DataQuicksightDataSetColumnGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    geo_spatial_column_group:
        Option<ListField<DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl>>,
}
impl DataQuicksightDataSetColumnGroupsEl {
    #[doc = "Set the field `geo_spatial_column_group`.\n"]
    pub fn set_geo_spatial_column_group(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupEl>>,
    ) -> Self {
        self.geo_spatial_column_group = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetColumnGroupsEl {
    type O = BlockAssignable<DataQuicksightDataSetColumnGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetColumnGroupsEl {}
impl BuildDataQuicksightDataSetColumnGroupsEl {
    pub fn build(self) -> DataQuicksightDataSetColumnGroupsEl {
        DataQuicksightDataSetColumnGroupsEl {
            geo_spatial_column_group: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetColumnGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetColumnGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightDataSetColumnGroupsElRef {
        DataQuicksightDataSetColumnGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetColumnGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `geo_spatial_column_group` after provisioning.\n"]
    pub fn geo_spatial_column_group(
        &self,
    ) -> ListRef<DataQuicksightDataSetColumnGroupsElGeoSpatialColumnGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.geo_spatial_column_group", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetColumnLevelPermissionRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_names: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principals: Option<ListField<PrimField<String>>>,
}
impl DataQuicksightDataSetColumnLevelPermissionRulesEl {
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
impl ToListMappable for DataQuicksightDataSetColumnLevelPermissionRulesEl {
    type O = BlockAssignable<DataQuicksightDataSetColumnLevelPermissionRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetColumnLevelPermissionRulesEl {}
impl BuildDataQuicksightDataSetColumnLevelPermissionRulesEl {
    pub fn build(self) -> DataQuicksightDataSetColumnLevelPermissionRulesEl {
        DataQuicksightDataSetColumnLevelPermissionRulesEl {
            column_names: core::default::Default::default(),
            principals: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetColumnLevelPermissionRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetColumnLevelPermissionRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetColumnLevelPermissionRulesElRef {
        DataQuicksightDataSetColumnLevelPermissionRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetColumnLevelPermissionRulesElRef {
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
pub struct DataQuicksightDataSetDataSetUsageConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_use_as_direct_query_source: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_use_as_imported_source: Option<PrimField<bool>>,
}
impl DataQuicksightDataSetDataSetUsageConfigurationEl {
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
impl ToListMappable for DataQuicksightDataSetDataSetUsageConfigurationEl {
    type O = BlockAssignable<DataQuicksightDataSetDataSetUsageConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetDataSetUsageConfigurationEl {}
impl BuildDataQuicksightDataSetDataSetUsageConfigurationEl {
    pub fn build(self) -> DataQuicksightDataSetDataSetUsageConfigurationEl {
        DataQuicksightDataSetDataSetUsageConfigurationEl {
            disable_use_as_direct_query_source: core::default::Default::default(),
            disable_use_as_imported_source: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetDataSetUsageConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetDataSetUsageConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetDataSetUsageConfigurationElRef {
        DataQuicksightDataSetDataSetUsageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetDataSetUsageConfigurationElRef {
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
pub struct DataQuicksightDataSetFieldFoldersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_folders_id: Option<PrimField<String>>,
}
impl DataQuicksightDataSetFieldFoldersEl {
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
    #[doc = "Set the field `field_folders_id`.\n"]
    pub fn set_field_folders_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_folders_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetFieldFoldersEl {
    type O = BlockAssignable<DataQuicksightDataSetFieldFoldersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetFieldFoldersEl {}
impl BuildDataQuicksightDataSetFieldFoldersEl {
    pub fn build(self) -> DataQuicksightDataSetFieldFoldersEl {
        DataQuicksightDataSetFieldFoldersEl {
            columns: core::default::Default::default(),
            description: core::default::Default::default(),
            field_folders_id: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetFieldFoldersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetFieldFoldersElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightDataSetFieldFoldersElRef {
        DataQuicksightDataSetFieldFoldersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetFieldFoldersElRef {
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
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_column_type: Option<PrimField<String>>,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_name = Some(v.into());
        self
    }
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }
    #[doc = "Set the field `new_column_type`.\n"]
    pub fn set_new_column_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.new_column_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl
{
    type O = BlockAssignable<
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
    pub fn build(
        self,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl {
            column_name: core::default::Default::default(),
            format: core::default::Default::default(),
            new_column_type: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef {
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
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<PrimField<String>>,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl {
    #[doc = "Set the field `column_id`.\n"]
    pub fn set_column_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_id = Some(v.into());
        self
    }
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_name = Some(v.into());
        self
    }
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl
{
    type O = BlockAssignable<
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl
{}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl {
    pub fn build(
        self,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl
    {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl {
            column_id: core::default::Default::default(),
            column_name: core::default::Default::default(),
            expression: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef
    {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef {
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
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<
        ListField<
            DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl,
        >,
    >,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v : impl Into < ListField < DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsEl > >,
    ) -> Self {
        self.columns = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl
{
    type O = BlockAssignable<
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
    pub fn build(
        self,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl {
            columns: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> ListRef<
        DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElColumnsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_expression: Option<PrimField<String>>,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
    #[doc = "Set the field `condition_expression`.\n"]
    pub fn set_condition_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.condition_expression = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
    type O =
        BlockAssignable<DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
    pub fn build(self) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl {
            condition_expression: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef {
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
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    projected_columns: Option<ListField<PrimField<String>>>,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
    #[doc = "Set the field `projected_columns`.\n"]
    pub fn set_projected_columns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.projected_columns = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
    type O =
        BlockAssignable<DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
    pub fn build(self) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl {
            projected_columns: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef {
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
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_column_name: Option<PrimField<String>>,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_name = Some(v.into());
        self
    }
    #[doc = "Set the field `new_column_name`.\n"]
    pub fn set_new_column_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.new_column_name = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl
{
    type O = BlockAssignable<
        DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
    pub fn build(
        self,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl {
            column_name: core::default::Default::default(),
            new_column_name: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef {
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
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<PrimField<String>>,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl { # [doc = "Set the field `text`.\n"] pub fn set_text (mut self , v : impl Into < PrimField < String > >) -> Self { self . text = Some (v . into ()) ; self } }
impl ToListMappable for DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl { type O = BlockAssignable < DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl
{}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl { pub fn build (self) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl { DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl { text : core :: default :: Default :: default () , } } }
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef { fn new (shared : StackShared , base : String) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef { DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef { shared : shared , base : base . to_string () , } } }
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `text` after provisioning.\n"] pub fn text (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.text" , self . base)) } }
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl { # [serde (skip_serializing_if = "Option::is_none")] column_description : Option < ListField < DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl > > , # [serde (skip_serializing_if = "Option::is_none")] column_geographic_role : Option < PrimField < String > > , }
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {
    #[doc = "Set the field `column_description`.\n"]
    pub fn set_column_description(
        mut self,
        v : impl Into < ListField < DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionEl > >,
    ) -> Self {
        self.column_description = Some(v.into());
        self
    }
    #[doc = "Set the field `column_geographic_role`.\n"]
    pub fn set_column_geographic_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_geographic_role = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl
{
    type O = BlockAssignable<
        DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {
}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {
    pub fn build(
        self,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl {
            column_description: core::default::Default::default(),
            column_geographic_role: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `column_description` after provisioning.\n"]    pub fn column_description (& self) -> ListRef < DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElColumnDescriptionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_description", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `column_geographic_role` after provisioning.\n"]
    pub fn column_geographic_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.column_geographic_role", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<
        ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl>,
    >,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl Into<
            ListField<
                DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsEl,
            >,
        >,
    ) -> Self {
        self.tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
    type O =
        BlockAssignable<DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
    pub fn build(
        self,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl {
            column_name: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef {
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
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElTagsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_names: Option<ListField<PrimField<String>>>,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_name = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_names`.\n"]
    pub fn set_tag_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tag_names = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl
{
    type O = BlockAssignable<
        DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
    pub fn build(
        self,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl {
            column_name: core::default::Default::default(),
            tag_names: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef {
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
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cast_column_type_operation: Option<
        ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_columns_operation: Option<
        ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_operation:
        Option<ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_operation:
        Option<ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rename_column_operation: Option<
        ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_column_operation: Option<
        ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    untag_column_operation: Option<
        ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl>,
    >,
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsEl {
    #[doc = "Set the field `cast_column_type_operation`.\n"]
    pub fn set_cast_column_type_operation(
        mut self,
        v: impl Into<
            ListField<
                DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationEl,
            >,
        >,
    ) -> Self {
        self.cast_column_type_operation = Some(v.into());
        self
    }
    #[doc = "Set the field `create_columns_operation`.\n"]
    pub fn set_create_columns_operation(
        mut self,
        v: impl Into<
            ListField<
                DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationEl,
            >,
        >,
    ) -> Self {
        self.create_columns_operation = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_operation`.\n"]
    pub fn set_filter_operation(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationEl>>,
    ) -> Self {
        self.filter_operation = Some(v.into());
        self
    }
    #[doc = "Set the field `project_operation`.\n"]
    pub fn set_project_operation(
        mut self,
        v: impl Into<
            ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationEl>,
        >,
    ) -> Self {
        self.project_operation = Some(v.into());
        self
    }
    #[doc = "Set the field `rename_column_operation`.\n"]
    pub fn set_rename_column_operation(
        mut self,
        v: impl Into<
            ListField<
                DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationEl,
            >,
        >,
    ) -> Self {
        self.rename_column_operation = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_column_operation`.\n"]
    pub fn set_tag_column_operation(
        mut self,
        v: impl Into<
            ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationEl>,
        >,
    ) -> Self {
        self.tag_column_operation = Some(v.into());
        self
    }
    #[doc = "Set the field `untag_column_operation`.\n"]
    pub fn set_untag_column_operation(
        mut self,
        v: impl Into<
            ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationEl>,
        >,
    ) -> Self {
        self.untag_column_operation = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetLogicalTableMapElDataTransformsEl {
    type O = BlockAssignable<DataQuicksightDataSetLogicalTableMapElDataTransformsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElDataTransformsEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElDataTransformsEl {
    pub fn build(self) -> DataQuicksightDataSetLogicalTableMapElDataTransformsEl {
        DataQuicksightDataSetLogicalTableMapElDataTransformsEl {
            cast_column_type_operation: core::default::Default::default(),
            create_columns_operation: core::default::Default::default(),
            filter_operation: core::default::Default::default(),
            project_operation: core::default::Default::default(),
            rename_column_operation: core::default::Default::default(),
            tag_column_operation: core::default::Default::default(),
            untag_column_operation: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElDataTransformsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElDataTransformsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElDataTransformsElRef {
        DataQuicksightDataSetLogicalTableMapElDataTransformsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElDataTransformsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cast_column_type_operation` after provisioning.\n"]
    pub fn cast_column_type_operation(
        &self,
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElDataTransformsElCastColumnTypeOperationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cast_column_type_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_columns_operation` after provisioning.\n"]
    pub fn create_columns_operation(
        &self,
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElDataTransformsElCreateColumnsOperationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_columns_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_operation` after provisioning.\n"]
    pub fn filter_operation(
        &self,
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElDataTransformsElFilterOperationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project_operation` after provisioning.\n"]
    pub fn project_operation(
        &self,
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElDataTransformsElProjectOperationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.project_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rename_column_operation` after provisioning.\n"]
    pub fn rename_column_operation(
        &self,
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElDataTransformsElRenameColumnOperationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rename_column_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tag_column_operation` after provisioning.\n"]
    pub fn tag_column_operation(
        &self,
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElDataTransformsElTagColumnOperationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tag_column_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `untag_column_operation` after provisioning.\n"]
    pub fn untag_column_operation(
        &self,
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElDataTransformsElUntagColumnOperationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.untag_column_operation", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_key: Option<PrimField<bool>>,
}
impl DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl {
    #[doc = "Set the field `unique_key`.\n"]
    pub fn set_unique_key(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_key = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl
{
    type O = BlockAssignable<
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl
{}
impl BuildDataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl {
    pub fn build(
        self,
    ) -> DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl
    {
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl {
            unique_key: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef
    {
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unique_key` after provisioning.\n"]
    pub fn unique_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_key", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_key: Option<PrimField<bool>>,
}
impl DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl {
    #[doc = "Set the field `unique_key`.\n"]
    pub fn set_unique_key(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_key = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl
{
    type O = BlockAssignable<
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl
{}
impl BuildDataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl {
    pub fn build(
        self,
    ) -> DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl
    {
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl {
            unique_key: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef
    {
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `unique_key` after provisioning.\n"]
    pub fn unique_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_key", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    left_join_key_properties: Option<
        ListField<
            DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    left_operand: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_clause: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    right_join_key_properties: Option<
        ListField<
            DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    right_operand: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
    #[doc = "Set the field `left_join_key_properties`.\n"]
    pub fn set_left_join_key_properties(
        mut self,
        v : impl Into < ListField < DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesEl > >,
    ) -> Self {
        self.left_join_key_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `left_operand`.\n"]
    pub fn set_left_operand(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.left_operand = Some(v.into());
        self
    }
    #[doc = "Set the field `on_clause`.\n"]
    pub fn set_on_clause(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_clause = Some(v.into());
        self
    }
    #[doc = "Set the field `right_join_key_properties`.\n"]
    pub fn set_right_join_key_properties(
        mut self,
        v : impl Into < ListField < DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesEl > >,
    ) -> Self {
        self.right_join_key_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `right_operand`.\n"]
    pub fn set_right_operand(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.right_operand = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
    type O = BlockAssignable<DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
    pub fn build(self) -> DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl {
            left_join_key_properties: core::default::Default::default(),
            left_operand: core::default::Default::default(),
            on_clause: core::default::Default::default(),
            right_join_key_properties: core::default::Default::default(),
            right_operand: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `left_join_key_properties` after provisioning.\n"]
    pub fn left_join_key_properties(
        &self,
    ) -> ListRef<
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElLeftJoinKeyPropertiesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.left_join_key_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `left_operand` after provisioning.\n"]
    pub fn left_operand(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.left_operand", self.base))
    }
    #[doc = "Get a reference to the value of field `on_clause` after provisioning.\n"]
    pub fn on_clause(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_clause", self.base))
    }
    #[doc = "Get a reference to the value of field `right_join_key_properties` after provisioning.\n"]
    pub fn right_join_key_properties(
        &self,
    ) -> ListRef<
        DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRightJoinKeyPropertiesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.right_join_key_properties", self.base),
        )
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
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapElSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_set_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    join_instruction:
        Option<ListField<DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_table_id: Option<PrimField<String>>,
}
impl DataQuicksightDataSetLogicalTableMapElSourceEl {
    #[doc = "Set the field `data_set_arn`.\n"]
    pub fn set_data_set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_set_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `join_instruction`.\n"]
    pub fn set_join_instruction(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionEl>>,
    ) -> Self {
        self.join_instruction = Some(v.into());
        self
    }
    #[doc = "Set the field `physical_table_id`.\n"]
    pub fn set_physical_table_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.physical_table_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetLogicalTableMapElSourceEl {
    type O = BlockAssignable<DataQuicksightDataSetLogicalTableMapElSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapElSourceEl {}
impl BuildDataQuicksightDataSetLogicalTableMapElSourceEl {
    pub fn build(self) -> DataQuicksightDataSetLogicalTableMapElSourceEl {
        DataQuicksightDataSetLogicalTableMapElSourceEl {
            data_set_arn: core::default::Default::default(),
            join_instruction: core::default::Default::default(),
            physical_table_id: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElSourceElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightDataSetLogicalTableMapElSourceElRef {
        DataQuicksightDataSetLogicalTableMapElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_set_arn` after provisioning.\n"]
    pub fn data_set_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_set_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `join_instruction` after provisioning.\n"]
    pub fn join_instruction(
        &self,
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElSourceElJoinInstructionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.join_instruction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `physical_table_id` after provisioning.\n"]
    pub fn physical_table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.physical_table_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetLogicalTableMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_transforms: Option<ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logical_table_map_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<ListField<DataQuicksightDataSetLogicalTableMapElSourceEl>>,
}
impl DataQuicksightDataSetLogicalTableMapEl {
    #[doc = "Set the field `alias`.\n"]
    pub fn set_alias(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.alias = Some(v.into());
        self
    }
    #[doc = "Set the field `data_transforms`.\n"]
    pub fn set_data_transforms(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetLogicalTableMapElDataTransformsEl>>,
    ) -> Self {
        self.data_transforms = Some(v.into());
        self
    }
    #[doc = "Set the field `logical_table_map_id`.\n"]
    pub fn set_logical_table_map_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.logical_table_map_id = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetLogicalTableMapElSourceEl>>,
    ) -> Self {
        self.source = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetLogicalTableMapEl {
    type O = BlockAssignable<DataQuicksightDataSetLogicalTableMapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetLogicalTableMapEl {}
impl BuildDataQuicksightDataSetLogicalTableMapEl {
    pub fn build(self) -> DataQuicksightDataSetLogicalTableMapEl {
        DataQuicksightDataSetLogicalTableMapEl {
            alias: core::default::Default::default(),
            data_transforms: core::default::Default::default(),
            logical_table_map_id: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetLogicalTableMapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetLogicalTableMapElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightDataSetLogicalTableMapElRef {
        DataQuicksightDataSetLogicalTableMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetLogicalTableMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.base))
    }
    #[doc = "Get a reference to the value of field `data_transforms` after provisioning.\n"]
    pub fn data_transforms(
        &self,
    ) -> ListRef<DataQuicksightDataSetLogicalTableMapElDataTransformsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_transforms", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `logical_table_map_id` after provisioning.\n"]
    pub fn logical_table_map_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.logical_table_map_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<DataQuicksightDataSetLogicalTableMapElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<PrimField<String>>,
}
impl DataQuicksightDataSetPermissionsEl {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.actions = Some(v.into());
        self
    }
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetPermissionsEl {
    type O = BlockAssignable<DataQuicksightDataSetPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetPermissionsEl {}
impl BuildDataQuicksightDataSetPermissionsEl {
    pub fn build(self) -> DataQuicksightDataSetPermissionsEl {
        DataQuicksightDataSetPermissionsEl {
            actions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetPermissionsElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightDataSetPermissionsElRef {
        DataQuicksightDataSetPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetPermissionsElRef {
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
pub struct DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
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
impl ToListMappable for DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
    type O = BlockAssignable<DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {}
impl BuildDataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
    pub fn build(self) -> DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
        DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
        DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef {
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
pub struct DataQuicksightDataSetPhysicalTableMapElCustomSqlEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<ListField<DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sql_query: Option<PrimField<String>>,
}
impl DataQuicksightDataSetPhysicalTableMapElCustomSqlEl {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsEl>>,
    ) -> Self {
        self.columns = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source_arn`.\n"]
    pub fn set_data_source_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `sql_query`.\n"]
    pub fn set_sql_query(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sql_query = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetPhysicalTableMapElCustomSqlEl {
    type O = BlockAssignable<DataQuicksightDataSetPhysicalTableMapElCustomSqlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetPhysicalTableMapElCustomSqlEl {}
impl BuildDataQuicksightDataSetPhysicalTableMapElCustomSqlEl {
    pub fn build(self) -> DataQuicksightDataSetPhysicalTableMapElCustomSqlEl {
        DataQuicksightDataSetPhysicalTableMapElCustomSqlEl {
            columns: core::default::Default::default(),
            data_source_arn: core::default::Default::default(),
            name: core::default::Default::default(),
            sql_query: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetPhysicalTableMapElCustomSqlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetPhysicalTableMapElCustomSqlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetPhysicalTableMapElCustomSqlElRef {
        DataQuicksightDataSetPhysicalTableMapElCustomSqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetPhysicalTableMapElCustomSqlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> ListRef<DataQuicksightDataSetPhysicalTableMapElCustomSqlElColumnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
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
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
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
impl ToListMappable for DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
    type O =
        BlockAssignable<DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {}
impl BuildDataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
    pub fn build(self) -> DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
        DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
        DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef {
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
pub struct DataQuicksightDataSetPhysicalTableMapElRelationalTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_columns:
        Option<ListField<DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
}
impl DataQuicksightDataSetPhysicalTableMapElRelationalTableEl {
    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source_arn`.\n"]
    pub fn set_data_source_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `input_columns`.\n"]
    pub fn set_input_columns(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsEl>>,
    ) -> Self {
        self.input_columns = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetPhysicalTableMapElRelationalTableEl {
    type O = BlockAssignable<DataQuicksightDataSetPhysicalTableMapElRelationalTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetPhysicalTableMapElRelationalTableEl {}
impl BuildDataQuicksightDataSetPhysicalTableMapElRelationalTableEl {
    pub fn build(self) -> DataQuicksightDataSetPhysicalTableMapElRelationalTableEl {
        DataQuicksightDataSetPhysicalTableMapElRelationalTableEl {
            catalog: core::default::Default::default(),
            data_source_arn: core::default::Default::default(),
            input_columns: core::default::Default::default(),
            name: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetPhysicalTableMapElRelationalTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetPhysicalTableMapElRelationalTableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetPhysicalTableMapElRelationalTableElRef {
        DataQuicksightDataSetPhysicalTableMapElRelationalTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetPhysicalTableMapElRelationalTableElRef {
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
    #[doc = "Get a reference to the value of field `input_columns` after provisioning.\n"]
    pub fn input_columns(
        &self,
    ) -> ListRef<DataQuicksightDataSetPhysicalTableMapElRelationalTableElInputColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_columns", self.base),
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
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
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
impl ToListMappable for DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
    type O = BlockAssignable<DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {}
impl BuildDataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
    pub fn build(self) -> DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
        DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
        DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef {
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
pub struct DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
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
impl DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
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
impl ToListMappable for DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
    type O = BlockAssignable<DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {}
impl BuildDataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
    pub fn build(self) -> DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
        DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl {
            contains_header: core::default::Default::default(),
            delimiter: core::default::Default::default(),
            format: core::default::Default::default(),
            start_from_row: core::default::Default::default(),
            text_qualifier: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
        DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef {
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
#[derive(Serialize)]
pub struct DataQuicksightDataSetPhysicalTableMapElS3SourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_columns:
        Option<ListField<DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_settings:
        Option<ListField<DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl>>,
}
impl DataQuicksightDataSetPhysicalTableMapElS3SourceEl {
    #[doc = "Set the field `data_source_arn`.\n"]
    pub fn set_data_source_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `input_columns`.\n"]
    pub fn set_input_columns(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsEl>>,
    ) -> Self {
        self.input_columns = Some(v.into());
        self
    }
    #[doc = "Set the field `upload_settings`.\n"]
    pub fn set_upload_settings(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsEl>>,
    ) -> Self {
        self.upload_settings = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetPhysicalTableMapElS3SourceEl {
    type O = BlockAssignable<DataQuicksightDataSetPhysicalTableMapElS3SourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetPhysicalTableMapElS3SourceEl {}
impl BuildDataQuicksightDataSetPhysicalTableMapElS3SourceEl {
    pub fn build(self) -> DataQuicksightDataSetPhysicalTableMapElS3SourceEl {
        DataQuicksightDataSetPhysicalTableMapElS3SourceEl {
            data_source_arn: core::default::Default::default(),
            input_columns: core::default::Default::default(),
            upload_settings: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetPhysicalTableMapElS3SourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetPhysicalTableMapElS3SourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetPhysicalTableMapElS3SourceElRef {
        DataQuicksightDataSetPhysicalTableMapElS3SourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetPhysicalTableMapElS3SourceElRef {
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
    ) -> ListRef<DataQuicksightDataSetPhysicalTableMapElS3SourceElInputColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_columns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upload_settings` after provisioning.\n"]
    pub fn upload_settings(
        &self,
    ) -> ListRef<DataQuicksightDataSetPhysicalTableMapElS3SourceElUploadSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upload_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetPhysicalTableMapEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_sql: Option<ListField<DataQuicksightDataSetPhysicalTableMapElCustomSqlEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_table_map_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relational_table: Option<ListField<DataQuicksightDataSetPhysicalTableMapElRelationalTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_source: Option<ListField<DataQuicksightDataSetPhysicalTableMapElS3SourceEl>>,
}
impl DataQuicksightDataSetPhysicalTableMapEl {
    #[doc = "Set the field `custom_sql`.\n"]
    pub fn set_custom_sql(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetPhysicalTableMapElCustomSqlEl>>,
    ) -> Self {
        self.custom_sql = Some(v.into());
        self
    }
    #[doc = "Set the field `physical_table_map_id`.\n"]
    pub fn set_physical_table_map_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.physical_table_map_id = Some(v.into());
        self
    }
    #[doc = "Set the field `relational_table`.\n"]
    pub fn set_relational_table(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetPhysicalTableMapElRelationalTableEl>>,
    ) -> Self {
        self.relational_table = Some(v.into());
        self
    }
    #[doc = "Set the field `s3_source`.\n"]
    pub fn set_s3_source(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetPhysicalTableMapElS3SourceEl>>,
    ) -> Self {
        self.s3_source = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetPhysicalTableMapEl {
    type O = BlockAssignable<DataQuicksightDataSetPhysicalTableMapEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetPhysicalTableMapEl {}
impl BuildDataQuicksightDataSetPhysicalTableMapEl {
    pub fn build(self) -> DataQuicksightDataSetPhysicalTableMapEl {
        DataQuicksightDataSetPhysicalTableMapEl {
            custom_sql: core::default::Default::default(),
            physical_table_map_id: core::default::Default::default(),
            relational_table: core::default::Default::default(),
            s3_source: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetPhysicalTableMapElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetPhysicalTableMapElRef {
    fn new(shared: StackShared, base: String) -> DataQuicksightDataSetPhysicalTableMapElRef {
        DataQuicksightDataSetPhysicalTableMapElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetPhysicalTableMapElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_sql` after provisioning.\n"]
    pub fn custom_sql(&self) -> ListRef<DataQuicksightDataSetPhysicalTableMapElCustomSqlElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_sql", self.base))
    }
    #[doc = "Get a reference to the value of field `physical_table_map_id` after provisioning.\n"]
    pub fn physical_table_map_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.physical_table_map_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `relational_table` after provisioning.\n"]
    pub fn relational_table(
        &self,
    ) -> ListRef<DataQuicksightDataSetPhysicalTableMapElRelationalTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.relational_table", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_source` after provisioning.\n"]
    pub fn s3_source(&self) -> ListRef<DataQuicksightDataSetPhysicalTableMapElS3SourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_source", self.base))
    }
}
#[derive(Serialize)]
pub struct DataQuicksightDataSetRowLevelPermissionDataSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl DataQuicksightDataSetRowLevelPermissionDataSetEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
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
    #[doc = "Set the field `permission_policy`.\n"]
    pub fn set_permission_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetRowLevelPermissionDataSetEl {
    type O = BlockAssignable<DataQuicksightDataSetRowLevelPermissionDataSetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetRowLevelPermissionDataSetEl {}
impl BuildDataQuicksightDataSetRowLevelPermissionDataSetEl {
    pub fn build(self) -> DataQuicksightDataSetRowLevelPermissionDataSetEl {
        DataQuicksightDataSetRowLevelPermissionDataSetEl {
            arn: core::default::Default::default(),
            format_version: core::default::Default::default(),
            namespace: core::default::Default::default(),
            permission_policy: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetRowLevelPermissionDataSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetRowLevelPermissionDataSetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetRowLevelPermissionDataSetElRef {
        DataQuicksightDataSetRowLevelPermissionDataSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetRowLevelPermissionDataSetElRef {
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
pub struct DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_all_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_multi_value_delimiter: Option<PrimField<String>>,
}
impl DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
    #[doc = "Set the field `column_name`.\n"]
    pub fn set_column_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.column_name = Some(v.into());
        self
    }
    #[doc = "Set the field `match_all_value`.\n"]
    pub fn set_match_all_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_all_value = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_key`.\n"]
    pub fn set_tag_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_key = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_multi_value_delimiter`.\n"]
    pub fn set_tag_multi_value_delimiter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag_multi_value_delimiter = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
    type O = BlockAssignable<DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {}
impl BuildDataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
    pub fn build(self) -> DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
        DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl {
            column_name: core::default::Default::default(),
            match_all_value: core::default::Default::default(),
            tag_key: core::default::Default::default(),
            tag_multi_value_delimiter: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
        DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef {
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
#[derive(Serialize)]
pub struct DataQuicksightDataSetRowLevelPermissionTagConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_rules:
        Option<ListField<DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl>>,
}
impl DataQuicksightDataSetRowLevelPermissionTagConfigurationEl {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `tag_rules`.\n"]
    pub fn set_tag_rules(
        mut self,
        v: impl Into<ListField<DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesEl>>,
    ) -> Self {
        self.tag_rules = Some(v.into());
        self
    }
}
impl ToListMappable for DataQuicksightDataSetRowLevelPermissionTagConfigurationEl {
    type O = BlockAssignable<DataQuicksightDataSetRowLevelPermissionTagConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataQuicksightDataSetRowLevelPermissionTagConfigurationEl {}
impl BuildDataQuicksightDataSetRowLevelPermissionTagConfigurationEl {
    pub fn build(self) -> DataQuicksightDataSetRowLevelPermissionTagConfigurationEl {
        DataQuicksightDataSetRowLevelPermissionTagConfigurationEl {
            status: core::default::Default::default(),
            tag_rules: core::default::Default::default(),
        }
    }
}
pub struct DataQuicksightDataSetRowLevelPermissionTagConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataQuicksightDataSetRowLevelPermissionTagConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataQuicksightDataSetRowLevelPermissionTagConfigurationElRef {
        DataQuicksightDataSetRowLevelPermissionTagConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataQuicksightDataSetRowLevelPermissionTagConfigurationElRef {
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
    ) -> ListRef<DataQuicksightDataSetRowLevelPermissionTagConfigurationElTagRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tag_rules", self.base))
    }
}
