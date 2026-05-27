use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SpannerDatabaseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_dialect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ddl: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_time_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_drop_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_retention_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<Vec<SpannerDatabaseEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SpannerDatabaseTimeoutsEl>,
    dynamic: SpannerDatabaseDynamic,
}
struct SpannerDatabase_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SpannerDatabaseData>,
}
#[derive(Clone)]
pub struct SpannerDatabase(Rc<SpannerDatabase_>);
impl SpannerDatabase {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `database_dialect`.\nThe dialect of the Cloud Spanner Database.\nIf it is not provided, \"GOOGLE_STANDARD_SQL\" will be used. Possible values: [\"GOOGLE_STANDARD_SQL\", \"POSTGRESQL\"]"]
    pub fn set_database_dialect(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().database_dialect = Some(v.into());
        self
    }
    #[doc = "Set the field `ddl`.\nAn optional list of DDL statements to run inside the database. Statements can create\ntables, indexes, etc.\n\nDuring creation these statements execute atomically with the creation of the database\nand if there is an error in any statement, the database is not created.\n\nTerraform does not perform drift detection on this field and assumes that the values\nrecorded in state are accurate. Limited updates to this field are supported, and\nnewly appended DDL statements can be executed in an update. However, modifications\nto prior statements will create a plan that marks the resource for recreation."]
    pub fn set_ddl(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ddl = Some(v.into());
        self
    }
    #[doc = "Set the field `default_time_zone`.\nThe default time zone for the database. The default time zone must be a valid name\nfrom the tz database. Default value is \"America/Los_angeles\"."]
    pub fn set_default_time_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_time_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the database. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the database,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the database will fail.\nWhen the field is set to false, deleting the database is allowed."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_drop_protection`.\nWhether drop protection is enabled for this database. Defaults to false.\nDrop protection is different from\nthe \"deletion_protection\" attribute in the following ways:\n(1) \"deletion_protection\" only protects the database from deletions in Terraform.\nwhereas setting “enableDropProtection” to true protects the database from deletions in all interfaces.\n(2) Setting \"enableDropProtection\" to true also prevents the deletion of the parent instance containing the database.\n\"deletion_protection\" attribute does not provide protection against the deletion of the parent instance."]
    pub fn set_enable_drop_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_drop_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `version_retention_period`.\nThe retention period for the database. The retention period must be between 1 hour\nand 7 days, and can be specified in days, hours, minutes, or seconds. For example,\nthe values 1d, 24h, 1440m, and 86400s are equivalent. Default value is 1h.\nIf this property is used, you must avoid adding new DDL statements to 'ddl' that\nupdate the database's version_retention_period."]
    pub fn set_version_retention_period(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_retention_period = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl Into<BlockAssignable<SpannerDatabaseEncryptionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SpannerDatabaseTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `database_dialect` after provisioning.\nThe dialect of the Cloud Spanner Database.\nIf it is not provided, \"GOOGLE_STANDARD_SQL\" will be used. Possible values: [\"GOOGLE_STANDARD_SQL\", \"POSTGRESQL\"]"]
    pub fn database_dialect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_dialect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ddl` after provisioning.\nAn optional list of DDL statements to run inside the database. Statements can create\ntables, indexes, etc.\n\nDuring creation these statements execute atomically with the creation of the database\nand if there is an error in any statement, the database is not created.\n\nTerraform does not perform drift detection on this field and assumes that the values\nrecorded in state are accurate. Limited updates to this field are supported, and\nnewly appended DDL statements can be executed in an update. However, modifications\nto prior statements will create a plan that marks the resource for recreation."]
    pub fn ddl(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ddl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `default_time_zone` after provisioning.\nThe default time zone for the database. The default time zone must be a valid name\nfrom the tz database. Default value is \"America/Los_angeles\"."]
    pub fn default_time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_time_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the database. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the database,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the database will fail.\nWhen the field is set to false, deleting the database is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_drop_protection` after provisioning.\nWhether drop protection is enabled for this database. Defaults to false.\nDrop protection is different from\nthe \"deletion_protection\" attribute in the following ways:\n(1) \"deletion_protection\" only protects the database from deletions in Terraform.\nwhereas setting “enableDropProtection” to true protects the database from deletions in all interfaces.\n(2) Setting \"enableDropProtection\" to true also prevents the deletion of the parent instance containing the database.\n\"deletion_protection\" attribute does not provide protection against the deletion of the parent instance."]
    pub fn enable_drop_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_drop_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the database on."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the database, which cannot be changed after the\ninstance is created. Values are of the form '[a-z][-_a-z0-9]*[a-z0-9]'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nAn explanation of the status of the database."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_retention_period` after provisioning.\nThe retention period for the database. The retention period must be between 1 hour\nand 7 days, and can be specified in days, hours, minutes, or seconds. For example,\nthe values 1d, 24h, 1440m, and 86400s are equivalent. Default value is 1h.\nIf this property is used, you must avoid adding new DDL statements to 'ddl' that\nupdate the database's version_retention_period."]
    pub fn version_retention_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_retention_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<SpannerDatabaseEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerDatabaseTimeoutsElRef {
        SpannerDatabaseTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SpannerDatabase {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SpannerDatabase {}
impl ToListMappable for SpannerDatabase {
    type O = ListRef<SpannerDatabaseRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SpannerDatabase_ {
    fn extract_resource_type(&self) -> String {
        "google_spanner_database".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSpannerDatabase {
    pub tf_id: String,
    #[doc = "The instance to create the database on."]
    pub instance: PrimField<String>,
    #[doc = "A unique identifier for the database, which cannot be changed after the\ninstance is created. Values are of the form '[a-z][-_a-z0-9]*[a-z0-9]'."]
    pub name: PrimField<String>,
}
impl BuildSpannerDatabase {
    pub fn build(self, stack: &mut Stack) -> SpannerDatabase {
        let out = SpannerDatabase(Rc::new(SpannerDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SpannerDatabaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                database_dialect: core::default::Default::default(),
                ddl: core::default::Default::default(),
                default_time_zone: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                enable_drop_protection: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                name: self.name,
                project: core::default::Default::default(),
                version_retention_period: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SpannerDatabaseRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerDatabaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SpannerDatabaseRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `database_dialect` after provisioning.\nThe dialect of the Cloud Spanner Database.\nIf it is not provided, \"GOOGLE_STANDARD_SQL\" will be used. Possible values: [\"GOOGLE_STANDARD_SQL\", \"POSTGRESQL\"]"]
    pub fn database_dialect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_dialect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ddl` after provisioning.\nAn optional list of DDL statements to run inside the database. Statements can create\ntables, indexes, etc.\n\nDuring creation these statements execute atomically with the creation of the database\nand if there is an error in any statement, the database is not created.\n\nTerraform does not perform drift detection on this field and assumes that the values\nrecorded in state are accurate. Limited updates to this field are supported, and\nnewly appended DDL statements can be executed in an update. However, modifications\nto prior statements will create a plan that marks the resource for recreation."]
    pub fn ddl(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ddl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `default_time_zone` after provisioning.\nThe default time zone for the database. The default time zone must be a valid name\nfrom the tz database. Default value is \"America/Los_angeles\"."]
    pub fn default_time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_time_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the database. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the database,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the database will fail.\nWhen the field is set to false, deleting the database is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_drop_protection` after provisioning.\nWhether drop protection is enabled for this database. Defaults to false.\nDrop protection is different from\nthe \"deletion_protection\" attribute in the following ways:\n(1) \"deletion_protection\" only protects the database from deletions in Terraform.\nwhereas setting “enableDropProtection” to true protects the database from deletions in all interfaces.\n(2) Setting \"enableDropProtection\" to true also prevents the deletion of the parent instance containing the database.\n\"deletion_protection\" attribute does not provide protection against the deletion of the parent instance."]
    pub fn enable_drop_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_drop_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the database on."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the database, which cannot be changed after the\ninstance is created. Values are of the form '[a-z][-_a-z0-9]*[a-z0-9]'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nAn explanation of the status of the database."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_retention_period` after provisioning.\nThe retention period for the database. The retention period must be between 1 hour\nand 7 days, and can be specified in days, hours, minutes, or seconds. For example,\nthe values 1d, 24h, 1440m, and 86400s are equivalent. Default value is 1h.\nIf this property is used, you must avoid adding new DDL statements to 'ddl' that\nupdate the database's version_retention_period."]
    pub fn version_retention_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_retention_period", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<SpannerDatabaseEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpannerDatabaseTimeoutsElRef {
        SpannerDatabaseTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerDatabaseEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_names: Option<ListField<PrimField<String>>>,
}
impl SpannerDatabaseEncryptionConfigEl {
    #[doc = "Set the field `kms_key_name`.\nFully qualified name of the KMS key to use to encrypt this database. This key must exist\nin the same location as the Spanner Database."]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_names`.\nFully qualified name of the KMS keys to use to encrypt this database. The keys must exist\nin the same locations as the Spanner Database."]
    pub fn set_kms_key_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.kms_key_names = Some(v.into());
        self
    }
}
impl ToListMappable for SpannerDatabaseEncryptionConfigEl {
    type O = BlockAssignable<SpannerDatabaseEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerDatabaseEncryptionConfigEl {}
impl BuildSpannerDatabaseEncryptionConfigEl {
    pub fn build(self) -> SpannerDatabaseEncryptionConfigEl {
        SpannerDatabaseEncryptionConfigEl {
            kms_key_name: core::default::Default::default(),
            kms_key_names: core::default::Default::default(),
        }
    }
}
pub struct SpannerDatabaseEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerDatabaseEncryptionConfigElRef {
    fn new(shared: StackShared, base: String) -> SpannerDatabaseEncryptionConfigElRef {
        SpannerDatabaseEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerDatabaseEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nFully qualified name of the KMS key to use to encrypt this database. This key must exist\nin the same location as the Spanner Database."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_key_names` after provisioning.\nFully qualified name of the KMS keys to use to encrypt this database. The keys must exist\nin the same locations as the Spanner Database."]
    pub fn kms_key_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_key_names", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SpannerDatabaseTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SpannerDatabaseTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for SpannerDatabaseTimeoutsEl {
    type O = BlockAssignable<SpannerDatabaseTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSpannerDatabaseTimeoutsEl {}
impl BuildSpannerDatabaseTimeoutsEl {
    pub fn build(self) -> SpannerDatabaseTimeoutsEl {
        SpannerDatabaseTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SpannerDatabaseTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SpannerDatabaseTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SpannerDatabaseTimeoutsElRef {
        SpannerDatabaseTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SpannerDatabaseTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct SpannerDatabaseDynamic {
    encryption_config: Option<DynamicBlock<SpannerDatabaseEncryptionConfigEl>>,
}
