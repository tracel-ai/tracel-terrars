use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRedshiftClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataRedshiftCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRedshiftClusterData>,
}

#[derive(Clone)]
pub struct DataRedshiftCluster(Rc<DataRedshiftCluster_>);

impl DataRedshiftCluster {
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

    #[doc = "Get a reference to the value of field `allow_version_upgrade` after provisioning.\n"]
    pub fn allow_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aqua_configuration_status` after provisioning.\n"]
    pub fn aqua_configuration_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aqua_configuration_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_snapshot_retention_period` after provisioning.\n"]
    pub fn automated_snapshot_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.automated_snapshot_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_relocation_enabled` after provisioning.\n"]
    pub fn availability_zone_relocation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_relocation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_namespace_arn` after provisioning.\n"]
    pub fn cluster_namespace_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_namespace_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_nodes` after provisioning.\n"]
    pub fn cluster_nodes(&self) -> ListRef<DataRedshiftClusterClusterNodesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_parameter_group_name` after provisioning.\n"]
    pub fn cluster_parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_public_key` after provisioning.\n"]
    pub fn cluster_public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_revision_number` after provisioning.\n"]
    pub fn cluster_revision_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_revision_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_subnet_group_name` after provisioning.\n"]
    pub fn cluster_subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_version` after provisioning.\n"]
    pub fn cluster_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_iam_role_arn` after provisioning.\n"]
    pub fn default_iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_ip` after provisioning.\n"]
    pub fn elastic_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elastic_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\n"]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_vpc_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.iam_roles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_destination_type` after provisioning.\n"]
    pub fn log_destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_destination_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_exports` after provisioning.\n"]
    pub fn log_exports(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.log_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_track_name` after provisioning.\n"]
    pub fn maintenance_track_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_track_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_snapshot_retention_period` after provisioning.\n"]
    pub fn manual_snapshot_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.manual_snapshot_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_nodes` after provisioning.\n"]
    pub fn number_of_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }
}

impl Referable for DataRedshiftCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataRedshiftCluster { }

impl ToListMappable for DataRedshiftCluster {
    type O = ListRef<DataRedshiftClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRedshiftCluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_redshift_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRedshiftCluster {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_identifier: PrimField<String>,
}

impl BuildDataRedshiftCluster {
    pub fn build(self, stack: &mut Stack) -> DataRedshiftCluster {
        let out = DataRedshiftCluster(Rc::new(DataRedshiftCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRedshiftClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_identifier: self.cluster_identifier,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRedshiftClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRedshiftClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataRedshiftClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `allow_version_upgrade` after provisioning.\n"]
    pub fn allow_version_upgrade(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_version_upgrade", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aqua_configuration_status` after provisioning.\n"]
    pub fn aqua_configuration_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aqua_configuration_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automated_snapshot_retention_period` after provisioning.\n"]
    pub fn automated_snapshot_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.automated_snapshot_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_relocation_enabled` after provisioning.\n"]
    pub fn availability_zone_relocation_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_relocation_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_namespace_arn` after provisioning.\n"]
    pub fn cluster_namespace_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_namespace_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_nodes` after provisioning.\n"]
    pub fn cluster_nodes(&self) -> ListRef<DataRedshiftClusterClusterNodesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_parameter_group_name` after provisioning.\n"]
    pub fn cluster_parameter_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_parameter_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_public_key` after provisioning.\n"]
    pub fn cluster_public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_public_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_revision_number` after provisioning.\n"]
    pub fn cluster_revision_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_revision_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_subnet_group_name` after provisioning.\n"]
    pub fn cluster_subnet_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_subnet_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_version` after provisioning.\n"]
    pub fn cluster_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_iam_role_arn` after provisioning.\n"]
    pub fn default_iam_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_ip` after provisioning.\n"]
    pub fn elastic_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elastic_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\n"]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enhanced_vpc_routing` after provisioning.\n"]
    pub fn enhanced_vpc_routing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enhanced_vpc_routing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_roles` after provisioning.\n"]
    pub fn iam_roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.iam_roles", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_destination_type` after provisioning.\n"]
    pub fn log_destination_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_destination_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_exports` after provisioning.\n"]
    pub fn log_exports(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.log_exports", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_track_name` after provisioning.\n"]
    pub fn maintenance_track_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maintenance_track_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_snapshot_retention_period` after provisioning.\n"]
    pub fn manual_snapshot_retention_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.manual_snapshot_retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_username` after provisioning.\n"]
    pub fn master_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_username", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_az` after provisioning.\n"]
    pub fn multi_az(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `number_of_nodes` after provisioning.\n"]
    pub fn number_of_nodes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_nodes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRedshiftClusterClusterNodesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ip_address: Option<PrimField<String>>,
}

impl DataRedshiftClusterClusterNodesEl {
    #[doc = "Set the field `node_role`.\n"]
    pub fn set_node_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_role = Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_ip_address = Some(v.into());
        self
    }

    #[doc = "Set the field `public_ip_address`.\n"]
    pub fn set_public_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_ip_address = Some(v.into());
        self
    }
}

impl ToListMappable for DataRedshiftClusterClusterNodesEl {
    type O = BlockAssignable<DataRedshiftClusterClusterNodesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRedshiftClusterClusterNodesEl {}

impl BuildDataRedshiftClusterClusterNodesEl {
    pub fn build(self) -> DataRedshiftClusterClusterNodesEl {
        DataRedshiftClusterClusterNodesEl {
            node_role: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            public_ip_address: core::default::Default::default(),
        }
    }
}

pub struct DataRedshiftClusterClusterNodesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRedshiftClusterClusterNodesElRef {
    fn new(shared: StackShared, base: String) -> DataRedshiftClusterClusterNodesElRef {
        DataRedshiftClusterClusterNodesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRedshiftClusterClusterNodesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_role` after provisioning.\n"]
    pub fn node_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_role", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `public_ip_address` after provisioning.\n"]
    pub fn public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ip_address", self.base))
    }
}
