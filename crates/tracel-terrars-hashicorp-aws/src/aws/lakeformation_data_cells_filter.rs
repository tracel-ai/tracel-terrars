use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LakeformationDataCellsFilterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_data: Option<Vec<LakeformationDataCellsFilterTableDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LakeformationDataCellsFilterTimeoutsEl>,
    dynamic: LakeformationDataCellsFilterDynamic,
}

struct LakeformationDataCellsFilter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LakeformationDataCellsFilterData>,
}

#[derive(Clone)]
pub struct LakeformationDataCellsFilter(Rc<LakeformationDataCellsFilter_>);

impl LakeformationDataCellsFilter {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `table_data`.\n"]
    pub fn set_table_data(self, v: impl Into<BlockAssignable<LakeformationDataCellsFilterTableDataEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().table_data = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.table_data = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<LakeformationDataCellsFilterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_data` after provisioning.\n"]
    pub fn table_data(&self) -> ListRef<LakeformationDataCellsFilterTableDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.table_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LakeformationDataCellsFilterTimeoutsElRef {
        LakeformationDataCellsFilterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for LakeformationDataCellsFilter {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LakeformationDataCellsFilter { }

impl ToListMappable for LakeformationDataCellsFilter {
    type O = ListRef<LakeformationDataCellsFilterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LakeformationDataCellsFilter_ {
    fn extract_resource_type(&self) -> String {
        "aws_lakeformation_data_cells_filter".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLakeformationDataCellsFilter {
    pub tf_id: String,
}

impl BuildLakeformationDataCellsFilter {
    pub fn build(self, stack: &mut Stack) -> LakeformationDataCellsFilter {
        let out = LakeformationDataCellsFilter(Rc::new(LakeformationDataCellsFilter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LakeformationDataCellsFilterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                table_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LakeformationDataCellsFilterRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationDataCellsFilterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl LakeformationDataCellsFilterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_data` after provisioning.\n"]
    pub fn table_data(&self) -> ListRef<LakeformationDataCellsFilterTableDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.table_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LakeformationDataCellsFilterTimeoutsElRef {
        LakeformationDataCellsFilterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilterTableDataElColumnWildcardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_column_names: Option<ListField<PrimField<String>>>,
}

impl LakeformationDataCellsFilterTableDataElColumnWildcardEl {
    #[doc = "Set the field `excluded_column_names`.\n"]
    pub fn set_excluded_column_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.excluded_column_names = Some(v.into());
        self
    }
}

impl ToListMappable for LakeformationDataCellsFilterTableDataElColumnWildcardEl {
    type O = BlockAssignable<LakeformationDataCellsFilterTableDataElColumnWildcardEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilterTableDataElColumnWildcardEl {}

impl BuildLakeformationDataCellsFilterTableDataElColumnWildcardEl {
    pub fn build(self) -> LakeformationDataCellsFilterTableDataElColumnWildcardEl {
        LakeformationDataCellsFilterTableDataElColumnWildcardEl {
            excluded_column_names: core::default::Default::default(),
        }
    }
}

pub struct LakeformationDataCellsFilterTableDataElColumnWildcardElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationDataCellsFilterTableDataElColumnWildcardElRef {
    fn new(shared: StackShared, base: String) -> LakeformationDataCellsFilterTableDataElColumnWildcardElRef {
        LakeformationDataCellsFilterTableDataElColumnWildcardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilterTableDataElColumnWildcardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excluded_column_names` after provisioning.\n"]
    pub fn excluded_column_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.excluded_column_names", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl {}

impl LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl { }

impl ToListMappable for LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl {
    type O = BlockAssignable<LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl {}

impl BuildLakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl {
    pub fn build(self) -> LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl {
        LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl {}
    }
}

pub struct LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardElRef {
        LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct LakeformationDataCellsFilterTableDataElRowFilterElDynamic {
    all_rows_wildcard: Option<DynamicBlock<LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl>>,
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilterTableDataElRowFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_rows_wildcard: Option<Vec<LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl>>,
    dynamic: LakeformationDataCellsFilterTableDataElRowFilterElDynamic,
}

impl LakeformationDataCellsFilterTableDataElRowFilterEl {
    #[doc = "Set the field `filter_expression`.\n"]
    pub fn set_filter_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_expression = Some(v.into());
        self
    }

    #[doc = "Set the field `all_rows_wildcard`.\n"]
    pub fn set_all_rows_wildcard(
        mut self,
        v: impl Into<BlockAssignable<LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.all_rows_wildcard = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.all_rows_wildcard = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for LakeformationDataCellsFilterTableDataElRowFilterEl {
    type O = BlockAssignable<LakeformationDataCellsFilterTableDataElRowFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilterTableDataElRowFilterEl {}

impl BuildLakeformationDataCellsFilterTableDataElRowFilterEl {
    pub fn build(self) -> LakeformationDataCellsFilterTableDataElRowFilterEl {
        LakeformationDataCellsFilterTableDataElRowFilterEl {
            filter_expression: core::default::Default::default(),
            all_rows_wildcard: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LakeformationDataCellsFilterTableDataElRowFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationDataCellsFilterTableDataElRowFilterElRef {
    fn new(shared: StackShared, base: String) -> LakeformationDataCellsFilterTableDataElRowFilterElRef {
        LakeformationDataCellsFilterTableDataElRowFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilterTableDataElRowFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_expression` after provisioning.\n"]
    pub fn filter_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `all_rows_wildcard` after provisioning.\n"]
    pub fn all_rows_wildcard(&self) -> ListRef<LakeformationDataCellsFilterTableDataElRowFilterElAllRowsWildcardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.all_rows_wildcard", self.base))
    }
}

#[derive(Serialize, Default)]
struct LakeformationDataCellsFilterTableDataElDynamic {
    column_wildcard: Option<DynamicBlock<LakeformationDataCellsFilterTableDataElColumnWildcardEl>>,
    row_filter: Option<DynamicBlock<LakeformationDataCellsFilterTableDataElRowFilterEl>>,
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilterTableDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    column_names: Option<SetField<PrimField<String>>>,
    database_name: PrimField<String>,
    name: PrimField<String>,
    table_catalog_id: PrimField<String>,
    table_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_wildcard: Option<Vec<LakeformationDataCellsFilterTableDataElColumnWildcardEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_filter: Option<Vec<LakeformationDataCellsFilterTableDataElRowFilterEl>>,
    dynamic: LakeformationDataCellsFilterTableDataElDynamic,
}

impl LakeformationDataCellsFilterTableDataEl {
    #[doc = "Set the field `column_names`.\n"]
    pub fn set_column_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.column_names = Some(v.into());
        self
    }

    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version_id = Some(v.into());
        self
    }

