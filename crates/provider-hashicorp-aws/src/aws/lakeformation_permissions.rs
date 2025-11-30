use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct LakeformationPermissionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_resource: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    permissions: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions_with_grant_option: Option<SetField<PrimField<String>>>,
    principal: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cells_filter: Option<Vec<LakeformationPermissionsDataCellsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_location: Option<Vec<LakeformationPermissionsDataLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<Vec<LakeformationPermissionsDatabaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lf_tag: Option<Vec<LakeformationPermissionsLfTagEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lf_tag_policy: Option<Vec<LakeformationPermissionsLfTagPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table: Option<Vec<LakeformationPermissionsTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_with_columns: Option<Vec<LakeformationPermissionsTableWithColumnsEl>>,
    dynamic: LakeformationPermissionsDynamic,
}

struct LakeformationPermissions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LakeformationPermissionsData>,
}

#[derive(Clone)]
pub struct LakeformationPermissions(Rc<LakeformationPermissions_>);

impl LakeformationPermissions {
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

    #[doc = "Set the field `permissions_with_grant_option`.\n"]
    pub fn set_permissions_with_grant_option(
        self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().permissions_with_grant_option = Some(v.into());
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
        v: impl Into<BlockAssignable<LakeformationPermissionsDataCellsFilterEl>>,
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
        v: impl Into<BlockAssignable<LakeformationPermissionsDataLocationEl>>,
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
        v: impl Into<BlockAssignable<LakeformationPermissionsDatabaseEl>>,
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
        v: impl Into<BlockAssignable<LakeformationPermissionsLfTagEl>>,
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
        v: impl Into<BlockAssignable<LakeformationPermissionsLfTagPolicyEl>>,
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
    pub fn set_table(self, v: impl Into<BlockAssignable<LakeformationPermissionsTableEl>>) -> Self {
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
        v: impl Into<BlockAssignable<LakeformationPermissionsTableWithColumnsEl>>,
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
    pub fn permissions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
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
    pub fn data_cells_filter(&self) -> ListRef<LakeformationPermissionsDataCellsFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_cells_filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> ListRef<LakeformationPermissionsDataLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_location", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<LakeformationPermissionsDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> ListRef<LakeformationPermissionsLfTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> ListRef<LakeformationPermissionsLfTagPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> ListRef<LakeformationPermissionsTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> ListRef<LakeformationPermissionsTableWithColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_with_columns", self.extract_ref()),
        )
    }
}

