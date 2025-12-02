use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GlueCatalogDatabaseData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location_uri: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_table_default_permission: Option<Vec<GlueCatalogDatabaseCreateTableDefaultPermissionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    federated_database: Option<Vec<GlueCatalogDatabaseFederatedDatabaseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_database: Option<Vec<GlueCatalogDatabaseTargetDatabaseEl>>,
    dynamic: GlueCatalogDatabaseDynamic,
}
struct GlueCatalogDatabase_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueCatalogDatabaseData>,
}
#[derive(Clone)]
pub struct GlueCatalogDatabase(Rc<GlueCatalogDatabase_>);
impl GlueCatalogDatabase {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location_uri`.\n"]
    pub fn set_location_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
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
    #[doc = "Set the field `create_table_default_permission`.\n"]
    pub fn set_create_table_default_permission(
        self,
        v: impl Into<BlockAssignable<GlueCatalogDatabaseCreateTableDefaultPermissionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().create_table_default_permission = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .create_table_default_permission = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `federated_database`.\n"]
    pub fn set_federated_database(
        self,
        v: impl Into<BlockAssignable<GlueCatalogDatabaseFederatedDatabaseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().federated_database = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.federated_database = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target_database`.\n"]
    pub fn set_target_database(
        self,
        v: impl Into<BlockAssignable<GlueCatalogDatabaseTargetDatabaseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_database = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_database = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location_uri` after provisioning.\n"]
    pub fn location_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `create_table_default_permission` after provisioning.\n"]
    pub fn create_table_default_permission(
        &self,
    ) -> ListRef<GlueCatalogDatabaseCreateTableDefaultPermissionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permission", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `federated_database` after provisioning.\n"]
    pub fn federated_database(&self) -> ListRef<GlueCatalogDatabaseFederatedDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.federated_database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_database` after provisioning.\n"]
    pub fn target_database(&self) -> ListRef<GlueCatalogDatabaseTargetDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_database", self.extract_ref()),
        )
    }
}
impl Referable for GlueCatalogDatabase {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GlueCatalogDatabase {}
impl ToListMappable for GlueCatalogDatabase {
    type O = ListRef<GlueCatalogDatabaseRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GlueCatalogDatabase_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_catalog_database".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGlueCatalogDatabase {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildGlueCatalogDatabase {
    pub fn build(self, stack: &mut Stack) -> GlueCatalogDatabase {
        let out = GlueCatalogDatabase(Rc::new(GlueCatalogDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueCatalogDatabaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                catalog_id: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location_uri: core::default::Default::default(),
                name: self.name,
                parameters: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                create_table_default_permission: core::default::Default::default(),
                federated_database: core::default::Default::default(),
                target_database: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GlueCatalogDatabaseRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCatalogDatabaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GlueCatalogDatabaseRef {
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
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location_uri` after provisioning.\n"]
    pub fn location_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `create_table_default_permission` after provisioning.\n"]
    pub fn create_table_default_permission(
        &self,
    ) -> ListRef<GlueCatalogDatabaseCreateTableDefaultPermissionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permission", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `federated_database` after provisioning.\n"]
    pub fn federated_database(&self) -> ListRef<GlueCatalogDatabaseFederatedDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.federated_database", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_database` after provisioning.\n"]
    pub fn target_database(&self) -> ListRef<GlueCatalogDatabaseTargetDatabaseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_database", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_lake_principal_identifier: Option<PrimField<String>>,
}
impl GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
    #[doc = "Set the field `data_lake_principal_identifier`.\n"]
    pub fn set_data_lake_principal_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_lake_principal_identifier = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
    type O = BlockAssignable<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {}
impl BuildGlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
    pub fn build(self) -> GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
        GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl {
            data_lake_principal_identifier: core::default::Default::default(),
        }
    }
}
pub struct GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
        GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef {
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
#[derive(Serialize, Default)]
struct GlueCatalogDatabaseCreateTableDefaultPermissionElDynamic {
    principal: Option<DynamicBlock<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl>>,
}
#[derive(Serialize)]
pub struct GlueCatalogDatabaseCreateTableDefaultPermissionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<Vec<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl>>,
    dynamic: GlueCatalogDatabaseCreateTableDefaultPermissionElDynamic,
}
impl GlueCatalogDatabaseCreateTableDefaultPermissionEl {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.permissions = Some(v.into());
        self
    }
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v: impl Into<BlockAssignable<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.principal = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GlueCatalogDatabaseCreateTableDefaultPermissionEl {
    type O = BlockAssignable<GlueCatalogDatabaseCreateTableDefaultPermissionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCatalogDatabaseCreateTableDefaultPermissionEl {}
impl BuildGlueCatalogDatabaseCreateTableDefaultPermissionEl {
    pub fn build(self) -> GlueCatalogDatabaseCreateTableDefaultPermissionEl {
        GlueCatalogDatabaseCreateTableDefaultPermissionEl {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
        GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCatalogDatabaseCreateTableDefaultPermissionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(
        &self,
    ) -> ListRef<GlueCatalogDatabaseCreateTableDefaultPermissionElPrincipalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueCatalogDatabaseFederatedDatabaseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier: Option<PrimField<String>>,
}
impl GlueCatalogDatabaseFederatedDatabaseEl {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_name = Some(v.into());
        self
    }
    #[doc = "Set the field `identifier`.\n"]
    pub fn set_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identifier = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCatalogDatabaseFederatedDatabaseEl {
    type O = BlockAssignable<GlueCatalogDatabaseFederatedDatabaseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCatalogDatabaseFederatedDatabaseEl {}
impl BuildGlueCatalogDatabaseFederatedDatabaseEl {
    pub fn build(self) -> GlueCatalogDatabaseFederatedDatabaseEl {
        GlueCatalogDatabaseFederatedDatabaseEl {
            connection_name: core::default::Default::default(),
            identifier: core::default::Default::default(),
        }
    }
}
pub struct GlueCatalogDatabaseFederatedDatabaseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCatalogDatabaseFederatedDatabaseElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogDatabaseFederatedDatabaseElRef {
        GlueCatalogDatabaseFederatedDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCatalogDatabaseFederatedDatabaseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueCatalogDatabaseTargetDatabaseEl {
    catalog_id: PrimField<String>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl GlueCatalogDatabaseTargetDatabaseEl {
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCatalogDatabaseTargetDatabaseEl {
    type O = BlockAssignable<GlueCatalogDatabaseTargetDatabaseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCatalogDatabaseTargetDatabaseEl {
    #[doc = ""]
    pub catalog_id: PrimField<String>,
    #[doc = ""]
    pub database_name: PrimField<String>,
}
impl BuildGlueCatalogDatabaseTargetDatabaseEl {
    pub fn build(self) -> GlueCatalogDatabaseTargetDatabaseEl {
        GlueCatalogDatabaseTargetDatabaseEl {
            catalog_id: self.catalog_id,
            database_name: self.database_name,
            region: core::default::Default::default(),
        }
    }
}
pub struct GlueCatalogDatabaseTargetDatabaseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCatalogDatabaseTargetDatabaseElRef {
    fn new(shared: StackShared, base: String) -> GlueCatalogDatabaseTargetDatabaseElRef {
        GlueCatalogDatabaseTargetDatabaseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCatalogDatabaseTargetDatabaseElRef {
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize, Default)]
struct GlueCatalogDatabaseDynamic {
    create_table_default_permission:
        Option<DynamicBlock<GlueCatalogDatabaseCreateTableDefaultPermissionEl>>,
    federated_database: Option<DynamicBlock<GlueCatalogDatabaseFederatedDatabaseEl>>,
    target_database: Option<DynamicBlock<GlueCatalogDatabaseTargetDatabaseEl>>,
}
