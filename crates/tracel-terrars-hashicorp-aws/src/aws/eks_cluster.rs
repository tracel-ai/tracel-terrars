use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EksClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bootstrap_self_managed_addons: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_cluster_log_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_update_version: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_config: Option<Vec<EksClusterAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_config: Option<Vec<EksClusterComputeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config: Option<Vec<EksClusterEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kubernetes_network_config: Option<Vec<EksClusterKubernetesNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outpost_config: Option<Vec<EksClusterOutpostConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_network_config: Option<Vec<EksClusterRemoteNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_config: Option<Vec<EksClusterStorageConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EksClusterTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upgrade_policy: Option<Vec<EksClusterUpgradePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_config: Option<Vec<EksClusterVpcConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zonal_shift_config: Option<Vec<EksClusterZonalShiftConfigEl>>,
    dynamic: EksClusterDynamic,
}

struct EksCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EksClusterData>,
}

#[derive(Clone)]
pub struct EksCluster(Rc<EksCluster_>);

impl EksCluster {
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

    #[doc = "Set the field `bootstrap_self_managed_addons`.\n"]
    pub fn set_bootstrap_self_managed_addons(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bootstrap_self_managed_addons = Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled_cluster_log_types`.\n"]
    pub fn set_enabled_cluster_log_types(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().enabled_cluster_log_types = Some(v.into());
        self
    }

    #[doc = "Set the field `force_update_version`.\n"]
    pub fn set_force_update_version(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_update_version = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }

    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(self, v: impl Into<BlockAssignable<EksClusterAccessConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compute_config`.\n"]
    pub fn set_compute_config(self, v: impl Into<BlockAssignable<EksClusterComputeConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(self, v: impl Into<BlockAssignable<EksClusterEncryptionConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kubernetes_network_config`.\n"]
    pub fn set_kubernetes_network_config(
        self,
        v: impl Into<BlockAssignable<EksClusterKubernetesNetworkConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kubernetes_network_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kubernetes_network_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outpost_config`.\n"]
    pub fn set_outpost_config(self, v: impl Into<BlockAssignable<EksClusterOutpostConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outpost_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outpost_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_network_config`.\n"]
    pub fn set_remote_network_config(self, v: impl Into<BlockAssignable<EksClusterRemoteNetworkConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_network_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_network_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_config`.\n"]
    pub fn set_storage_config(self, v: impl Into<BlockAssignable<EksClusterStorageConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EksClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade_policy`.\n"]
    pub fn set_upgrade_policy(self, v: impl Into<BlockAssignable<EksClusterUpgradePolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upgrade_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upgrade_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(self, v: impl Into<BlockAssignable<EksClusterVpcConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zonal_shift_config`.\n"]
    pub fn set_zonal_shift_config(self, v: impl Into<BlockAssignable<EksClusterZonalShiftConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().zonal_shift_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.zonal_shift_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_self_managed_addons` after provisioning.\n"]
    pub fn bootstrap_self_managed_addons(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_self_managed_addons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> ListRef<EksClusterCertificateAuthorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `force_update_version` after provisioning.\n"]
    pub fn force_update_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_update_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> ListRef<EksClusterIdentityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<EksClusterAccessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> ListRef<EksClusterComputeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.compute_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<EksClusterEncryptionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> ListRef<EksClusterKubernetesNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> ListRef<EksClusterOutpostConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_network_config` after provisioning.\n"]
    pub fn remote_network_config(&self) -> ListRef<EksClusterRemoteNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<EksClusterStorageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksClusterTimeoutsElRef {
        EksClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> ListRef<EksClusterUpgradePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<EksClusterVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_shift_config` after provisioning.\n"]
    pub fn zonal_shift_config(&self) -> ListRef<EksClusterZonalShiftConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zonal_shift_config", self.extract_ref()))
    }
}

impl Referable for EksCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EksCluster { }

impl ToListMappable for EksCluster {
    type O = ListRef<EksClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EksCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_eks_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEksCluster {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildEksCluster {
    pub fn build(self, stack: &mut Stack) -> EksCluster {
        let out = EksCluster(Rc::new(EksCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EksClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bootstrap_self_managed_addons: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                enabled_cluster_log_types: core::default::Default::default(),
                force_update_version: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: core::default::Default::default(),
                access_config: core::default::Default::default(),
                compute_config: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                kubernetes_network_config: core::default::Default::default(),
                outpost_config: core::default::Default::default(),
                remote_network_config: core::default::Default::default(),
                storage_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                upgrade_policy: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                zonal_shift_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EksClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EksClusterRef {
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

    #[doc = "Get a reference to the value of field `bootstrap_self_managed_addons` after provisioning.\n"]
    pub fn bootstrap_self_managed_addons(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_self_managed_addons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority` after provisioning.\n"]
    pub fn certificate_authority(&self) -> ListRef<EksClusterCertificateAuthorityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.certificate_authority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `force_update_version` after provisioning.\n"]
    pub fn force_update_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_update_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> ListRef<EksClusterIdentityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<EksClusterAccessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_config` after provisioning.\n"]
    pub fn compute_config(&self) -> ListRef<EksClusterComputeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.compute_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> ListRef<EksClusterEncryptionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kubernetes_network_config` after provisioning.\n"]
    pub fn kubernetes_network_config(&self) -> ListRef<EksClusterKubernetesNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubernetes_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_config` after provisioning.\n"]
    pub fn outpost_config(&self) -> ListRef<EksClusterOutpostConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.outpost_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_network_config` after provisioning.\n"]
    pub fn remote_network_config(&self) -> ListRef<EksClusterRemoteNetworkConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_config` after provisioning.\n"]
    pub fn storage_config(&self) -> ListRef<EksClusterStorageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksClusterTimeoutsElRef {
        EksClusterTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `upgrade_policy` after provisioning.\n"]
    pub fn upgrade_policy(&self) -> ListRef<EksClusterUpgradePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upgrade_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> ListRef<EksClusterVpcConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_shift_config` after provisioning.\n"]
    pub fn zonal_shift_config(&self) -> ListRef<EksClusterZonalShiftConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.zonal_shift_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EksClusterCertificateAuthorityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
}

impl EksClusterCertificateAuthorityEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}

impl ToListMappable for EksClusterCertificateAuthorityEl {
    type O = BlockAssignable<EksClusterCertificateAuthorityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterCertificateAuthorityEl {}

impl BuildEksClusterCertificateAuthorityEl {
    pub fn build(self) -> EksClusterCertificateAuthorityEl {
        EksClusterCertificateAuthorityEl { data: core::default::Default::default() }
    }
}

pub struct EksClusterCertificateAuthorityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterCertificateAuthorityElRef {
    fn new(shared: StackShared, base: String) -> EksClusterCertificateAuthorityElRef {
        EksClusterCertificateAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterCertificateAuthorityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterIdentityElOidcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}

impl EksClusterIdentityElOidcEl {
    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }
}

impl ToListMappable for EksClusterIdentityElOidcEl {
    type O = BlockAssignable<EksClusterIdentityElOidcEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterIdentityElOidcEl {}

impl BuildEksClusterIdentityElOidcEl {
    pub fn build(self) -> EksClusterIdentityElOidcEl {
        EksClusterIdentityElOidcEl { issuer: core::default::Default::default() }
    }
}

pub struct EksClusterIdentityElOidcElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterIdentityElOidcElRef {
    fn new(shared: StackShared, base: String) -> EksClusterIdentityElOidcElRef {
        EksClusterIdentityElOidcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterIdentityElOidcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterIdentityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc: Option<ListField<EksClusterIdentityElOidcEl>>,
}

impl EksClusterIdentityEl {
    #[doc = "Set the field `oidc`.\n"]
    pub fn set_oidc(mut self, v: impl Into<ListField<EksClusterIdentityElOidcEl>>) -> Self {
        self.oidc = Some(v.into());
        self
    }
}

impl ToListMappable for EksClusterIdentityEl {
    type O = BlockAssignable<EksClusterIdentityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterIdentityEl {}

impl BuildEksClusterIdentityEl {
    pub fn build(self) -> EksClusterIdentityEl {
        EksClusterIdentityEl { oidc: core::default::Default::default() }
    }
}

pub struct EksClusterIdentityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterIdentityElRef {
    fn new(shared: StackShared, base: String) -> EksClusterIdentityElRef {
        EksClusterIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oidc` after provisioning.\n"]
    pub fn oidc(&self) -> ListRef<EksClusterIdentityElOidcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bootstrap_cluster_creator_admin_permissions: Option<PrimField<bool>>,
}

impl EksClusterAccessConfigEl {
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

impl ToListMappable for EksClusterAccessConfigEl {
    type O = BlockAssignable<EksClusterAccessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterAccessConfigEl {}

impl BuildEksClusterAccessConfigEl {
    pub fn build(self) -> EksClusterAccessConfigEl {
        EksClusterAccessConfigEl {
            authentication_mode: core::default::Default::default(),
            bootstrap_cluster_creator_admin_permissions: core::default::Default::default(),
        }
    }
}

pub struct EksClusterAccessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterAccessConfigElRef {
    fn new(shared: StackShared, base: String) -> EksClusterAccessConfigElRef {
        EksClusterAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterAccessConfigElRef {
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
pub struct EksClusterComputeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_pools: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_role_arn: Option<PrimField<String>>,
}

impl EksClusterComputeConfigEl {
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

impl ToListMappable for EksClusterComputeConfigEl {
    type O = BlockAssignable<EksClusterComputeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterComputeConfigEl {}

impl BuildEksClusterComputeConfigEl {
    pub fn build(self) -> EksClusterComputeConfigEl {
        EksClusterComputeConfigEl {
            enabled: core::default::Default::default(),
            node_pools: core::default::Default::default(),
            node_role_arn: core::default::Default::default(),
        }
    }
}

pub struct EksClusterComputeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterComputeConfigElRef {
    fn new(shared: StackShared, base: String) -> EksClusterComputeConfigElRef {
        EksClusterComputeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterComputeConfigElRef {
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
pub struct EksClusterEncryptionConfigElProviderEl {
    key_arn: PrimField<String>,
}

impl EksClusterEncryptionConfigElProviderEl { }

impl ToListMappable for EksClusterEncryptionConfigElProviderEl {
    type O = BlockAssignable<EksClusterEncryptionConfigElProviderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterEncryptionConfigElProviderEl {
    #[doc = ""]
    pub key_arn: PrimField<String>,
}

impl BuildEksClusterEncryptionConfigElProviderEl {
    pub fn build(self) -> EksClusterEncryptionConfigElProviderEl {
        EksClusterEncryptionConfigElProviderEl { key_arn: self.key_arn }
    }
}

pub struct EksClusterEncryptionConfigElProviderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterEncryptionConfigElProviderElRef {
    fn new(shared: StackShared, base: String) -> EksClusterEncryptionConfigElProviderElRef {
        EksClusterEncryptionConfigElProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterEncryptionConfigElProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_arn` after provisioning.\n"]
    pub fn key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksClusterEncryptionConfigElDynamic {
    provider: Option<DynamicBlock<EksClusterEncryptionConfigElProviderEl>>,
}

#[derive(Serialize)]
pub struct EksClusterEncryptionConfigEl {
    resources: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<Vec<EksClusterEncryptionConfigElProviderEl>>,
    dynamic: EksClusterEncryptionConfigElDynamic,
}

impl EksClusterEncryptionConfigEl {
    #[doc = "Set the field `provider`.\n"]
    pub fn set_provider(mut self, v: impl Into<BlockAssignable<EksClusterEncryptionConfigElProviderEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.provider = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.provider = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EksClusterEncryptionConfigEl {
    type O = BlockAssignable<EksClusterEncryptionConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterEncryptionConfigEl {
    #[doc = ""]
    pub resources: SetField<PrimField<String>>,
}

impl BuildEksClusterEncryptionConfigEl {
    pub fn build(self) -> EksClusterEncryptionConfigEl {
        EksClusterEncryptionConfigEl {
            resources: self.resources,
            provider: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EksClusterEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterEncryptionConfigElRef {
    fn new(shared: StackShared, base: String) -> EksClusterEncryptionConfigElRef {
        EksClusterEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> ListRef<EksClusterEncryptionConfigElProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl EksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for EksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
    type O = BlockAssignable<EksClusterKubernetesNetworkConfigElElasticLoadBalancingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {}

impl BuildEksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
    pub fn build(self) -> EksClusterKubernetesNetworkConfigElElasticLoadBalancingEl {
        EksClusterKubernetesNetworkConfigElElasticLoadBalancingEl { enabled: core::default::Default::default() }
    }
}

pub struct EksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
    fn new(shared: StackShared, base: String) -> EksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
        EksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksClusterKubernetesNetworkConfigElDynamic {
    elastic_load_balancing: Option<DynamicBlock<EksClusterKubernetesNetworkConfigElElasticLoadBalancingEl>>,
}

#[derive(Serialize)]
pub struct EksClusterKubernetesNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_ipv4_cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elastic_load_balancing: Option<Vec<EksClusterKubernetesNetworkConfigElElasticLoadBalancingEl>>,
    dynamic: EksClusterKubernetesNetworkConfigElDynamic,
}

impl EksClusterKubernetesNetworkConfigEl {
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

    #[doc = "Set the field `elastic_load_balancing`.\n"]
    pub fn set_elastic_load_balancing(
        mut self,
        v: impl Into<BlockAssignable<EksClusterKubernetesNetworkConfigElElasticLoadBalancingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.elastic_load_balancing = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.elastic_load_balancing = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EksClusterKubernetesNetworkConfigEl {
    type O = BlockAssignable<EksClusterKubernetesNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterKubernetesNetworkConfigEl {}

impl BuildEksClusterKubernetesNetworkConfigEl {
    pub fn build(self) -> EksClusterKubernetesNetworkConfigEl {
        EksClusterKubernetesNetworkConfigEl {
            ip_family: core::default::Default::default(),
            service_ipv4_cidr: core::default::Default::default(),
            elastic_load_balancing: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EksClusterKubernetesNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterKubernetesNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> EksClusterKubernetesNetworkConfigElRef {
        EksClusterKubernetesNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterKubernetesNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `elastic_load_balancing` after provisioning.\n"]
    pub fn elastic_load_balancing(&self) -> ListRef<EksClusterKubernetesNetworkConfigElElasticLoadBalancingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.elastic_load_balancing", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterOutpostConfigElControlPlanePlacementEl {
    group_name: PrimField<String>,
}

impl EksClusterOutpostConfigElControlPlanePlacementEl { }

impl ToListMappable for EksClusterOutpostConfigElControlPlanePlacementEl {
    type O = BlockAssignable<EksClusterOutpostConfigElControlPlanePlacementEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterOutpostConfigElControlPlanePlacementEl {
    #[doc = ""]
    pub group_name: PrimField<String>,
}

impl BuildEksClusterOutpostConfigElControlPlanePlacementEl {
    pub fn build(self) -> EksClusterOutpostConfigElControlPlanePlacementEl {
        EksClusterOutpostConfigElControlPlanePlacementEl { group_name: self.group_name }
    }
}

pub struct EksClusterOutpostConfigElControlPlanePlacementElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterOutpostConfigElControlPlanePlacementElRef {
    fn new(shared: StackShared, base: String) -> EksClusterOutpostConfigElControlPlanePlacementElRef {
        EksClusterOutpostConfigElControlPlanePlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterOutpostConfigElControlPlanePlacementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksClusterOutpostConfigElDynamic {
    control_plane_placement: Option<DynamicBlock<EksClusterOutpostConfigElControlPlanePlacementEl>>,
}

#[derive(Serialize)]
pub struct EksClusterOutpostConfigEl {
    control_plane_instance_type: PrimField<String>,
    outpost_arns: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_plane_placement: Option<Vec<EksClusterOutpostConfigElControlPlanePlacementEl>>,
    dynamic: EksClusterOutpostConfigElDynamic,
}

impl EksClusterOutpostConfigEl {
    #[doc = "Set the field `control_plane_placement`.\n"]
    pub fn set_control_plane_placement(
        mut self,
        v: impl Into<BlockAssignable<EksClusterOutpostConfigElControlPlanePlacementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.control_plane_placement = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.control_plane_placement = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EksClusterOutpostConfigEl {
    type O = BlockAssignable<EksClusterOutpostConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterOutpostConfigEl {
    #[doc = ""]
    pub control_plane_instance_type: PrimField<String>,
    #[doc = ""]
    pub outpost_arns: SetField<PrimField<String>>,
}

impl BuildEksClusterOutpostConfigEl {
    pub fn build(self) -> EksClusterOutpostConfigEl {
        EksClusterOutpostConfigEl {
            control_plane_instance_type: self.control_plane_instance_type,
            outpost_arns: self.outpost_arns,
            control_plane_placement: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EksClusterOutpostConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterOutpostConfigElRef {
    fn new(shared: StackShared, base: String) -> EksClusterOutpostConfigElRef {
        EksClusterOutpostConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterOutpostConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `control_plane_instance_type` after provisioning.\n"]
    pub fn control_plane_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.control_plane_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `outpost_arns` after provisioning.\n"]
    pub fn outpost_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.outpost_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `control_plane_placement` after provisioning.\n"]
    pub fn control_plane_placement(&self) -> ListRef<EksClusterOutpostConfigElControlPlanePlacementElRef> {
        ListRef::new(self.shared().clone(), format!("{}.control_plane_placement", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidrs: Option<SetField<PrimField<String>>>,
}

impl EksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cidrs = Some(v.into());
        self
    }
}

impl ToListMappable for EksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
    type O = BlockAssignable<EksClusterRemoteNetworkConfigElRemoteNodeNetworksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {}

impl BuildEksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
    pub fn build(self) -> EksClusterRemoteNetworkConfigElRemoteNodeNetworksEl {
        EksClusterRemoteNetworkConfigElRemoteNodeNetworksEl { cidrs: core::default::Default::default() }
    }
}

pub struct EksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
    fn new(shared: StackShared, base: String) -> EksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
        EksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterRemoteNetworkConfigElRemotePodNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidrs: Option<SetField<PrimField<String>>>,
}

impl EksClusterRemoteNetworkConfigElRemotePodNetworksEl {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cidrs = Some(v.into());
        self
    }
}

impl ToListMappable for EksClusterRemoteNetworkConfigElRemotePodNetworksEl {
    type O = BlockAssignable<EksClusterRemoteNetworkConfigElRemotePodNetworksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterRemoteNetworkConfigElRemotePodNetworksEl {}

impl BuildEksClusterRemoteNetworkConfigElRemotePodNetworksEl {
    pub fn build(self) -> EksClusterRemoteNetworkConfigElRemotePodNetworksEl {
        EksClusterRemoteNetworkConfigElRemotePodNetworksEl { cidrs: core::default::Default::default() }
    }
}

pub struct EksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
    fn new(shared: StackShared, base: String) -> EksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
        EksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterRemoteNetworkConfigElRemotePodNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksClusterRemoteNetworkConfigElDynamic {
    remote_node_networks: Option<DynamicBlock<EksClusterRemoteNetworkConfigElRemoteNodeNetworksEl>>,
    remote_pod_networks: Option<DynamicBlock<EksClusterRemoteNetworkConfigElRemotePodNetworksEl>>,
}

#[derive(Serialize)]
pub struct EksClusterRemoteNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_node_networks: Option<Vec<EksClusterRemoteNetworkConfigElRemoteNodeNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_pod_networks: Option<Vec<EksClusterRemoteNetworkConfigElRemotePodNetworksEl>>,
    dynamic: EksClusterRemoteNetworkConfigElDynamic,
}

impl EksClusterRemoteNetworkConfigEl {
    #[doc = "Set the field `remote_node_networks`.\n"]
    pub fn set_remote_node_networks(
        mut self,
        v: impl Into<BlockAssignable<EksClusterRemoteNetworkConfigElRemoteNodeNetworksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remote_node_networks = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remote_node_networks = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_pod_networks`.\n"]
    pub fn set_remote_pod_networks(
        mut self,
        v: impl Into<BlockAssignable<EksClusterRemoteNetworkConfigElRemotePodNetworksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.remote_pod_networks = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.remote_pod_networks = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EksClusterRemoteNetworkConfigEl {
    type O = BlockAssignable<EksClusterRemoteNetworkConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterRemoteNetworkConfigEl {}

impl BuildEksClusterRemoteNetworkConfigEl {
    pub fn build(self) -> EksClusterRemoteNetworkConfigEl {
        EksClusterRemoteNetworkConfigEl {
            remote_node_networks: core::default::Default::default(),
            remote_pod_networks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EksClusterRemoteNetworkConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterRemoteNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> EksClusterRemoteNetworkConfigElRef {
        EksClusterRemoteNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterRemoteNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `remote_node_networks` after provisioning.\n"]
    pub fn remote_node_networks(&self) -> ListRef<EksClusterRemoteNetworkConfigElRemoteNodeNetworksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_node_networks", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_pod_networks` after provisioning.\n"]
    pub fn remote_pod_networks(&self) -> ListRef<EksClusterRemoteNetworkConfigElRemotePodNetworksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_pod_networks", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterStorageConfigElBlockStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl EksClusterStorageConfigElBlockStorageEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for EksClusterStorageConfigElBlockStorageEl {
    type O = BlockAssignable<EksClusterStorageConfigElBlockStorageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterStorageConfigElBlockStorageEl {}

impl BuildEksClusterStorageConfigElBlockStorageEl {
    pub fn build(self) -> EksClusterStorageConfigElBlockStorageEl {
        EksClusterStorageConfigElBlockStorageEl { enabled: core::default::Default::default() }
    }
}

pub struct EksClusterStorageConfigElBlockStorageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterStorageConfigElBlockStorageElRef {
    fn new(shared: StackShared, base: String) -> EksClusterStorageConfigElBlockStorageElRef {
        EksClusterStorageConfigElBlockStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterStorageConfigElBlockStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksClusterStorageConfigElDynamic {
    block_storage: Option<DynamicBlock<EksClusterStorageConfigElBlockStorageEl>>,
}

#[derive(Serialize)]
pub struct EksClusterStorageConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_storage: Option<Vec<EksClusterStorageConfigElBlockStorageEl>>,
    dynamic: EksClusterStorageConfigElDynamic,
}

impl EksClusterStorageConfigEl {
    #[doc = "Set the field `block_storage`.\n"]
    pub fn set_block_storage(mut self, v: impl Into<BlockAssignable<EksClusterStorageConfigElBlockStorageEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.block_storage = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.block_storage = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EksClusterStorageConfigEl {
    type O = BlockAssignable<EksClusterStorageConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterStorageConfigEl {}

impl BuildEksClusterStorageConfigEl {
    pub fn build(self) -> EksClusterStorageConfigEl {
        EksClusterStorageConfigEl {
            block_storage: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EksClusterStorageConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterStorageConfigElRef {
    fn new(shared: StackShared, base: String) -> EksClusterStorageConfigElRef {
        EksClusterStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterStorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_storage` after provisioning.\n"]
    pub fn block_storage(&self) -> ListRef<EksClusterStorageConfigElBlockStorageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.block_storage", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl EksClusterTimeoutsEl {
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

impl ToListMappable for EksClusterTimeoutsEl {
    type O = BlockAssignable<EksClusterTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterTimeoutsEl {}

impl BuildEksClusterTimeoutsEl {
    pub fn build(self) -> EksClusterTimeoutsEl {
        EksClusterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EksClusterTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EksClusterTimeoutsElRef {
        EksClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterTimeoutsElRef {
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

#[derive(Serialize)]
pub struct EksClusterUpgradePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    support_type: Option<PrimField<String>>,
}

impl EksClusterUpgradePolicyEl {
    #[doc = "Set the field `support_type`.\n"]
    pub fn set_support_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.support_type = Some(v.into());
        self
    }
}

impl ToListMappable for EksClusterUpgradePolicyEl {
    type O = BlockAssignable<EksClusterUpgradePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterUpgradePolicyEl {}

impl BuildEksClusterUpgradePolicyEl {
    pub fn build(self) -> EksClusterUpgradePolicyEl {
        EksClusterUpgradePolicyEl { support_type: core::default::Default::default() }
    }
}

pub struct EksClusterUpgradePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterUpgradePolicyElRef {
    fn new(shared: StackShared, base: String) -> EksClusterUpgradePolicyElRef {
        EksClusterUpgradePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterUpgradePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `support_type` after provisioning.\n"]
    pub fn support_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.support_type", self.base))
    }
}

#[derive(Serialize)]
pub struct EksClusterVpcConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_private_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_public_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_access_cidrs: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    subnet_ids: SetField<PrimField<String>>,
}

impl EksClusterVpcConfigEl {
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
}

impl ToListMappable for EksClusterVpcConfigEl {
    type O = BlockAssignable<EksClusterVpcConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterVpcConfigEl {
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}

impl BuildEksClusterVpcConfigEl {
    pub fn build(self) -> EksClusterVpcConfigEl {
        EksClusterVpcConfigEl {
            endpoint_private_access: core::default::Default::default(),
            endpoint_public_access: core::default::Default::default(),
            public_access_cidrs: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
        }
    }
}

pub struct EksClusterVpcConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterVpcConfigElRef {
    fn new(shared: StackShared, base: String) -> EksClusterVpcConfigElRef {
        EksClusterVpcConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterVpcConfigElRef {
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
pub struct EksClusterZonalShiftConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl EksClusterZonalShiftConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for EksClusterZonalShiftConfigEl {
    type O = BlockAssignable<EksClusterZonalShiftConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksClusterZonalShiftConfigEl {}

impl BuildEksClusterZonalShiftConfigEl {
    pub fn build(self) -> EksClusterZonalShiftConfigEl {
        EksClusterZonalShiftConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct EksClusterZonalShiftConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksClusterZonalShiftConfigElRef {
    fn new(shared: StackShared, base: String) -> EksClusterZonalShiftConfigElRef {
        EksClusterZonalShiftConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksClusterZonalShiftConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksClusterDynamic {
    access_config: Option<DynamicBlock<EksClusterAccessConfigEl>>,
    compute_config: Option<DynamicBlock<EksClusterComputeConfigEl>>,
    encryption_config: Option<DynamicBlock<EksClusterEncryptionConfigEl>>,
    kubernetes_network_config: Option<DynamicBlock<EksClusterKubernetesNetworkConfigEl>>,
    outpost_config: Option<DynamicBlock<EksClusterOutpostConfigEl>>,
    remote_network_config: Option<DynamicBlock<EksClusterRemoteNetworkConfigEl>>,
    storage_config: Option<DynamicBlock<EksClusterStorageConfigEl>>,
    upgrade_policy: Option<DynamicBlock<EksClusterUpgradePolicyEl>>,
    vpc_config: Option<DynamicBlock<EksClusterVpcConfigEl>>,
    zonal_shift_config: Option<DynamicBlock<EksClusterZonalShiftConfigEl>>,
}
