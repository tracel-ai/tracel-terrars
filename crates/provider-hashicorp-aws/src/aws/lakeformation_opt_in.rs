use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LakeformationOptInData {
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
    condition: Option<Vec<LakeformationOptInConditionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<Vec<LakeformationOptInPrincipalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_data: Option<Vec<LakeformationOptInResourceDataEl>>,
    dynamic: LakeformationOptInDynamic,
}
struct LakeformationOptIn_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LakeformationOptInData>,
}
#[derive(Clone)]
pub struct LakeformationOptIn(Rc<LakeformationOptIn_>);
impl LakeformationOptIn {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl Into<BlockAssignable<LakeformationOptInConditionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        self,
        v: impl Into<BlockAssignable<LakeformationOptInPrincipalEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().principal = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.principal = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `resource_data`.\n"]
    pub fn set_resource_data(
        self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_data = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_updated_by` after provisioning.\n"]
    pub fn last_updated_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<LakeformationOptInConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> ListRef<LakeformationOptInPrincipalElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.principal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_data` after provisioning.\n"]
    pub fn resource_data(&self) -> ListRef<LakeformationOptInResourceDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_data", self.extract_ref()),
        )
    }
}
impl Referable for LakeformationOptIn {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LakeformationOptIn {}
impl ToListMappable for LakeformationOptIn {
    type O = ListRef<LakeformationOptInRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LakeformationOptIn_ {
    fn extract_resource_type(&self) -> String {
        "aws_lakeformation_opt_in".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLakeformationOptIn {
    pub tf_id: String,
}
impl BuildLakeformationOptIn {
    pub fn build(self, stack: &mut Stack) -> LakeformationOptIn {
        let out = LakeformationOptIn(Rc::new(LakeformationOptIn_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LakeformationOptInData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                condition: core::default::Default::default(),
                principal: core::default::Default::default(),
                resource_data: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LakeformationOptInRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LakeformationOptInRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_updated_by` after provisioning.\n"]
    pub fn last_updated_by(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> ListRef<LakeformationOptInConditionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> ListRef<LakeformationOptInPrincipalElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.principal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_data` after provisioning.\n"]
    pub fn resource_data(&self) -> ListRef<LakeformationOptInResourceDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_data", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LakeformationOptInConditionEl {}
impl LakeformationOptInConditionEl {}
impl ToListMappable for LakeformationOptInConditionEl {
    type O = BlockAssignable<LakeformationOptInConditionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInConditionEl {}
impl BuildLakeformationOptInConditionEl {
    pub fn build(self) -> LakeformationOptInConditionEl {
        LakeformationOptInConditionEl {}
    }
}
pub struct LakeformationOptInConditionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInConditionElRef {
    fn new(shared: StackShared, base: String) -> LakeformationOptInConditionElRef {
        LakeformationOptInConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}
#[derive(Serialize)]
pub struct LakeformationOptInPrincipalEl {
    data_lake_principal_identifier: PrimField<String>,
}
impl LakeformationOptInPrincipalEl {}
impl ToListMappable for LakeformationOptInPrincipalEl {
    type O = BlockAssignable<LakeformationOptInPrincipalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInPrincipalEl {
    #[doc = ""]
    pub data_lake_principal_identifier: PrimField<String>,
}
impl BuildLakeformationOptInPrincipalEl {
    pub fn build(self) -> LakeformationOptInPrincipalEl {
        LakeformationOptInPrincipalEl {
            data_lake_principal_identifier: self.data_lake_principal_identifier,
        }
    }
}
pub struct LakeformationOptInPrincipalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInPrincipalElRef {
    fn new(shared: StackShared, base: String) -> LakeformationOptInPrincipalElRef {
        LakeformationOptInPrincipalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInPrincipalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_lake_principal_identifier` after provisioning.\n"]
    pub fn data_lake_principal_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_lake_principal_identifier", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct LakeformationOptInResourceDataElCatalogEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
impl LakeformationOptInResourceDataElCatalogEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationOptInResourceDataElCatalogEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElCatalogEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElCatalogEl {}
impl BuildLakeformationOptInResourceDataElCatalogEl {
    pub fn build(self) -> LakeformationOptInResourceDataElCatalogEl {
        LakeformationOptInResourceDataElCatalogEl {
            id: core::default::Default::default(),
        }
    }
}
pub struct LakeformationOptInResourceDataElCatalogElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElCatalogElRef {
    fn new(shared: StackShared, base: String) -> LakeformationOptInResourceDataElCatalogElRef {
        LakeformationOptInResourceDataElCatalogElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElCatalogElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize)]
pub struct LakeformationOptInResourceDataElDataCellsFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    database_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_name: Option<PrimField<String>>,
}
impl LakeformationOptInResourceDataElDataCellsFilterEl {
    #[doc = "Set the field `database_name`.\n"]
    pub fn set_database_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.database_name = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `table_catalog_id`.\n"]
    pub fn set_table_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_catalog_id = Some(v.into());
        self
    }
    #[doc = "Set the field `table_name`.\n"]
    pub fn set_table_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.table_name = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationOptInResourceDataElDataCellsFilterEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElDataCellsFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElDataCellsFilterEl {}
impl BuildLakeformationOptInResourceDataElDataCellsFilterEl {
    pub fn build(self) -> LakeformationOptInResourceDataElDataCellsFilterEl {
        LakeformationOptInResourceDataElDataCellsFilterEl {
            database_name: core::default::Default::default(),
            name: core::default::Default::default(),
            table_catalog_id: core::default::Default::default(),
            table_name: core::default::Default::default(),
        }
    }
}
pub struct LakeformationOptInResourceDataElDataCellsFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElDataCellsFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LakeformationOptInResourceDataElDataCellsFilterElRef {
        LakeformationOptInResourceDataElDataCellsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElDataCellsFilterElRef {
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
pub struct LakeformationOptInResourceDataElDataLocationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    resource_arn: PrimField<String>,
}
impl LakeformationOptInResourceDataElDataLocationEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationOptInResourceDataElDataLocationEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElDataLocationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElDataLocationEl {
    #[doc = ""]
    pub resource_arn: PrimField<String>,
}
impl BuildLakeformationOptInResourceDataElDataLocationEl {
    pub fn build(self) -> LakeformationOptInResourceDataElDataLocationEl {
        LakeformationOptInResourceDataElDataLocationEl {
            catalog_id: core::default::Default::default(),
            resource_arn: self.resource_arn,
        }
    }
}
pub struct LakeformationOptInResourceDataElDataLocationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElDataLocationElRef {
    fn new(shared: StackShared, base: String) -> LakeformationOptInResourceDataElDataLocationElRef {
        LakeformationOptInResourceDataElDataLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElDataLocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct LakeformationOptInResourceDataElDatabaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl LakeformationOptInResourceDataElDatabaseEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationOptInResourceDataElDatabaseEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElDatabaseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElDatabaseEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildLakeformationOptInResourceDataElDatabaseEl {
    pub fn build(self) -> LakeformationOptInResourceDataElDatabaseEl {
        LakeformationOptInResourceDataElDatabaseEl {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct LakeformationOptInResourceDataElDatabaseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElDatabaseElRef {
    fn new(shared: StackShared, base: String) -> LakeformationOptInResourceDataElDatabaseElRef {
        LakeformationOptInResourceDataElDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElDatabaseElRef {
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
pub struct LakeformationOptInResourceDataElLfTagEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    key: PrimField<String>,
    value: PrimField<String>,
}
impl LakeformationOptInResourceDataElLfTagEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationOptInResourceDataElLfTagEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElLfTagEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElLfTagEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildLakeformationOptInResourceDataElLfTagEl {
    pub fn build(self) -> LakeformationOptInResourceDataElLfTagEl {
        LakeformationOptInResourceDataElLfTagEl {
            catalog_id: core::default::Default::default(),
            key: self.key,
            value: self.value,
        }
    }
}
pub struct LakeformationOptInResourceDataElLfTagElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElLfTagElRef {
    fn new(shared: StackShared, base: String) -> LakeformationOptInResourceDataElLfTagElRef {
        LakeformationOptInResourceDataElLfTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElLfTagElRef {
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
pub struct LakeformationOptInResourceDataElLfTagExpressionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    name: PrimField<String>,
}
impl LakeformationOptInResourceDataElLfTagExpressionEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationOptInResourceDataElLfTagExpressionEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElLfTagExpressionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElLfTagExpressionEl {
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildLakeformationOptInResourceDataElLfTagExpressionEl {
    pub fn build(self) -> LakeformationOptInResourceDataElLfTagExpressionEl {
        LakeformationOptInResourceDataElLfTagExpressionEl {
            catalog_id: core::default::Default::default(),
            name: self.name,
        }
    }
}
pub struct LakeformationOptInResourceDataElLfTagExpressionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElLfTagExpressionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LakeformationOptInResourceDataElLfTagExpressionElRef {
        LakeformationOptInResourceDataElLfTagExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElLfTagExpressionElRef {
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
pub struct LakeformationOptInResourceDataElLfTagPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expression_name: Option<PrimField<String>>,
    resource_type: PrimField<String>,
}
impl LakeformationOptInResourceDataElLfTagPolicyEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expression = Some(v.into());
        self
    }
    #[doc = "Set the field `expression_name`.\n"]
    pub fn set_expression_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.expression_name = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationOptInResourceDataElLfTagPolicyEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElLfTagPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElLfTagPolicyEl {
    #[doc = ""]
    pub resource_type: PrimField<String>,
}
impl BuildLakeformationOptInResourceDataElLfTagPolicyEl {
    pub fn build(self) -> LakeformationOptInResourceDataElLfTagPolicyEl {
        LakeformationOptInResourceDataElLfTagPolicyEl {
            catalog_id: core::default::Default::default(),
            expression: core::default::Default::default(),
            expression_name: core::default::Default::default(),
            resource_type: self.resource_type,
        }
    }
}
pub struct LakeformationOptInResourceDataElLfTagPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElLfTagPolicyElRef {
    fn new(shared: StackShared, base: String) -> LakeformationOptInResourceDataElLfTagPolicyElRef {
        LakeformationOptInResourceDataElLfTagPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElLfTagPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `expression_name` after provisioning.\n"]
    pub fn expression_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expression_name", self.base),
        )
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
pub struct LakeformationOptInResourceDataElTableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wildcard: Option<PrimField<bool>>,
}
impl LakeformationOptInResourceDataElTableEl {
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
impl ToListMappable for LakeformationOptInResourceDataElTableEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElTableEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElTableEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
}
impl BuildLakeformationOptInResourceDataElTableEl {
    pub fn build(self) -> LakeformationOptInResourceDataElTableEl {
        LakeformationOptInResourceDataElTableEl {
            catalog_id: core::default::Default::default(),
            database_name: self.database_name,
            name: core::default::Default::default(),
            wildcard: core::default::Default::default(),
        }
    }
}
pub struct LakeformationOptInResourceDataElTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElTableElRef {
    fn new(shared: StackShared, base: String) -> LakeformationOptInResourceDataElTableElRef {
        LakeformationOptInResourceDataElTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElTableElRef {
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
pub struct LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_column_names: Option<SetField<PrimField<String>>>,
}
impl LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl {
    #[doc = "Set the field `excluded_column_names`.\n"]
    pub fn set_excluded_column_names(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.excluded_column_names = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl {}
impl BuildLakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl {
    pub fn build(self) -> LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl {
        LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl {
            excluded_column_names: core::default::Default::default(),
        }
    }
}
pub struct LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardElRef {
        LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardElRef {
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
struct LakeformationOptInResourceDataElTableWithColumnsElDynamic {
    column_wildcard:
        Option<DynamicBlock<LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl>>,
}
#[derive(Serialize)]
pub struct LakeformationOptInResourceDataElTableWithColumnsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_names: Option<SetField<PrimField<String>>>,
    database_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    column_wildcard:
        Option<Vec<LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl>>,
    dynamic: LakeformationOptInResourceDataElTableWithColumnsElDynamic,
}
impl LakeformationOptInResourceDataElTableWithColumnsEl {
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
        v: impl Into<
            BlockAssignable<LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardEl>,
        >,
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
impl ToListMappable for LakeformationOptInResourceDataElTableWithColumnsEl {
    type O = BlockAssignable<LakeformationOptInResourceDataElTableWithColumnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataElTableWithColumnsEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildLakeformationOptInResourceDataElTableWithColumnsEl {
    pub fn build(self) -> LakeformationOptInResourceDataElTableWithColumnsEl {
        LakeformationOptInResourceDataElTableWithColumnsEl {
            catalog_id: core::default::Default::default(),
            column_names: core::default::Default::default(),
            database_name: self.database_name,
            name: self.name,
            column_wildcard: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LakeformationOptInResourceDataElTableWithColumnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElTableWithColumnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LakeformationOptInResourceDataElTableWithColumnsElRef {
        LakeformationOptInResourceDataElTableWithColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElTableWithColumnsElRef {
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
    ) -> ListRef<LakeformationOptInResourceDataElTableWithColumnsElColumnWildcardElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.column_wildcard", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct LakeformationOptInResourceDataElDynamic {
    catalog: Option<DynamicBlock<LakeformationOptInResourceDataElCatalogEl>>,
    data_cells_filter: Option<DynamicBlock<LakeformationOptInResourceDataElDataCellsFilterEl>>,
    data_location: Option<DynamicBlock<LakeformationOptInResourceDataElDataLocationEl>>,
    database: Option<DynamicBlock<LakeformationOptInResourceDataElDatabaseEl>>,
    lf_tag: Option<DynamicBlock<LakeformationOptInResourceDataElLfTagEl>>,
    lf_tag_expression: Option<DynamicBlock<LakeformationOptInResourceDataElLfTagExpressionEl>>,
    lf_tag_policy: Option<DynamicBlock<LakeformationOptInResourceDataElLfTagPolicyEl>>,
    table: Option<DynamicBlock<LakeformationOptInResourceDataElTableEl>>,
    table_with_columns: Option<DynamicBlock<LakeformationOptInResourceDataElTableWithColumnsEl>>,
}
#[derive(Serialize)]
pub struct LakeformationOptInResourceDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog: Option<Vec<LakeformationOptInResourceDataElCatalogEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_cells_filter: Option<Vec<LakeformationOptInResourceDataElDataCellsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_location: Option<Vec<LakeformationOptInResourceDataElDataLocationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database: Option<Vec<LakeformationOptInResourceDataElDatabaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lf_tag: Option<Vec<LakeformationOptInResourceDataElLfTagEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lf_tag_expression: Option<Vec<LakeformationOptInResourceDataElLfTagExpressionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lf_tag_policy: Option<Vec<LakeformationOptInResourceDataElLfTagPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table: Option<Vec<LakeformationOptInResourceDataElTableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_with_columns: Option<Vec<LakeformationOptInResourceDataElTableWithColumnsEl>>,
    dynamic: LakeformationOptInResourceDataElDynamic,
}
impl LakeformationOptInResourceDataEl {
    #[doc = "Set the field `catalog`.\n"]
    pub fn set_catalog(
        mut self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataElCatalogEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.catalog = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.catalog = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_cells_filter`.\n"]
    pub fn set_data_cells_filter(
        mut self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataElDataCellsFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_cells_filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_cells_filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_location`.\n"]
    pub fn set_data_location(
        mut self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataElDataLocationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_location = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_location = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `database`.\n"]
    pub fn set_database(
        mut self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataElDatabaseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.database = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.database = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lf_tag`.\n"]
    pub fn set_lf_tag(
        mut self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataElLfTagEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lf_tag = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lf_tag = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lf_tag_expression`.\n"]
    pub fn set_lf_tag_expression(
        mut self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataElLfTagExpressionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lf_tag_expression = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lf_tag_expression = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lf_tag_policy`.\n"]
    pub fn set_lf_tag_policy(
        mut self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataElLfTagPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lf_tag_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lf_tag_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        mut self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataElTableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.table = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `table_with_columns`.\n"]
    pub fn set_table_with_columns(
        mut self,
        v: impl Into<BlockAssignable<LakeformationOptInResourceDataElTableWithColumnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.table_with_columns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.table_with_columns = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LakeformationOptInResourceDataEl {
    type O = BlockAssignable<LakeformationOptInResourceDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationOptInResourceDataEl {}
impl BuildLakeformationOptInResourceDataEl {
    pub fn build(self) -> LakeformationOptInResourceDataEl {
        LakeformationOptInResourceDataEl {
            catalog: core::default::Default::default(),
            data_cells_filter: core::default::Default::default(),
            data_location: core::default::Default::default(),
            database: core::default::Default::default(),
            lf_tag: core::default::Default::default(),
            lf_tag_expression: core::default::Default::default(),
            lf_tag_policy: core::default::Default::default(),
            table: core::default::Default::default(),
            table_with_columns: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LakeformationOptInResourceDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationOptInResourceDataElRef {
    fn new(shared: StackShared, base: String) -> LakeformationOptInResourceDataElRef {
        LakeformationOptInResourceDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationOptInResourceDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> ListRef<LakeformationOptInResourceDataElCatalogElRef> {
        ListRef::new(self.shared().clone(), format!("{}.catalog", self.base))
    }
    #[doc = "Get a reference to the value of field `data_cells_filter` after provisioning.\n"]
    pub fn data_cells_filter(
        &self,
    ) -> ListRef<LakeformationOptInResourceDataElDataCellsFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_cells_filter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_location` after provisioning.\n"]
    pub fn data_location(&self) -> ListRef<LakeformationOptInResourceDataElDataLocationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> ListRef<LakeformationOptInResourceDataElDatabaseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.database", self.base))
    }
    #[doc = "Get a reference to the value of field `lf_tag` after provisioning.\n"]
    pub fn lf_tag(&self) -> ListRef<LakeformationOptInResourceDataElLfTagElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lf_tag", self.base))
    }
    #[doc = "Get a reference to the value of field `lf_tag_expression` after provisioning.\n"]
    pub fn lf_tag_expression(
        &self,
    ) -> ListRef<LakeformationOptInResourceDataElLfTagExpressionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag_expression", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lf_tag_policy` after provisioning.\n"]
    pub fn lf_tag_policy(&self) -> ListRef<LakeformationOptInResourceDataElLfTagPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lf_tag_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> ListRef<LakeformationOptInResourceDataElTableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }
    #[doc = "Get a reference to the value of field `table_with_columns` after provisioning.\n"]
    pub fn table_with_columns(
        &self,
    ) -> ListRef<LakeformationOptInResourceDataElTableWithColumnsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.table_with_columns", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct LakeformationOptInDynamic {
    condition: Option<DynamicBlock<LakeformationOptInConditionEl>>,
    principal: Option<DynamicBlock<LakeformationOptInPrincipalEl>>,
    resource_data: Option<DynamicBlock<LakeformationOptInResourceDataEl>>,
}
