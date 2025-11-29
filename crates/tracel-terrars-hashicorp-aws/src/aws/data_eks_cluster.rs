use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEksClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataEksCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEksClusterData>,
}

#[derive(Clone)]
pub struct DataEksCluster(Rc<DataEksCluster_>);

impl DataEksCluster {
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

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<DataEksClusterAccessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> ListRef<DataEksClusterCertificateAuthorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> ListRef<DataEksClusterComputeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.compute_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cluster_log_types` after provisioning.\n"]
    pub fn enabled_cluster_log_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.enabled_cluster_log_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> ListRef<DataEksClusterIdentityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> ListRef<DataEksClusterKubernetesNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> ListRef<DataEksClusterOutpostConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_network_config` after provisioning.\n"]
    pub fn remote_network_config(&self) -> ListRef<DataEksClusterRemoteNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<DataEksClusterStorageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> ListRef<DataEksClusterUpgradePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<DataEksClusterVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_shift_config` after provisioning.\n"]
    pub fn zonal_shift_config(&self) -> ListRef<DataEksClusterZonalShiftConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zonal_shift_config", self.extract_ref()))
    }
}

impl Referable for DataEksCluster {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEksCluster { }

impl ToListMappable for DataEksCluster {
    type O = ListRef<DataEksClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEksCluster_ {
    fn extract_datasource_type(&self) -> String {
        "aws_eks_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEksCluster {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataEksCluster {
    pub fn build(self, stack: &mut Stack) -> DataEksCluster {
        let out = DataEksCluster(Rc::new(DataEksCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEksClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEksClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEksClusterRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<DataEksClusterAccessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> ListRef<DataEksClusterCertificateAuthorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> ListRef<DataEksClusterComputeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.compute_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_cluster_log_types` after provisioning.\n"]
    pub fn enabled_cluster_log_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.enabled_cluster_log_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> ListRef<DataEksClusterIdentityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> ListRef<DataEksClusterKubernetesNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> ListRef<DataEksClusterOutpostConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_network_config` after provisioning.\n"]
    pub fn remote_network_config(&self) -> ListRef<DataEksClusterRemoteNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<DataEksClusterStorageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> ListRef<DataEksClusterUpgradePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<DataEksClusterVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_shift_config` after provisioning.\n"]
    pub fn zonal_shift_config(&self) -> ListRef<DataEksClusterZonalShiftConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zonal_shift_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bootstrap_cluster_creator_admin_permissions: Option<PrimField<bool>>,
}

