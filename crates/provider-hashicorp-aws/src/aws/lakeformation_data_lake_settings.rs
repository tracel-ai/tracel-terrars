use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LakeformationDataLakeSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admins: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_external_data_filtering: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_full_table_external_data_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_session_tag_value_list: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_data_filtering_allow_list: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only_admins: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trusted_resource_owners: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_database_default_permissions:
        Option<Vec<LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_table_default_permissions:
        Option<Vec<LakeformationDataLakeSettingsCreateTableDefaultPermissionsEl>>,
    dynamic: LakeformationDataLakeSettingsDynamic,
}
struct LakeformationDataLakeSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LakeformationDataLakeSettingsData>,
}
#[derive(Clone)]
pub struct LakeformationDataLakeSettings(Rc<LakeformationDataLakeSettings_>);
impl LakeformationDataLakeSettings {
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
    #[doc = "Set the field `admins`.\n"]
    pub fn set_admins(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().admins = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_external_data_filtering`.\n"]
    pub fn set_allow_external_data_filtering(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_external_data_filtering = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_full_table_external_data_access`.\n"]
    pub fn set_allow_full_table_external_data_access(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0
            .data
            .borrow_mut()
            .allow_full_table_external_data_access = Some(v.into());
        self
    }
    #[doc = "Set the field `authorized_session_tag_value_list`.\n"]
    pub fn set_authorized_session_tag_value_list(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().authorized_session_tag_value_list = Some(v.into());
        self
    }
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = Some(v.into());
        self
    }
    #[doc = "Set the field `external_data_filtering_allow_list`.\n"]
    pub fn set_external_data_filtering_allow_list(
        self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().external_data_filtering_allow_list = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().parameters = Some(v.into());
        self
    }
    #[doc = "Set the field `read_only_admins`.\n"]
    pub fn set_read_only_admins(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().read_only_admins = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `trusted_resource_owners`.\n"]
    pub fn set_trusted_resource_owners(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().trusted_resource_owners = Some(v.into());
        self
    }
    #[doc = "Set the field `create_database_default_permissions`.\n"]
    pub fn set_create_database_default_permissions(
        self,
        v: impl Into<BlockAssignable<LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().create_database_default_permissions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .create_database_default_permissions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `create_table_default_permissions`.\n"]
    pub fn set_create_table_default_permissions(
        self,
        v: impl Into<BlockAssignable<LakeformationDataLakeSettingsCreateTableDefaultPermissionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().create_table_default_permissions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .create_table_default_permissions = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `admins` after provisioning.\n"]
    pub fn admins(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.admins", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_external_data_filtering` after provisioning.\n"]
    pub fn allow_external_data_filtering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_external_data_filtering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_full_table_external_data_access` after provisioning.\n"]
    pub fn allow_full_table_external_data_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.allow_full_table_external_data_access",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_session_tag_value_list` after provisioning.\n"]
    pub fn authorized_session_tag_value_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_session_tag_value_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_data_filtering_allow_list` after provisioning.\n"]
    pub fn external_data_filtering_allow_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.external_data_filtering_allow_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_only_admins` after provisioning.\n"]
    pub fn read_only_admins(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.read_only_admins", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_resource_owners` after provisioning.\n"]
    pub fn trusted_resource_owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_resource_owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> ListRef<LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(
        &self,
    ) -> ListRef<LakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }
}
impl Referable for LakeformationDataLakeSettings {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LakeformationDataLakeSettings {}
impl ToListMappable for LakeformationDataLakeSettings {
    type O = ListRef<LakeformationDataLakeSettingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LakeformationDataLakeSettings_ {
    fn extract_resource_type(&self) -> String {
        "aws_lakeformation_data_lake_settings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLakeformationDataLakeSettings {
    pub tf_id: String,
}
impl BuildLakeformationDataLakeSettings {
    pub fn build(self, stack: &mut Stack) -> LakeformationDataLakeSettings {
        let out = LakeformationDataLakeSettings(Rc::new(LakeformationDataLakeSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LakeformationDataLakeSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admins: core::default::Default::default(),
                allow_external_data_filtering: core::default::Default::default(),
                allow_full_table_external_data_access: core::default::Default::default(),
                authorized_session_tag_value_list: core::default::Default::default(),
                catalog_id: core::default::Default::default(),
                external_data_filtering_allow_list: core::default::Default::default(),
                id: core::default::Default::default(),
                parameters: core::default::Default::default(),
                read_only_admins: core::default::Default::default(),
                region: core::default::Default::default(),
                trusted_resource_owners: core::default::Default::default(),
                create_database_default_permissions: core::default::Default::default(),
                create_table_default_permissions: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LakeformationDataLakeSettingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationDataLakeSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LakeformationDataLakeSettingsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admins` after provisioning.\n"]
    pub fn admins(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.admins", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_external_data_filtering` after provisioning.\n"]
    pub fn allow_external_data_filtering(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_external_data_filtering", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `allow_full_table_external_data_access` after provisioning.\n"]
    pub fn allow_full_table_external_data_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.allow_full_table_external_data_access",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_session_tag_value_list` after provisioning.\n"]
    pub fn authorized_session_tag_value_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_session_tag_value_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.catalog_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_data_filtering_allow_list` after provisioning.\n"]
    pub fn external_data_filtering_allow_list(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.external_data_filtering_allow_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_only_admins` after provisioning.\n"]
    pub fn read_only_admins(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.read_only_admins", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `trusted_resource_owners` after provisioning.\n"]
    pub fn trusted_resource_owners(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.trusted_resource_owners", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> ListRef<LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(
        &self,
    ) -> ListRef<LakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<PrimField<String>>,
}
impl LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.permissions = Some(v.into());
        self
    }
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
    type O = BlockAssignable<LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {}
impl BuildLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
    pub fn build(self) -> LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
        LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}
pub struct LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
        LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct LakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<PrimField<String>>,
}
impl LakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.permissions = Some(v.into());
        self
    }
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal = Some(v.into());
        self
    }
}
impl ToListMappable for LakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
    type O = BlockAssignable<LakeformationDataLakeSettingsCreateTableDefaultPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {}
impl BuildLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
    pub fn build(self) -> LakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
        LakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}
pub struct LakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
        LakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize, Default)]
struct LakeformationDataLakeSettingsDynamic {
    create_database_default_permissions:
        Option<DynamicBlock<LakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl>>,
    create_table_default_permissions:
        Option<DynamicBlock<LakeformationDataLakeSettingsCreateTableDefaultPermissionsEl>>,
}
