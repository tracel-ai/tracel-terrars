use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDbInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    db_instance_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataDbInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDbInstanceData>,
}

#[derive(Clone)]
pub struct DataDbInstance(Rc<DataDbInstance_>);

impl DataDbInstance {
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

    #[doc = "Set the field `db_instance_identifier`.\n"]
    pub fn set_db_instance_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().db_instance_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocated_storage` after provisioning.\n"]
    pub fn allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_cert_identifier` after provisioning.\n"]
    pub fn ca_cert_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_cert_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_insights_mode` after provisioning.\n"]
    pub fn database_insights_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_insights_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_cluster_identifier` after provisioning.\n"]
    pub fn db_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_instance_arn` after provisioning.\n"]
    pub fn db_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_instance_class` after provisioning.\n"]
    pub fn db_instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_instance_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_instance_identifier` after provisioning.\n"]
    pub fn db_instance_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_instance_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_instance_port` after provisioning.\n"]
    pub fn db_instance_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_instance_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_parameter_groups` after provisioning.\n"]
    pub fn db_parameter_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.db_parameter_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_subnet_group` after provisioning.\n"]
    pub fn db_subnet_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_subnet_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cloudwatch_logs_exports` after provisioning.\n"]
    pub fn enabled_cloudwatch_logs_exports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enabled_cloudwatch_logs_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_user_secret` after provisioning.\n"]
    pub fn master_user_secret(&self) -> ListRef<DataDbInstanceMasterUserSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_user_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_allocated_storage` after provisioning.\n"]
    pub fn max_allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_interval` after provisioning.\n"]
    pub fn monitoring_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_role_arn` after provisioning.\n"]
    pub fn monitoring_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `option_group_memberships` after provisioning.\n"]
    pub fn option_group_memberships(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.option_group_memberships", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicate_source_db` after provisioning.\n"]
    pub fn replicate_source_db(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replicate_source_db", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_throughput` after provisioning.\n"]
    pub fn storage_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_groups` after provisioning.\n"]
    pub fn vpc_security_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_security_groups", self.extract_ref()))
    }
}

impl Referable for DataDbInstance {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataDbInstance { }

impl ToListMappable for DataDbInstance {
    type O = ListRef<DataDbInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDbInstance_ {
    fn extract_datasource_type(&self) -> String {
        "aws_db_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDbInstance {
    pub tf_id: String,
}

impl BuildDataDbInstance {
    pub fn build(self, stack: &mut Stack) -> DataDbInstance {
        let out = DataDbInstance(Rc::new(DataDbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDbInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                db_instance_identifier: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDbInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDbInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataDbInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allocated_storage` after provisioning.\n"]
    pub fn allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_minor_version_upgrade` after provisioning.\n"]
    pub fn auto_minor_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_minor_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_retention_period` after provisioning.\n"]
    pub fn backup_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.backup_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_cert_identifier` after provisioning.\n"]
    pub fn ca_cert_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ca_cert_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_insights_mode` after provisioning.\n"]
    pub fn database_insights_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_insights_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_cluster_identifier` after provisioning.\n"]
    pub fn db_cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_instance_arn` after provisioning.\n"]
    pub fn db_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_instance_class` after provisioning.\n"]
    pub fn db_instance_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_instance_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_instance_identifier` after provisioning.\n"]
    pub fn db_instance_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_instance_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_instance_port` after provisioning.\n"]
    pub fn db_instance_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_instance_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_parameter_groups` after provisioning.\n"]
    pub fn db_parameter_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.db_parameter_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_subnet_group` after provisioning.\n"]
    pub fn db_subnet_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.db_subnet_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cloudwatch_logs_exports` after provisioning.\n"]
    pub fn enabled_cloudwatch_logs_exports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enabled_cloudwatch_logs_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `license_model` after provisioning.\n"]
    pub fn license_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_user_secret` after provisioning.\n"]
    pub fn master_user_secret(&self) -> ListRef<DataDbInstanceMasterUserSecretElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_user_secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_allocated_storage` after provisioning.\n"]
    pub fn max_allocated_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_allocated_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_interval` after provisioning.\n"]
    pub fn monitoring_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring_interval", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring_role_arn` after provisioning.\n"]
    pub fn monitoring_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.monitoring_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_type` after provisioning.\n"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `option_group_memberships` after provisioning.\n"]
    pub fn option_group_memberships(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.option_group_memberships", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replicate_source_db` after provisioning.\n"]
    pub fn replicate_source_db(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.replicate_source_db", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_encrypted` after provisioning.\n"]
    pub fn storage_encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_throughput` after provisioning.\n"]
    pub fn storage_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_throughput", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timezone` after provisioning.\n"]
    pub fn timezone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_groups` after provisioning.\n"]
    pub fn vpc_security_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_security_groups", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDbInstanceMasterUserSecretEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_status: Option<PrimField<String>>,
}

impl DataDbInstanceMasterUserSecretEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `secret_status`.\n"]
    pub fn set_secret_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_status = Some(v.into());
        self
    }
}

impl ToListMappable for DataDbInstanceMasterUserSecretEl {
    type O = BlockAssignable<DataDbInstanceMasterUserSecretEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDbInstanceMasterUserSecretEl {}

impl BuildDataDbInstanceMasterUserSecretEl {
    pub fn build(self) -> DataDbInstanceMasterUserSecretEl {
        DataDbInstanceMasterUserSecretEl {
            kms_key_id: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
            secret_status: core::default::Default::default(),
        }
    }
}

pub struct DataDbInstanceMasterUserSecretElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDbInstanceMasterUserSecretElRef {
    fn new(shared: StackShared, base: String) -> DataDbInstanceMasterUserSecretElRef {
        DataDbInstanceMasterUserSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDbInstanceMasterUserSecretElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_status` after provisioning.\n"]
    pub fn secret_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_status", self.base))
    }
}
