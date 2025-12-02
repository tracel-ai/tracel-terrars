use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ElasticsearchDomainData {
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
    elasticsearch_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_security_options: Option<Vec<ElasticsearchDomainAdvancedSecurityOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_tune_options: Option<Vec<ElasticsearchDomainAutoTuneOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cluster_config: Option<Vec<ElasticsearchDomainClusterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cognito_options: Option<Vec<ElasticsearchDomainCognitoOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_endpoint_options: Option<Vec<ElasticsearchDomainDomainEndpointOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_options: Option<Vec<ElasticsearchDomainEbsOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypt_at_rest: Option<Vec<ElasticsearchDomainEncryptAtRestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_publishing_options: Option<Vec<ElasticsearchDomainLogPublishingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_to_node_encryption: Option<Vec<ElasticsearchDomainNodeToNodeEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snapshot_options: Option<Vec<ElasticsearchDomainSnapshotOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ElasticsearchDomainTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_options: Option<Vec<ElasticsearchDomainVpcOptionsEl>>,
    dynamic: ElasticsearchDomainDynamic,
}
struct ElasticsearchDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ElasticsearchDomainData>,
}
#[derive(Clone)]
pub struct ElasticsearchDomain(Rc<ElasticsearchDomain_>);
impl ElasticsearchDomain {
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
    #[doc = "Set the field `elasticsearch_version`.\n"]
    pub fn set_elasticsearch_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().elasticsearch_version = Some(v.into());
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
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_security_options`.\n"]
    pub fn set_advanced_security_options(
        self,
        v: impl Into<BlockAssignable<ElasticsearchDomainAdvancedSecurityOptionsEl>>,
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
    #[doc = "Set the field `auto_tune_options`.\n"]
    pub fn set_auto_tune_options(
        self,
        v: impl Into<BlockAssignable<ElasticsearchDomainAutoTuneOptionsEl>>,
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
        v: impl Into<BlockAssignable<ElasticsearchDomainClusterConfigEl>>,
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
        v: impl Into<BlockAssignable<ElasticsearchDomainCognitoOptionsEl>>,
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
        v: impl Into<BlockAssignable<ElasticsearchDomainDomainEndpointOptionsEl>>,
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
        v: impl Into<BlockAssignable<ElasticsearchDomainEbsOptionsEl>>,
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
        v: impl Into<BlockAssignable<ElasticsearchDomainEncryptAtRestEl>>,
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
    #[doc = "Set the field `log_publishing_options`.\n"]
    pub fn set_log_publishing_options(
        self,
        v: impl Into<BlockAssignable<ElasticsearchDomainLogPublishingOptionsEl>>,
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
        v: impl Into<BlockAssignable<ElasticsearchDomainNodeToNodeEncryptionEl>>,
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
    #[doc = "Set the field `snapshot_options`.\n"]
    pub fn set_snapshot_options(
        self,
        v: impl Into<BlockAssignable<ElasticsearchDomainSnapshotOptionsEl>>,
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ElasticsearchDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_options`.\n"]
    pub fn set_vpc_options(
        self,
        v: impl Into<BlockAssignable<ElasticsearchDomainVpcOptionsEl>>,
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
    #[doc = "Get a reference to the value of field `elasticsearch_version` after provisioning.\n"]
    pub fn elasticsearch_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.elasticsearch_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kibana_endpoint` after provisioning.\n"]
    pub fn kibana_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kibana_endpoint", self.extract_ref()),
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
    ) -> ListRef<ElasticsearchDomainAdvancedSecurityOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_security_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> ListRef<ElasticsearchDomainAutoTuneOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_tune_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<ElasticsearchDomainClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> ListRef<ElasticsearchDomainCognitoOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cognito_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(
        &self,
    ) -> ListRef<ElasticsearchDomainDomainEndpointOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_endpoint_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> ListRef<ElasticsearchDomainEbsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> ListRef<ElasticsearchDomainEncryptAtRestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encrypt_at_rest", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> ListRef<ElasticsearchDomainNodeToNodeEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_to_node_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> ListRef<ElasticsearchDomainSnapshotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomainTimeoutsElRef {
        ElasticsearchDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> ListRef<ElasticsearchDomainVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}
impl Referable for ElasticsearchDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ElasticsearchDomain {}
impl ToListMappable for ElasticsearchDomain {
    type O = ListRef<ElasticsearchDomainRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ElasticsearchDomain_ {
    fn extract_resource_type(&self) -> String {
        "aws_elasticsearch_domain".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildElasticsearchDomain {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}
impl BuildElasticsearchDomain {
    pub fn build(self, stack: &mut Stack) -> ElasticsearchDomain {
        let out = ElasticsearchDomain(Rc::new(ElasticsearchDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ElasticsearchDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_policies: core::default::Default::default(),
                advanced_options: core::default::Default::default(),
                domain_name: self.domain_name,
                elasticsearch_version: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                advanced_security_options: core::default::Default::default(),
                auto_tune_options: core::default::Default::default(),
                cluster_config: core::default::Default::default(),
                cognito_options: core::default::Default::default(),
                domain_endpoint_options: core::default::Default::default(),
                ebs_options: core::default::Default::default(),
                encrypt_at_rest: core::default::Default::default(),
                log_publishing_options: core::default::Default::default(),
                node_to_node_encryption: core::default::Default::default(),
                snapshot_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ElasticsearchDomainRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ElasticsearchDomainRef {
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
    #[doc = "Get a reference to the value of field `elasticsearch_version` after provisioning.\n"]
    pub fn elasticsearch_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.elasticsearch_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kibana_endpoint` after provisioning.\n"]
    pub fn kibana_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kibana_endpoint", self.extract_ref()),
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
    ) -> ListRef<ElasticsearchDomainAdvancedSecurityOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_security_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_tune_options` after provisioning.\n"]
    pub fn auto_tune_options(&self) -> ListRef<ElasticsearchDomainAutoTuneOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_tune_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<ElasticsearchDomainClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> ListRef<ElasticsearchDomainCognitoOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cognito_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_endpoint_options` after provisioning.\n"]
    pub fn domain_endpoint_options(
        &self,
    ) -> ListRef<ElasticsearchDomainDomainEndpointOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_endpoint_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ebs_options` after provisioning.\n"]
    pub fn ebs_options(&self) -> ListRef<ElasticsearchDomainEbsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encrypt_at_rest` after provisioning.\n"]
    pub fn encrypt_at_rest(&self) -> ListRef<ElasticsearchDomainEncryptAtRestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encrypt_at_rest", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(&self) -> ListRef<ElasticsearchDomainNodeToNodeEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_to_node_encryption", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `snapshot_options` after provisioning.\n"]
    pub fn snapshot_options(&self) -> ListRef<ElasticsearchDomainSnapshotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ElasticsearchDomainTimeoutsElRef {
        ElasticsearchDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_options` after provisioning.\n"]
    pub fn vpc_options(&self) -> ListRef<ElasticsearchDomainVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_password: Option<PrimField<String>>,
}
impl ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
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
impl ToListMappable for ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {}
impl BuildElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
    pub fn build(self) -> ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
        ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl {
            master_user_arn: core::default::Default::default(),
            master_user_name: core::default::Default::default(),
            master_user_password: core::default::Default::default(),
        }
    }
}
pub struct ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
        ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef {
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
struct ElasticsearchDomainAdvancedSecurityOptionsElDynamic {
    master_user_options:
        Option<DynamicBlock<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>>,
}
#[derive(Serialize)]
pub struct ElasticsearchDomainAdvancedSecurityOptionsEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_user_database_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_user_options:
        Option<Vec<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>>,
    dynamic: ElasticsearchDomainAdvancedSecurityOptionsElDynamic,
}
impl ElasticsearchDomainAdvancedSecurityOptionsEl {
    #[doc = "Set the field `internal_user_database_enabled`.\n"]
    pub fn set_internal_user_database_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.internal_user_database_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `master_user_options`.\n"]
    pub fn set_master_user_options(
        mut self,
        v: impl Into<BlockAssignable<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsEl>>,
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
impl ToListMappable for ElasticsearchDomainAdvancedSecurityOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainAdvancedSecurityOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainAdvancedSecurityOptionsEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildElasticsearchDomainAdvancedSecurityOptionsEl {
    pub fn build(self) -> ElasticsearchDomainAdvancedSecurityOptionsEl {
        ElasticsearchDomainAdvancedSecurityOptionsEl {
            enabled: self.enabled,
            internal_user_database_enabled: core::default::Default::default(),
            master_user_options: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ElasticsearchDomainAdvancedSecurityOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainAdvancedSecurityOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainAdvancedSecurityOptionsElRef {
        ElasticsearchDomainAdvancedSecurityOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainAdvancedSecurityOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
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
    ) -> ListRef<ElasticsearchDomainAdvancedSecurityOptionsElMasterUserOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.master_user_options", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    unit: PrimField<String>,
    value: PrimField<f64>,
}
impl ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {}
impl ToListMappable for ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    type O = BlockAssignable<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    #[doc = ""]
    pub unit: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<f64>,
}
impl BuildElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    pub fn build(self) -> ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
        ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
            unit: self.unit,
            value: self.value,
        }
    }
}
pub struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
        ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
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
struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDynamic {
    duration:
        Option<DynamicBlock<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
}
#[derive(Serialize)]
pub struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    cron_expression_for_recurrence: PrimField<String>,
    start_at: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Vec<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
    dynamic: ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDynamic,
}
impl ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v: impl Into<
            BlockAssignable<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>,
        >,
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
impl ToListMappable for ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    type O = BlockAssignable<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[doc = ""]
    pub cron_expression_for_recurrence: PrimField<String>,
    #[doc = ""]
    pub start_at: PrimField<String>,
}
impl BuildElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    pub fn build(self) -> ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
        ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
            cron_expression_for_recurrence: self.cron_expression_for_recurrence,
            start_at: self.start_at,
            duration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
        ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
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
    ) -> ListRef<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }
}
#[derive(Serialize, Default)]
struct ElasticsearchDomainAutoTuneOptionsElDynamic {
    maintenance_schedule:
        Option<DynamicBlock<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
}
#[derive(Serialize)]
pub struct ElasticsearchDomainAutoTuneOptionsEl {
    desired_state: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback_on_disable: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_schedule: Option<Vec<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
    dynamic: ElasticsearchDomainAutoTuneOptionsElDynamic,
}
impl ElasticsearchDomainAutoTuneOptionsEl {
    #[doc = "Set the field `rollback_on_disable`.\n"]
    pub fn set_rollback_on_disable(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rollback_on_disable = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_schedule`.\n"]
    pub fn set_maintenance_schedule(
        mut self,
        v: impl Into<BlockAssignable<ElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
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
impl ToListMappable for ElasticsearchDomainAutoTuneOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainAutoTuneOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainAutoTuneOptionsEl {
    #[doc = ""]
    pub desired_state: PrimField<String>,
}
impl BuildElasticsearchDomainAutoTuneOptionsEl {
    pub fn build(self) -> ElasticsearchDomainAutoTuneOptionsEl {
        ElasticsearchDomainAutoTuneOptionsEl {
            desired_state: self.desired_state,
            rollback_on_disable: core::default::Default::default(),
            maintenance_schedule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ElasticsearchDomainAutoTuneOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainAutoTuneOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainAutoTuneOptionsElRef {
        ElasticsearchDomainAutoTuneOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainAutoTuneOptionsElRef {
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
}
#[derive(Serialize)]
pub struct ElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl ElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for ElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainClusterConfigElColdStorageOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainClusterConfigElColdStorageOptionsEl {}
impl BuildElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    pub fn build(self) -> ElasticsearchDomainClusterConfigElColdStorageOptionsEl {
        ElasticsearchDomainClusterConfigElColdStorageOptionsEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
        ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_count: Option<PrimField<f64>>,
}
impl ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[doc = "Set the field `availability_zone_count`.\n"]
    pub fn set_availability_zone_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.availability_zone_count = Some(v.into());
        self
    }
}
impl ToListMappable for ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    type O = BlockAssignable<ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {}
impl BuildElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    pub fn build(self) -> ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
        ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
            availability_zone_count: core::default::Default::default(),
        }
    }
}
pub struct ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
        ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
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
struct ElasticsearchDomainClusterConfigElDynamic {
    cold_storage_options:
        Option<DynamicBlock<ElasticsearchDomainClusterConfigElColdStorageOptionsEl>>,
    zone_awareness_config:
        Option<DynamicBlock<ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>>,
}
#[derive(Serialize)]
pub struct ElasticsearchDomainClusterConfigEl {
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
    warm_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warm_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warm_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_awareness_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cold_storage_options: Option<Vec<ElasticsearchDomainClusterConfigElColdStorageOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_awareness_config: Option<Vec<ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>>,
    dynamic: ElasticsearchDomainClusterConfigElDynamic,
}
impl ElasticsearchDomainClusterConfigEl {
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
        v: impl Into<BlockAssignable<ElasticsearchDomainClusterConfigElColdStorageOptionsEl>>,
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
    #[doc = "Set the field `zone_awareness_config`.\n"]
    pub fn set_zone_awareness_config(
        mut self,
        v: impl Into<BlockAssignable<ElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>>,
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
impl ToListMappable for ElasticsearchDomainClusterConfigEl {
    type O = BlockAssignable<ElasticsearchDomainClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainClusterConfigEl {}
impl BuildElasticsearchDomainClusterConfigEl {
    pub fn build(self) -> ElasticsearchDomainClusterConfigEl {
        ElasticsearchDomainClusterConfigEl {
            dedicated_master_count: core::default::Default::default(),
            dedicated_master_enabled: core::default::Default::default(),
            dedicated_master_type: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            warm_count: core::default::Default::default(),
            warm_enabled: core::default::Default::default(),
            warm_type: core::default::Default::default(),
            zone_awareness_enabled: core::default::Default::default(),
            cold_storage_options: core::default::Default::default(),
            zone_awareness_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ElasticsearchDomainClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainClusterConfigElRef {
        ElasticsearchDomainClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainClusterConfigElRef {
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
    ) -> ListRef<ElasticsearchDomainClusterConfigElColdStorageOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cold_storage_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `zone_awareness_config` after provisioning.\n"]
    pub fn zone_awareness_config(
        &self,
    ) -> ListRef<ElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_awareness_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ElasticsearchDomainCognitoOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    identity_pool_id: PrimField<String>,
    role_arn: PrimField<String>,
    user_pool_id: PrimField<String>,
}
impl ElasticsearchDomainCognitoOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for ElasticsearchDomainCognitoOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainCognitoOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainCognitoOptionsEl {
    #[doc = ""]
    pub identity_pool_id: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub user_pool_id: PrimField<String>,
}
impl BuildElasticsearchDomainCognitoOptionsEl {
    pub fn build(self) -> ElasticsearchDomainCognitoOptionsEl {
        ElasticsearchDomainCognitoOptionsEl {
            enabled: core::default::Default::default(),
            identity_pool_id: self.identity_pool_id,
            role_arn: self.role_arn,
            user_pool_id: self.user_pool_id,
        }
    }
}
pub struct ElasticsearchDomainCognitoOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainCognitoOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainCognitoOptionsElRef {
        ElasticsearchDomainCognitoOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainCognitoOptionsElRef {
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
pub struct ElasticsearchDomainDomainEndpointOptionsEl {
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
impl ElasticsearchDomainDomainEndpointOptionsEl {
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
impl ToListMappable for ElasticsearchDomainDomainEndpointOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainDomainEndpointOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainDomainEndpointOptionsEl {}
impl BuildElasticsearchDomainDomainEndpointOptionsEl {
    pub fn build(self) -> ElasticsearchDomainDomainEndpointOptionsEl {
        ElasticsearchDomainDomainEndpointOptionsEl {
            custom_endpoint: core::default::Default::default(),
            custom_endpoint_certificate_arn: core::default::Default::default(),
            custom_endpoint_enabled: core::default::Default::default(),
            enforce_https: core::default::Default::default(),
            tls_security_policy: core::default::Default::default(),
        }
    }
}
pub struct ElasticsearchDomainDomainEndpointOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainDomainEndpointOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainDomainEndpointOptionsElRef {
        ElasticsearchDomainDomainEndpointOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainDomainEndpointOptionsElRef {
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
pub struct ElasticsearchDomainEbsOptionsEl {
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
impl ElasticsearchDomainEbsOptionsEl {
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
impl ToListMappable for ElasticsearchDomainEbsOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainEbsOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainEbsOptionsEl {
    #[doc = ""]
    pub ebs_enabled: PrimField<bool>,
}
impl BuildElasticsearchDomainEbsOptionsEl {
    pub fn build(self) -> ElasticsearchDomainEbsOptionsEl {
        ElasticsearchDomainEbsOptionsEl {
            ebs_enabled: self.ebs_enabled,
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}
pub struct ElasticsearchDomainEbsOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainEbsOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainEbsOptionsElRef {
        ElasticsearchDomainEbsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainEbsOptionsElRef {
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
pub struct ElasticsearchDomainEncryptAtRestEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}
impl ElasticsearchDomainEncryptAtRestEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}
impl ToListMappable for ElasticsearchDomainEncryptAtRestEl {
    type O = BlockAssignable<ElasticsearchDomainEncryptAtRestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainEncryptAtRestEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildElasticsearchDomainEncryptAtRestEl {
    pub fn build(self) -> ElasticsearchDomainEncryptAtRestEl {
        ElasticsearchDomainEncryptAtRestEl {
            enabled: self.enabled,
            kms_key_id: core::default::Default::default(),
        }
    }
}
pub struct ElasticsearchDomainEncryptAtRestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainEncryptAtRestElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainEncryptAtRestElRef {
        ElasticsearchDomainEncryptAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainEncryptAtRestElRef {
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
pub struct ElasticsearchDomainLogPublishingOptionsEl {
    cloudwatch_log_group_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    log_type: PrimField<String>,
}
impl ElasticsearchDomainLogPublishingOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for ElasticsearchDomainLogPublishingOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainLogPublishingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainLogPublishingOptionsEl {
    #[doc = ""]
    pub cloudwatch_log_group_arn: PrimField<String>,
    #[doc = ""]
    pub log_type: PrimField<String>,
}
impl BuildElasticsearchDomainLogPublishingOptionsEl {
    pub fn build(self) -> ElasticsearchDomainLogPublishingOptionsEl {
        ElasticsearchDomainLogPublishingOptionsEl {
            cloudwatch_log_group_arn: self.cloudwatch_log_group_arn,
            enabled: core::default::Default::default(),
            log_type: self.log_type,
        }
    }
}
pub struct ElasticsearchDomainLogPublishingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainLogPublishingOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainLogPublishingOptionsElRef {
        ElasticsearchDomainLogPublishingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainLogPublishingOptionsElRef {
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
pub struct ElasticsearchDomainNodeToNodeEncryptionEl {
    enabled: PrimField<bool>,
}
impl ElasticsearchDomainNodeToNodeEncryptionEl {}
impl ToListMappable for ElasticsearchDomainNodeToNodeEncryptionEl {
    type O = BlockAssignable<ElasticsearchDomainNodeToNodeEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainNodeToNodeEncryptionEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildElasticsearchDomainNodeToNodeEncryptionEl {
    pub fn build(self) -> ElasticsearchDomainNodeToNodeEncryptionEl {
        ElasticsearchDomainNodeToNodeEncryptionEl {
            enabled: self.enabled,
        }
    }
}
pub struct ElasticsearchDomainNodeToNodeEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainNodeToNodeEncryptionElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainNodeToNodeEncryptionElRef {
        ElasticsearchDomainNodeToNodeEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainNodeToNodeEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ElasticsearchDomainSnapshotOptionsEl {
    automated_snapshot_start_hour: PrimField<f64>,
}
impl ElasticsearchDomainSnapshotOptionsEl {}
impl ToListMappable for ElasticsearchDomainSnapshotOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainSnapshotOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainSnapshotOptionsEl {
    #[doc = ""]
    pub automated_snapshot_start_hour: PrimField<f64>,
}
impl BuildElasticsearchDomainSnapshotOptionsEl {
    pub fn build(self) -> ElasticsearchDomainSnapshotOptionsEl {
        ElasticsearchDomainSnapshotOptionsEl {
            automated_snapshot_start_hour: self.automated_snapshot_start_hour,
        }
    }
}
pub struct ElasticsearchDomainSnapshotOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainSnapshotOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainSnapshotOptionsElRef {
        ElasticsearchDomainSnapshotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainSnapshotOptionsElRef {
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
pub struct ElasticsearchDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ElasticsearchDomainTimeoutsEl {
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
impl ToListMappable for ElasticsearchDomainTimeoutsEl {
    type O = BlockAssignable<ElasticsearchDomainTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainTimeoutsEl {}
impl BuildElasticsearchDomainTimeoutsEl {
    pub fn build(self) -> ElasticsearchDomainTimeoutsEl {
        ElasticsearchDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ElasticsearchDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainTimeoutsElRef {
        ElasticsearchDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainTimeoutsElRef {
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
pub struct ElasticsearchDomainVpcOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
}
impl ElasticsearchDomainVpcOptionsEl {
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
impl ToListMappable for ElasticsearchDomainVpcOptionsEl {
    type O = BlockAssignable<ElasticsearchDomainVpcOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildElasticsearchDomainVpcOptionsEl {}
impl BuildElasticsearchDomainVpcOptionsEl {
    pub fn build(self) -> ElasticsearchDomainVpcOptionsEl {
        ElasticsearchDomainVpcOptionsEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}
pub struct ElasticsearchDomainVpcOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ElasticsearchDomainVpcOptionsElRef {
    fn new(shared: StackShared, base: String) -> ElasticsearchDomainVpcOptionsElRef {
        ElasticsearchDomainVpcOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ElasticsearchDomainVpcOptionsElRef {
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
struct ElasticsearchDomainDynamic {
    advanced_security_options: Option<DynamicBlock<ElasticsearchDomainAdvancedSecurityOptionsEl>>,
    auto_tune_options: Option<DynamicBlock<ElasticsearchDomainAutoTuneOptionsEl>>,
    cluster_config: Option<DynamicBlock<ElasticsearchDomainClusterConfigEl>>,
    cognito_options: Option<DynamicBlock<ElasticsearchDomainCognitoOptionsEl>>,
    domain_endpoint_options: Option<DynamicBlock<ElasticsearchDomainDomainEndpointOptionsEl>>,
    ebs_options: Option<DynamicBlock<ElasticsearchDomainEbsOptionsEl>>,
    encrypt_at_rest: Option<DynamicBlock<ElasticsearchDomainEncryptAtRestEl>>,
    log_publishing_options: Option<DynamicBlock<ElasticsearchDomainLogPublishingOptionsEl>>,
    node_to_node_encryption: Option<DynamicBlock<ElasticsearchDomainNodeToNodeEncryptionEl>>,
    snapshot_options: Option<DynamicBlock<ElasticsearchDomainSnapshotOptionsEl>>,
    vpc_options: Option<DynamicBlock<ElasticsearchDomainVpcOptionsEl>>,
}