impl DataEksClusterAccessConfigEl {
    #[doc = "Set the field `authentication_mode`.\n"]
    pub fn set_authentication_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authentication_mode = Some(v.into());
        self
    }

    #[doc = "Set the field `bootstrap_cluster_creator_admin_permissions`.\n"]
    pub fn set_bootstrap_cluster_creator_admin_permissions(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.bootstrap_cluster_creator_admin_permissions = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterAccessConfigEl {
    type O = BlockAssignable<DataEksClusterAccessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterAccessConfigEl {}

impl BuildDataEksClusterAccessConfigEl {
    pub fn build(self) -> DataEksClusterAccessConfigEl {
        DataEksClusterAccessConfigEl {
            authentication_mode: core::default::Default::default(),
            bootstrap_cluster_creator_admin_permissions: core::default::Default::default(),
        }
    }
}

pub struct DataEksClusterAccessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterAccessConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterAccessConfigElRef {
        DataEksClusterAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_mode` after provisioning.\n"]
    pub fn authentication_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `bootstrap_cluster_creator_admin_permissions` after provisioning.\n"]
    pub fn bootstrap_cluster_creator_admin_permissions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_cluster_creator_admin_permissions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterCertificateAuthorityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
}

impl DataEksClusterCertificateAuthorityEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterCertificateAuthorityEl {
    type O = BlockAssignable<DataEksClusterCertificateAuthorityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterCertificateAuthorityEl {}

impl BuildDataEksClusterCertificateAuthorityEl {
    pub fn build(self) -> DataEksClusterCertificateAuthorityEl {
        DataEksClusterCertificateAuthorityEl { data: core::default::Default::default() }
    }
}

pub struct DataEksClusterCertificateAuthorityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterCertificateAuthorityElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterCertificateAuthorityElRef {
        DataEksClusterCertificateAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterCertificateAuthorityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterComputeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pools: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_role_arn: Option<PrimField<String>>,
}

impl DataEksClusterComputeConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `node_pools`.\n"]
    pub fn set_node_pools(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.node_pools = Some(v.into());
        self
    }

    #[doc = "Set the field `node_role_arn`.\n"]
    pub fn set_node_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_role_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterComputeConfigEl {
    type O = BlockAssignable<DataEksClusterComputeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterComputeConfigEl {}

impl BuildDataEksClusterComputeConfigEl {
    pub fn build(self) -> DataEksClusterComputeConfigEl {
        DataEksClusterComputeConfigEl {
            enabled: core::default::Default::default(),
            node_pools: core::default::Default::default(),
            node_role_arn: core::default::Default::default(),
        }
    }
}

pub struct DataEksClusterComputeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterComputeConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterComputeConfigElRef {
        DataEksClusterComputeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterComputeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `node_pools` after provisioning.\n"]
    pub fn node_pools(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.node_pools", self.base))
    }

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterIdentityElOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}

impl DataEksClusterIdentityElOidcEl {
    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterIdentityElOidcEl {
    type O = BlockAssignable<DataEksClusterIdentityElOidcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterIdentityElOidcEl {}

impl BuildDataEksClusterIdentityElOidcEl {
    pub fn build(self) -> DataEksClusterIdentityElOidcEl {
        DataEksClusterIdentityElOidcEl { issuer: core::default::Default::default() }
    }
}

pub struct DataEksClusterIdentityElOidcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterIdentityElOidcElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterIdentityElOidcElRef {
        DataEksClusterIdentityElOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterIdentityElOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterIdentityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc: Option<ListField<DataEksClusterIdentityElOidcEl>>,
}

impl DataEksClusterIdentityEl {
    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl Into<ListField<DataEksClusterIdentityElOidcEl>>) -> Self {
        self.oidc = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterIdentityEl {
    type O = BlockAssignable<DataEksClusterIdentityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterIdentityEl {}

impl BuildDataEksClusterIdentityEl {
    pub fn build(self) -> DataEksClusterIdentityEl {
        DataEksClusterIdentityEl { oidc: core::default::Default::default() }
    }
}

pub struct DataEksClusterIdentityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterIdentityElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterIdentityElRef {
        DataEksClusterIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ListRef<DataEksClusterIdentityElOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
    type O = BlockAssignable<DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {}

impl BuildDataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
    pub fn build(self) -> DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
        DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl { enabled: core::default::Default::default() }
    }
}

pub struct DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
        DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterKubernetesNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    elastic_load_balancing: Option<ListField<DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_ipv4_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_ipv6_cidr: Option<PrimField<String>>,
}

impl DataEksClusterKubernetesNetworkConfigEl {
    #[doc = "Set the field `elastic_load_balancing`.\n"]
    pub fn set_elastic_load_balancing(
        mut self,
        v: impl Into<ListField<DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl>>,
    ) -> Self {
        self.elastic_load_balancing = Some(v.into());
        self
    }

    #[doc = "Set the field `ip_family`.\n"]
    pub fn set_ip_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_family = Some(v.into());
        self
    }

    #[doc = "Set the field `service_ipv4_cidr`.\n"]
    pub fn set_service_ipv4_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_ipv4_cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `service_ipv6_cidr`.\n"]
    pub fn set_service_ipv6_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_ipv6_cidr = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterKubernetesNetworkConfigEl {
    type O = BlockAssignable<DataEksClusterKubernetesNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterKubernetesNetworkConfigEl {}

impl BuildDataEksClusterKubernetesNetworkConfigEl {
    pub fn build(self) -> DataEksClusterKubernetesNetworkConfigEl {
        DataEksClusterKubernetesNetworkConfigEl {
            elastic_load_balancing: core::default::Default::default(),
            ip_family: core::default::Default::default(),
            service_ipv4_cidr: core::default::Default::default(),
            service_ipv6_cidr: core::default::Default::default(),
        }
    }
}

pub struct DataEksClusterKubernetesNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterKubernetesNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterKubernetesNetworkConfigElRef {
        DataEksClusterKubernetesNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterKubernetesNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `elastic_load_balancing` after provisioning.\n"]
    pub fn elastic_load_balancing(&self) -> ListRef<DataEksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.elastic_load_balancing", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_family` after provisioning.\n"]
    pub fn ip_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_family", self.base))
    }

    #[doc = "Get a reference to the value of field `service_ipv4_cidr` after provisioning.\n"]
    pub fn service_ipv4_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_ipv4_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `service_ipv6_cidr` after provisioning.\n"]
    pub fn service_ipv6_cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_ipv6_cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterOutpostConfigElControlPlanePlacementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<PrimField<String>>,
}

impl DataEksClusterOutpostConfigElControlPlanePlacementEl {
    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterOutpostConfigElControlPlanePlacementEl {
    type O = BlockAssignable<DataEksClusterOutpostConfigElControlPlanePlacementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterOutpostConfigElControlPlanePlacementEl {}

impl BuildDataEksClusterOutpostConfigElControlPlanePlacementEl {
    pub fn build(self) -> DataEksClusterOutpostConfigElControlPlanePlacementEl {
        DataEksClusterOutpostConfigElControlPlanePlacementEl { group_name: core::default::Default::default() }
    }
}

pub struct DataEksClusterOutpostConfigElControlPlanePlacementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterOutpostConfigElControlPlanePlacementElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterOutpostConfigElControlPlanePlacementElRef {
        DataEksClusterOutpostConfigElControlPlanePlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterOutpostConfigElControlPlanePlacementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterOutpostConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_placement: Option<ListField<DataEksClusterOutpostConfigElControlPlanePlacementEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outpost_arns: Option<SetField<PrimField<String>>>,
}

impl DataEksClusterOutpostConfigEl {
    #[doc = "Set the field `control_plane_instance_type`.\n"]
    pub fn set_control_plane_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.control_plane_instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `control_plane_placement`.\n"]
    pub fn set_control_plane_placement(
        mut self,
        v: impl Into<ListField<DataEksClusterOutpostConfigElControlPlanePlacementEl>>,
    ) -> Self {
        self.control_plane_placement = Some(v.into());
        self
    }

    #[doc = "Set the field `outpost_arns`.\n"]
    pub fn set_outpost_arns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.outpost_arns = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterOutpostConfigEl {
    type O = BlockAssignable<DataEksClusterOutpostConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterOutpostConfigEl {}

impl BuildDataEksClusterOutpostConfigEl {
    pub fn build(self) -> DataEksClusterOutpostConfigEl {
        DataEksClusterOutpostConfigEl {
            control_plane_instance_type: core::default::Default::default(),
            control_plane_placement: core::default::Default::default(),
            outpost_arns: core::default::Default::default(),
        }
    }
}

pub struct DataEksClusterOutpostConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterOutpostConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterOutpostConfigElRef {
        DataEksClusterOutpostConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterOutpostConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `control_plane_instance_type` after provisioning.\n"]
    pub fn control_plane_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_plane_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_placement` after provisioning.\n"]
    pub fn control_plane_placement(&self) -> ListRef<DataEksClusterOutpostConfigElControlPlanePlacementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.control_plane_placement", self.base))
    }

    #[doc = "Get a reference to the value of field `outpost_arns` after provisioning.\n"]
    pub fn outpost_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.outpost_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidrs: Option<SetField<PrimField<String>>>,
}

impl DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cidrs = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
    type O = BlockAssignable<DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {}

impl BuildDataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
    pub fn build(self) -> DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
        DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl { cidrs: core::default::Default::default() }
    }
}

pub struct DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
        DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterRemoteNetworkConfigElRemotePodNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidrs: Option<SetField<PrimField<String>>>,
}

impl DataEksClusterRemoteNetworkConfigElRemotePodNetworksEl {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cidrs = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterRemoteNetworkConfigElRemotePodNetworksEl {
    type O = BlockAssignable<DataEksClusterRemoteNetworkConfigElRemotePodNetworksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterRemoteNetworkConfigElRemotePodNetworksEl {}

impl BuildDataEksClusterRemoteNetworkConfigElRemotePodNetworksEl {
    pub fn build(self) -> DataEksClusterRemoteNetworkConfigElRemotePodNetworksEl {
        DataEksClusterRemoteNetworkConfigElRemotePodNetworksEl { cidrs: core::default::Default::default() }
    }
}

pub struct DataEksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
        DataEksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterRemoteNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_node_networks: Option<ListField<DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_pod_networks: Option<ListField<DataEksClusterRemoteNetworkConfigElRemotePodNetworksEl>>,
}

impl DataEksClusterRemoteNetworkConfigEl {
    #[doc = "Set the field `remote_node_networks`.\n"]
    pub fn set_remote_node_networks(
        mut self,
        v: impl Into<ListField<DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl>>,
    ) -> Self {
        self.remote_node_networks = Some(v.into());
        self
    }

    #[doc = "Set the field `remote_pod_networks`.\n"]
    pub fn set_remote_pod_networks(
        mut self,
        v: impl Into<ListField<DataEksClusterRemoteNetworkConfigElRemotePodNetworksEl>>,
    ) -> Self {
        self.remote_pod_networks = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterRemoteNetworkConfigEl {
    type O = BlockAssignable<DataEksClusterRemoteNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterRemoteNetworkConfigEl {}

impl BuildDataEksClusterRemoteNetworkConfigEl {
    pub fn build(self) -> DataEksClusterRemoteNetworkConfigEl {
        DataEksClusterRemoteNetworkConfigEl {
            remote_node_networks: core::default::Default::default(),
            remote_pod_networks: core::default::Default::default(),
        }
    }
}

pub struct DataEksClusterRemoteNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterRemoteNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterRemoteNetworkConfigElRef {
        DataEksClusterRemoteNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterRemoteNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `remote_node_networks` after provisioning.\n"]
    pub fn remote_node_networks(&self) -> ListRef<DataEksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_node_networks", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_pod_networks` after provisioning.\n"]
    pub fn remote_pod_networks(&self) -> ListRef<DataEksClusterRemoteNetworkConfigElRemotePodNetworksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_pod_networks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterStorageConfigElBlockStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataEksClusterStorageConfigElBlockStorageEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterStorageConfigElBlockStorageEl {
    type O = BlockAssignable<DataEksClusterStorageConfigElBlockStorageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterStorageConfigElBlockStorageEl {}

impl BuildDataEksClusterStorageConfigElBlockStorageEl {
    pub fn build(self) -> DataEksClusterStorageConfigElBlockStorageEl {
        DataEksClusterStorageConfigElBlockStorageEl { enabled: core::default::Default::default() }
    }
}

pub struct DataEksClusterStorageConfigElBlockStorageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterStorageConfigElBlockStorageElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterStorageConfigElBlockStorageElRef {
        DataEksClusterStorageConfigElBlockStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterStorageConfigElBlockStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterStorageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_storage: Option<ListField<DataEksClusterStorageConfigElBlockStorageEl>>,
}

impl DataEksClusterStorageConfigEl {
    #[doc = "Set the field `block_storage`.\n"]
    pub fn set_block_storage(mut self, v: impl Into<ListField<DataEksClusterStorageConfigElBlockStorageEl>>) -> Self {
        self.block_storage = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterStorageConfigEl {
    type O = BlockAssignable<DataEksClusterStorageConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterStorageConfigEl {}

impl BuildDataEksClusterStorageConfigEl {
    pub fn build(self) -> DataEksClusterStorageConfigEl {
        DataEksClusterStorageConfigEl { block_storage: core::default::Default::default() }
    }
}

pub struct DataEksClusterStorageConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterStorageConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterStorageConfigElRef {
        DataEksClusterStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterStorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_storage` after provisioning.\n"]
    pub fn block_storage(&self) -> ListRef<DataEksClusterStorageConfigElBlockStorageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.block_storage", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterUpgradePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    support_type: Option<PrimField<String>>,
}

impl DataEksClusterUpgradePolicyEl {
    #[doc = "Set the field `support_type`.\n"]
    pub fn set_support_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.support_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterUpgradePolicyEl {
    type O = BlockAssignable<DataEksClusterUpgradePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterUpgradePolicyEl {}

impl BuildDataEksClusterUpgradePolicyEl {
    pub fn build(self) -> DataEksClusterUpgradePolicyEl {
        DataEksClusterUpgradePolicyEl { support_type: core::default::Default::default() }
    }
}

pub struct DataEksClusterUpgradePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterUpgradePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterUpgradePolicyElRef {
        DataEksClusterUpgradePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterUpgradePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `support_type` after provisioning.\n"]
    pub fn support_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.support_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_security_group_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_private_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_public_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_access_cidrs: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl DataEksClusterVpcConfigEl {
    #[doc = "Set the field `cluster_security_group_id`.\n"]
    pub fn set_cluster_security_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cluster_security_group_id = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_private_access`.\n"]
    pub fn set_endpoint_private_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.endpoint_private_access = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_public_access`.\n"]
    pub fn set_endpoint_public_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.endpoint_public_access = Some(v.into());
        self
    }

    #[doc = "Set the field `public_access_cidrs`.\n"]
    pub fn set_public_access_cidrs(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.public_access_cidrs = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterVpcConfigEl {
    type O = BlockAssignable<DataEksClusterVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterVpcConfigEl {}

impl BuildDataEksClusterVpcConfigEl {
    pub fn build(self) -> DataEksClusterVpcConfigEl {
        DataEksClusterVpcConfigEl {
            cluster_security_group_id: core::default::Default::default(),
            endpoint_private_access: core::default::Default::default(),
            endpoint_public_access: core::default::Default::default(),
            public_access_cidrs: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct DataEksClusterVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterVpcConfigElRef {
        DataEksClusterVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterVpcConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_security_group_id` after provisioning.\n"]
    pub fn cluster_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_security_group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_private_access` after provisioning.\n"]
    pub fn endpoint_private_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_private_access", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_public_access` after provisioning.\n"]
    pub fn endpoint_public_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_public_access", self.base))
    }

    #[doc = "Get a reference to the value of field `public_access_cidrs` after provisioning.\n"]
    pub fn public_access_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.public_access_cidrs", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksClusterZonalShiftConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataEksClusterZonalShiftConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksClusterZonalShiftConfigEl {
    type O = BlockAssignable<DataEksClusterZonalShiftConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksClusterZonalShiftConfigEl {}

impl BuildDataEksClusterZonalShiftConfigEl {
    pub fn build(self) -> DataEksClusterZonalShiftConfigEl {
        DataEksClusterZonalShiftConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct DataEksClusterZonalShiftConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksClusterZonalShiftConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksClusterZonalShiftConfigElRef {
        DataEksClusterZonalShiftConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksClusterZonalShiftConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
