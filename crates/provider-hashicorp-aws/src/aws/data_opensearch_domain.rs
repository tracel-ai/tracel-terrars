use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataOpensearchDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataOpensearchDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOpensearchDomainData>,
}

#[derive(Clone)]
pub struct DataOpensearchDomain(Rc<DataOpensearchDomain_>);

impl DataOpensearchDomain {
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

    #[doc = "Get a reference to the value of field `access_policies` after provisioning.\n"]
    pub fn access_policies(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_policies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `advanced_options` after provisioning.\n"]
    pub fn advanced_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.advanced_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(
        &self,
    ) -> ListRef<DataOpensearchDomainAdvancedSecurityOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_security_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> ListRef<DataOpensearchDomainAutoTuneOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_tune_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<DataOpensearchDomainClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> ListRef<DataOpensearchDomainCognitoOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cognito_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint` after provisioning.\n"]
    pub fn dashboard_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint_v2` after provisioning.\n"]
    pub fn dashboard_endpoint_v2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_endpoint_v2", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deleted", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_v2_hosted_zone_id` after provisioning.\n"]
    pub fn domain_endpoint_v2_hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_endpoint_v2_hosted_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> ListRef<DataOpensearchDomainEbsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(&self) -> ListRef<DataOpensearchDomainEncryptionAtRestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_at_rest", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_v2` after provisioning.\n"]
    pub fn endpoint_v2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_v2", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_options` after provisioning.\n"]
    pub fn identity_center_options(
        &self,
    ) -> ListRef<DataOpensearchDomainIdentityCenterOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_center_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> SetRef<DataOpensearchDomainLogPublishingOptionsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.log_publishing_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(
        &self,
    ) -> ListRef<DataOpensearchDomainNodeToNodeEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_to_node_encryption", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `off_peak_window_options` after provisioning.\n"]
    pub fn off_peak_window_options(
        &self,
    ) -> ListRef<DataOpensearchDomainOffPeakWindowOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.off_peak_window_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `processing` after provisioning.\n"]
    pub fn processing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.processing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> ListRef<DataOpensearchDomainSnapshotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `software_update_options` after provisioning.\n"]
    pub fn software_update_options(
        &self,
    ) -> ListRef<DataOpensearchDomainSoftwareUpdateOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_update_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> ListRef<DataOpensearchDomainVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}

