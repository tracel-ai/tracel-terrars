use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AlloydbInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activation_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_type: Option<PrimField<String>>,
    cluster: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    database_flags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_connection_config: Option<Vec<AlloydbInstanceClientConnectionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_pool_config: Option<Vec<AlloydbInstanceConnectionPoolConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_config: Option<Vec<AlloydbInstanceMachineConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<AlloydbInstanceNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_instance_config: Option<Vec<AlloydbInstancePscInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_insights_config: Option<Vec<AlloydbInstanceQueryInsightsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read_pool_config: Option<Vec<AlloydbInstanceReadPoolConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AlloydbInstanceTimeoutsEl>,
    dynamic: AlloydbInstanceDynamic,
}
struct AlloydbInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AlloydbInstanceData>,
}
#[derive(Clone)]
pub struct AlloydbInstance(Rc<AlloydbInstance_>);
impl AlloydbInstance {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `activation_policy`.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn set_activation_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().activation_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `annotations`.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `availability_type`.\n'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.\nNote that primary and read instances can have different availability types.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances can also be either ZONAL or REGIONAL.\nRead pools of size 1 can only have zonal availability. Read pools with a node count of 2 or more\ncan have regional availability (nodes are present in 2 or more zones in a region).\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn set_availability_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().availability_type = Some(v.into());
        self
    }
    #[doc = "Set the field `database_flags`.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn set_database_flags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().database_flags = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser-settable and human-readable display name for the Instance."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `gce_zone`.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn set_gce_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().gce_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `client_connection_config`.\n"]
    pub fn set_client_connection_config(
        self,
        v: impl Into<BlockAssignable<AlloydbInstanceClientConnectionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_connection_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_connection_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `connection_pool_config`.\n"]
    pub fn set_connection_pool_config(
        self,
        v: impl Into<BlockAssignable<AlloydbInstanceConnectionPoolConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_pool_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `machine_config`.\n"]
    pub fn set_machine_config(
        self,
        v: impl Into<BlockAssignable<AlloydbInstanceMachineConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().machine_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.machine_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl Into<BlockAssignable<AlloydbInstanceNetworkConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `psc_instance_config`.\n"]
    pub fn set_psc_instance_config(
        self,
        v: impl Into<BlockAssignable<AlloydbInstancePscInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_instance_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_insights_config`.\n"]
    pub fn set_query_insights_config(
        self,
        v: impl Into<BlockAssignable<AlloydbInstanceQueryInsightsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().query_insights_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.query_insights_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `read_pool_config`.\n"]
    pub fn set_read_pool_config(
        self,
        v: impl Into<BlockAssignable<AlloydbInstanceReadPoolConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().read_pool_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.read_pool_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AlloydbInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `activation_policy` after provisioning.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.\nNote that primary and read instances can have different availability types.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances can also be either ZONAL or REGIONAL.\nRead pools of size 1 can only have zonal availability. Read pools with a node count of 2 or more\ncan have regional availability (nodes are present in 2 or more zones in a region).\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.database_flags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gce_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `outbound_public_ip_addresses` after provisioning.\nThe outbound public IP addresses for the instance. This is available ONLY when\nnetworkConfig.enableOutboundPublicIp is set to true. These IP addresses are used\nfor outbound connections."]
    pub fn outbound_public_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outbound_public_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_ip_address` after provisioning.\nThe public IP addresses for the Instance. This is available ONLY when\nnetworkConfig.enablePublicIp is set to true. This is the connection\nendpoint for an end-user application."]
    pub fn public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_connection_config` after provisioning.\n"]
    pub fn client_connection_config(&self) -> ListRef<AlloydbInstanceClientConnectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_connection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\n"]
    pub fn connection_pool_config(&self) -> ListRef<AlloydbInstanceConnectionPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\n"]
    pub fn machine_config(&self) -> ListRef<AlloydbInstanceMachineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<AlloydbInstanceNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_instance_config` after provisioning.\n"]
    pub fn psc_instance_config(&self) -> ListRef<AlloydbInstancePscInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_insights_config` after provisioning.\n"]
    pub fn query_insights_config(&self) -> ListRef<AlloydbInstanceQueryInsightsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_insights_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_pool_config` after provisioning.\n"]
    pub fn read_pool_config(&self) -> ListRef<AlloydbInstanceReadPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbInstanceTimeoutsElRef {
        AlloydbInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AlloydbInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AlloydbInstance {}
impl ToListMappable for AlloydbInstance {
    type O = ListRef<AlloydbInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AlloydbInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_alloydb_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAlloydbInstance {
    pub tf_id: String,
    #[doc = "Identifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub cluster: PrimField<String>,
    #[doc = "The ID of the alloydb instance."]
    pub instance_id: PrimField<String>,
    #[doc = "The type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub instance_type: PrimField<String>,
}
impl BuildAlloydbInstance {
    pub fn build(self, stack: &mut Stack) -> AlloydbInstance {
        let out = AlloydbInstance(Rc::new(AlloydbInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AlloydbInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                activation_policy: core::default::Default::default(),
                annotations: core::default::Default::default(),
                availability_type: core::default::Default::default(),
                cluster: self.cluster,
                database_flags: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                gce_zone: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                instance_type: self.instance_type,
                labels: core::default::Default::default(),
                client_connection_config: core::default::Default::default(),
                connection_pool_config: core::default::Default::default(),
                machine_config: core::default::Default::default(),
                network_config: core::default::Default::default(),
                psc_instance_config: core::default::Default::default(),
                query_insights_config: core::default::Default::default(),
                read_pool_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AlloydbInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AlloydbInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `activation_policy` after provisioning.\n'Specifies whether an instance needs to spin up. Once the instance is\nactive, the activation policy can be updated to the 'NEVER' to stop the\ninstance. Likewise, the activation policy can be updated to 'ALWAYS' to\nstart the instance.\nThere are restrictions around when an instance can/cannot be activated (for\nexample, a read pool instance should be stopped before stopping primary\netc.). Please refer to the API documentation for more details.\nPossible values are: 'ACTIVATION_POLICY_UNSPECIFIED', 'ALWAYS', 'NEVER'.' Possible values: [\"ACTIVATION_POLICY_UNSPECIFIED\", \"ALWAYS\", \"NEVER\"]"]
    pub fn activation_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.activation_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations to allow client tools to store small amount of arbitrary data. This is distinct from labels.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `availability_type` after provisioning.\n'Availability type of an Instance. Defaults to REGIONAL for both primary and read instances.\nNote that primary and read instances can have different availability types.\nPrimary instances can be either ZONAL or REGIONAL. Read Pool instances can also be either ZONAL or REGIONAL.\nRead pools of size 1 can only have zonal availability. Read pools with a node count of 2 or more\ncan have regional availability (nodes are present in 2 or more zones in a region).\nPossible values are: 'AVAILABILITY_TYPE_UNSPECIFIED', 'ZONAL', 'REGIONAL'.' Possible values: [\"AVAILABILITY_TYPE_UNSPECIFIED\", \"ZONAL\", \"REGIONAL\"]"]
    pub fn availability_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cluster` after provisioning.\nIdentifies the alloydb cluster. Must be in the format\n'projects/{project}/locations/{location}/clusters/{cluster_id}'"]
    pub fn cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_flags` after provisioning.\nDatabase flags. Set at instance level. * They are copied from primary instance on read instance creation. * Read instances can set new or override existing flags that are relevant for reads, e.g. for enabling columnar cache on a read instance. Flags set on read instance may or may not be present on primary."]
    pub fn database_flags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.database_flags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-settable and human-readable display name for the Instance."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gce_zone` after provisioning.\nThe Compute Engine zone that the instance should serve from, per https://cloud.google.com/compute/docs/regions-zones This can ONLY be specified for ZONAL instances. If present for a REGIONAL instance, an error will be thrown. If this is absent for a ZONAL instance, instance is created in a random zone with available capacity."]
    pub fn gce_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gce_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe ID of the alloydb instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\nThe type of the instance.\nIf the instance type is READ_POOL, provide the associated PRIMARY/SECONDARY instance in the 'depends_on' meta-data attribute.\nIf the instance type is SECONDARY, point to the cluster_type of the associated secondary cluster instead of mentioning SECONDARY.\nExample: {instance_type = google_alloydb_cluster.<secondary_cluster_name>.cluster_type} instead of {instance_type = SECONDARY}\nIf the instance type is SECONDARY, the terraform delete instance operation does not delete the secondary instance but abandons it instead.\nUse deletion_policy = \"FORCE\" in the associated secondary cluster and delete the cluster forcefully to delete the secondary cluster as well its associated secondary instance.\nUsers can undo the delete secondary instance action by importing the deleted secondary instance by calling terraform import. Possible values: [\"PRIMARY\", \"READ_POOL\", \"SECONDARY\"]"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address for the Instance. This is the connection endpoint for an end-user application."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the alloydb instance.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the instance resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `outbound_public_ip_addresses` after provisioning.\nThe outbound public IP addresses for the instance. This is available ONLY when\nnetworkConfig.enableOutboundPublicIp is set to true. These IP addresses are used\nfor outbound connections."]
    pub fn outbound_public_ip_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.outbound_public_ip_addresses", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_ip_address` after provisioning.\nThe public IP addresses for the Instance. This is available ONLY when\nnetworkConfig.enablePublicIp is set to true. This is the connection\nendpoint for an end-user application."]
    pub fn public_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciling` after provisioning.\nSet to true if the current state of Instance does not match the user's intended state, and the service is actively updating the resource to reconcile them. This can happen due to user-triggered updates or system actions like failover or maintenance."]
    pub fn reconciling(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciling", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the alloydb instance."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe system-generated UID of the resource."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_connection_config` after provisioning.\n"]
    pub fn client_connection_config(&self) -> ListRef<AlloydbInstanceClientConnectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_connection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_pool_config` after provisioning.\n"]
    pub fn connection_pool_config(&self) -> ListRef<AlloydbInstanceConnectionPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_config` after provisioning.\n"]
    pub fn machine_config(&self) -> ListRef<AlloydbInstanceMachineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<AlloydbInstanceNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_instance_config` after provisioning.\n"]
    pub fn psc_instance_config(&self) -> ListRef<AlloydbInstancePscInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_instance_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_insights_config` after provisioning.\n"]
    pub fn query_insights_config(&self) -> ListRef<AlloydbInstanceQueryInsightsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_insights_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `read_pool_config` after provisioning.\n"]
    pub fn read_pool_config(&self) -> ListRef<AlloydbInstanceReadPoolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.read_pool_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AlloydbInstanceTimeoutsElRef {
        AlloydbInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AlloydbInstanceClientConnectionConfigElSslConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_mode: Option<PrimField<String>>,
}
impl AlloydbInstanceClientConnectionConfigElSslConfigEl {
    #[doc = "Set the field `ssl_mode`.\nSSL mode. Specifies client-server SSL/TLS connection behavior. Possible values: [\"ENCRYPTED_ONLY\", \"ALLOW_UNENCRYPTED_AND_ENCRYPTED\"]"]
    pub fn set_ssl_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ssl_mode = Some(v.into());
        self
    }
}
impl ToListMappable for AlloydbInstanceClientConnectionConfigElSslConfigEl {
    type O = BlockAssignable<AlloydbInstanceClientConnectionConfigElSslConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstanceClientConnectionConfigElSslConfigEl {}
impl BuildAlloydbInstanceClientConnectionConfigElSslConfigEl {
    pub fn build(self) -> AlloydbInstanceClientConnectionConfigElSslConfigEl {
        AlloydbInstanceClientConnectionConfigElSslConfigEl {
            ssl_mode: core::default::Default::default(),
        }
    }
}
pub struct AlloydbInstanceClientConnectionConfigElSslConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceClientConnectionConfigElSslConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AlloydbInstanceClientConnectionConfigElSslConfigElRef {
        AlloydbInstanceClientConnectionConfigElSslConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstanceClientConnectionConfigElSslConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ssl_mode` after provisioning.\nSSL mode. Specifies client-server SSL/TLS connection behavior. Possible values: [\"ENCRYPTED_ONLY\", \"ALLOW_UNENCRYPTED_AND_ENCRYPTED\"]"]
    pub fn ssl_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl_mode", self.base))
    }
}
#[derive(Serialize, Default)]
struct AlloydbInstanceClientConnectionConfigElDynamic {
    ssl_config: Option<DynamicBlock<AlloydbInstanceClientConnectionConfigElSslConfigEl>>,
}
#[derive(Serialize)]
pub struct AlloydbInstanceClientConnectionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    require_connectors: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_config: Option<Vec<AlloydbInstanceClientConnectionConfigElSslConfigEl>>,
    dynamic: AlloydbInstanceClientConnectionConfigElDynamic,
}
impl AlloydbInstanceClientConnectionConfigEl {
    #[doc = "Set the field `require_connectors`.\nConfiguration to enforce connectors only (ex: AuthProxy) connections to the database."]
    pub fn set_require_connectors(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_connectors = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        mut self,
        v: impl Into<BlockAssignable<AlloydbInstanceClientConnectionConfigElSslConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ssl_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AlloydbInstanceClientConnectionConfigEl {
    type O = BlockAssignable<AlloydbInstanceClientConnectionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstanceClientConnectionConfigEl {}
impl BuildAlloydbInstanceClientConnectionConfigEl {
    pub fn build(self) -> AlloydbInstanceClientConnectionConfigEl {
        AlloydbInstanceClientConnectionConfigEl {
            require_connectors: core::default::Default::default(),
            ssl_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AlloydbInstanceClientConnectionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceClientConnectionConfigElRef {
    fn new(shared: StackShared, base: String) -> AlloydbInstanceClientConnectionConfigElRef {
        AlloydbInstanceClientConnectionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstanceClientConnectionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `require_connectors` after provisioning.\nConfiguration to enforce connectors only (ex: AuthProxy) connections to the database."]
    pub fn require_connectors(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_connectors", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> ListRef<AlloydbInstanceClientConnectionConfigElSslConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.base))
    }
}
#[derive(Serialize)]
pub struct AlloydbInstanceConnectionPoolConfigEl {
    enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<RecField<PrimField<String>>>,
}
impl AlloydbInstanceConnectionPoolConfigEl {
    #[doc = "Set the field `flags`.\nFlags for configuring managed connection pooling when it is enabled.\nThese flags will only be set if 'connection_pool_config.enabled' is\ntrue.\nPlease see\nhttps://cloud.google.com/alloydb/docs/configure-managed-connection-pooling#configuration-options\nfor a comprehensive list of flags that can be set. To specify the flags\nin Terraform, please remove the \"connection-pooling-\" prefix and use\nunderscores instead of dashes in the name. For example,\n\"connection-pooling-pool-mode\" would be \"pool_mode\"."]
    pub fn set_flags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.flags = Some(v.into());
        self
    }
}
impl ToListMappable for AlloydbInstanceConnectionPoolConfigEl {
    type O = BlockAssignable<AlloydbInstanceConnectionPoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstanceConnectionPoolConfigEl {
    #[doc = "Whether to enabled Managed Connection Pool."]
    pub enabled: PrimField<bool>,
}
impl BuildAlloydbInstanceConnectionPoolConfigEl {
    pub fn build(self) -> AlloydbInstanceConnectionPoolConfigEl {
        AlloydbInstanceConnectionPoolConfigEl {
            enabled: self.enabled,
            flags: core::default::Default::default(),
        }
    }
}
pub struct AlloydbInstanceConnectionPoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceConnectionPoolConfigElRef {
    fn new(shared: StackShared, base: String) -> AlloydbInstanceConnectionPoolConfigElRef {
        AlloydbInstanceConnectionPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstanceConnectionPoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether to enabled Managed Connection Pool."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `flags` after provisioning.\nFlags for configuring managed connection pooling when it is enabled.\nThese flags will only be set if 'connection_pool_config.enabled' is\ntrue.\nPlease see\nhttps://cloud.google.com/alloydb/docs/configure-managed-connection-pooling#configuration-options\nfor a comprehensive list of flags that can be set. To specify the flags\nin Terraform, please remove the \"connection-pooling-\" prefix and use\nunderscores instead of dashes in the name. For example,\n\"connection-pooling-pool-mode\" would be \"pool_mode\"."]
    pub fn flags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.flags", self.base))
    }
    #[doc = "Get a reference to the value of field `pooler_count` after provisioning.\nThe number of running poolers per instance."]
    pub fn pooler_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pooler_count", self.base))
    }
}
#[derive(Serialize)]
pub struct AlloydbInstanceMachineConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
}
impl AlloydbInstanceMachineConfigEl {
    #[doc = "Set the field `cpu_count`.\nThe number of CPU's in the VM instance."]
    pub fn set_cpu_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_count = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nMachine type of the VM instance.\nE.g. \"n2-highmem-4\", \"n2-highmem-8\", \"c4a-highmem-4-lssd\".\n'cpu_count' must match the number of vCPUs in the machine type."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
}
impl ToListMappable for AlloydbInstanceMachineConfigEl {
    type O = BlockAssignable<AlloydbInstanceMachineConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstanceMachineConfigEl {}
impl BuildAlloydbInstanceMachineConfigEl {
    pub fn build(self) -> AlloydbInstanceMachineConfigEl {
        AlloydbInstanceMachineConfigEl {
            cpu_count: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}
pub struct AlloydbInstanceMachineConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceMachineConfigElRef {
    fn new(shared: StackShared, base: String) -> AlloydbInstanceMachineConfigElRef {
        AlloydbInstanceMachineConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstanceMachineConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_count` after provisioning.\nThe number of CPU's in the VM instance."]
    pub fn cpu_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_count", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nMachine type of the VM instance.\nE.g. \"n2-highmem-4\", \"n2-highmem-8\", \"c4a-highmem-4-lssd\".\n'cpu_count' must match the number of vCPUs in the machine type."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}
#[derive(Serialize)]
pub struct AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_range: Option<PrimField<String>>,
}
impl AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
    #[doc = "Set the field `cidr_range`.\nCIDR range for one authorized network of the instance."]
    pub fn set_cidr_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_range = Some(v.into());
        self
    }
}
impl ToListMappable for AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
    type O = BlockAssignable<AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {}
impl BuildAlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
    pub fn build(self) -> AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
        AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl {
            cidr_range: core::default::Default::default(),
        }
    }
}
pub struct AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
        AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cidr_range` after provisioning.\nCIDR range for one authorized network of the instance."]
    pub fn cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_range", self.base))
    }
}
#[derive(Serialize, Default)]
struct AlloydbInstanceNetworkConfigElDynamic {
    authorized_external_networks:
        Option<DynamicBlock<AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl>>,
}
#[derive(Serialize)]
pub struct AlloydbInstanceNetworkConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allocated_ip_range_override: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_outbound_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_public_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_external_networks:
        Option<Vec<AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl>>,
    dynamic: AlloydbInstanceNetworkConfigElDynamic,
}
impl AlloydbInstanceNetworkConfigEl {
    #[doc = "Set the field `allocated_ip_range_override`.\nName of the allocated IP range for the private IP AlloyDB instance, for example: \"google-managed-services-default\".\nIf set, the instance IPs will be created from this allocated range and will override the IP range used by the parent cluster.\nThe range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn set_allocated_ip_range_override(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allocated_ip_range_override = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_outbound_public_ip`.\nEnabling outbound public ip for the instance."]
    pub fn set_enable_outbound_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_outbound_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_public_ip`.\nEnabling public ip for the instance. If a user wishes to disable this,\nplease also clear the list of the authorized external networks set on\nthe same instance."]
    pub fn set_enable_public_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_public_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `authorized_external_networks`.\n"]
    pub fn set_authorized_external_networks(
        mut self,
        v: impl Into<BlockAssignable<AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authorized_external_networks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authorized_external_networks = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AlloydbInstanceNetworkConfigEl {
    type O = BlockAssignable<AlloydbInstanceNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstanceNetworkConfigEl {}
impl BuildAlloydbInstanceNetworkConfigEl {
    pub fn build(self) -> AlloydbInstanceNetworkConfigEl {
        AlloydbInstanceNetworkConfigEl {
            allocated_ip_range_override: core::default::Default::default(),
            enable_outbound_public_ip: core::default::Default::default(),
            enable_public_ip: core::default::Default::default(),
            authorized_external_networks: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AlloydbInstanceNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> AlloydbInstanceNetworkConfigElRef {
        AlloydbInstanceNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstanceNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocated_ip_range_override` after provisioning.\nName of the allocated IP range for the private IP AlloyDB instance, for example: \"google-managed-services-default\".\nIf set, the instance IPs will be created from this allocated range and will override the IP range used by the parent cluster.\nThe range name must comply with RFC 1035. Specifically, the name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?."]
    pub fn allocated_ip_range_override(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allocated_ip_range_override", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_outbound_public_ip` after provisioning.\nEnabling outbound public ip for the instance."]
    pub fn enable_outbound_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_outbound_public_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_public_ip` after provisioning.\nEnabling public ip for the instance. If a user wishes to disable this,\nplease also clear the list of the authorized external networks set on\nthe same instance."]
    pub fn enable_public_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_public_ip", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_external_networks` after provisioning.\n"]
    pub fn authorized_external_networks(
        &self,
    ) -> ListRef<AlloydbInstanceNetworkConfigElAuthorizedExternalNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_external_networks", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_project: Option<PrimField<String>>,
}
impl AlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
    #[doc = "Set the field `consumer_network`.\nThe consumer network for the PSC service automation, example:\n\"projects/vpc-host-project/global/networks/default\".\nThe consumer network might be hosted a different project than the\nconsumer project. The API expects the consumer project specified to be\nthe project ID (and not the project number)"]
    pub fn set_consumer_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_network = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_project`.\nThe consumer project to which the PSC service automation endpoint will\nbe created. The API expects the consumer project to be the project ID(\nand not the project number)."]
    pub fn set_consumer_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_project = Some(v.into());
        self
    }
}
impl ToListMappable for AlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
    type O = BlockAssignable<AlloydbInstancePscInstanceConfigElPscAutoConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {}
impl BuildAlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
    pub fn build(self) -> AlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
        AlloydbInstancePscInstanceConfigElPscAutoConnectionsEl {
            consumer_network: core::default::Default::default(),
            consumer_project: core::default::Default::default(),
        }
    }
}
pub struct AlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
        AlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consumer_network` after provisioning.\nThe consumer network for the PSC service automation, example:\n\"projects/vpc-host-project/global/networks/default\".\nThe consumer network might be hosted a different project than the\nconsumer project. The API expects the consumer project specified to be\nthe project ID (and not the project number)"]
    pub fn consumer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_network_status` after provisioning.\nThe status of the service connection policy."]
    pub fn consumer_network_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_network_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_project` after provisioning.\nThe consumer project to which the PSC service automation endpoint will\nbe created. The API expects the consumer project to be the project ID(\nand not the project number)."]
    pub fn consumer_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_project", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the PSC service automation endpoint."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the PSC service automation connection."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct AlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    network_attachment_resource: Option<PrimField<String>>,
}
impl AlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
    #[doc = "Set the field `network_attachment_resource`.\nThe network attachment resource created in the consumer project to which the PSC interface will be linked.\nThis is of the format: \"projects/${CONSUMER_PROJECT}/regions/${REGION}/networkAttachments/${NETWORK_ATTACHMENT_NAME}\".\nThe network attachment must be in the same region as the instance."]
    pub fn set_network_attachment_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_attachment_resource = Some(v.into());
        self
    }
}
impl ToListMappable for AlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
    type O = BlockAssignable<AlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {}
