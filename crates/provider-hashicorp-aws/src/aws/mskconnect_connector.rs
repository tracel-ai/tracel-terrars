use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MskconnectConnectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    connector_configuration: RecField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    kafkaconnect_version: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_execution_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<Vec<MskconnectConnectorCapacityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka_cluster: Option<Vec<MskconnectConnectorKafkaClusterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka_cluster_client_authentication:
        Option<Vec<MskconnectConnectorKafkaClusterClientAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kafka_cluster_encryption_in_transit:
        Option<Vec<MskconnectConnectorKafkaClusterEncryptionInTransitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_delivery: Option<Vec<MskconnectConnectorLogDeliveryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plugin: Option<Vec<MskconnectConnectorPluginEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MskconnectConnectorTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_configuration: Option<Vec<MskconnectConnectorWorkerConfigurationEl>>,
    dynamic: MskconnectConnectorDynamic,
}
struct MskconnectConnector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MskconnectConnectorData>,
}
#[derive(Clone)]
pub struct MskconnectConnector(Rc<MskconnectConnector_>);
impl MskconnectConnector {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(
        self,
        v: impl Into<BlockAssignable<MskconnectConnectorCapacityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kafka_cluster`.\n"]
    pub fn set_kafka_cluster(
        self,
        v: impl Into<BlockAssignable<MskconnectConnectorKafkaClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_cluster = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kafka_cluster = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kafka_cluster_client_authentication`.\n"]
    pub fn set_kafka_cluster_client_authentication(
        self,
        v: impl Into<BlockAssignable<MskconnectConnectorKafkaClusterClientAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_cluster_client_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .kafka_cluster_client_authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `kafka_cluster_encryption_in_transit`.\n"]
    pub fn set_kafka_cluster_encryption_in_transit(
        self,
        v: impl Into<BlockAssignable<MskconnectConnectorKafkaClusterEncryptionInTransitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kafka_cluster_encryption_in_transit = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .kafka_cluster_encryption_in_transit = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_delivery`.\n"]
    pub fn set_log_delivery(
        self,
        v: impl Into<BlockAssignable<MskconnectConnectorLogDeliveryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_delivery = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_delivery = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `plugin`.\n"]
    pub fn set_plugin(self, v: impl Into<BlockAssignable<MskconnectConnectorPluginEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().plugin = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.plugin = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MskconnectConnectorTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_configuration`.\n"]
    pub fn set_worker_configuration(
        self,
        v: impl Into<BlockAssignable<MskconnectConnectorWorkerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().worker_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.worker_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `connector_configuration` after provisioning.\n"]
    pub fn connector_configuration(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.connector_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kafkaconnect_version` after provisioning.\n"]
    pub fn kafkaconnect_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kafkaconnect_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_execution_role_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> ListRef<MskconnectConnectorCapacityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> ListRef<MskconnectConnectorKafkaClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kafka_cluster_client_authentication` after provisioning.\n"]
    pub fn kafka_cluster_client_authentication(
        &self,
    ) -> ListRef<MskconnectConnectorKafkaClusterClientAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_client_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kafka_cluster_encryption_in_transit` after provisioning.\n"]
    pub fn kafka_cluster_encryption_in_transit(
        &self,
    ) -> ListRef<MskconnectConnectorKafkaClusterEncryptionInTransitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_encryption_in_transit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_delivery` after provisioning.\n"]
    pub fn log_delivery(&self) -> ListRef<MskconnectConnectorLogDeliveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_delivery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskconnectConnectorTimeoutsElRef {
        MskconnectConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(&self) -> ListRef<MskconnectConnectorWorkerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.worker_configuration", self.extract_ref()),
        )
    }
}
impl Referable for MskconnectConnector {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MskconnectConnector {}
impl ToListMappable for MskconnectConnector {
    type O = ListRef<MskconnectConnectorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MskconnectConnector_ {
    fn extract_resource_type(&self) -> String {
        "aws_mskconnect_connector".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMskconnectConnector {
    pub tf_id: String,
    #[doc = ""]
    pub connector_configuration: RecField<PrimField<String>>,
    #[doc = ""]
    pub kafkaconnect_version: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub service_execution_role_arn: PrimField<String>,
}
impl BuildMskconnectConnector {
    pub fn build(self, stack: &mut Stack) -> MskconnectConnector {
        let out = MskconnectConnector(Rc::new(MskconnectConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MskconnectConnectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connector_configuration: self.connector_configuration,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kafkaconnect_version: self.kafkaconnect_version,
                name: self.name,
                region: core::default::Default::default(),
                service_execution_role_arn: self.service_execution_role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                capacity: core::default::Default::default(),
                kafka_cluster: core::default::Default::default(),
                kafka_cluster_client_authentication: core::default::Default::default(),
                kafka_cluster_encryption_in_transit: core::default::Default::default(),
                log_delivery: core::default::Default::default(),
                plugin: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                worker_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MskconnectConnectorRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MskconnectConnectorRef {
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
    #[doc = "Get a reference to the value of field `connector_configuration` after provisioning.\n"]
    pub fn connector_configuration(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.connector_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kafkaconnect_version` after provisioning.\n"]
    pub fn kafkaconnect_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kafkaconnect_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_execution_role_arn` after provisioning.\n"]
    pub fn service_execution_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_execution_role_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> ListRef<MskconnectConnectorCapacityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kafka_cluster` after provisioning.\n"]
    pub fn kafka_cluster(&self) -> ListRef<MskconnectConnectorKafkaClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kafka_cluster_client_authentication` after provisioning.\n"]
    pub fn kafka_cluster_client_authentication(
        &self,
    ) -> ListRef<MskconnectConnectorKafkaClusterClientAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_client_authentication", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kafka_cluster_encryption_in_transit` after provisioning.\n"]
    pub fn kafka_cluster_encryption_in_transit(
        &self,
    ) -> ListRef<MskconnectConnectorKafkaClusterEncryptionInTransitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kafka_cluster_encryption_in_transit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_delivery` after provisioning.\n"]
    pub fn log_delivery(&self) -> ListRef<MskconnectConnectorLogDeliveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_delivery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MskconnectConnectorTimeoutsElRef {
        MskconnectConnectorTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(&self) -> ListRef<MskconnectConnectorWorkerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.worker_configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_utilization_percentage: Option<PrimField<f64>>,
}
impl MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
    #[doc = "Set the field `cpu_utilization_percentage`.\n"]
    pub fn set_cpu_utilization_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_utilization_percentage = Some(v.into());
        self
    }
}
impl ToListMappable for MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
    type O = BlockAssignable<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {}
impl BuildMskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
    pub fn build(self) -> MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
        MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl {
            cpu_utilization_percentage: core::default::Default::default(),
        }
    }
}
pub struct MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
        MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_utilization_percentage` after provisioning.\n"]
    pub fn cpu_utilization_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_utilization_percentage", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_utilization_percentage: Option<PrimField<f64>>,
}
impl MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
    #[doc = "Set the field `cpu_utilization_percentage`.\n"]
    pub fn set_cpu_utilization_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_utilization_percentage = Some(v.into());
        self
    }
}
impl ToListMappable for MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
    type O = BlockAssignable<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {}
impl BuildMskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
    pub fn build(self) -> MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
        MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl {
            cpu_utilization_percentage: core::default::Default::default(),
        }
    }
}
pub struct MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
        MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_utilization_percentage` after provisioning.\n"]
    pub fn cpu_utilization_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_utilization_percentage", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MskconnectConnectorCapacityElAutoscalingElDynamic {
    scale_in_policy:
        Option<DynamicBlock<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl>>,
    scale_out_policy:
        Option<DynamicBlock<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl>>,
}
#[derive(Serialize)]
pub struct MskconnectConnectorCapacityElAutoscalingEl {
    max_worker_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mcu_count: Option<PrimField<f64>>,
    min_worker_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_in_policy: Option<Vec<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_out_policy: Option<Vec<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl>>,
    dynamic: MskconnectConnectorCapacityElAutoscalingElDynamic,
}
impl MskconnectConnectorCapacityElAutoscalingEl {
    #[doc = "Set the field `mcu_count`.\n"]
    pub fn set_mcu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mcu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `scale_in_policy`.\n"]
    pub fn set_scale_in_policy(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scale_in_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scale_in_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `scale_out_policy`.\n"]
    pub fn set_scale_out_policy(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scale_out_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scale_out_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskconnectConnectorCapacityElAutoscalingEl {
    type O = BlockAssignable<MskconnectConnectorCapacityElAutoscalingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorCapacityElAutoscalingEl {
    #[doc = ""]
    pub max_worker_count: PrimField<f64>,
    #[doc = ""]
    pub min_worker_count: PrimField<f64>,
}
impl BuildMskconnectConnectorCapacityElAutoscalingEl {
    pub fn build(self) -> MskconnectConnectorCapacityElAutoscalingEl {
        MskconnectConnectorCapacityElAutoscalingEl {
            max_worker_count: self.max_worker_count,
            mcu_count: core::default::Default::default(),
            min_worker_count: self.min_worker_count,
            scale_in_policy: core::default::Default::default(),
            scale_out_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskconnectConnectorCapacityElAutoscalingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorCapacityElAutoscalingElRef {
    fn new(shared: StackShared, base: String) -> MskconnectConnectorCapacityElAutoscalingElRef {
        MskconnectConnectorCapacityElAutoscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorCapacityElAutoscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_worker_count` after provisioning.\n"]
    pub fn max_worker_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_worker_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mcu_count` after provisioning.\n"]
    pub fn mcu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mcu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `min_worker_count` after provisioning.\n"]
    pub fn min_worker_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_worker_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scale_in_policy` after provisioning.\n"]
    pub fn scale_in_policy(
        &self,
    ) -> ListRef<MskconnectConnectorCapacityElAutoscalingElScaleInPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scale_in_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scale_out_policy` after provisioning.\n"]
    pub fn scale_out_policy(
        &self,
    ) -> ListRef<MskconnectConnectorCapacityElAutoscalingElScaleOutPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scale_out_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorCapacityElProvisionedCapacityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mcu_count: Option<PrimField<f64>>,
    worker_count: PrimField<f64>,
}
impl MskconnectConnectorCapacityElProvisionedCapacityEl {
    #[doc = "Set the field `mcu_count`.\n"]
    pub fn set_mcu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.mcu_count = Some(v.into());
        self
    }
}
impl ToListMappable for MskconnectConnectorCapacityElProvisionedCapacityEl {
    type O = BlockAssignable<MskconnectConnectorCapacityElProvisionedCapacityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorCapacityElProvisionedCapacityEl {
    #[doc = ""]
    pub worker_count: PrimField<f64>,
}
impl BuildMskconnectConnectorCapacityElProvisionedCapacityEl {
    pub fn build(self) -> MskconnectConnectorCapacityElProvisionedCapacityEl {
        MskconnectConnectorCapacityElProvisionedCapacityEl {
            mcu_count: core::default::Default::default(),
            worker_count: self.worker_count,
        }
    }
}
pub struct MskconnectConnectorCapacityElProvisionedCapacityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorCapacityElProvisionedCapacityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorCapacityElProvisionedCapacityElRef {
        MskconnectConnectorCapacityElProvisionedCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorCapacityElProvisionedCapacityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mcu_count` after provisioning.\n"]
    pub fn mcu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.mcu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `worker_count` after provisioning.\n"]
    pub fn worker_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_count", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskconnectConnectorCapacityElDynamic {
    autoscaling: Option<DynamicBlock<MskconnectConnectorCapacityElAutoscalingEl>>,
    provisioned_capacity: Option<DynamicBlock<MskconnectConnectorCapacityElProvisionedCapacityEl>>,
}
#[derive(Serialize)]
pub struct MskconnectConnectorCapacityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling: Option<Vec<MskconnectConnectorCapacityElAutoscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_capacity: Option<Vec<MskconnectConnectorCapacityElProvisionedCapacityEl>>,
    dynamic: MskconnectConnectorCapacityElDynamic,
}
impl MskconnectConnectorCapacityEl {
    #[doc = "Set the field `autoscaling`.\n"]
    pub fn set_autoscaling(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorCapacityElAutoscalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.autoscaling = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `provisioned_capacity`.\n"]
    pub fn set_provisioned_capacity(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorCapacityElProvisionedCapacityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.provisioned_capacity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.provisioned_capacity = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskconnectConnectorCapacityEl {
    type O = BlockAssignable<MskconnectConnectorCapacityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorCapacityEl {}
impl BuildMskconnectConnectorCapacityEl {
    pub fn build(self) -> MskconnectConnectorCapacityEl {
        MskconnectConnectorCapacityEl {
            autoscaling: core::default::Default::default(),
            provisioned_capacity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskconnectConnectorCapacityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorCapacityElRef {
    fn new(shared: StackShared, base: String) -> MskconnectConnectorCapacityElRef {
        MskconnectConnectorCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorCapacityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `autoscaling` after provisioning.\n"]
    pub fn autoscaling(&self) -> ListRef<MskconnectConnectorCapacityElAutoscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscaling", self.base))
    }
    #[doc = "Get a reference to the value of field `provisioned_capacity` after provisioning.\n"]
    pub fn provisioned_capacity(
        &self,
    ) -> ListRef<MskconnectConnectorCapacityElProvisionedCapacityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.provisioned_capacity", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
    security_groups: SetField<PrimField<String>>,
    subnets: SetField<PrimField<String>>,
}
impl MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {}
impl ToListMappable for MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
    type O = BlockAssignable<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
    #[doc = ""]
    pub security_groups: SetField<PrimField<String>>,
    #[doc = ""]
    pub subnets: SetField<PrimField<String>>,
}
impl BuildMskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
        MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl {
            security_groups: self.security_groups,
            subnets: self.subnets,
        }
    }
}
pub struct MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
        MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskconnectConnectorKafkaClusterElApacheKafkaClusterElDynamic {
    vpc: Option<DynamicBlock<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl>>,
}
#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    bootstrap_servers: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<Vec<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl>>,
    dynamic: MskconnectConnectorKafkaClusterElApacheKafkaClusterElDynamic,
}
impl MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    type O = BlockAssignable<MskconnectConnectorKafkaClusterElApacheKafkaClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    #[doc = ""]
    pub bootstrap_servers: PrimField<String>,
}
impl BuildMskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
        MskconnectConnectorKafkaClusterElApacheKafkaClusterEl {
            bootstrap_servers: self.bootstrap_servers,
            vpc: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
        MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bootstrap_servers` after provisioning.\n"]
    pub fn bootstrap_servers(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bootstrap_servers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<MskconnectConnectorKafkaClusterElApacheKafkaClusterElVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskconnectConnectorKafkaClusterElDynamic {
    apache_kafka_cluster:
        Option<DynamicBlock<MskconnectConnectorKafkaClusterElApacheKafkaClusterEl>>,
}
#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    apache_kafka_cluster: Option<Vec<MskconnectConnectorKafkaClusterElApacheKafkaClusterEl>>,
    dynamic: MskconnectConnectorKafkaClusterElDynamic,
}
impl MskconnectConnectorKafkaClusterEl {
    #[doc = "Set the field `apache_kafka_cluster`.\n"]
    pub fn set_apache_kafka_cluster(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorKafkaClusterElApacheKafkaClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.apache_kafka_cluster = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.apache_kafka_cluster = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskconnectConnectorKafkaClusterEl {
    type O = BlockAssignable<MskconnectConnectorKafkaClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorKafkaClusterEl {}
impl BuildMskconnectConnectorKafkaClusterEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterEl {
        MskconnectConnectorKafkaClusterEl {
            apache_kafka_cluster: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskconnectConnectorKafkaClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorKafkaClusterElRef {
    fn new(shared: StackShared, base: String) -> MskconnectConnectorKafkaClusterElRef {
        MskconnectConnectorKafkaClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorKafkaClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `apache_kafka_cluster` after provisioning.\n"]
    pub fn apache_kafka_cluster(
        &self,
    ) -> ListRef<MskconnectConnectorKafkaClusterElApacheKafkaClusterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.apache_kafka_cluster", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterClientAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_type: Option<PrimField<String>>,
}
impl MskconnectConnectorKafkaClusterClientAuthenticationEl {
    #[doc = "Set the field `authentication_type`.\n"]
    pub fn set_authentication_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authentication_type = Some(v.into());
        self
    }
}
impl ToListMappable for MskconnectConnectorKafkaClusterClientAuthenticationEl {
    type O = BlockAssignable<MskconnectConnectorKafkaClusterClientAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorKafkaClusterClientAuthenticationEl {}
impl BuildMskconnectConnectorKafkaClusterClientAuthenticationEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterClientAuthenticationEl {
        MskconnectConnectorKafkaClusterClientAuthenticationEl {
            authentication_type: core::default::Default::default(),
        }
    }
}
pub struct MskconnectConnectorKafkaClusterClientAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorKafkaClusterClientAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorKafkaClusterClientAuthenticationElRef {
        MskconnectConnectorKafkaClusterClientAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorKafkaClusterClientAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorKafkaClusterEncryptionInTransitEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
}
impl MskconnectConnectorKafkaClusterEncryptionInTransitEl {
    #[doc = "Set the field `encryption_type`.\n"]
    pub fn set_encryption_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_type = Some(v.into());
        self
    }
}
impl ToListMappable for MskconnectConnectorKafkaClusterEncryptionInTransitEl {
    type O = BlockAssignable<MskconnectConnectorKafkaClusterEncryptionInTransitEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorKafkaClusterEncryptionInTransitEl {}
impl BuildMskconnectConnectorKafkaClusterEncryptionInTransitEl {
    pub fn build(self) -> MskconnectConnectorKafkaClusterEncryptionInTransitEl {
        MskconnectConnectorKafkaClusterEncryptionInTransitEl {
            encryption_type: core::default::Default::default(),
        }
    }
}
pub struct MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
        MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorKafkaClusterEncryptionInTransitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\n"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_group: Option<PrimField<String>>,
}
impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_group = Some(v.into());
        self
    }
}
impl ToListMappable for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    type O = BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
    pub fn build(self) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl {
            enabled: self.enabled,
            log_group: core::default::Default::default(),
        }
    }
}
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_stream: Option<PrimField<String>>,
    enabled: PrimField<bool>,
}
impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    #[doc = "Set the field `delivery_stream`.\n"]
    pub fn set_delivery_stream(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delivery_stream = Some(v.into());
        self
    }
}
impl ToListMappable for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    type O = BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
    pub fn build(self) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl {
            delivery_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delivery_stream` after provisioning.\n"]
    pub fn delivery_stream(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_stream", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket: Option<PrimField<String>>,
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}
impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}
impl ToListMappable for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    type O = BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}
impl BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
    pub fn build(self) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El {
            bucket: core::default::Default::default(),
            enabled: self.enabled,
            prefix: core::default::Default::default(),
        }
    }
}
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElDynamic {
    cloudwatch_logs:
        Option<DynamicBlock<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl>>,
    firehose: Option<DynamicBlock<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl>>,
    s3: Option<DynamicBlock<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El>>,
}
#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_logs:
        Option<Vec<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firehose: Option<Vec<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3: Option<Vec<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El>>,
    dynamic: MskconnectConnectorLogDeliveryElWorkerLogDeliveryElDynamic,
}
impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v: impl Into<
            BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `firehose`.\n"]
    pub fn set_firehose(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.firehose = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.firehose = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
    type O = BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {}
impl BuildMskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
    pub fn build(self) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl {
            cloudwatch_logs: core::default::Default::default(),
            firehose: core::default::Default::default(),
            s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
        MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> ListRef<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElCloudwatchLogsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloudwatch_logs", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `firehose` after provisioning.\n"]
    pub fn firehose(
        &self,
    ) -> ListRef<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElFirehoseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.firehose", self.base))
    }
    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> ListRef<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskconnectConnectorLogDeliveryElDynamic {
    worker_log_delivery: Option<DynamicBlock<MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl>>,
}
#[derive(Serialize)]
pub struct MskconnectConnectorLogDeliveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_log_delivery: Option<Vec<MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl>>,
    dynamic: MskconnectConnectorLogDeliveryElDynamic,
}
impl MskconnectConnectorLogDeliveryEl {
    #[doc = "Set the field `worker_log_delivery`.\n"]
    pub fn set_worker_log_delivery(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorLogDeliveryElWorkerLogDeliveryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.worker_log_delivery = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_log_delivery = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskconnectConnectorLogDeliveryEl {
    type O = BlockAssignable<MskconnectConnectorLogDeliveryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorLogDeliveryEl {}
impl BuildMskconnectConnectorLogDeliveryEl {
    pub fn build(self) -> MskconnectConnectorLogDeliveryEl {
        MskconnectConnectorLogDeliveryEl {
            worker_log_delivery: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskconnectConnectorLogDeliveryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorLogDeliveryElRef {
    fn new(shared: StackShared, base: String) -> MskconnectConnectorLogDeliveryElRef {
        MskconnectConnectorLogDeliveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorLogDeliveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `worker_log_delivery` after provisioning.\n"]
    pub fn worker_log_delivery(
        &self,
    ) -> ListRef<MskconnectConnectorLogDeliveryElWorkerLogDeliveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.worker_log_delivery", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorPluginElCustomPluginEl {
    arn: PrimField<String>,
    revision: PrimField<f64>,
}
impl MskconnectConnectorPluginElCustomPluginEl {}
impl ToListMappable for MskconnectConnectorPluginElCustomPluginEl {
    type O = BlockAssignable<MskconnectConnectorPluginElCustomPluginEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorPluginElCustomPluginEl {
    #[doc = ""]
    pub arn: PrimField<String>,
    #[doc = ""]
    pub revision: PrimField<f64>,
}
impl BuildMskconnectConnectorPluginElCustomPluginEl {
    pub fn build(self) -> MskconnectConnectorPluginElCustomPluginEl {
        MskconnectConnectorPluginElCustomPluginEl {
            arn: self.arn,
            revision: self.revision,
        }
    }
}
pub struct MskconnectConnectorPluginElCustomPluginElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorPluginElCustomPluginElRef {
    fn new(shared: StackShared, base: String) -> MskconnectConnectorPluginElCustomPluginElRef {
        MskconnectConnectorPluginElCustomPluginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorPluginElCustomPluginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskconnectConnectorPluginElDynamic {
    custom_plugin: Option<DynamicBlock<MskconnectConnectorPluginElCustomPluginEl>>,
}
#[derive(Serialize)]
pub struct MskconnectConnectorPluginEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_plugin: Option<Vec<MskconnectConnectorPluginElCustomPluginEl>>,
    dynamic: MskconnectConnectorPluginElDynamic,
}
impl MskconnectConnectorPluginEl {
    #[doc = "Set the field `custom_plugin`.\n"]
    pub fn set_custom_plugin(
        mut self,
        v: impl Into<BlockAssignable<MskconnectConnectorPluginElCustomPluginEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_plugin = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_plugin = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MskconnectConnectorPluginEl {
    type O = BlockAssignable<MskconnectConnectorPluginEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorPluginEl {}
impl BuildMskconnectConnectorPluginEl {
    pub fn build(self) -> MskconnectConnectorPluginEl {
        MskconnectConnectorPluginEl {
            custom_plugin: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MskconnectConnectorPluginElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorPluginElRef {
    fn new(shared: StackShared, base: String) -> MskconnectConnectorPluginElRef {
        MskconnectConnectorPluginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorPluginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_plugin` after provisioning.\n"]
    pub fn custom_plugin(&self) -> ListRef<MskconnectConnectorPluginElCustomPluginElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_plugin", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MskconnectConnectorTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MskconnectConnectorTimeoutsEl {
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
impl ToListMappable for MskconnectConnectorTimeoutsEl {
    type O = BlockAssignable<MskconnectConnectorTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorTimeoutsEl {}
impl BuildMskconnectConnectorTimeoutsEl {
    pub fn build(self) -> MskconnectConnectorTimeoutsEl {
        MskconnectConnectorTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MskconnectConnectorTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MskconnectConnectorTimeoutsElRef {
        MskconnectConnectorTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorTimeoutsElRef {
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
pub struct MskconnectConnectorWorkerConfigurationEl {
    arn: PrimField<String>,
    revision: PrimField<f64>,
}
impl MskconnectConnectorWorkerConfigurationEl {}
impl ToListMappable for MskconnectConnectorWorkerConfigurationEl {
    type O = BlockAssignable<MskconnectConnectorWorkerConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMskconnectConnectorWorkerConfigurationEl {
    #[doc = ""]
    pub arn: PrimField<String>,
    #[doc = ""]
    pub revision: PrimField<f64>,
}
impl BuildMskconnectConnectorWorkerConfigurationEl {
    pub fn build(self) -> MskconnectConnectorWorkerConfigurationEl {
        MskconnectConnectorWorkerConfigurationEl {
            arn: self.arn,
            revision: self.revision,
        }
    }
}
pub struct MskconnectConnectorWorkerConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MskconnectConnectorWorkerConfigurationElRef {
    fn new(shared: StackShared, base: String) -> MskconnectConnectorWorkerConfigurationElRef {
        MskconnectConnectorWorkerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MskconnectConnectorWorkerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.base))
    }
}
#[derive(Serialize, Default)]
struct MskconnectConnectorDynamic {
    capacity: Option<DynamicBlock<MskconnectConnectorCapacityEl>>,
    kafka_cluster: Option<DynamicBlock<MskconnectConnectorKafkaClusterEl>>,
    kafka_cluster_client_authentication:
        Option<DynamicBlock<MskconnectConnectorKafkaClusterClientAuthenticationEl>>,
    kafka_cluster_encryption_in_transit:
        Option<DynamicBlock<MskconnectConnectorKafkaClusterEncryptionInTransitEl>>,
    log_delivery: Option<DynamicBlock<MskconnectConnectorLogDeliveryEl>>,
    plugin: Option<DynamicBlock<MskconnectConnectorPluginEl>>,
    worker_configuration: Option<DynamicBlock<MskconnectConnectorWorkerConfigurationEl>>,
}