impl Referable for DataOpensearchDomain {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataOpensearchDomain {}

impl ToListMappable for DataOpensearchDomain {
    type O = ListRef<DataOpensearchDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOpensearchDomain_ {
    fn extract_datasource_type(&self) -> String {
        "aws_opensearch_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOpensearchDomain {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildDataOpensearchDomain {
    pub fn build(self, stack: &mut Stack) -> DataOpensearchDomain {
        let out = DataOpensearchDomain(Rc::new(DataOpensearchDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOpensearchDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOpensearchDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataOpensearchDomainRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_policies` after provisioning.\n"]
    pub fn access_policies(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_policies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `advanced_options` after provisioning.\n"]
    pub fn advanced_options(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.advanced_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(
        &self,
    ) -> ListRef<DataOpensearchDomainAdvancedSecurityOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_security_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> ListRef<DataOpensearchDomainAutoTuneOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_tune_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<DataOpensearchDomainClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> ListRef<DataOpensearchDomainCognitoOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cognito_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint` after provisioning.\n"]
    pub fn dashboard_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dashboard_endpoint_v2` after provisioning.\n"]
    pub fn dashboard_endpoint_v2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dashboard_endpoint_v2", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deleted", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_v2_hosted_zone_id` after provisioning.\n"]
    pub fn domain_endpoint_v2_hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_endpoint_v2_hosted_zone_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> ListRef<DataOpensearchDomainEbsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(&self) -> ListRef<DataOpensearchDomainEncryptionAtRestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_at_rest", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `endpoint_v2` after provisioning.\n"]
    pub fn endpoint_v2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_v2", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_version` after provisioning.\n"]
    pub fn engine_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_options` after provisioning.\n"]
    pub fn identity_center_options(
        &self,
    ) -> ListRef<DataOpensearchDomainIdentityCenterOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_center_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(&self) -> SetRef<DataOpensearchDomainLogPublishingOptionsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.log_publishing_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(
        &self,
    ) -> ListRef<DataOpensearchDomainNodeToNodeEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_to_node_encryption", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `off_peak_window_options` after provisioning.\n"]
    pub fn off_peak_window_options(
        &self,
    ) -> ListRef<DataOpensearchDomainOffPeakWindowOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.off_peak_window_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `processing` after provisioning.\n"]
    pub fn processing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.processing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> ListRef<DataOpensearchDomainSnapshotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `software_update_options` after provisioning.\n"]
    pub fn software_update_options(
        &self,
    ) -> ListRef<DataOpensearchDomainSoftwareUpdateOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_update_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> ListRef<DataOpensearchDomainVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainAdvancedSecurityOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    anonymous_auth_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_user_database_enabled: Option<PrimField<bool>>,
}

impl DataOpensearchDomainAdvancedSecurityOptionsEl {
    #[doc = "Set the field `anonymous_auth_enabled`.\n"]
    pub fn set_anonymous_auth_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.anonymous_auth_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `internal_user_database_enabled`.\n"]
    pub fn set_internal_user_database_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.internal_user_database_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainAdvancedSecurityOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainAdvancedSecurityOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainAdvancedSecurityOptionsEl {}

impl BuildDataOpensearchDomainAdvancedSecurityOptionsEl {
    pub fn build(self) -> DataOpensearchDomainAdvancedSecurityOptionsEl {
        DataOpensearchDomainAdvancedSecurityOptionsEl {
            anonymous_auth_enabled: core::default::Default::default(),
            enabled: core::default::Default::default(),
            internal_user_database_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainAdvancedSecurityOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainAdvancedSecurityOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainAdvancedSecurityOptionsElRef {
        DataOpensearchDomainAdvancedSecurityOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainAdvancedSecurityOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `anonymous_auth_enabled` after provisioning.\n"]
    pub fn anonymous_auth_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.anonymous_auth_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `internal_user_database_enabled` after provisioning.\n"]
    pub fn internal_user_database_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal_user_database_enabled", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}

impl DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    type O = BlockAssignable<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {}

impl BuildDataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    pub fn build(self) -> DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
        DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
        DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cron_expression_for_recurrence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration:
        Option<ListField<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_at: Option<PrimField<String>>,
}

impl DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[doc = "Set the field `cron_expression_for_recurrence`.\n"]
    pub fn set_cron_expression_for_recurrence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cron_expression_for_recurrence = Some(v.into());
        self
    }

    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v: impl Into<ListField<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
    ) -> Self {
        self.duration = Some(v.into());
        self
    }

    #[doc = "Set the field `start_at`.\n"]
    pub fn set_start_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_at = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    type O = BlockAssignable<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {}

impl BuildDataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    pub fn build(self) -> DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
        DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
            cron_expression_for_recurrence: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_at: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
        DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cron_expression_for_recurrence` after provisioning.\n"]
    pub fn cron_expression_for_recurrence(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cron_expression_for_recurrence", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(
        &self,
    ) -> ListRef<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_at` after provisioning.\n"]
    pub fn start_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainAutoTuneOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_schedule:
        Option<SetField<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback_on_disable: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_off_peak_window: Option<PrimField<bool>>,
}

impl DataOpensearchDomainAutoTuneOptionsEl {
    #[doc = "Set the field `desired_state`.\n"]
    pub fn set_desired_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.desired_state = Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_schedule`.\n"]
    pub fn set_maintenance_schedule(
        mut self,
        v: impl Into<SetField<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
    ) -> Self {
        self.maintenance_schedule = Some(v.into());
        self
    }

    #[doc = "Set the field `rollback_on_disable`.\n"]
    pub fn set_rollback_on_disable(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rollback_on_disable = Some(v.into());
        self
    }

    #[doc = "Set the field `use_off_peak_window`.\n"]
    pub fn set_use_off_peak_window(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_off_peak_window = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainAutoTuneOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainAutoTuneOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainAutoTuneOptionsEl {}

impl BuildDataOpensearchDomainAutoTuneOptionsEl {
    pub fn build(self) -> DataOpensearchDomainAutoTuneOptionsEl {
        DataOpensearchDomainAutoTuneOptionsEl {
            desired_state: core::default::Default::default(),
            maintenance_schedule: core::default::Default::default(),
            rollback_on_disable: core::default::Default::default(),
            use_off_peak_window: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainAutoTuneOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainAutoTuneOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainAutoTuneOptionsElRef {
        DataOpensearchDomainAutoTuneOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainAutoTuneOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_state` after provisioning.\n"]
    pub fn desired_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.desired_state", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `maintenance_schedule` after provisioning.\n"]
    pub fn maintenance_schedule(
        &self,
    ) -> SetRef<DataOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.maintenance_schedule", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `rollback_on_disable` after provisioning.\n"]
    pub fn rollback_on_disable(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollback_on_disable", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `use_off_peak_window` after provisioning.\n"]
    pub fn use_off_peak_window(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_off_peak_window", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainClusterConfigElColdStorageOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataOpensearchDomainClusterConfigElColdStorageOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainClusterConfigElColdStorageOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainClusterConfigElColdStorageOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainClusterConfigElColdStorageOptionsEl {}

impl BuildDataOpensearchDomainClusterConfigElColdStorageOptionsEl {
    pub fn build(self) -> DataOpensearchDomainClusterConfigElColdStorageOptionsEl {
        DataOpensearchDomainClusterConfigElColdStorageOptionsEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
        DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainClusterConfigElColdStorageOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
    type O = BlockAssignable<DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {}

impl BuildDataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
    pub fn build(self) -> DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
        DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
            count: core::default::Default::default(),
            enabled: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
        DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainClusterConfigElNodeOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<ListField<DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type: Option<PrimField<String>>,
}

impl DataOpensearchDomainClusterConfigElNodeOptionsEl {
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl Into<ListField<DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl>>,
    ) -> Self {
        self.node_config = Some(v.into());
        self
    }

    #[doc = "Set the field `node_type`.\n"]
    pub fn set_node_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainClusterConfigElNodeOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainClusterConfigElNodeOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainClusterConfigElNodeOptionsEl {}

impl BuildDataOpensearchDomainClusterConfigElNodeOptionsEl {
    pub fn build(self) -> DataOpensearchDomainClusterConfigElNodeOptionsEl {
        DataOpensearchDomainClusterConfigElNodeOptionsEl {
            node_config: core::default::Default::default(),
            node_type: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainClusterConfigElNodeOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainClusterConfigElNodeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOpensearchDomainClusterConfigElNodeOptionsElRef {
        DataOpensearchDomainClusterConfigElNodeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainClusterConfigElNodeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(
        &self,
    ) -> ListRef<DataOpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_count: Option<PrimField<f64>>,
}

impl DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[doc = "Set the field `availability_zone_count`.\n"]
    pub fn set_availability_zone_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.availability_zone_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    type O = BlockAssignable<DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {}

impl BuildDataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    pub fn build(self) -> DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
        DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
            availability_zone_count: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
        DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone_count` after provisioning.\n"]
    pub fn availability_zone_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_count", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainClusterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cold_storage_options:
        Option<ListField<DataOpensearchDomainClusterConfigElColdStorageOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_master_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_master_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_master_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_az_with_standby_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_options: Option<ListField<DataOpensearchDomainClusterConfigElNodeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warm_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warm_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warm_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_awareness_config:
        Option<ListField<DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_awareness_enabled: Option<PrimField<bool>>,
}

impl DataOpensearchDomainClusterConfigEl {
    #[doc = "Set the field `cold_storage_options`.\n"]
    pub fn set_cold_storage_options(
        mut self,
        v: impl Into<ListField<DataOpensearchDomainClusterConfigElColdStorageOptionsEl>>,
    ) -> Self {
        self.cold_storage_options = Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_master_count`.\n"]
    pub fn set_dedicated_master_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.dedicated_master_count = Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_master_enabled`.\n"]
    pub fn set_dedicated_master_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.dedicated_master_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_master_type`.\n"]
    pub fn set_dedicated_master_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dedicated_master_type = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `multi_az_with_standby_enabled`.\n"]
    pub fn set_multi_az_with_standby_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.multi_az_with_standby_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `node_options`.\n"]
    pub fn set_node_options(
        mut self,
        v: impl Into<ListField<DataOpensearchDomainClusterConfigElNodeOptionsEl>>,
    ) -> Self {
        self.node_options = Some(v.into());
        self
    }

    #[doc = "Set the field `warm_count`.\n"]
    pub fn set_warm_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.warm_count = Some(v.into());
        self
    }

    #[doc = "Set the field `warm_enabled`.\n"]
    pub fn set_warm_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.warm_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `warm_type`.\n"]
    pub fn set_warm_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.warm_type = Some(v.into());
        self
    }

    #[doc = "Set the field `zone_awareness_config`.\n"]
    pub fn set_zone_awareness_config(
        mut self,
        v: impl Into<ListField<DataOpensearchDomainClusterConfigElZoneAwarenessConfigEl>>,
    ) -> Self {
        self.zone_awareness_config = Some(v.into());
        self
    }

    #[doc = "Set the field `zone_awareness_enabled`.\n"]
    pub fn set_zone_awareness_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.zone_awareness_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainClusterConfigEl {
    type O = BlockAssignable<DataOpensearchDomainClusterConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainClusterConfigEl {}

impl BuildDataOpensearchDomainClusterConfigEl {
    pub fn build(self) -> DataOpensearchDomainClusterConfigEl {
        DataOpensearchDomainClusterConfigEl {
            cold_storage_options: core::default::Default::default(),
            dedicated_master_count: core::default::Default::default(),
            dedicated_master_enabled: core::default::Default::default(),
            dedicated_master_type: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            multi_az_with_standby_enabled: core::default::Default::default(),
            node_options: core::default::Default::default(),
            warm_count: core::default::Default::default(),
            warm_enabled: core::default::Default::default(),
            warm_type: core::default::Default::default(),
            zone_awareness_config: core::default::Default::default(),
            zone_awareness_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainClusterConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainClusterConfigElRef {
        DataOpensearchDomainClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cold_storage_options` after provisioning.\n"]
    pub fn cold_storage_options(
        &self,
    ) -> ListRef<DataOpensearchDomainClusterConfigElColdStorageOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cold_storage_options", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dedicated_master_count` after provisioning.\n"]
    pub fn dedicated_master_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dedicated_master_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dedicated_master_enabled` after provisioning.\n"]
    pub fn dedicated_master_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dedicated_master_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dedicated_master_type` after provisioning.\n"]
    pub fn dedicated_master_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dedicated_master_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `multi_az_with_standby_enabled` after provisioning.\n"]
    pub fn multi_az_with_standby_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multi_az_with_standby_enabled", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `node_options` after provisioning.\n"]
    pub fn node_options(&self) -> ListRef<DataOpensearchDomainClusterConfigElNodeOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_options", self.base))
    }

    #[doc = "Get a reference to the value of field `warm_count` after provisioning.\n"]
    pub fn warm_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.warm_count", self.base))
    }

    #[doc = "Get a reference to the value of field `warm_enabled` after provisioning.\n"]
    pub fn warm_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.warm_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `warm_type` after provisioning.\n"]
    pub fn warm_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.warm_type", self.base))
    }

    #[doc = "Get a reference to the value of field `zone_awareness_config` after provisioning.\n"]
    pub fn zone_awareness_config(
        &self,
    ) -> ListRef<DataOpensearchDomainClusterConfigElZoneAwarenessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_awareness_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `zone_awareness_enabled` after provisioning.\n"]
    pub fn zone_awareness_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone_awareness_enabled", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainCognitoOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_pool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_pool_id: Option<PrimField<String>>,
}

impl DataOpensearchDomainCognitoOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `identity_pool_id`.\n"]
    pub fn set_identity_pool_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_pool_id = Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `user_pool_id`.\n"]
    pub fn set_user_pool_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_pool_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainCognitoOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainCognitoOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainCognitoOptionsEl {}

impl BuildDataOpensearchDomainCognitoOptionsEl {
    pub fn build(self) -> DataOpensearchDomainCognitoOptionsEl {
        DataOpensearchDomainCognitoOptionsEl {
            enabled: core::default::Default::default(),
            identity_pool_id: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            user_pool_id: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainCognitoOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainCognitoOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainCognitoOptionsElRef {
        DataOpensearchDomainCognitoOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainCognitoOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_pool_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `user_pool_id` after provisioning.\n"]
    pub fn user_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_pool_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainEbsOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}

impl DataOpensearchDomainEbsOptionsEl {
    #[doc = "Set the field `ebs_enabled`.\n"]
    pub fn set_ebs_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ebs_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_size = Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.volume_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainEbsOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainEbsOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainEbsOptionsEl {}

impl BuildDataOpensearchDomainEbsOptionsEl {
    pub fn build(self) -> DataOpensearchDomainEbsOptionsEl {
        DataOpensearchDomainEbsOptionsEl {
            ebs_enabled: core::default::Default::default(),
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainEbsOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainEbsOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainEbsOptionsElRef {
        DataOpensearchDomainEbsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainEbsOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_enabled` after provisioning.\n"]
    pub fn ebs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainEncryptionAtRestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}

impl DataOpensearchDomainEncryptionAtRestEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainEncryptionAtRestEl {
    type O = BlockAssignable<DataOpensearchDomainEncryptionAtRestEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainEncryptionAtRestEl {}

impl BuildDataOpensearchDomainEncryptionAtRestEl {
    pub fn build(self) -> DataOpensearchDomainEncryptionAtRestEl {
        DataOpensearchDomainEncryptionAtRestEl {
            enabled: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainEncryptionAtRestElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainEncryptionAtRestElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainEncryptionAtRestElRef {
        DataOpensearchDomainEncryptionAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainEncryptionAtRestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainIdentityCenterOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_api_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_center_instance_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_key: Option<PrimField<String>>,
}

impl DataOpensearchDomainIdentityCenterOptionsEl {
    #[doc = "Set the field `enabled_api_access`.\n"]
    pub fn set_enabled_api_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled_api_access = Some(v.into());
        self
    }

    #[doc = "Set the field `identity_center_instance_arn`.\n"]
    pub fn set_identity_center_instance_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_center_instance_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `roles_key`.\n"]
    pub fn set_roles_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.roles_key = Some(v.into());
        self
    }

    #[doc = "Set the field `subject_key`.\n"]
    pub fn set_subject_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subject_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainIdentityCenterOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainIdentityCenterOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainIdentityCenterOptionsEl {}

impl BuildDataOpensearchDomainIdentityCenterOptionsEl {
    pub fn build(self) -> DataOpensearchDomainIdentityCenterOptionsEl {
        DataOpensearchDomainIdentityCenterOptionsEl {
            enabled_api_access: core::default::Default::default(),
            identity_center_instance_arn: core::default::Default::default(),
            roles_key: core::default::Default::default(),
            subject_key: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainIdentityCenterOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainIdentityCenterOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainIdentityCenterOptionsElRef {
        DataOpensearchDomainIdentityCenterOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainIdentityCenterOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled_api_access` after provisioning.\n"]
    pub fn enabled_api_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enabled_api_access", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `identity_center_instance_arn` after provisioning.\n"]
    pub fn identity_center_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_center_instance_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `roles_key` after provisioning.\n"]
    pub fn roles_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.roles_key", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_key` after provisioning.\n"]
    pub fn subject_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subject_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainLogPublishingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_type: Option<PrimField<String>>,
}

impl DataOpensearchDomainLogPublishingOptionsEl {
    #[doc = "Set the field `cloudwatch_log_group_arn`.\n"]
    pub fn set_cloudwatch_log_group_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloudwatch_log_group_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `log_type`.\n"]
    pub fn set_log_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainLogPublishingOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainLogPublishingOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainLogPublishingOptionsEl {}

impl BuildDataOpensearchDomainLogPublishingOptionsEl {
    pub fn build(self) -> DataOpensearchDomainLogPublishingOptionsEl {
        DataOpensearchDomainLogPublishingOptionsEl {
            cloudwatch_log_group_arn: core::default::Default::default(),
            enabled: core::default::Default::default(),
            log_type: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainLogPublishingOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainLogPublishingOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainLogPublishingOptionsElRef {
        DataOpensearchDomainLogPublishingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainLogPublishingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_log_group_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_type` after provisioning.\n"]
    pub fn log_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainNodeToNodeEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl DataOpensearchDomainNodeToNodeEncryptionEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainNodeToNodeEncryptionEl {
    type O = BlockAssignable<DataOpensearchDomainNodeToNodeEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainNodeToNodeEncryptionEl {}

impl BuildDataOpensearchDomainNodeToNodeEncryptionEl {
    pub fn build(self) -> DataOpensearchDomainNodeToNodeEncryptionEl {
        DataOpensearchDomainNodeToNodeEncryptionEl {
            enabled: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainNodeToNodeEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainNodeToNodeEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainNodeToNodeEncryptionElRef {
        DataOpensearchDomainNodeToNodeEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainNodeToNodeEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
}

impl DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.hours = Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minutes = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
    type O =
        BlockAssignable<DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {}

impl BuildDataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
    pub fn build(
        self,
    ) -> DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
        DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
        DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    window_start_time: Option<
        ListField<DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl>,
    >,
}

impl DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
    #[doc = "Set the field `window_start_time`.\n"]
    pub fn set_window_start_time(
        mut self,
        v: impl Into<
            ListField<DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl>,
        >,
    ) -> Self {
        self.window_start_time = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
    type O = BlockAssignable<DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {}

impl BuildDataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
    pub fn build(self) -> DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
        DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
            window_start_time: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
        DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `window_start_time` after provisioning.\n"]
    pub fn window_start_time(
        &self,
    ) -> ListRef<DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.window_start_time", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainOffPeakWindowOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    off_peak_window: Option<ListField<DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl>>,
}

impl DataOpensearchDomainOffPeakWindowOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `off_peak_window`.\n"]
    pub fn set_off_peak_window(
        mut self,
        v: impl Into<ListField<DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl>>,
    ) -> Self {
        self.off_peak_window = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainOffPeakWindowOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainOffPeakWindowOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainOffPeakWindowOptionsEl {}

impl BuildDataOpensearchDomainOffPeakWindowOptionsEl {
    pub fn build(self) -> DataOpensearchDomainOffPeakWindowOptionsEl {
        DataOpensearchDomainOffPeakWindowOptionsEl {
            enabled: core::default::Default::default(),
            off_peak_window: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainOffPeakWindowOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainOffPeakWindowOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainOffPeakWindowOptionsElRef {
        DataOpensearchDomainOffPeakWindowOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainOffPeakWindowOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `off_peak_window` after provisioning.\n"]
    pub fn off_peak_window(
        &self,
    ) -> ListRef<DataOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.off_peak_window", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainSnapshotOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automated_snapshot_start_hour: Option<PrimField<f64>>,
}

impl DataOpensearchDomainSnapshotOptionsEl {
    #[doc = "Set the field `automated_snapshot_start_hour`.\n"]
    pub fn set_automated_snapshot_start_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.automated_snapshot_start_hour = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainSnapshotOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainSnapshotOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainSnapshotOptionsEl {}

impl BuildDataOpensearchDomainSnapshotOptionsEl {
    pub fn build(self) -> DataOpensearchDomainSnapshotOptionsEl {
        DataOpensearchDomainSnapshotOptionsEl {
            automated_snapshot_start_hour: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainSnapshotOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainSnapshotOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainSnapshotOptionsElRef {
        DataOpensearchDomainSnapshotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainSnapshotOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automated_snapshot_start_hour` after provisioning.\n"]
    pub fn automated_snapshot_start_hour(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.automated_snapshot_start_hour", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainSoftwareUpdateOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_software_update_enabled: Option<PrimField<bool>>,
}

impl DataOpensearchDomainSoftwareUpdateOptionsEl {
    #[doc = "Set the field `auto_software_update_enabled`.\n"]
    pub fn set_auto_software_update_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_software_update_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataOpensearchDomainSoftwareUpdateOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainSoftwareUpdateOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainSoftwareUpdateOptionsEl {}

impl BuildDataOpensearchDomainSoftwareUpdateOptionsEl {
    pub fn build(self) -> DataOpensearchDomainSoftwareUpdateOptionsEl {
        DataOpensearchDomainSoftwareUpdateOptionsEl {
            auto_software_update_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainSoftwareUpdateOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainSoftwareUpdateOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainSoftwareUpdateOptionsElRef {
        DataOpensearchDomainSoftwareUpdateOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainSoftwareUpdateOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_software_update_enabled` after provisioning.\n"]
    pub fn auto_software_update_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_software_update_enabled", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataOpensearchDomainVpcOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zones: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl DataOpensearchDomainVpcOptionsEl {
    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.availability_zones = Some(v.into());
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

impl ToListMappable for DataOpensearchDomainVpcOptionsEl {
    type O = BlockAssignable<DataOpensearchDomainVpcOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOpensearchDomainVpcOptionsEl {}

impl BuildDataOpensearchDomainVpcOptionsEl {
    pub fn build(self) -> DataOpensearchDomainVpcOptionsEl {
        DataOpensearchDomainVpcOptionsEl {
            availability_zones: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct DataOpensearchDomainVpcOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchDomainVpcOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataOpensearchDomainVpcOptionsElRef {
        DataOpensearchDomainVpcOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOpensearchDomainVpcOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.base),
        )
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
