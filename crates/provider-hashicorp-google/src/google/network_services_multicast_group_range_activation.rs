use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesMulticastGroupRangeActivationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    multicast_domain_activation: PrimField<String>,
    multicast_group_range: PrimField<String>,
    multicast_group_range_activation_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<NetworkServicesMulticastGroupRangeActivationLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesMulticastGroupRangeActivationTimeoutsEl>,
    dynamic: NetworkServicesMulticastGroupRangeActivationDynamic,
}
struct NetworkServicesMulticastGroupRangeActivation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesMulticastGroupRangeActivationData>,
}
#[derive(Clone)]
pub struct NetworkServicesMulticastGroupRangeActivation(
    Rc<NetworkServicesMulticastGroupRangeActivation_>,
);
impl NetworkServicesMulticastGroupRangeActivation {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional text description of the multicast group range activation."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key-value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl Into<BlockAssignable<NetworkServicesMulticastGroupRangeActivationLogConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<NetworkServicesMulticastGroupRangeActivationTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] The timestamp when the multicast group range activation was\ncreated."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional text description of the multicast group range activation."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n[Output only] The multicast group IP address range."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key-value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_domain_activation` after provisioning.\nThe resource name of a multicast domain activation that is in the\nsame zone as this multicast group.\nUse the following format:\n'projects/*/locations/*/multicastDomainActivations/*'"]
    pub fn multicast_domain_activation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_domain_activation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_group_consumer_activations` after provisioning.\nThe resource names of associated multicast group consumer activations.\nUse the following format:\n'projects/*/locations/*/multicastGroupConsumerActivations/*'."]
    pub fn multicast_group_consumer_activations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.multicast_group_consumer_activations",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_group_range` after provisioning.\nThe resource name of the global multicast group range for the\ngroup. Use the following format:\n'projects/*/locations/global/multicastGroupRanges/*'"]
    pub fn multicast_group_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_group_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_group_range_activation_id` after provisioning.\nA unique name for the multicast group range activation.\nThe name is restricted to letters, numbers, and hyphen, with the first\ncharacter a letter, and the last a letter or a number. The name must not\nexceed 48 characters."]
    pub fn multicast_group_range_activation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_group_range_activation_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the multicast group range activation.\nUse the following format:\n'projects/*/locations/*/multicastGroupRangeActivations/*'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe multicast resource's state."]
    pub fn state(&self) -> ListRef<NetworkServicesMulticastGroupRangeActivationStateElRef> {
        ListRef::new(
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
    #[doc = "Get a reference to the value of field `unique_id` after provisioning.\n[Output only] The Google-generated UUID for the resource. This value is\nunique across all multicast group resources. If a group is deleted and\nanother with the same name is created, the new group is assigned a\ndifferent unique_id."]
    pub fn unique_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] The timestamp when the multicast group range activation was\nmost recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(
        &self,
    ) -> ListRef<NetworkServicesMulticastGroupRangeActivationLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesMulticastGroupRangeActivationTimeoutsElRef {
        NetworkServicesMulticastGroupRangeActivationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesMulticastGroupRangeActivation {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesMulticastGroupRangeActivation {}
impl ToListMappable for NetworkServicesMulticastGroupRangeActivation {
    type O = ListRef<NetworkServicesMulticastGroupRangeActivationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesMulticastGroupRangeActivation_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_multicast_group_range_activation".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesMulticastGroupRangeActivation {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The resource name of a multicast domain activation that is in the\nsame zone as this multicast group.\nUse the following format:\n'projects/*/locations/*/multicastDomainActivations/*'"]
    pub multicast_domain_activation: PrimField<String>,
    #[doc = "The resource name of the global multicast group range for the\ngroup. Use the following format:\n'projects/*/locations/global/multicastGroupRanges/*'"]
    pub multicast_group_range: PrimField<String>,
    #[doc = "A unique name for the multicast group range activation.\nThe name is restricted to letters, numbers, and hyphen, with the first\ncharacter a letter, and the last a letter or a number. The name must not\nexceed 48 characters."]
    pub multicast_group_range_activation_id: PrimField<String>,
}
impl BuildNetworkServicesMulticastGroupRangeActivation {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesMulticastGroupRangeActivation {
        let out = NetworkServicesMulticastGroupRangeActivation(Rc::new(
            NetworkServicesMulticastGroupRangeActivation_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(NetworkServicesMulticastGroupRangeActivationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    multicast_domain_activation: self.multicast_domain_activation,
                    multicast_group_range: self.multicast_group_range,
                    multicast_group_range_activation_id: self.multicast_group_range_activation_id,
                    project: core::default::Default::default(),
                    log_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesMulticastGroupRangeActivationRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesMulticastGroupRangeActivationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesMulticastGroupRangeActivationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] The timestamp when the multicast group range activation was\ncreated."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional text description of the multicast group range activation."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n[Output only] The multicast group IP address range."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_cidr_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key-value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_domain_activation` after provisioning.\nThe resource name of a multicast domain activation that is in the\nsame zone as this multicast group.\nUse the following format:\n'projects/*/locations/*/multicastDomainActivations/*'"]
    pub fn multicast_domain_activation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_domain_activation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_group_consumer_activations` after provisioning.\nThe resource names of associated multicast group consumer activations.\nUse the following format:\n'projects/*/locations/*/multicastGroupConsumerActivations/*'."]
    pub fn multicast_group_consumer_activations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.multicast_group_consumer_activations",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_group_range` after provisioning.\nThe resource name of the global multicast group range for the\ngroup. Use the following format:\n'projects/*/locations/global/multicastGroupRanges/*'"]
    pub fn multicast_group_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_group_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_group_range_activation_id` after provisioning.\nA unique name for the multicast group range activation.\nThe name is restricted to letters, numbers, and hyphen, with the first\ncharacter a letter, and the last a letter or a number. The name must not\nexceed 48 characters."]
    pub fn multicast_group_range_activation_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_group_range_activation_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the multicast group range activation.\nUse the following format:\n'projects/*/locations/*/multicastGroupRangeActivations/*'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe multicast resource's state."]
    pub fn state(&self) -> ListRef<NetworkServicesMulticastGroupRangeActivationStateElRef> {
        ListRef::new(
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
    #[doc = "Get a reference to the value of field `unique_id` after provisioning.\n[Output only] The Google-generated UUID for the resource. This value is\nunique across all multicast group resources. If a group is deleted and\nanother with the same name is created, the new group is assigned a\ndifferent unique_id."]
    pub fn unique_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] The timestamp when the multicast group range activation was\nmost recently updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(
        &self,
    ) -> ListRef<NetworkServicesMulticastGroupRangeActivationLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesMulticastGroupRangeActivationTimeoutsElRef {
        NetworkServicesMulticastGroupRangeActivationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesMulticastGroupRangeActivationStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl NetworkServicesMulticastGroupRangeActivationStateEl {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesMulticastGroupRangeActivationStateEl {
    type O = BlockAssignable<NetworkServicesMulticastGroupRangeActivationStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesMulticastGroupRangeActivationStateEl {}
impl BuildNetworkServicesMulticastGroupRangeActivationStateEl {
    pub fn build(self) -> NetworkServicesMulticastGroupRangeActivationStateEl {
        NetworkServicesMulticastGroupRangeActivationStateEl {
            state: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesMulticastGroupRangeActivationStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesMulticastGroupRangeActivationStateElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesMulticastGroupRangeActivationStateElRef {
        NetworkServicesMulticastGroupRangeActivationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesMulticastGroupRangeActivationStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesMulticastGroupRangeActivationLogConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl NetworkServicesMulticastGroupRangeActivationLogConfigEl {
    #[doc = "Set the field `enabled`.\nWhether to enable logging or not."]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesMulticastGroupRangeActivationLogConfigEl {
    type O = BlockAssignable<NetworkServicesMulticastGroupRangeActivationLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesMulticastGroupRangeActivationLogConfigEl {}
impl BuildNetworkServicesMulticastGroupRangeActivationLogConfigEl {
    pub fn build(self) -> NetworkServicesMulticastGroupRangeActivationLogConfigEl {
        NetworkServicesMulticastGroupRangeActivationLogConfigEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesMulticastGroupRangeActivationLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesMulticastGroupRangeActivationLogConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesMulticastGroupRangeActivationLogConfigElRef {
        NetworkServicesMulticastGroupRangeActivationLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesMulticastGroupRangeActivationLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether to enable logging or not."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesMulticastGroupRangeActivationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesMulticastGroupRangeActivationTimeoutsEl {
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
impl ToListMappable for NetworkServicesMulticastGroupRangeActivationTimeoutsEl {
    type O = BlockAssignable<NetworkServicesMulticastGroupRangeActivationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesMulticastGroupRangeActivationTimeoutsEl {}
impl BuildNetworkServicesMulticastGroupRangeActivationTimeoutsEl {
    pub fn build(self) -> NetworkServicesMulticastGroupRangeActivationTimeoutsEl {
        NetworkServicesMulticastGroupRangeActivationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesMulticastGroupRangeActivationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesMulticastGroupRangeActivationTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesMulticastGroupRangeActivationTimeoutsElRef {
        NetworkServicesMulticastGroupRangeActivationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesMulticastGroupRangeActivationTimeoutsElRef {
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
struct NetworkServicesMulticastGroupRangeActivationDynamic {
    log_config: Option<DynamicBlock<NetworkServicesMulticastGroupRangeActivationLogConfigEl>>,
}