impl Referable for LakeformationPermissions {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for LakeformationPermissions {}

impl ToListMappable for LakeformationPermissions {
    type O = ListRef<LakeformationPermissionsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LakeformationPermissions_ {
    fn extract_resource_type(&self) -> String {
        "aws_lakeformation_permissions".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLakeformationPermissions {
    pub tf_id: String,
    #[doc = ""]
    pub permissions: SetField<PrimField<String>>,
    #[doc = ""]
    pub principal: PrimField<String>,
}

impl BuildLakeformationPermissions {
    pub fn build(self, stack: &mut Stack) -> LakeformationPermissions {
        let out = LakeformationPermissions(Rc::new(LakeformationPermissions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LakeformationPermissionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog_id: core::default::Default::default(),
                catalog_resource: core::default::Default::default(),
                id: core::default::Default::default(),
                permissions: self.permissions,
                permissions_with_grant_option: core::default::Default::default(),
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
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LakeformationPermissionsRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationPermissionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl LakeformationPermissionsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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
    pub fn permissions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `permissions_with_grant_option` after provisioning.\n"]
    pub fn permissions_with_grant_option(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
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
    pub fn data_cells_filter(&self) -> ListRef<LakeformationPermissionsDataCellsFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_cells_filter", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> ListRef<LakeformationPermissionsDataLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_location", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<LakeformationPermissionsDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> ListRef<LakeformationPermissionsLfTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> ListRef<LakeformationPermissionsLfTagPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> ListRef<LakeformationPermissionsTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> ListRef<LakeformationPermissionsTableWithColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_with_columns", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct LakeformationPermissionsDataCellsFilterEl {
    database_name: PrimField<String>,
    name: PrimField<String>,
    table_catalog_id: PrimField<String>,
    table_name: PrimField<String>,
}

impl LakeformationPermissionsDataCellsFilterEl {}

impl ToListMappable for LakeformationPermissionsDataCellsFilterEl {
    type O = BlockAssignable<LakeformationPermissionsDataCellsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissionsDataCellsFilterEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub table_catalog_id: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}

impl BuildLakeformationPermissionsDataCellsFilterEl {
    pub fn build(self) -> LakeformationPermissionsDataCellsFilterEl {
        LakeformationPermissionsDataCellsFilterEl {
            database_name: self.database_name,
            name: self.name,
            table_catalog_id: self.table_catalog_id,
            table_name: self.table_name,
        }
    }
}

pub struct LakeformationPermissionsDataCellsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationPermissionsDataCellsFilterElRef {
    fn new(shared: StackShared, base: String) -> LakeformationPermissionsDataCellsFilterElRef {
        LakeformationPermissionsDataCellsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissionsDataCellsFilterElRef {
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
pub struct LakeformationPermissionsDataLocationEl {
    arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
}

impl LakeformationPermissionsDataLocationEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}

impl ToListMappable for LakeformationPermissionsDataLocationEl {
    type O = BlockAssignable<LakeformationPermissionsDataLocationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissionsDataLocationEl {
    #[doc = ""]
    pub arn: PrimField<String>,
}

impl BuildLakeformationPermissionsDataLocationEl {
    pub fn build(self) -> LakeformationPermissionsDataLocationEl {
        LakeformationPermissionsDataLocationEl {
            arn: self.arn,
            catalog_id: core::default::Default::default(),
        }
    }
}

pub struct LakeformationPermissionsDataLocationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationPermissionsDataLocationElRef {
    fn new(shared: StackShared, base: String) -> LakeformationPermissionsDataLocationElRef {
        LakeformationPermissionsDataLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissionsDataLocationElRef {
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
pub struct LakeformationPermissionsDatabaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    name: PrimField<String>,
}

impl LakeformationPermissionsDatabaseEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}

impl ToListMappable for LakeformationPermissionsDatabaseEl {
    type O = BlockAssignable<LakeformationPermissionsDatabaseEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissionsDatabaseEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildLakeformationPermissionsDatabaseEl {
    pub fn build(self) -> LakeformationPermissionsDatabaseEl {
        LakeformationPermissionsDatabaseEl {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct LakeformationPermissionsDatabaseElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationPermissionsDatabaseElRef {
    fn new(shared: StackShared, base: String) -> LakeformationPermissionsDatabaseElRef {
        LakeformationPermissionsDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissionsDatabaseElRef {
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
pub struct LakeformationPermissionsLfTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    key: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl LakeformationPermissionsLfTagEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}

impl ToListMappable for LakeformationPermissionsLfTagEl {
    type O = BlockAssignable<LakeformationPermissionsLfTagEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissionsLfTagEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildLakeformationPermissionsLfTagEl {
    pub fn build(self) -> LakeformationPermissionsLfTagEl {
        LakeformationPermissionsLfTagEl {
            catalog_id: core::default::Default::default(),
            key: self.key,
            values: self.values,
        }
    }
}

pub struct LakeformationPermissionsLfTagElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationPermissionsLfTagElRef {
    fn new(shared: StackShared, base: String) -> LakeformationPermissionsLfTagElRef {
        LakeformationPermissionsLfTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissionsLfTagElRef {
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
pub struct LakeformationPermissionsLfTagPolicyElExpressionEl {
    key: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl LakeformationPermissionsLfTagPolicyElExpressionEl {}

impl ToListMappable for LakeformationPermissionsLfTagPolicyElExpressionEl {
    type O = BlockAssignable<LakeformationPermissionsLfTagPolicyElExpressionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissionsLfTagPolicyElExpressionEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildLakeformationPermissionsLfTagPolicyElExpressionEl {
    pub fn build(self) -> LakeformationPermissionsLfTagPolicyElExpressionEl {
        LakeformationPermissionsLfTagPolicyElExpressionEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct LakeformationPermissionsLfTagPolicyElExpressionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationPermissionsLfTagPolicyElExpressionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LakeformationPermissionsLfTagPolicyElExpressionElRef {
        LakeformationPermissionsLfTagPolicyElExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissionsLfTagPolicyElExpressionElRef {
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
struct LakeformationPermissionsLfTagPolicyElDynamic {
    expression: Option<DynamicBlock<LakeformationPermissionsLfTagPolicyElExpressionEl>>,
}

#[derive(Serialize)]
pub struct LakeformationPermissionsLfTagPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    resource_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<Vec<LakeformationPermissionsLfTagPolicyElExpressionEl>>,
    dynamic: LakeformationPermissionsLfTagPolicyElDynamic,
}

impl LakeformationPermissionsLfTagPolicyEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }

    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(
        mut self,
        v: impl Into<BlockAssignable<LakeformationPermissionsLfTagPolicyElExpressionEl>>,
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

impl ToListMappable for LakeformationPermissionsLfTagPolicyEl {
    type O = BlockAssignable<LakeformationPermissionsLfTagPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissionsLfTagPolicyEl {
    #[doc = ""]
    pub resource_type: PrimField<String>,
}

impl BuildLakeformationPermissionsLfTagPolicyEl {
    pub fn build(self) -> LakeformationPermissionsLfTagPolicyEl {
        LakeformationPermissionsLfTagPolicyEl {
            catalog_id: core::default::Default::default(),
            resource_type: self.resource_type,
            expression: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LakeformationPermissionsLfTagPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationPermissionsLfTagPolicyElRef {
    fn new(shared: StackShared, base: String) -> LakeformationPermissionsLfTagPolicyElRef {
        LakeformationPermissionsLfTagPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissionsLfTagPolicyElRef {
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
pub struct LakeformationPermissionsTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wildcard: Option<PrimField<bool>>,
}

impl LakeformationPermissionsTableEl {
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

impl ToListMappable for LakeformationPermissionsTableEl {
    type O = BlockAssignable<LakeformationPermissionsTableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissionsTableEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
}

impl BuildLakeformationPermissionsTableEl {
    pub fn build(self) -> LakeformationPermissionsTableEl {
        LakeformationPermissionsTableEl {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            name: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}

pub struct LakeformationPermissionsTableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationPermissionsTableElRef {
    fn new(shared: StackShared, base: String) -> LakeformationPermissionsTableElRef {
        LakeformationPermissionsTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissionsTableElRef {
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
pub struct LakeformationPermissionsTableWithColumnsEl {
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

impl LakeformationPermissionsTableWithColumnsEl {
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

impl ToListMappable for LakeformationPermissionsTableWithColumnsEl {
    type O = BlockAssignable<LakeformationPermissionsTableWithColumnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationPermissionsTableWithColumnsEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildLakeformationPermissionsTableWithColumnsEl {
    pub fn build(self) -> LakeformationPermissionsTableWithColumnsEl {
        LakeformationPermissionsTableWithColumnsEl {
            catalog_id: core::default::Default::default(),
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            excluded_column_names: core::default::Default::default(),
            name: self.name,
            wildcard: core::default::Default::default(),
        }
    }
}

pub struct LakeformationPermissionsTableWithColumnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationPermissionsTableWithColumnsElRef {
    fn new(shared: StackShared, base: String) -> LakeformationPermissionsTableWithColumnsElRef {
        LakeformationPermissionsTableWithColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationPermissionsTableWithColumnsElRef {
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
struct LakeformationPermissionsDynamic {
    data_cells_filter: Option<DynamicBlock<LakeformationPermissionsDataCellsFilterEl>>,
    data_location: Option<DynamicBlock<LakeformationPermissionsDataLocationEl>>,
    database: Option<DynamicBlock<LakeformationPermissionsDatabaseEl>>,
    lf_tag: Option<DynamicBlock<LakeformationPermissionsLfTagEl>>,
    lf_tag_policy: Option<DynamicBlock<LakeformationPermissionsLfTagPolicyEl>>,
    table: Option<DynamicBlock<LakeformationPermissionsTableEl>>,
    table_with_columns: Option<DynamicBlock<LakeformationPermissionsTableWithColumnsEl>>,
}
