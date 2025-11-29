use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LightsailDatabaseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apply_immediately: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_retention_enabled: Option<PrimField<bool>>,
    blueprint_id: PrimField<String>,
    bundle_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_snapshot_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    master_database_name: PrimField<String>,
    master_password: PrimField<String>,
    master_username: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_backup_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_maintenance_window: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publicly_accessible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    relational_database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_final_snapshot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}

struct LightsailDatabase_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LightsailDatabaseData>,
}

#[derive(Clone)]
pub struct LightsailDatabase(Rc<LightsailDatabase_>);

impl LightsailDatabase {
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

    #[doc = "Set the field `apply_immediately`.\n"]
    pub fn set_apply_immediately(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().apply_immediately = Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = Some(v.into());
        self
    }

    #[doc = "Set the field `backup_retention_enabled`.\n"]
    pub fn set_backup_retention_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().backup_retention_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `final_snapshot_name`.\n"]
    pub fn set_final_snapshot_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().final_snapshot_name = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_backup_window`.\n"]
    pub fn set_preferred_backup_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().preferred_backup_window = Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_maintenance_window`.\n"]
    pub fn set_preferred_maintenance_window(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().preferred_maintenance_window = Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_accessible`.\n"]
    pub fn set_publicly_accessible(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publicly_accessible = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `skip_final_snapshot`.\n"]
    pub fn set_skip_final_snapshot(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_final_snapshot = Some(v.into());
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

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_enabled` after provisioning.\n"]
    pub fn backup_retention_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blueprint_id` after provisioning.\n"]
    pub fn blueprint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blueprint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_certificate_identifier` after provisioning.\n"]
    pub fn ca_certificate_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_name` after provisioning.\n"]
    pub fn final_snapshot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_database_name` after provisioning.\n"]
    pub fn master_database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_endpoint_address` after provisioning.\n"]
    pub fn master_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_endpoint_port` after provisioning.\n"]
    pub fn master_endpoint_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_endpoint_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ram_size` after provisioning.\n"]
    pub fn ram_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ram_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relational_database_name` after provisioning.\n"]
    pub fn relational_database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.relational_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_availability_zone` after provisioning.\n"]
    pub fn secondary_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secondary_availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `support_code` after provisioning.\n"]
    pub fn support_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.support_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl Referable for LightsailDatabase {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LightsailDatabase { }

impl ToListMappable for LightsailDatabase {
    type O = ListRef<LightsailDatabaseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LightsailDatabase_ {
    fn extract_resource_type(&self) -> String {
        "aws_lightsail_database".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLightsailDatabase {
    pub tf_id: String,
    #[doc = ""]
    pub blueprint_id: PrimField<String>,
    #[doc = ""]
    pub bundle_id: PrimField<String>,
    #[doc = ""]
    pub master_database_name: PrimField<String>,
    #[doc = ""]
    pub master_password: PrimField<String>,
    #[doc = ""]
    pub master_username: PrimField<String>,
    #[doc = ""]
    pub relational_database_name: PrimField<String>,
}

impl BuildLightsailDatabase {
    pub fn build(self, stack: &mut Stack) -> LightsailDatabase {
        let out = LightsailDatabase(Rc::new(LightsailDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LightsailDatabaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                apply_immediately: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                backup_retention_enabled: core::default::Default::default(),
                blueprint_id: self.blueprint_id,
                bundle_id: self.bundle_id,
                final_snapshot_name: core::default::Default::default(),
                id: core::default::Default::default(),
                master_database_name: self.master_database_name,
                master_password: self.master_password,
                master_username: self.master_username,
                preferred_backup_window: core::default::Default::default(),
                preferred_maintenance_window: core::default::Default::default(),
                publicly_accessible: core::default::Default::default(),
                region: core::default::Default::default(),
                relational_database_name: self.relational_database_name,
                skip_final_snapshot: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LightsailDatabaseRef {
    shared: StackShared,
    base: String,
}

impl Ref for LightsailDatabaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl LightsailDatabaseRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `apply_immediately` after provisioning.\n"]
    pub fn apply_immediately(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.apply_immediately", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_enabled` after provisioning.\n"]
    pub fn backup_retention_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blueprint_id` after provisioning.\n"]
    pub fn blueprint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.blueprint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_certificate_identifier` after provisioning.\n"]
    pub fn ca_certificate_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\n"]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `final_snapshot_name` after provisioning.\n"]
    pub fn final_snapshot_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.final_snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_database_name` after provisioning.\n"]
    pub fn master_database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_endpoint_address` after provisioning.\n"]
    pub fn master_endpoint_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_endpoint_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_endpoint_port` after provisioning.\n"]
    pub fn master_endpoint_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_endpoint_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_password` after provisioning.\n"]
    pub fn master_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_backup_window` after provisioning.\n"]
    pub fn preferred_backup_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_backup_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preferred_maintenance_window` after provisioning.\n"]
    pub fn preferred_maintenance_window(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preferred_maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publicly_accessible` after provisioning.\n"]
    pub fn publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publicly_accessible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ram_size` after provisioning.\n"]
    pub fn ram_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ram_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `relational_database_name` after provisioning.\n"]
    pub fn relational_database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.relational_database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_availability_zone` after provisioning.\n"]
    pub fn secondary_availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secondary_availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_final_snapshot` after provisioning.\n"]
    pub fn skip_final_snapshot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.skip_final_snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `support_code` after provisioning.\n"]
    pub fn support_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.support_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}