impl BuildAlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
    pub fn build(self) -> AlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
        AlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl {
            network_attachment_resource: core::default::Default::default(),
        }
    }
}
pub struct AlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
        AlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_attachment_resource` after provisioning.\nThe network attachment resource created in the consumer project to which the PSC interface will be linked.\nThis is of the format: \"projects/${CONSUMER_PROJECT}/regions/${REGION}/networkAttachments/${NETWORK_ATTACHMENT_NAME}\".\nThe network attachment must be in the same region as the instance."]
    pub fn network_attachment_resource(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment_resource", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AlloydbInstancePscInstanceConfigElDynamic {
    psc_auto_connections:
        Option<DynamicBlock<AlloydbInstancePscInstanceConfigElPscAutoConnectionsEl>>,
    psc_interface_configs:
        Option<DynamicBlock<AlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl>>,
}
#[derive(Serialize)]
pub struct AlloydbInstancePscInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_consumer_projects: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_auto_connections: Option<Vec<AlloydbInstancePscInstanceConfigElPscAutoConnectionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_interface_configs: Option<Vec<AlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl>>,
    dynamic: AlloydbInstancePscInstanceConfigElDynamic,
}
impl AlloydbInstancePscInstanceConfigEl {
    #[doc = "Set the field `allowed_consumer_projects`.\nList of consumer projects that are allowed to create PSC endpoints to service-attachments to this instance.\nThese should be specified as project numbers only."]
    pub fn set_allowed_consumer_projects(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_consumer_projects = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_auto_connections`.\n"]
    pub fn set_psc_auto_connections(
        mut self,
        v: impl Into<BlockAssignable<AlloydbInstancePscInstanceConfigElPscAutoConnectionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.psc_auto_connections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_auto_connections = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `psc_interface_configs`.\n"]
    pub fn set_psc_interface_configs(
        mut self,
        v: impl Into<BlockAssignable<AlloydbInstancePscInstanceConfigElPscInterfaceConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.psc_interface_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_interface_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AlloydbInstancePscInstanceConfigEl {
    type O = BlockAssignable<AlloydbInstancePscInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstancePscInstanceConfigEl {}
impl BuildAlloydbInstancePscInstanceConfigEl {
    pub fn build(self) -> AlloydbInstancePscInstanceConfigEl {
        AlloydbInstancePscInstanceConfigEl {
            allowed_consumer_projects: core::default::Default::default(),
            psc_auto_connections: core::default::Default::default(),
            psc_interface_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AlloydbInstancePscInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstancePscInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> AlloydbInstancePscInstanceConfigElRef {
        AlloydbInstancePscInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstancePscInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_consumer_projects` after provisioning.\nList of consumer projects that are allowed to create PSC endpoints to service-attachments to this instance.\nThese should be specified as project numbers only."]
    pub fn allowed_consumer_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_consumer_projects", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_dns_name` after provisioning.\nThe DNS name of the instance for PSC connectivity.\nName convention: <uid>.<uid>.<region>.alloydb-psc.goog"]
    pub fn psc_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.psc_dns_name", self.base))
    }
    #[doc = "Get a reference to the value of field `service_attachment_link` after provisioning.\nThe service attachment created when Private Service Connect (PSC) is enabled for the instance.\nThe name of the resource will be in the format of\n'projects/<alloydb-tenant-project-number>/regions/<region-name>/serviceAttachments/<service-attachment-name>'"]
    pub fn service_attachment_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment_link", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_auto_connections` after provisioning.\n"]
    pub fn psc_auto_connections(
        &self,
    ) -> ListRef<AlloydbInstancePscInstanceConfigElPscAutoConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_auto_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_interface_configs` after provisioning.\n"]
    pub fn psc_interface_configs(
        &self,
    ) -> ListRef<AlloydbInstancePscInstanceConfigElPscInterfaceConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_interface_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AlloydbInstanceQueryInsightsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_plans_per_minute: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_application_tags: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_client_address: Option<PrimField<bool>>,
}
impl AlloydbInstanceQueryInsightsConfigEl {
    #[doc = "Set the field `query_plans_per_minute`.\nNumber of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 20 is considered valid."]
    pub fn set_query_plans_per_minute(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_plans_per_minute = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_length`.\nQuery string length. The default value is 1024. Any integer between 256 and 4500 is considered valid."]
    pub fn set_query_string_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_string_length = Some(v.into());
        self
    }
    #[doc = "Set the field `record_application_tags`.\nRecord application tags for an instance. This flag is turned \"on\" by default."]
    pub fn set_record_application_tags(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.record_application_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `record_client_address`.\nRecord client address for an instance. Client address is PII information. This flag is turned \"on\" by default."]
    pub fn set_record_client_address(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.record_client_address = Some(v.into());
        self
    }
}
impl ToListMappable for AlloydbInstanceQueryInsightsConfigEl {
    type O = BlockAssignable<AlloydbInstanceQueryInsightsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstanceQueryInsightsConfigEl {}
impl BuildAlloydbInstanceQueryInsightsConfigEl {
    pub fn build(self) -> AlloydbInstanceQueryInsightsConfigEl {
        AlloydbInstanceQueryInsightsConfigEl {
            query_plans_per_minute: core::default::Default::default(),
            query_string_length: core::default::Default::default(),
            record_application_tags: core::default::Default::default(),
            record_client_address: core::default::Default::default(),
        }
    }
}
pub struct AlloydbInstanceQueryInsightsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceQueryInsightsConfigElRef {
    fn new(shared: StackShared, base: String) -> AlloydbInstanceQueryInsightsConfigElRef {
        AlloydbInstanceQueryInsightsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstanceQueryInsightsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `query_plans_per_minute` after provisioning.\nNumber of query execution plans captured by Insights per minute for all queries combined. The default value is 5. Any integer between 0 and 20 is considered valid."]
    pub fn query_plans_per_minute(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_plans_per_minute", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_length` after provisioning.\nQuery string length. The default value is 1024. Any integer between 256 and 4500 is considered valid."]
    pub fn query_string_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_string_length", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `record_application_tags` after provisioning.\nRecord application tags for an instance. This flag is turned \"on\" by default."]
    pub fn record_application_tags(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_application_tags", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `record_client_address` after provisioning.\nRecord client address for an instance. Client address is PII information. This flag is turned \"on\" by default."]
    pub fn record_client_address(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.record_client_address", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AlloydbInstanceReadPoolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    node_count: Option<PrimField<f64>>,
}
impl AlloydbInstanceReadPoolConfigEl {
    #[doc = "Set the field `node_count`.\nRead capacity, i.e. number of nodes in a read pool instance."]
    pub fn set_node_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.node_count = Some(v.into());
        self
    }
}
impl ToListMappable for AlloydbInstanceReadPoolConfigEl {
    type O = BlockAssignable<AlloydbInstanceReadPoolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstanceReadPoolConfigEl {}
impl BuildAlloydbInstanceReadPoolConfigEl {
    pub fn build(self) -> AlloydbInstanceReadPoolConfigEl {
        AlloydbInstanceReadPoolConfigEl {
            node_count: core::default::Default::default(),
        }
    }
}
pub struct AlloydbInstanceReadPoolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceReadPoolConfigElRef {
    fn new(shared: StackShared, base: String) -> AlloydbInstanceReadPoolConfigElRef {
        AlloydbInstanceReadPoolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstanceReadPoolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_count` after provisioning.\nRead capacity, i.e. number of nodes in a read pool instance."]
    pub fn node_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_count", self.base))
    }
}
#[derive(Serialize)]
pub struct AlloydbInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AlloydbInstanceTimeoutsEl {
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
impl ToListMappable for AlloydbInstanceTimeoutsEl {
    type O = BlockAssignable<AlloydbInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAlloydbInstanceTimeoutsEl {}
impl BuildAlloydbInstanceTimeoutsEl {
    pub fn build(self) -> AlloydbInstanceTimeoutsEl {
        AlloydbInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AlloydbInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AlloydbInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AlloydbInstanceTimeoutsElRef {
        AlloydbInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AlloydbInstanceTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct AlloydbInstanceDynamic {
    client_connection_config: Option<DynamicBlock<AlloydbInstanceClientConnectionConfigEl>>,
    connection_pool_config: Option<DynamicBlock<AlloydbInstanceConnectionPoolConfigEl>>,
    machine_config: Option<DynamicBlock<AlloydbInstanceMachineConfigEl>>,
    network_config: Option<DynamicBlock<AlloydbInstanceNetworkConfigEl>>,
    psc_instance_config: Option<DynamicBlock<AlloydbInstancePscInstanceConfigEl>>,
    query_insights_config: Option<DynamicBlock<AlloydbInstanceQueryInsightsConfigEl>>,
    read_pool_config: Option<DynamicBlock<AlloydbInstanceReadPoolConfigEl>>,
}
