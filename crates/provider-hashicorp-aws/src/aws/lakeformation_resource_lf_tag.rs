use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LakeformationResourceLfTagData {
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
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<Vec<LakeformationResourceLfTagDatabaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lf_tag: Option<Vec<LakeformationResourceLfTagLfTagEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table: Option<Vec<LakeformationResourceLfTagTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_with_columns: Option<Vec<LakeformationResourceLfTagTableWithColumnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LakeformationResourceLfTagTimeoutsEl>,
    dynamic: LakeformationResourceLfTagDynamic,
}
struct LakeformationResourceLfTag_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LakeformationResourceLfTagData>,
}
#[derive(Clone)]
pub struct LakeformationResourceLfTag(Rc<LakeformationResourceLfTag_>);
impl LakeformationResourceLfTag {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `database`.\n"]
    pub fn set_database(
        self,
        v: impl Into<BlockAssignable<LakeformationResourceLfTagDatabaseEl>>,
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
        v: impl Into<BlockAssignable<LakeformationResourceLfTagLfTagEl>>,
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
    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        self,
        v: impl Into<BlockAssignable<LakeformationResourceLfTagTableEl>>,
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
        v: impl Into<BlockAssignable<LakeformationResourceLfTagTableWithColumnsEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LakeformationResourceLfTagTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<LakeformationResourceLfTagDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> ListRef<LakeformationResourceLfTagLfTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> ListRef<LakeformationResourceLfTagTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> ListRef<LakeformationResourceLfTagTableWithColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_with_columns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LakeformationResourceLfTagTimeoutsElRef {
        LakeformationResourceLfTagTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LakeformationResourceLfTag {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LakeformationResourceLfTag {}
impl ToListMappable for LakeformationResourceLfTag {
    type O = ListRef<LakeformationResourceLfTagRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LakeformationResourceLfTag_ {
    fn extract_resource_type(&self) -> String {
        "aws_lakeformation_resource_lf_tag".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLakeformationResourceLfTag {
    pub tf_id: String,
}
impl BuildLakeformationResourceLfTag {
    pub fn build(self, stack: &mut Stack) -> LakeformationResourceLfTag {
        let out = LakeformationResourceLfTag(Rc::new(LakeformationResourceLfTag_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LakeformationResourceLfTagData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog_id: core::default::Default::default(),
                region: core::default::Default::default(),
                database: core::default::Default::default(),
                lf_tag: core::default::Default::default(),
                table: core::default::Default::default(),
                table_with_columns: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LakeformationResourceLfTagRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationResourceLfTagRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LakeformationResourceLfTagRef {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<LakeformationResourceLfTagDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> ListRef<LakeformationResourceLfTagLfTagElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> ListRef<LakeformationResourceLfTagTableElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(&self) -> ListRef<LakeformationResourceLfTagTableWithColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_with_columns", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LakeformationResourceLfTagTimeoutsElRef {
        LakeformationResourceLfTagTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LakeformationResourceLfTagDatabaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl LakeformationResourceLfTagDatabaseEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationResourceLfTagDatabaseEl {
    type O = BlockAssignable<LakeformationResourceLfTagDatabaseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationResourceLfTagDatabaseEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildLakeformationResourceLfTagDatabaseEl {
    pub fn build(self) -> LakeformationResourceLfTagDatabaseEl {
        LakeformationResourceLfTagDatabaseEl {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct LakeformationResourceLfTagDatabaseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationResourceLfTagDatabaseElRef {
    fn new(shared: StackShared, base: String) -> LakeformationResourceLfTagDatabaseElRef {
        LakeformationResourceLfTagDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationResourceLfTagDatabaseElRef {
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
pub struct LakeformationResourceLfTagLfTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    key: PrimField<String>,
    value: PrimField<String>,
}
impl LakeformationResourceLfTagLfTagEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationResourceLfTagLfTagEl {
    type O = BlockAssignable<LakeformationResourceLfTagLfTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationResourceLfTagLfTagEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLakeformationResourceLfTagLfTagEl {
    pub fn build(self) -> LakeformationResourceLfTagLfTagEl {
        LakeformationResourceLfTagLfTagEl {
            catalog_id: core::default::Default::default(),
            key: self.key,
            value: self.value,
        }
    }
}
pub struct LakeformationResourceLfTagLfTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationResourceLfTagLfTagElRef {
    fn new(shared: StackShared, base: String) -> LakeformationResourceLfTagLfTagElRef {
        LakeformationResourceLfTagLfTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationResourceLfTagLfTagElRef {
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
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct LakeformationResourceLfTagTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wildcard: Option<PrimField<bool>>,
}
impl LakeformationResourceLfTagTableEl {
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
impl ToListMappable for LakeformationResourceLfTagTableEl {
    type O = BlockAssignable<LakeformationResourceLfTagTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationResourceLfTagTableEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
}
impl BuildLakeformationResourceLfTagTableEl {
    pub fn build(self) -> LakeformationResourceLfTagTableEl {
        LakeformationResourceLfTagTableEl {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            name: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}
pub struct LakeformationResourceLfTagTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationResourceLfTagTableElRef {
    fn new(shared: StackShared, base: String) -> LakeformationResourceLfTagTableElRef {
        LakeformationResourceLfTagTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationResourceLfTagTableElRef {
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
pub struct LakeformationResourceLfTagTableWithColumnsElColumnWildcardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_column_names: Option<SetField<PrimField<String>>>,
}
impl LakeformationResourceLfTagTableWithColumnsElColumnWildcardEl {
    #[doc = "Set the field `excluded_column_names`.\n"]
    pub fn set_excluded_column_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.excluded_column_names = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationResourceLfTagTableWithColumnsElColumnWildcardEl {
    type O = BlockAssignable<LakeformationResourceLfTagTableWithColumnsElColumnWildcardEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationResourceLfTagTableWithColumnsElColumnWildcardEl {}
impl BuildLakeformationResourceLfTagTableWithColumnsElColumnWildcardEl {
    pub fn build(self) -> LakeformationResourceLfTagTableWithColumnsElColumnWildcardEl {
        LakeformationResourceLfTagTableWithColumnsElColumnWildcardEl {
            excluded_column_names: core::default::Default::default(),
        }
    }
}
pub struct LakeformationResourceLfTagTableWithColumnsElColumnWildcardElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationResourceLfTagTableWithColumnsElColumnWildcardElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LakeformationResourceLfTagTableWithColumnsElColumnWildcardElRef {
        LakeformationResourceLfTagTableWithColumnsElColumnWildcardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationResourceLfTagTableWithColumnsElColumnWildcardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `excluded_column_names` after provisioning.\n"]
    pub fn excluded_column_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.excluded_column_names", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct LakeformationResourceLfTagTableWithColumnsElDynamic {
    column_wildcard:
        Option<DynamicBlock<LakeformationResourceLfTagTableWithColumnsElColumnWildcardEl>>,
}
#[derive(Serialize)]
pub struct LakeformationResourceLfTagTableWithColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_names: Option<SetField<PrimField<String>>>,
    database_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_wildcard: Option<Vec<LakeformationResourceLfTagTableWithColumnsElColumnWildcardEl>>,
    dynamic: LakeformationResourceLfTagTableWithColumnsElDynamic,
}
impl LakeformationResourceLfTagTableWithColumnsEl {
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
    #[doc = "Set the field `column_wildcard`.\n"]
    pub fn set_column_wildcard(
        mut self,
        v: impl Into<BlockAssignable<LakeformationResourceLfTagTableWithColumnsElColumnWildcardEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.column_wildcard = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.column_wildcard = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LakeformationResourceLfTagTableWithColumnsEl {
    type O = BlockAssignable<LakeformationResourceLfTagTableWithColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationResourceLfTagTableWithColumnsEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildLakeformationResourceLfTagTableWithColumnsEl {
    pub fn build(self) -> LakeformationResourceLfTagTableWithColumnsEl {
        LakeformationResourceLfTagTableWithColumnsEl {
            catalog_id: core::default::Default::default(),
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            name: self.name,
            column_wildcard: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LakeformationResourceLfTagTableWithColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationResourceLfTagTableWithColumnsElRef {
    fn new(shared: StackShared, base: String) -> LakeformationResourceLfTagTableWithColumnsElRef {
        LakeformationResourceLfTagTableWithColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationResourceLfTagTableWithColumnsElRef {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `column_wildcard` after provisioning.\n"]
    pub fn column_wildcard(
        &self,
    ) -> ListRef<LakeformationResourceLfTagTableWithColumnsElColumnWildcardElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_wildcard", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LakeformationResourceLfTagTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl LakeformationResourceLfTagTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationResourceLfTagTimeoutsEl {
    type O = BlockAssignable<LakeformationResourceLfTagTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationResourceLfTagTimeoutsEl {}
impl BuildLakeformationResourceLfTagTimeoutsEl {
    pub fn build(self) -> LakeformationResourceLfTagTimeoutsEl {
        LakeformationResourceLfTagTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct LakeformationResourceLfTagTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationResourceLfTagTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LakeformationResourceLfTagTimeoutsElRef {
        LakeformationResourceLfTagTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationResourceLfTagTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
#[derive(Serialize, Default)]
struct LakeformationResourceLfTagDynamic {
    database: Option<DynamicBlock<LakeformationResourceLfTagDatabaseEl>>,
    lf_tag: Option<DynamicBlock<LakeformationResourceLfTagLfTagEl>>,
    table: Option<DynamicBlock<LakeformationResourceLfTagTableEl>>,
    table_with_columns: Option<DynamicBlock<LakeformationResourceLfTagTableWithColumnsEl>>,
}
