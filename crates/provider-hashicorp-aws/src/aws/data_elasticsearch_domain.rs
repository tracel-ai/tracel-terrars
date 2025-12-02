use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataElasticsearchDomainData {
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
struct DataElasticsearchDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataElasticsearchDomainData>,
}
#[derive(Clone)]
pub struct DataElasticsearchDomain(Rc<DataElasticsearchDomain_>);
impl DataElasticsearchDomain {
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
    ) -> ListRef<DataElasticsearchDomainAdvancedSecurityOptionsElRef> {
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
    pub fn auto_tune_options(&self) -> ListRef<DataElasticsearchDomainAutoTuneOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_tune_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<DataElasticsearchDomainClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> ListRef<DataElasticsearchDomainCognitoOptionsElRef> {
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
    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deleted", self.extract_ref()),
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
    pub fn ebs_options(&self) -> ListRef<DataElasticsearchDomainEbsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `elasticsearch_version` after provisioning.\n"]
    pub fn elasticsearch_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.elasticsearch_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(&self) -> ListRef<DataElasticsearchDomainEncryptionAtRestElRef> {
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
    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(
        &self,
    ) -> SetRef<DataElasticsearchDomainLogPublishingOptionsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.log_publishing_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(
        &self,
    ) -> ListRef<DataElasticsearchDomainNodeToNodeEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_to_node_encryption", self.extract_ref()),
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
    pub fn snapshot_options(&self) -> ListRef<DataElasticsearchDomainSnapshotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_options", self.extract_ref()),
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
    pub fn vpc_options(&self) -> ListRef<DataElasticsearchDomainVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}
impl Referable for DataElasticsearchDomain {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataElasticsearchDomain {}
impl ToListMappable for DataElasticsearchDomain {
    type O = ListRef<DataElasticsearchDomainRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataElasticsearchDomain_ {
    fn extract_datasource_type(&self) -> String {
        "aws_elasticsearch_domain".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataElasticsearchDomain {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}
impl BuildDataElasticsearchDomain {
    pub fn build(self, stack: &mut Stack) -> DataElasticsearchDomain {
        let out = DataElasticsearchDomain(Rc::new(DataElasticsearchDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataElasticsearchDomainData {
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
pub struct DataElasticsearchDomainRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataElasticsearchDomainRef {
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
    ) -> ListRef<DataElasticsearchDomainAdvancedSecurityOptionsElRef> {
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
    pub fn auto_tune_options(&self) -> ListRef<DataElasticsearchDomainAutoTuneOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auto_tune_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster_config` after provisioning.\n"]
    pub fn cluster_config(&self) -> ListRef<DataElasticsearchDomainClusterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cluster_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cognito_options` after provisioning.\n"]
    pub fn cognito_options(&self) -> ListRef<DataElasticsearchDomainCognitoOptionsElRef> {
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
    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deleted", self.extract_ref()),
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
    pub fn ebs_options(&self) -> ListRef<DataElasticsearchDomainEbsOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ebs_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `elasticsearch_version` after provisioning.\n"]
    pub fn elasticsearch_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.elasticsearch_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_at_rest` after provisioning.\n"]
    pub fn encryption_at_rest(&self) -> ListRef<DataElasticsearchDomainEncryptionAtRestElRef> {
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
    #[doc = "Get a reference to the value of field `log_publishing_options` after provisioning.\n"]
    pub fn log_publishing_options(
        &self,
    ) -> SetRef<DataElasticsearchDomainLogPublishingOptionsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.log_publishing_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_to_node_encryption` after provisioning.\n"]
    pub fn node_to_node_encryption(
        &self,
    ) -> ListRef<DataElasticsearchDomainNodeToNodeEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_to_node_encryption", self.extract_ref()),
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
    pub fn snapshot_options(&self) -> ListRef<DataElasticsearchDomainSnapshotOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.snapshot_options", self.extract_ref()),
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
    pub fn vpc_options(&self) -> ListRef<DataElasticsearchDomainVpcOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_options", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataElasticsearchDomainAdvancedSecurityOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_user_database_enabled: Option<PrimField<bool>>,
}
impl DataElasticsearchDomainAdvancedSecurityOptionsEl {
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
impl ToListMappable for DataElasticsearchDomainAdvancedSecurityOptionsEl {
    type O = BlockAssignable<DataElasticsearchDomainAdvancedSecurityOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainAdvancedSecurityOptionsEl {}
impl BuildDataElasticsearchDomainAdvancedSecurityOptionsEl {
    pub fn build(self) -> DataElasticsearchDomainAdvancedSecurityOptionsEl {
        DataElasticsearchDomainAdvancedSecurityOptionsEl {
            enabled: core::default::Default::default(),
            internal_user_database_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainAdvancedSecurityOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainAdvancedSecurityOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataElasticsearchDomainAdvancedSecurityOptionsElRef {
        DataElasticsearchDomainAdvancedSecurityOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainAdvancedSecurityOptionsElRef {
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
}
#[derive(Serialize)]
pub struct DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<f64>>,
}
impl DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
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
impl ToListMappable for DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    type O =
        BlockAssignable<DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {}
impl BuildDataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
    pub fn build(self) -> DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
        DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
        DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef {
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
pub struct DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cron_expression_for_recurrence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration:
        Option<ListField<DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_at: Option<PrimField<String>>,
}
impl DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    #[doc = "Set the field `cron_expression_for_recurrence`.\n"]
    pub fn set_cron_expression_for_recurrence(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cron_expression_for_recurrence = Some(v.into());
        self
    }
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(
        mut self,
        v: impl Into<ListField<DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationEl>>,
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
impl ToListMappable for DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    type O = BlockAssignable<DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {}
impl BuildDataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
    pub fn build(self) -> DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
        DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl {
            cron_expression_for_recurrence: core::default::Default::default(),
            duration: core::default::Default::default(),
            start_at: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
        DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef {
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
    ) -> ListRef<DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElDurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.duration", self.base))
    }
    #[doc = "Get a reference to the value of field `start_at` after provisioning.\n"]
    pub fn start_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_at", self.base))
    }
}
#[derive(Serialize)]
pub struct DataElasticsearchDomainAutoTuneOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintenance_schedule:
        Option<SetField<DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollback_on_disable: Option<PrimField<String>>,
}
impl DataElasticsearchDomainAutoTuneOptionsEl {
    #[doc = "Set the field `desired_state`.\n"]
    pub fn set_desired_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.desired_state = Some(v.into());
        self
    }
    #[doc = "Set the field `maintenance_schedule`.\n"]
    pub fn set_maintenance_schedule(
        mut self,
        v: impl Into<SetField<DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleEl>>,
    ) -> Self {
        self.maintenance_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `rollback_on_disable`.\n"]
    pub fn set_rollback_on_disable(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rollback_on_disable = Some(v.into());
        self
    }
}
impl ToListMappable for DataElasticsearchDomainAutoTuneOptionsEl {
    type O = BlockAssignable<DataElasticsearchDomainAutoTuneOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainAutoTuneOptionsEl {}
impl BuildDataElasticsearchDomainAutoTuneOptionsEl {
    pub fn build(self) -> DataElasticsearchDomainAutoTuneOptionsEl {
        DataElasticsearchDomainAutoTuneOptionsEl {
            desired_state: core::default::Default::default(),
            maintenance_schedule: core::default::Default::default(),
            rollback_on_disable: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainAutoTuneOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainAutoTuneOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataElasticsearchDomainAutoTuneOptionsElRef {
        DataElasticsearchDomainAutoTuneOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainAutoTuneOptionsElRef {
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
    ) -> SetRef<DataElasticsearchDomainAutoTuneOptionsElMaintenanceScheduleElRef> {
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
}
#[derive(Serialize)]
pub struct DataElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    type O = BlockAssignable<DataElasticsearchDomainClusterConfigElColdStorageOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainClusterConfigElColdStorageOptionsEl {}
impl BuildDataElasticsearchDomainClusterConfigElColdStorageOptionsEl {
    pub fn build(self) -> DataElasticsearchDomainClusterConfigElColdStorageOptionsEl {
        DataElasticsearchDomainClusterConfigElColdStorageOptionsEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
        DataElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainClusterConfigElColdStorageOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone_count: Option<PrimField<f64>>,
}
impl DataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    #[doc = "Set the field `availability_zone_count`.\n"]
    pub fn set_availability_zone_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.availability_zone_count = Some(v.into());
        self
    }
}
impl ToListMappable for DataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    type O = BlockAssignable<DataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {}
impl BuildDataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
    pub fn build(self) -> DataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
        DataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl {
            availability_zone_count: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
        DataElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef {
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
pub struct DataElasticsearchDomainClusterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cold_storage_options:
        Option<ListField<DataElasticsearchDomainClusterConfigElColdStorageOptionsEl>>,
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
    zone_awareness_config:
        Option<ListField<DataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone_awareness_enabled: Option<PrimField<bool>>,
}
impl DataElasticsearchDomainClusterConfigEl {
    #[doc = "Set the field `cold_storage_options`.\n"]
    pub fn set_cold_storage_options(
        mut self,
        v: impl Into<ListField<DataElasticsearchDomainClusterConfigElColdStorageOptionsEl>>,
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
        v: impl Into<ListField<DataElasticsearchDomainClusterConfigElZoneAwarenessConfigEl>>,
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
impl ToListMappable for DataElasticsearchDomainClusterConfigEl {
    type O = BlockAssignable<DataElasticsearchDomainClusterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainClusterConfigEl {}
impl BuildDataElasticsearchDomainClusterConfigEl {
    pub fn build(self) -> DataElasticsearchDomainClusterConfigEl {
        DataElasticsearchDomainClusterConfigEl {
            cold_storage_options: core::default::Default::default(),
            dedicated_master_count: core::default::Default::default(),
            dedicated_master_enabled: core::default::Default::default(),
            dedicated_master_type: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            warm_count: core::default::Default::default(),
            warm_enabled: core::default::Default::default(),
            warm_type: core::default::Default::default(),
            zone_awareness_config: core::default::Default::default(),
            zone_awareness_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainClusterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainClusterConfigElRef {
    fn new(shared: StackShared, base: String) -> DataElasticsearchDomainClusterConfigElRef {
        DataElasticsearchDomainClusterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainClusterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cold_storage_options` after provisioning.\n"]
    pub fn cold_storage_options(
        &self,
    ) -> ListRef<DataElasticsearchDomainClusterConfigElColdStorageOptionsElRef> {
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
    ) -> ListRef<DataElasticsearchDomainClusterConfigElZoneAwarenessConfigElRef> {
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
pub struct DataElasticsearchDomainCognitoOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_pool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_pool_id: Option<PrimField<String>>,
}
impl DataElasticsearchDomainCognitoOptionsEl {
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
impl ToListMappable for DataElasticsearchDomainCognitoOptionsEl {
    type O = BlockAssignable<DataElasticsearchDomainCognitoOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainCognitoOptionsEl {}
impl BuildDataElasticsearchDomainCognitoOptionsEl {
    pub fn build(self) -> DataElasticsearchDomainCognitoOptionsEl {
        DataElasticsearchDomainCognitoOptionsEl {
            enabled: core::default::Default::default(),
            identity_pool_id: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            user_pool_id: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainCognitoOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainCognitoOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataElasticsearchDomainCognitoOptionsElRef {
        DataElasticsearchDomainCognitoOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainCognitoOptionsElRef {
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
pub struct DataElasticsearchDomainEbsOptionsEl {
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
impl DataElasticsearchDomainEbsOptionsEl {
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
impl ToListMappable for DataElasticsearchDomainEbsOptionsEl {
    type O = BlockAssignable<DataElasticsearchDomainEbsOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainEbsOptionsEl {}
impl BuildDataElasticsearchDomainEbsOptionsEl {
    pub fn build(self) -> DataElasticsearchDomainEbsOptionsEl {
        DataElasticsearchDomainEbsOptionsEl {
            ebs_enabled: core::default::Default::default(),
            iops: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainEbsOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainEbsOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataElasticsearchDomainEbsOptionsElRef {
        DataElasticsearchDomainEbsOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainEbsOptionsElRef {
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
pub struct DataElasticsearchDomainEncryptionAtRestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}
impl DataElasticsearchDomainEncryptionAtRestEl {
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
impl ToListMappable for DataElasticsearchDomainEncryptionAtRestEl {
    type O = BlockAssignable<DataElasticsearchDomainEncryptionAtRestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainEncryptionAtRestEl {}
impl BuildDataElasticsearchDomainEncryptionAtRestEl {
    pub fn build(self) -> DataElasticsearchDomainEncryptionAtRestEl {
        DataElasticsearchDomainEncryptionAtRestEl {
            enabled: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainEncryptionAtRestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainEncryptionAtRestElRef {
    fn new(shared: StackShared, base: String) -> DataElasticsearchDomainEncryptionAtRestElRef {
        DataElasticsearchDomainEncryptionAtRestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainEncryptionAtRestElRef {
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
pub struct DataElasticsearchDomainLogPublishingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_group_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_type: Option<PrimField<String>>,
}
impl DataElasticsearchDomainLogPublishingOptionsEl {
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
impl ToListMappable for DataElasticsearchDomainLogPublishingOptionsEl {
    type O = BlockAssignable<DataElasticsearchDomainLogPublishingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainLogPublishingOptionsEl {}
impl BuildDataElasticsearchDomainLogPublishingOptionsEl {
    pub fn build(self) -> DataElasticsearchDomainLogPublishingOptionsEl {
        DataElasticsearchDomainLogPublishingOptionsEl {
            cloudwatch_log_group_arn: core::default::Default::default(),
            enabled: core::default::Default::default(),
            log_type: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainLogPublishingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainLogPublishingOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataElasticsearchDomainLogPublishingOptionsElRef {
        DataElasticsearchDomainLogPublishingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainLogPublishingOptionsElRef {
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
pub struct DataElasticsearchDomainNodeToNodeEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataElasticsearchDomainNodeToNodeEncryptionEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataElasticsearchDomainNodeToNodeEncryptionEl {
    type O = BlockAssignable<DataElasticsearchDomainNodeToNodeEncryptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainNodeToNodeEncryptionEl {}
impl BuildDataElasticsearchDomainNodeToNodeEncryptionEl {
    pub fn build(self) -> DataElasticsearchDomainNodeToNodeEncryptionEl {
        DataElasticsearchDomainNodeToNodeEncryptionEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainNodeToNodeEncryptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainNodeToNodeEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataElasticsearchDomainNodeToNodeEncryptionElRef {
        DataElasticsearchDomainNodeToNodeEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainNodeToNodeEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataElasticsearchDomainSnapshotOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automated_snapshot_start_hour: Option<PrimField<f64>>,
}
impl DataElasticsearchDomainSnapshotOptionsEl {
    #[doc = "Set the field `automated_snapshot_start_hour`.\n"]
    pub fn set_automated_snapshot_start_hour(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.automated_snapshot_start_hour = Some(v.into());
        self
    }
}
impl ToListMappable for DataElasticsearchDomainSnapshotOptionsEl {
    type O = BlockAssignable<DataElasticsearchDomainSnapshotOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainSnapshotOptionsEl {}
impl BuildDataElasticsearchDomainSnapshotOptionsEl {
    pub fn build(self) -> DataElasticsearchDomainSnapshotOptionsEl {
        DataElasticsearchDomainSnapshotOptionsEl {
            automated_snapshot_start_hour: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainSnapshotOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainSnapshotOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataElasticsearchDomainSnapshotOptionsElRef {
        DataElasticsearchDomainSnapshotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainSnapshotOptionsElRef {
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
pub struct DataElasticsearchDomainVpcOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zones: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}
impl DataElasticsearchDomainVpcOptionsEl {
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
impl ToListMappable for DataElasticsearchDomainVpcOptionsEl {
    type O = BlockAssignable<DataElasticsearchDomainVpcOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataElasticsearchDomainVpcOptionsEl {}
impl BuildDataElasticsearchDomainVpcOptionsEl {
    pub fn build(self) -> DataElasticsearchDomainVpcOptionsEl {
        DataElasticsearchDomainVpcOptionsEl {
            availability_zones: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}
pub struct DataElasticsearchDomainVpcOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataElasticsearchDomainVpcOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataElasticsearchDomainVpcOptionsElRef {
        DataElasticsearchDomainVpcOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataElasticsearchDomainVpcOptionsElRef {
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