    #[doc = "Set the field `column_wildcard`.\n"]
    pub fn set_column_wildcard(
        mut self,
        v: impl Into<BlockAssignable<LakeformationDataCellsFilterTableDataElColumnWildcardEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.column_wildcard = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.column_wildcard = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `row_filter`.\n"]
    pub fn set_row_filter(
        mut self,
        v: impl Into<BlockAssignable<LakeformationDataCellsFilterTableDataElRowFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.row_filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.row_filter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for LakeformationDataCellsFilterTableDataEl {
    type O = BlockAssignable<LakeformationDataCellsFilterTableDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilterTableDataEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub table_catalog_id: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}

impl BuildLakeformationDataCellsFilterTableDataEl {
    pub fn build(self) -> LakeformationDataCellsFilterTableDataEl {
        LakeformationDataCellsFilterTableDataEl {
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            name: self.name,
            table_catalog_id: self.table_catalog_id,
            table_name: self.table_name,
            version_id: core::default::Default::default(),
            column_wildcard: core::default::Default::default(),
            row_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct LakeformationDataCellsFilterTableDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationDataCellsFilterTableDataElRef {
    fn new(shared: StackShared, base: String) -> LakeformationDataCellsFilterTableDataElRef {
        LakeformationDataCellsFilterTableDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilterTableDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column_names` after provisioning.\n"]
    pub fn column_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.column_names", self.base))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `table_catalog_id` after provisioning.\n"]
    pub fn table_catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_catalog_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }

    #[doc = "Get a reference to the value of field `column_wildcard` after provisioning.\n"]
    pub fn column_wildcard(&self) -> ListRef<LakeformationDataCellsFilterTableDataElColumnWildcardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.column_wildcard", self.base))
    }

    #[doc = "Get a reference to the value of field `row_filter` after provisioning.\n"]
    pub fn row_filter(&self) -> ListRef<LakeformationDataCellsFilterTableDataElRowFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.row_filter", self.base))
    }
}

#[derive(Serialize)]
pub struct LakeformationDataCellsFilterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl LakeformationDataCellsFilterTimeoutsEl {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for LakeformationDataCellsFilterTimeoutsEl {
    type O = BlockAssignable<LakeformationDataCellsFilterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLakeformationDataCellsFilterTimeoutsEl {}

impl BuildLakeformationDataCellsFilterTimeoutsEl {
    pub fn build(self) -> LakeformationDataCellsFilterTimeoutsEl {
        LakeformationDataCellsFilterTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct LakeformationDataCellsFilterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LakeformationDataCellsFilterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> LakeformationDataCellsFilterTimeoutsElRef {
        LakeformationDataCellsFilterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LakeformationDataCellsFilterTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct LakeformationDataCellsFilterDynamic {
    table_data: Option<DynamicBlock<LakeformationDataCellsFilterTableDataEl>>,
}
