use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct OpensearchDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_policies: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_options: Option<RecField<PrimField<String>>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_security_options: Option<Vec<OpensearchDomainAdvancedSecurityOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aiml_options: Option<Vec<OpensearchDomainAimlOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_tune_options: Option<Vec<OpensearchDomainAutoTuneOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_config: Option<Vec<OpensearchDomainClusterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cognito_options: Option<Vec<OpensearchDomainCognitoOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_endpoint_options: Option<Vec<OpensearchDomainDomainEndpointOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_options: Option<Vec<OpensearchDomainEbsOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypt_at_rest: Option<Vec<OpensearchDomainEncryptAtRestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_center_options: Option<Vec<OpensearchDomainIdentityCenterOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_publishing_options: Option<Vec<OpensearchDomainLogPublishingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_to_node_encryption: Option<Vec<OpensearchDomainNodeToNodeEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    off_peak_window_options: Option<Vec<OpensearchDomainOffPeakWindowOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_options: Option<Vec<OpensearchDomainSnapshotOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software_update_options: Option<Vec<OpensearchDomainSoftwareUpdateOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<OpensearchDomainTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_options: Option<Vec<OpensearchDomainVpcOptionsEl>>,
    dynamic: OpensearchDomainDynamic,
}
struct OpensearchDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpensearchDomainData>,
}
#[derive(Clone)]
pub struct OpensearchDomain(Rc<OpensearchDomain_>);
impl OpensearchDomain {
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
    #[doc = "Set the field `access_policies`.\n"]
    pub fn set_access_policies(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().access_policies = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_options`.\n"]
    pub fn set_advanced_options(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().advanced_options = Some(v.into());
        self
    }
    #[doc = "Set the field `engine_version`.\n"]
    pub fn set_engine_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_version = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_address_type = Some(v.into());
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
    #[doc = "Set the field `advanced_security_options`.\n"]
    pub fn set_advanced_security_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainAdvancedSecurityOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_security_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_security_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `aiml_options`.\n"]
    pub fn set_aiml_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainAimlOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aiml_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aiml_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `auto_tune_options`.\n"]
    pub fn set_auto_tune_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainAutoTuneOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_tune_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_tune_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cluster_config`.\n"]
    pub fn set_cluster_config(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainClusterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cluster_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cluster_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cognito_options`.\n"]
    pub fn set_cognito_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainCognitoOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cognito_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cognito_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `domain_endpoint_options`.\n"]
    pub fn set_domain_endpoint_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainDomainEndpointOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domain_endpoint_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domain_endpoint_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ebs_options`.\n"]
    pub fn set_ebs_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainEbsOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ebs_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ebs_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encrypt_at_rest`.\n"]
    pub fn set_encrypt_at_rest(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainEncryptAtRestEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encrypt_at_rest = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encrypt_at_rest = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `identity_center_options`.\n"]
    pub fn set_identity_center_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainIdentityCenterOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_center_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_center_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_publishing_options`.\n"]
    pub fn set_log_publishing_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainLogPublishingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_publishing_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_publishing_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_to_node_encryption`.\n"]
    pub fn set_node_to_node_encryption(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainNodeToNodeEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_to_node_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_to_node_encryption = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `off_peak_window_options`.\n"]
    pub fn set_off_peak_window_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainOffPeakWindowOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().off_peak_window_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.off_peak_window_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `snapshot_options`.\n"]
    pub fn set_snapshot_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainSnapshotOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().snapshot_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.snapshot_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `software_update_options`.\n"]
    pub fn set_software_update_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainSoftwareUpdateOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().software_update_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.software_update_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<OpensearchDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_options`.\n"]
    pub fn set_vpc_options(
        self,
        v: impl Into<BlockAssignable<OpensearchDomainVpcOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_options = Some(d);
            }
        }
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
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(
        &self,
    ) -> ListRef<OpensearchDomainAdvancedSecurityOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_security_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aiml_options` after provisioning.\n"]
    pub fn aiml_options(&self) -> ListRef<OpensearchDomainAimlOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aiml_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> ListRef<OpensearchDomainAutoTuneOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_tune_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<OpensearchDomainClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> ListRef<OpensearchDomainCognitoOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cognito_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(&self) -> ListRef<OpensearchDomainDomainEndpointOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_endpoint_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> ListRef<OpensearchDomainEbsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> ListRef<OpensearchDomainEncryptAtRestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encrypt_at_rest", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_center_options` after provisioning.\n"]
    pub fn identity_center_options(&self) -> ListRef<OpensearchDomainIdentityCenterOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_center_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> ListRef<OpensearchDomainNodeToNodeEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_to_node_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `off_peak_window_options` after provisioning.\n"]
    pub fn off_peak_window_options(&self) -> ListRef<OpensearchDomainOffPeakWindowOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.off_peak_window_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> ListRef<OpensearchDomainSnapshotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `software_update_options` after provisioning.\n"]
    pub fn software_update_options(&self) -> ListRef<OpensearchDomainSoftwareUpdateOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_update_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchDomainTimeoutsElRef {
        OpensearchDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> ListRef<OpensearchDomainVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}
impl Referable for OpensearchDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for OpensearchDomain {}
impl ToListMappable for OpensearchDomain {
    type O = ListRef<OpensearchDomainRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for OpensearchDomain_ {
    fn extract_resource_type(&self) -> String {
        "aws_opensearch_domain".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildOpensearchDomain {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}
impl BuildOpensearchDomain {
    pub fn build(self, stack: &mut Stack) -> OpensearchDomain {
        let out = OpensearchDomain(Rc::new(OpensearchDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpensearchDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_policies: core::default::Default::default(),
                advanced_options: core::default::Default::default(),
                domain_name: self.domain_name,
                engine_version: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_address_type: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                advanced_security_options: core::default::Default::default(),
                aiml_options: core::default::Default::default(),
                auto_tune_options: core::default::Default::default(),
                cluster_config: core::default::Default::default(),
                cognito_options: core::default::Default::default(),
                domain_endpoint_options: core::default::Default::default(),
                ebs_options: core::default::Default::default(),
                encrypt_at_rest: core::default::Default::default(),
                identity_center_options: core::default::Default::default(),
                log_publishing_options: core::default::Default::default(),
                node_to_node_encryption: core::default::Default::default(),
                off_peak_window_options: core::default::Default::default(),
                snapshot_options: core::default::Default::default(),
                software_update_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct OpensearchDomainRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl OpensearchDomainRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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
    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `advanced_security_options` after provisioning.\n"]
    pub fn advanced_security_options(
        &self,
    ) -> ListRef<OpensearchDomainAdvancedSecurityOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_security_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aiml_options` after provisioning.\n"]
    pub fn aiml_options(&self) -> ListRef<OpensearchDomainAimlOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.aiml_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> ListRef<OpensearchDomainAutoTuneOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_tune_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<OpensearchDomainClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> ListRef<OpensearchDomainCognitoOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cognito_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(&self) -> ListRef<OpensearchDomainDomainEndpointOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_endpoint_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> ListRef<OpensearchDomainEbsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> ListRef<OpensearchDomainEncryptAtRestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encrypt_at_rest", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_center_options` after provisioning.\n"]
    pub fn identity_center_options(&self) -> ListRef<OpensearchDomainIdentityCenterOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identity_center_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> ListRef<OpensearchDomainNodeToNodeEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_to_node_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `off_peak_window_options` after provisioning.\n"]
    pub fn off_peak_window_options(&self) -> ListRef<OpensearchDomainOffPeakWindowOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.off_peak_window_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> ListRef<OpensearchDomainSnapshotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `software_update_options` after provisioning.\n"]
    pub fn software_update_options(&self) -> ListRef<OpensearchDomainSoftwareUpdateOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_update_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpensearchDomainTimeoutsElRef {
        OpensearchDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> ListRef<OpensearchDomainVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_password: Option<PrimField<String>>,
}
impl OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    #[doc = "Set the field `master_user_arn`.\n"]
    pub fn set_master_user_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.master_user_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `master_user_name`.\n"]
    pub fn set_master_user_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.master_user_name = Some(v.into());
        self
    }
    #[doc = "Set the field `master_user_password`.\n"]
    pub fn set_master_user_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.master_user_password = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    type O = BlockAssignable<OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {}
impl BuildOpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    pub fn build(self) -> OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
        OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
            master_user_arn: core::default::Default::default(),
            master_user_name: core::default::Default::default(),
            master_user_password: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
        OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `master_user_arn` after provisioning.\n"]
    pub fn master_user_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_user_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_user_name` after provisioning.\n"]
    pub fn master_user_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_user_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `master_user_password` after provisioning.\n"]
    pub fn master_user_password(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.master_user_password", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OpensearchDomainAdvancedSecurityOptionsElDynamic {
    master_user_options:
        Option<DynamicBlock<OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>>,
}
#[derive(Serialize)]
pub struct OpensearchDomainAdvancedSecurityOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    anonymous_auth_enabled: Option<PrimField<bool>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_user_database_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_options: Option<Vec<OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>>,
    dynamic: OpensearchDomainAdvancedSecurityOptionsElDynamic,
}
impl OpensearchDomainAdvancedSecurityOptionsEl {
    #[doc = "Set the field `anonymous_auth_enabled`.\n"]
    pub fn set_anonymous_auth_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.anonymous_auth_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `internal_user_database_enabled`.\n"]
    pub fn set_internal_user_database_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.internal_user_database_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `master_user_options`.\n"]
    pub fn set_master_user_options(
        mut self,
        v: impl Into<BlockAssignable<OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.master_user_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.master_user_options = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OpensearchDomainAdvancedSecurityOptionsEl {
    type O = BlockAssignable<OpensearchDomainAdvancedSecurityOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainAdvancedSecurityOptionsEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildOpensearchDomainAdvancedSecurityOptionsEl {
    pub fn build(self) -> OpensearchDomainAdvancedSecurityOptionsEl {
        OpensearchDomainAdvancedSecurityOptionsEl {
            anonymous_auth_enabled: core::default::Default::default(),
            enabled: self.enabled,
            internal_user_database_enabled: core::default::Default::default(),
            master_user_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OpensearchDomainAdvancedSecurityOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainAdvancedSecurityOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainAdvancedSecurityOptionsElRef {
        OpensearchDomainAdvancedSecurityOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainAdvancedSecurityOptionsElRef {
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
    #[doc = "Get a reference to the value of field `master_user_options` after provisioning.\n"]
    pub fn master_user_options(
        &self,
    ) -> ListRef<OpensearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_user_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_state: Option<PrimField<String>>,
}
impl OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl {
    #[doc = "Set the field `desired_state`.\n"]
    pub fn set_desired_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.desired_state = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl {
    type O = BlockAssignable<OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl {}
impl BuildOpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl {
    pub fn build(self) -> OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl {
        OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl {
            desired_state: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsElRef {
        OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsElRef {
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
}
#[derive(Serialize)]
pub struct OpensearchDomainAimlOptionsElS3VectorsEngineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl OpensearchDomainAimlOptionsElS3VectorsEngineEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainAimlOptionsElS3VectorsEngineEl {
    type O = BlockAssignable<OpensearchDomainAimlOptionsElS3VectorsEngineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainAimlOptionsElS3VectorsEngineEl {}
impl BuildOpensearchDomainAimlOptionsElS3VectorsEngineEl {
    pub fn build(self) -> OpensearchDomainAimlOptionsElS3VectorsEngineEl {
        OpensearchDomainAimlOptionsElS3VectorsEngineEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainAimlOptionsElS3VectorsEngineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainAimlOptionsElS3VectorsEngineElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainAimlOptionsElS3VectorsEngineElRef {
        OpensearchDomainAimlOptionsElS3VectorsEngineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainAimlOptionsElS3VectorsEngineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct OpensearchDomainAimlOptionsElDynamic {
    natural_language_query_generation_options:
        Option<DynamicBlock<OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl>>,
    s3_vectors_engine: Option<DynamicBlock<OpensearchDomainAimlOptionsElS3VectorsEngineEl>>,
}
#[derive(Serialize)]
pub struct OpensearchDomainAimlOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    natural_language_query_generation_options:
        Option<Vec<OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_vectors_engine: Option<Vec<OpensearchDomainAimlOptionsElS3VectorsEngineEl>>,
    dynamic: OpensearchDomainAimlOptionsElDynamic,
}
impl OpensearchDomainAimlOptionsEl {
    #[doc = "Set the field `natural_language_query_generation_options`.\n"]
    pub fn set_natural_language_query_generation_options(
        mut self,
        v: impl Into<
            BlockAssignable<OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.natural_language_query_generation_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.natural_language_query_generation_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_vectors_engine`.\n"]
    pub fn set_s3_vectors_engine(
        mut self,
        v: impl Into<BlockAssignable<OpensearchDomainAimlOptionsElS3VectorsEngineEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_vectors_engine = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_vectors_engine = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OpensearchDomainAimlOptionsEl {
    type O = BlockAssignable<OpensearchDomainAimlOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainAimlOptionsEl {}
impl BuildOpensearchDomainAimlOptionsEl {
    pub fn build(self) -> OpensearchDomainAimlOptionsEl {
        OpensearchDomainAimlOptionsEl {
            natural_language_query_generation_options: core::default::Default::default(),
            s3_vectors_engine: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OpensearchDomainAimlOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainAimlOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainAimlOptionsElRef {
        OpensearchDomainAimlOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainAimlOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `natural_language_query_generation_options` after provisioning.\n"]
    pub fn natural_language_query_generation_options(
        &self,
    ) -> ListRef<OpensearchDomainAimlOptionsElNaturalLanguageQueryGenerationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.natural_language_query_generation_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_vectors_engine` after provisioning.\n"]
    pub fn s3_vectors_engine(&self) -> ListRef<OpensearchDomainAimlOptionsElS3VectorsEngineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_vectors_engine", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {}
impl ToListMappable for OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    type O = BlockAssignable<OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildOpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    pub fn build(self) -> OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
        OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
        OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
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
#[derive(Serialize, Default)]
struct OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDynamic {
    duration:
        Option<DynamicBlock<OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
}
#[derive(Serialize)]
pub struct OpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    cron_expression_for_recurrence: PrimField<String>,
    start_at: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Vec<OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
    dynamic: OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDynamic,
}
impl OpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v: impl Into<BlockAssignable<OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.duration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.duration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    type O = BlockAssignable<OpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[doc = ""]
    pub cron_expression_for_recurrence: PrimField<String>,
    #[doc = ""]
    pub start_at: PrimField<String>,
}
impl BuildOpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    pub fn build(self) -> OpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
        OpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
            cron_expression_for_recurrence: self.cron_expression_for_recurrence,
            start_at: self.start_at,
            duration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
        OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
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
    #[doc = "Get a reference to the value of field `start_at` after provisioning.\n"]
    pub fn start_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_at", self.base))
    }
    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(
        &self,
    ) -> ListRef<OpensearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }
}
#[derive(Serialize, Default)]
struct OpensearchDomainAutoTuneOptionsElDynamic {
    maintenance_schedule:
        Option<DynamicBlock<OpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
}
#[derive(Serialize)]
pub struct OpensearchDomainAutoTuneOptionsEl {
    desired_state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback_on_disable: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_off_peak_window: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_schedule: Option<Vec<OpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
    dynamic: OpensearchDomainAutoTuneOptionsElDynamic,
}
impl OpensearchDomainAutoTuneOptionsEl {
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
    #[doc = "Set the field `maintenance_schedule`.\n"]
    pub fn set_maintenance_schedule(
        mut self,
        v: impl Into<BlockAssignable<OpensearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.maintenance_schedule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_schedule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OpensearchDomainAutoTuneOptionsEl {
    type O = BlockAssignable<OpensearchDomainAutoTuneOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainAutoTuneOptionsEl {
    #[doc = ""]
    pub desired_state: PrimField<String>,
}
impl BuildOpensearchDomainAutoTuneOptionsEl {
    pub fn build(self) -> OpensearchDomainAutoTuneOptionsEl {
        OpensearchDomainAutoTuneOptionsEl {
            desired_state: self.desired_state,
            rollback_on_disable: core::default::Default::default(),
            use_off_peak_window: core::default::Default::default(),
            maintenance_schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OpensearchDomainAutoTuneOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainAutoTuneOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainAutoTuneOptionsElRef {
        OpensearchDomainAutoTuneOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainAutoTuneOptionsElRef {
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
pub struct OpensearchDomainClusterConfigElColdStorageOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl OpensearchDomainClusterConfigElColdStorageOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainClusterConfigElColdStorageOptionsEl {
    type O = BlockAssignable<OpensearchDomainClusterConfigElColdStorageOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainClusterConfigElColdStorageOptionsEl {}
impl BuildOpensearchDomainClusterConfigElColdStorageOptionsEl {
    pub fn build(self) -> OpensearchDomainClusterConfigElColdStorageOptionsEl {
        OpensearchDomainClusterConfigElColdStorageOptionsEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainClusterConfigElColdStorageOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainClusterConfigElColdStorageOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchDomainClusterConfigElColdStorageOptionsElRef {
        OpensearchDomainClusterConfigElColdStorageOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainClusterConfigElColdStorageOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct OpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl OpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
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
impl ToListMappable for OpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
    type O = BlockAssignable<OpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {}
impl BuildOpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
    pub fn build(self) -> OpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
        OpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl {
            count: core::default::Default::default(),
            enabled: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
        OpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef {
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
#[derive(Serialize, Default)]
struct OpensearchDomainClusterConfigElNodeOptionsElDynamic {
    node_config: Option<DynamicBlock<OpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl>>,
}
#[derive(Serialize)]
pub struct OpensearchDomainClusterConfigElNodeOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<Vec<OpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl>>,
    dynamic: OpensearchDomainClusterConfigElNodeOptionsElDynamic,
}
impl OpensearchDomainClusterConfigElNodeOptionsEl {
    #[doc = "Set the field `node_type`.\n"]
    pub fn set_node_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_type = Some(v.into());
        self
    }
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        mut self,
        v: impl Into<BlockAssignable<OpensearchDomainClusterConfigElNodeOptionsElNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OpensearchDomainClusterConfigElNodeOptionsEl {
    type O = BlockAssignable<OpensearchDomainClusterConfigElNodeOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainClusterConfigElNodeOptionsEl {}
impl BuildOpensearchDomainClusterConfigElNodeOptionsEl {
    pub fn build(self) -> OpensearchDomainClusterConfigElNodeOptionsEl {
        OpensearchDomainClusterConfigElNodeOptionsEl {
            node_type: core::default::Default::default(),
            node_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OpensearchDomainClusterConfigElNodeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainClusterConfigElNodeOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainClusterConfigElNodeOptionsElRef {
        OpensearchDomainClusterConfigElNodeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainClusterConfigElNodeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(
        &self,
    ) -> ListRef<OpensearchDomainClusterConfigElNodeOptionsElNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.base))
    }
}
#[derive(Serialize)]
pub struct OpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_count: Option<PrimField<f64>>,
}
impl OpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[doc = "Set the field `availability_zone_count`.\n"]
    pub fn set_availability_zone_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.availability_zone_count = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    type O = BlockAssignable<OpensearchDomainClusterConfigElZoneAwarenessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainClusterConfigElZoneAwarenessConfigEl {}
impl BuildOpensearchDomainClusterConfigElZoneAwarenessConfigEl {
    pub fn build(self) -> OpensearchDomainClusterConfigElZoneAwarenessConfigEl {
        OpensearchDomainClusterConfigElZoneAwarenessConfigEl {
            availability_zone_count: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
        OpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainClusterConfigElZoneAwarenessConfigElRef {
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
#[derive(Serialize, Default)]
struct OpensearchDomainClusterConfigElDynamic {
    cold_storage_options: Option<DynamicBlock<OpensearchDomainClusterConfigElColdStorageOptionsEl>>,
    node_options: Option<DynamicBlock<OpensearchDomainClusterConfigElNodeOptionsEl>>,
    zone_awareness_config:
        Option<DynamicBlock<OpensearchDomainClusterConfigElZoneAwarenessConfigEl>>,
}
#[derive(Serialize)]
pub struct OpensearchDomainClusterConfigEl {
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
    warm_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warm_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warm_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_awareness_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cold_storage_options: Option<Vec<OpensearchDomainClusterConfigElColdStorageOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_options: Option<Vec<OpensearchDomainClusterConfigElNodeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_awareness_config: Option<Vec<OpensearchDomainClusterConfigElZoneAwarenessConfigEl>>,
    dynamic: OpensearchDomainClusterConfigElDynamic,
}
impl OpensearchDomainClusterConfigEl {
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
    #[doc = "Set the field `zone_awareness_enabled`.\n"]
    pub fn set_zone_awareness_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.zone_awareness_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `cold_storage_options`.\n"]
    pub fn set_cold_storage_options(
        mut self,
        v: impl Into<BlockAssignable<OpensearchDomainClusterConfigElColdStorageOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cold_storage_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cold_storage_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_options`.\n"]
    pub fn set_node_options(
        mut self,
        v: impl Into<BlockAssignable<OpensearchDomainClusterConfigElNodeOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `zone_awareness_config`.\n"]
    pub fn set_zone_awareness_config(
        mut self,
        v: impl Into<BlockAssignable<OpensearchDomainClusterConfigElZoneAwarenessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.zone_awareness_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.zone_awareness_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OpensearchDomainClusterConfigEl {
    type O = BlockAssignable<OpensearchDomainClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainClusterConfigEl {}
impl BuildOpensearchDomainClusterConfigEl {
    pub fn build(self) -> OpensearchDomainClusterConfigEl {
        OpensearchDomainClusterConfigEl {
            dedicated_master_count: core::default::Default::default(),
            dedicated_master_enabled: core::default::Default::default(),
            dedicated_master_type: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            multi_az_with_standby_enabled: core::default::Default::default(),
            warm_count: core::default::Default::default(),
            warm_enabled: core::default::Default::default(),
            warm_type: core::default::Default::default(),
            zone_awareness_enabled: core::default::Default::default(),
            cold_storage_options: core::default::Default::default(),
            node_options: core::default::Default::default(),
            zone_awareness_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OpensearchDomainClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainClusterConfigElRef {
        OpensearchDomainClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `zone_awareness_enabled` after provisioning.\n"]
    pub fn zone_awareness_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone_awareness_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cold_storage_options` after provisioning.\n"]
    pub fn cold_storage_options(
        &self,
    ) -> ListRef<OpensearchDomainClusterConfigElColdStorageOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cold_storage_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_options` after provisioning.\n"]
    pub fn node_options(&self) -> ListRef<OpensearchDomainClusterConfigElNodeOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_options", self.base))
    }
    #[doc = "Get a reference to the value of field `zone_awareness_config` after provisioning.\n"]
    pub fn zone_awareness_config(
        &self,
    ) -> ListRef<OpensearchDomainClusterConfigElZoneAwarenessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_awareness_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OpensearchDomainCognitoOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    identity_pool_id: PrimField<String>,
    role_arn: PrimField<String>,
    user_pool_id: PrimField<String>,
}
impl OpensearchDomainCognitoOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainCognitoOptionsEl {
    type O = BlockAssignable<OpensearchDomainCognitoOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainCognitoOptionsEl {
    #[doc = ""]
    pub identity_pool_id: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}
impl BuildOpensearchDomainCognitoOptionsEl {
    pub fn build(self) -> OpensearchDomainCognitoOptionsEl {
        OpensearchDomainCognitoOptionsEl {
            enabled: core::default::Default::default(),
            identity_pool_id: self.identity_pool_id,
            role_arn: self.role_arn,
            user_pool_id: self.user_pool_id,
        }
    }
}
pub struct OpensearchDomainCognitoOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainCognitoOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainCognitoOptionsElRef {
        OpensearchDomainCognitoOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainCognitoOptionsElRef {
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
pub struct OpensearchDomainDomainEndpointOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_endpoint_certificate_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_endpoint_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_https: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_security_policy: Option<PrimField<String>>,
}
impl OpensearchDomainDomainEndpointOptionsEl {
    #[doc = "Set the field `custom_endpoint`.\n"]
    pub fn set_custom_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_endpoint_certificate_arn`.\n"]
    pub fn set_custom_endpoint_certificate_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_endpoint_certificate_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_endpoint_enabled`.\n"]
    pub fn set_custom_endpoint_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.custom_endpoint_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `enforce_https`.\n"]
    pub fn set_enforce_https(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce_https = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_security_policy`.\n"]
    pub fn set_tls_security_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tls_security_policy = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainDomainEndpointOptionsEl {
    type O = BlockAssignable<OpensearchDomainDomainEndpointOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainDomainEndpointOptionsEl {}
impl BuildOpensearchDomainDomainEndpointOptionsEl {
    pub fn build(self) -> OpensearchDomainDomainEndpointOptionsEl {
        OpensearchDomainDomainEndpointOptionsEl {
            custom_endpoint: core::default::Default::default(),
            custom_endpoint_certificate_arn: core::default::Default::default(),
            custom_endpoint_enabled: core::default::Default::default(),
            enforce_https: core::default::Default::default(),
            tls_security_policy: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainDomainEndpointOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainDomainEndpointOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainDomainEndpointOptionsElRef {
        OpensearchDomainDomainEndpointOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainDomainEndpointOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_endpoint` after provisioning.\n"]
    pub fn custom_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_endpoint_certificate_arn` after provisioning.\n"]
    pub fn custom_endpoint_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_endpoint_certificate_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_endpoint_enabled` after provisioning.\n"]
    pub fn custom_endpoint_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_endpoint_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforce_https` after provisioning.\n"]
    pub fn enforce_https(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_https", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_security_policy` after provisioning.\n"]
    pub fn tls_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tls_security_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OpensearchDomainEbsOptionsEl {
    ebs_enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_type: Option<PrimField<String>>,
}
impl OpensearchDomainEbsOptionsEl {
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
impl ToListMappable for OpensearchDomainEbsOptionsEl {
    type O = BlockAssignable<OpensearchDomainEbsOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainEbsOptionsEl {
    #[doc = ""]
    pub ebs_enabled: PrimField<bool>,
}
impl BuildOpensearchDomainEbsOptionsEl {
    pub fn build(self) -> OpensearchDomainEbsOptionsEl {
        OpensearchDomainEbsOptionsEl {
            ebs_enabled: self.ebs_enabled,
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainEbsOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainEbsOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainEbsOptionsElRef {
        OpensearchDomainEbsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainEbsOptionsElRef {
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
pub struct OpensearchDomainEncryptAtRestEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}
impl OpensearchDomainEncryptAtRestEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainEncryptAtRestEl {
    type O = BlockAssignable<OpensearchDomainEncryptAtRestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainEncryptAtRestEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildOpensearchDomainEncryptAtRestEl {
    pub fn build(self) -> OpensearchDomainEncryptAtRestEl {
        OpensearchDomainEncryptAtRestEl {
            enabled: self.enabled,
            kms_key_id: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainEncryptAtRestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainEncryptAtRestElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainEncryptAtRestElRef {
        OpensearchDomainEncryptAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainEncryptAtRestElRef {
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
pub struct OpensearchDomainIdentityCenterOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled_api_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_center_instance_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subject_key: Option<PrimField<String>>,
}
impl OpensearchDomainIdentityCenterOptionsEl {
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
impl ToListMappable for OpensearchDomainIdentityCenterOptionsEl {
    type O = BlockAssignable<OpensearchDomainIdentityCenterOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainIdentityCenterOptionsEl {}
impl BuildOpensearchDomainIdentityCenterOptionsEl {
    pub fn build(self) -> OpensearchDomainIdentityCenterOptionsEl {
        OpensearchDomainIdentityCenterOptionsEl {
            enabled_api_access: core::default::Default::default(),
            identity_center_instance_arn: core::default::Default::default(),
            roles_key: core::default::Default::default(),
            subject_key: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainIdentityCenterOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainIdentityCenterOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainIdentityCenterOptionsElRef {
        OpensearchDomainIdentityCenterOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainIdentityCenterOptionsElRef {
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
pub struct OpensearchDomainLogPublishingOptionsEl {
    cloudwatch_log_group_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    log_type: PrimField<String>,
}
impl OpensearchDomainLogPublishingOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainLogPublishingOptionsEl {
    type O = BlockAssignable<OpensearchDomainLogPublishingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainLogPublishingOptionsEl {
    #[doc = ""]
    pub cloudwatch_log_group_arn: PrimField<String>,
    #[doc = ""]
    pub log_type: PrimField<String>,
}
impl BuildOpensearchDomainLogPublishingOptionsEl {
    pub fn build(self) -> OpensearchDomainLogPublishingOptionsEl {
        OpensearchDomainLogPublishingOptionsEl {
            cloudwatch_log_group_arn: self.cloudwatch_log_group_arn,
            enabled: core::default::Default::default(),
            log_type: self.log_type,
        }
    }
}
pub struct OpensearchDomainLogPublishingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainLogPublishingOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainLogPublishingOptionsElRef {
        OpensearchDomainLogPublishingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainLogPublishingOptionsElRef {
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
pub struct OpensearchDomainNodeToNodeEncryptionEl {
    enabled: PrimField<bool>,
}
impl OpensearchDomainNodeToNodeEncryptionEl {}
impl ToListMappable for OpensearchDomainNodeToNodeEncryptionEl {
    type O = BlockAssignable<OpensearchDomainNodeToNodeEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainNodeToNodeEncryptionEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildOpensearchDomainNodeToNodeEncryptionEl {
    pub fn build(self) -> OpensearchDomainNodeToNodeEncryptionEl {
        OpensearchDomainNodeToNodeEncryptionEl {
            enabled: self.enabled,
        }
    }
}
pub struct OpensearchDomainNodeToNodeEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainNodeToNodeEncryptionElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainNodeToNodeEncryptionElRef {
        OpensearchDomainNodeToNodeEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainNodeToNodeEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minutes: Option<PrimField<f64>>,
}
impl OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
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
impl ToListMappable for OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
    type O =
        BlockAssignable<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {}
impl BuildOpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
    pub fn build(self) -> OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
        OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
        OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef {
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
#[derive(Serialize, Default)]
struct OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElDynamic {
    window_start_time: Option<
        DynamicBlock<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl>,
    >,
}
#[derive(Serialize)]
pub struct OpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    window_start_time:
        Option<Vec<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl>>,
    dynamic: OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElDynamic,
}
impl OpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
    #[doc = "Set the field `window_start_time`.\n"]
    pub fn set_window_start_time(
        mut self,
        v: impl Into<
            BlockAssignable<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.window_start_time = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.window_start_time = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
    type O = BlockAssignable<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {}
impl BuildOpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
    pub fn build(self) -> OpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
        OpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl {
            window_start_time: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
        OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `window_start_time` after provisioning.\n"]
    pub fn window_start_time(
        &self,
    ) -> ListRef<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElWindowStartTimeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.window_start_time", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct OpensearchDomainOffPeakWindowOptionsElDynamic {
    off_peak_window: Option<DynamicBlock<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl>>,
}
#[derive(Serialize)]
pub struct OpensearchDomainOffPeakWindowOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    off_peak_window: Option<Vec<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl>>,
    dynamic: OpensearchDomainOffPeakWindowOptionsElDynamic,
}
impl OpensearchDomainOffPeakWindowOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `off_peak_window`.\n"]
    pub fn set_off_peak_window(
        mut self,
        v: impl Into<BlockAssignable<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.off_peak_window = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.off_peak_window = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for OpensearchDomainOffPeakWindowOptionsEl {
    type O = BlockAssignable<OpensearchDomainOffPeakWindowOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainOffPeakWindowOptionsEl {}
impl BuildOpensearchDomainOffPeakWindowOptionsEl {
    pub fn build(self) -> OpensearchDomainOffPeakWindowOptionsEl {
        OpensearchDomainOffPeakWindowOptionsEl {
            enabled: core::default::Default::default(),
            off_peak_window: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct OpensearchDomainOffPeakWindowOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainOffPeakWindowOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainOffPeakWindowOptionsElRef {
        OpensearchDomainOffPeakWindowOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainOffPeakWindowOptionsElRef {
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
    ) -> ListRef<OpensearchDomainOffPeakWindowOptionsElOffPeakWindowElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.off_peak_window", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct OpensearchDomainSnapshotOptionsEl {
    automated_snapshot_start_hour: PrimField<f64>,
}
impl OpensearchDomainSnapshotOptionsEl {}
impl ToListMappable for OpensearchDomainSnapshotOptionsEl {
    type O = BlockAssignable<OpensearchDomainSnapshotOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainSnapshotOptionsEl {
    #[doc = ""]
    pub automated_snapshot_start_hour: PrimField<f64>,
}
impl BuildOpensearchDomainSnapshotOptionsEl {
    pub fn build(self) -> OpensearchDomainSnapshotOptionsEl {
        OpensearchDomainSnapshotOptionsEl {
            automated_snapshot_start_hour: self.automated_snapshot_start_hour,
        }
    }
}
pub struct OpensearchDomainSnapshotOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainSnapshotOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainSnapshotOptionsElRef {
        OpensearchDomainSnapshotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainSnapshotOptionsElRef {
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
pub struct OpensearchDomainSoftwareUpdateOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_software_update_enabled: Option<PrimField<bool>>,
}
impl OpensearchDomainSoftwareUpdateOptionsEl {
    #[doc = "Set the field `auto_software_update_enabled`.\n"]
    pub fn set_auto_software_update_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_software_update_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for OpensearchDomainSoftwareUpdateOptionsEl {
    type O = BlockAssignable<OpensearchDomainSoftwareUpdateOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainSoftwareUpdateOptionsEl {}
impl BuildOpensearchDomainSoftwareUpdateOptionsEl {
    pub fn build(self) -> OpensearchDomainSoftwareUpdateOptionsEl {
        OpensearchDomainSoftwareUpdateOptionsEl {
            auto_software_update_enabled: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainSoftwareUpdateOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainSoftwareUpdateOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainSoftwareUpdateOptionsElRef {
        OpensearchDomainSoftwareUpdateOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainSoftwareUpdateOptionsElRef {
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
pub struct OpensearchDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl OpensearchDomainTimeoutsEl {
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
impl ToListMappable for OpensearchDomainTimeoutsEl {
    type O = BlockAssignable<OpensearchDomainTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainTimeoutsEl {}
impl BuildOpensearchDomainTimeoutsEl {
    pub fn build(self) -> OpensearchDomainTimeoutsEl {
        OpensearchDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainTimeoutsElRef {
        OpensearchDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainTimeoutsElRef {
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
pub struct OpensearchDomainVpcOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
}
impl OpensearchDomainVpcOptionsEl {
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
}
impl ToListMappable for OpensearchDomainVpcOptionsEl {
    type O = BlockAssignable<OpensearchDomainVpcOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildOpensearchDomainVpcOptionsEl {}
impl BuildOpensearchDomainVpcOptionsEl {
    pub fn build(self) -> OpensearchDomainVpcOptionsEl {
        OpensearchDomainVpcOptionsEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}
pub struct OpensearchDomainVpcOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for OpensearchDomainVpcOptionsElRef {
    fn new(shared: StackShared, base: String) -> OpensearchDomainVpcOptionsElRef {
        OpensearchDomainVpcOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl OpensearchDomainVpcOptionsElRef {
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
#[derive(Serialize, Default)]
struct OpensearchDomainDynamic {
    advanced_security_options: Option<DynamicBlock<OpensearchDomainAdvancedSecurityOptionsEl>>,
    aiml_options: Option<DynamicBlock<OpensearchDomainAimlOptionsEl>>,
    auto_tune_options: Option<DynamicBlock<OpensearchDomainAutoTuneOptionsEl>>,
    cluster_config: Option<DynamicBlock<OpensearchDomainClusterConfigEl>>,
    cognito_options: Option<DynamicBlock<OpensearchDomainCognitoOptionsEl>>,
    domain_endpoint_options: Option<DynamicBlock<OpensearchDomainDomainEndpointOptionsEl>>,
    ebs_options: Option<DynamicBlock<OpensearchDomainEbsOptionsEl>>,
    encrypt_at_rest: Option<DynamicBlock<OpensearchDomainEncryptAtRestEl>>,
    identity_center_options: Option<DynamicBlock<OpensearchDomainIdentityCenterOptionsEl>>,
    log_publishing_options: Option<DynamicBlock<OpensearchDomainLogPublishingOptionsEl>>,
    node_to_node_encryption: Option<DynamicBlock<OpensearchDomainNodeToNodeEncryptionEl>>,
    off_peak_window_options: Option<DynamicBlock<OpensearchDomainOffPeakWindowOptionsEl>>,
    snapshot_options: Option<DynamicBlock<OpensearchDomainSnapshotOptionsEl>>,
    software_update_options: Option<DynamicBlock<OpensearchDomainSoftwareUpdateOptionsEl>>,
    vpc_options: Option<DynamicBlock<OpensearchDomainVpcOptionsEl>>,
}
