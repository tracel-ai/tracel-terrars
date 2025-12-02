use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLakeformationPermissionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_resource: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    principal: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cells_filter: Option<Vec<DataLakeformationPermissionsDataCellsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_location: Option<Vec<DataLakeformationPermissionsDataLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<Vec<DataLakeformationPermissionsDatabaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lf_tag: Option<Vec<DataLakeformationPermissionsLfTagEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lf_tag_policy: Option<Vec<DataLakeformationPermissionsLfTagPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table: Option<Vec<DataLakeformationPermissionsTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_with_columns: Option<Vec<DataLakeformationPermissionsTableWithColumnsEl>>,
    dynamic: DataLakeformationPermissionsDynamic,
}
struct DataLakeformationPermissions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLakeformationPermissionsData>,
}
#[derive(Clone)]
pub struct DataLakeformationPermissions(Rc<DataLakeformationPermissions_>);
impl DataLakeformationPermissions {
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
    #[doc = "Set the field `catalog_resource`.\n"]
    pub fn set_catalog_resource(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().catalog_resource = Some(v.into());
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
    #[doc = "Set the field `data_cells_filter`.\n"]
    pub fn set_data_cells_filter(
        self,
        v: impl Into<BlockAssignable<DataLakeformationPermissionsDataCellsFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_cells_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_cells_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_location`.\n"]
    pub fn set_data_location(
        self,
        v: impl Into<BlockAssignable<DataLakeformationPermissionsDataLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_location = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `database`.\n"]
    pub fn set_database(
        self,
        v: impl Into<BlockAssignable<DataLakeformationPermissionsDatabaseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().database = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.database = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lf_tag`.\n"]
    pub fn set_lf_tag(
        self,
        v: impl Into<BlockAssignable<DataLakeformationPermissionsLfTagEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lf_tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lf_tag = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lf_tag_policy`.\n"]
    pub fn set_lf_tag_policy(
        self,
        v: impl Into<BlockAssignable<DataLakeformationPermissionsLfTagPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lf_tag_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lf_tag_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        self,
        v: impl Into<BlockAssignable<DataLakeformationPermissionsTableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `table_with_columns`.\n"]
    pub fn set_table_with_columns(
        self,
        v: impl Into<BlockAssignable<DataLakeformationPermissionsTableWithColumnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table_with_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table_with_columns = Some(d);
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
    #[doc = "Get a reference to the value of field `catalog_resource` after provisioning.\n"]
    pub fn catalog_resource(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions_with_grant_option", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_cells_filter` after provisioning.\n"]
    pub fn data_cells_filter(&self) -> ListRef<DataLakeformationPermissionsDataCellsFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_cells_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> ListRef<DataLakeformationPermissionsDataLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<DataLakeformationPermissionsDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> ListRef<DataLakeformationPermissionsLfTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> ListRef<DataLakeformationPermissionsLfTagPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> ListRef<DataLakeformationPermissionsTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> ListRef<DataLakeformationPermissionsTableWithColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_with_columns", self.extract_ref()),
        )
    }
}
impl Referable for DataLakeformationPermissions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataLakeformationPermissions {}
impl ToListMappable for DataLakeformationPermissions {
    type O = ListRef<DataLakeformationPermissionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataLakeformationPermissions_ {
    fn extract_datasource_type(&self) -> String {
        "aws_lakeformation_permissions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLakeformationPermissions {
    pub tf_id: String,
    #[doc = ""]
    pub principal: PrimField<String>,
}
impl BuildDataLakeformationPermissions {
    pub fn build(self, stack: &mut Stack) -> DataLakeformationPermissions {
        let out = DataLakeformationPermissions(Rc::new(DataLakeformationPermissions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLakeformationPermissionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                catalog_id: core::default::Default::default(),
                catalog_resource: core::default::Default::default(),
                id: core::default::Default::default(),
                principal: self.principal,
                region: core::default::Default::default(),
                data_cells_filter: core::default::Default::default(),
                data_location: core::default::Default::default(),
                database: core::default::Default::default(),
                lf_tag: core::default::Default::default(),
                lf_tag_policy: core::default::Default::default(),
                table: core::default::Default::default(),
                table_with_columns: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataLakeformationPermissionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLakeformationPermissionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLakeformationPermissionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `catalog_resource` after provisioning.\n"]
    pub fn catalog_resource(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_resource", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions_with_grant_option", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_cells_filter` after provisioning.\n"]
    pub fn data_cells_filter(&self) -> ListRef<DataLakeformationPermissionsDataCellsFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_cells_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> ListRef<DataLakeformationPermissionsDataLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<DataLakeformationPermissionsDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> ListRef<DataLakeformationPermissionsLfTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> ListRef<DataLakeformationPermissionsLfTagPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> ListRef<DataLakeformationPermissionsTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> ListRef<DataLakeformationPermissionsTableWithColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_with_columns", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLakeformationPermissionsDataCellsFilterEl {
    database_name: PrimField<String>,
    name: PrimField<String>,
    table_catalog_id: PrimField<String>,
    table_name: PrimField<String>,
}
impl DataLakeformationPermissionsDataCellsFilterEl {}
impl ToListMappable for DataLakeformationPermissionsDataCellsFilterEl {
    type O = BlockAssignable<DataLakeformationPermissionsDataCellsFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLakeformationPermissionsDataCellsFilterEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub table_catalog_id: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}
impl BuildDataLakeformationPermissionsDataCellsFilterEl {
    pub fn build(self) -> DataLakeformationPermissionsDataCellsFilterEl {
        DataLakeformationPermissionsDataCellsFilterEl {
            database_name: self.database_name,
            name: self.name,
            table_catalog_id: self.table_catalog_id,
            table_name: self.table_name,
        }
    }
}
pub struct DataLakeformationPermissionsDataCellsFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLakeformationPermissionsDataCellsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataLakeformationPermissionsDataCellsFilterElRef {
        DataLakeformationPermissionsDataCellsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLakeformationPermissionsDataCellsFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `table_catalog_id` after provisioning.\n"]
    pub fn table_catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_catalog_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLakeformationPermissionsDataLocationEl {
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
}
impl DataLakeformationPermissionsDataLocationEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataLakeformationPermissionsDataLocationEl {
    type O = BlockAssignable<DataLakeformationPermissionsDataLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLakeformationPermissionsDataLocationEl {
    #[doc = ""]
    pub arn: PrimField<String>,
}
impl BuildDataLakeformationPermissionsDataLocationEl {
    pub fn build(self) -> DataLakeformationPermissionsDataLocationEl {
        DataLakeformationPermissionsDataLocationEl {
            arn: self.arn,
            catalog_id: core::default::Default::default(),
        }
    }
}
pub struct DataLakeformationPermissionsDataLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLakeformationPermissionsDataLocationElRef {
    fn new(shared: StackShared, base: String) -> DataLakeformationPermissionsDataLocationElRef {
        DataLakeformationPermissionsDataLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLakeformationPermissionsDataLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLakeformationPermissionsDatabaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl DataLakeformationPermissionsDatabaseEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataLakeformationPermissionsDatabaseEl {
    type O = BlockAssignable<DataLakeformationPermissionsDatabaseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLakeformationPermissionsDatabaseEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataLakeformationPermissionsDatabaseEl {
    pub fn build(self) -> DataLakeformationPermissionsDatabaseEl {
        DataLakeformationPermissionsDatabaseEl {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct DataLakeformationPermissionsDatabaseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLakeformationPermissionsDatabaseElRef {
    fn new(shared: StackShared, base: String) -> DataLakeformationPermissionsDatabaseElRef {
        DataLakeformationPermissionsDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLakeformationPermissionsDatabaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLakeformationPermissionsLfTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    key: PrimField<String>,
    values: SetField<PrimField<String>>,
}
impl DataLakeformationPermissionsLfTagEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataLakeformationPermissionsLfTagEl {
    type O = BlockAssignable<DataLakeformationPermissionsLfTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLakeformationPermissionsLfTagEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}
impl BuildDataLakeformationPermissionsLfTagEl {
    pub fn build(self) -> DataLakeformationPermissionsLfTagEl {
        DataLakeformationPermissionsLfTagEl {
            catalog_id: core::default::Default::default(),
            key: self.key,
            values: self.values,
        }
    }
}
pub struct DataLakeformationPermissionsLfTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLakeformationPermissionsLfTagElRef {
    fn new(shared: StackShared, base: String) -> DataLakeformationPermissionsLfTagElRef {
        DataLakeformationPermissionsLfTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLakeformationPermissionsLfTagElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLakeformationPermissionsLfTagPolicyElExpressionEl {
    key: PrimField<String>,
    values: SetField<PrimField<String>>,
}
impl DataLakeformationPermissionsLfTagPolicyElExpressionEl {}
impl ToListMappable for DataLakeformationPermissionsLfTagPolicyElExpressionEl {
    type O = BlockAssignable<DataLakeformationPermissionsLfTagPolicyElExpressionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLakeformationPermissionsLfTagPolicyElExpressionEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}
impl BuildDataLakeformationPermissionsLfTagPolicyElExpressionEl {
    pub fn build(self) -> DataLakeformationPermissionsLfTagPolicyElExpressionEl {
        DataLakeformationPermissionsLfTagPolicyElExpressionEl {
            key: self.key,
            values: self.values,
        }
    }
}
pub struct DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
        DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLakeformationPermissionsLfTagPolicyElExpressionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLakeformationPermissionsLfTagPolicyElDynamic {
    expression: Option<DynamicBlock<DataLakeformationPermissionsLfTagPolicyElExpressionEl>>,
}
#[derive(Serialize)]
pub struct DataLakeformationPermissionsLfTagPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<Vec<DataLakeformationPermissionsLfTagPolicyElExpressionEl>>,
    dynamic: DataLakeformationPermissionsLfTagPolicyElDynamic,
}
impl DataLakeformationPermissionsLfTagPolicyEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(
        mut self,
        v: impl Into<BlockAssignable<DataLakeformationPermissionsLfTagPolicyElExpressionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expression = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expression = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLakeformationPermissionsLfTagPolicyEl {
    type O = BlockAssignable<DataLakeformationPermissionsLfTagPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLakeformationPermissionsLfTagPolicyEl {
    #[doc = ""]
    pub resource_type: PrimField<String>,
}
impl BuildDataLakeformationPermissionsLfTagPolicyEl {
    pub fn build(self) -> DataLakeformationPermissionsLfTagPolicyEl {
        DataLakeformationPermissionsLfTagPolicyEl {
            catalog_id: core::default::Default::default(),
            resource_type: self.resource_type,
            expression: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLakeformationPermissionsLfTagPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLakeformationPermissionsLfTagPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataLakeformationPermissionsLfTagPolicyElRef {
        DataLakeformationPermissionsLfTagPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLakeformationPermissionsLfTagPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLakeformationPermissionsTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wildcard: Option<PrimField<bool>>,
}
impl DataLakeformationPermissionsTableEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `wildcard`.\n"]
    pub fn set_wildcard(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.wildcard = Some(v.into());
        self
    }
}
impl ToListMappable for DataLakeformationPermissionsTableEl {
    type O = BlockAssignable<DataLakeformationPermissionsTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLakeformationPermissionsTableEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
}
impl BuildDataLakeformationPermissionsTableEl {
    pub fn build(self) -> DataLakeformationPermissionsTableEl {
        DataLakeformationPermissionsTableEl {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            name: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}
pub struct DataLakeformationPermissionsTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLakeformationPermissionsTableElRef {
    fn new(shared: StackShared, base: String) -> DataLakeformationPermissionsTableElRef {
        DataLakeformationPermissionsTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLakeformationPermissionsTableElRef {
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
    #[doc = "Get a reference to the value of field `wildcard` after provisioning.\n"]
    pub fn wildcard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLakeformationPermissionsTableWithColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_names: Option<SetField<PrimField<String>>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_column_names: Option<SetField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wildcard: Option<PrimField<bool>>,
}
impl DataLakeformationPermissionsTableWithColumnsEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
    #[doc = "Set the field `column_names`.\n"]
    pub fn set_column_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.column_names = Some(v.into());
        self
    }
    #[doc = "Set the field `excluded_column_names`.\n"]
    pub fn set_excluded_column_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.excluded_column_names = Some(v.into());
        self
    }
    #[doc = "Set the field `wildcard`.\n"]
    pub fn set_wildcard(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.wildcard = Some(v.into());
        self
    }
}
impl ToListMappable for DataLakeformationPermissionsTableWithColumnsEl {
    type O = BlockAssignable<DataLakeformationPermissionsTableWithColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLakeformationPermissionsTableWithColumnsEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataLakeformationPermissionsTableWithColumnsEl {
    pub fn build(self) -> DataLakeformationPermissionsTableWithColumnsEl {
        DataLakeformationPermissionsTableWithColumnsEl {
            catalog_id: core::default::Default::default(),
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            excluded_column_names: core::default::Default::default(),
            name: self.name,
            wildcard: core::default::Default::default(),
        }
    }
}
pub struct DataLakeformationPermissionsTableWithColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLakeformationPermissionsTableWithColumnsElRef {
    fn new(shared: StackShared, base: String) -> DataLakeformationPermissionsTableWithColumnsElRef {
        DataLakeformationPermissionsTableWithColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLakeformationPermissionsTableWithColumnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
    #[doc = "Get a reference to the value of field `column_names` after provisioning.\n"]
    pub fn column_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.column_names", self.base))
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `excluded_column_names` after provisioning.\n"]
    pub fn excluded_column_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.excluded_column_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `wildcard` after provisioning.\n"]
    pub fn wildcard(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.wildcard", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLakeformationPermissionsDynamic {
    data_cells_filter: Option<DynamicBlock<DataLakeformationPermissionsDataCellsFilterEl>>,
    data_location: Option<DynamicBlock<DataLakeformationPermissionsDataLocationEl>>,
    database: Option<DynamicBlock<DataLakeformationPermissionsDatabaseEl>>,
    lf_tag: Option<DynamicBlock<DataLakeformationPermissionsLfTagEl>>,
    lf_tag_policy: Option<DynamicBlock<DataLakeformationPermissionsLfTagPolicyEl>>,
    table: Option<DynamicBlock<DataLakeformationPermissionsTableEl>>,
    table_with_columns: Option<DynamicBlock<DataLakeformationPermissionsTableWithColumnsEl>>,
}
