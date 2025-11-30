use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataLakeformationDataLakeSettingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataLakeformationDataLakeSettings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLakeformationDataLakeSettingsData>,
}

#[derive(Clone)]
pub struct DataLakeformationDataLakeSettings(Rc<DataLakeformationDataLakeSettings_>);

impl DataLakeformationDataLakeSettings {
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

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> ListRef<DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(
        &self,
    ) -> ListRef<DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
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
}

impl Referable for DataLakeformationDataLakeSettings {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataLakeformationDataLakeSettings {}

impl ToListMappable for DataLakeformationDataLakeSettings {
    type O = ListRef<DataLakeformationDataLakeSettingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataLakeformationDataLakeSettings_ {
    fn extract_datasource_type(&self) -> String {
        "aws_lakeformation_data_lake_settings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLakeformationDataLakeSettings {
    pub tf_id: String,
}

impl BuildDataLakeformationDataLakeSettings {
    pub fn build(self, stack: &mut Stack) -> DataLakeformationDataLakeSettings {
        let out = DataLakeformationDataLakeSettings(Rc::new(DataLakeformationDataLakeSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLakeformationDataLakeSettingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                catalog_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLakeformationDataLakeSettingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLakeformationDataLakeSettingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataLakeformationDataLakeSettingsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc = "Get a reference to the value of field `create_database_default_permissions` after provisioning.\n"]
    pub fn create_database_default_permissions(
        &self,
    ) -> ListRef<DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_database_default_permissions", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `create_table_default_permissions` after provisioning.\n"]
    pub fn create_table_default_permissions(
        &self,
    ) -> ListRef<DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_table_default_permissions", self.extract_ref()),
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
}

#[derive(Serialize)]
pub struct DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<PrimField<String>>,
}

impl DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
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

impl ToListMappable for DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
    type O = BlockAssignable<DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {}

impl BuildDataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
    pub fn build(self) -> DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
        DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsEl {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

pub struct DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
        DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationDataLakeSettingsCreateDatabaseDefaultPermissionsElRef {
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
pub struct DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<PrimField<String>>,
}

impl DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
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

impl ToListMappable for DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
    type O = BlockAssignable<DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {}

impl BuildDataLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
    pub fn build(self) -> DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
        DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsEl {
            permissions: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

pub struct DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
        DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLakeformationDataLakeSettingsCreateTableDefaultPermissionsElRef {
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
