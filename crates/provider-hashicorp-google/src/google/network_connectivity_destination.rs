use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkConnectivityDestinationData {
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
    ip_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    multicloud_data_transfer_config: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<Vec<NetworkConnectivityDestinationEndpointsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkConnectivityDestinationTimeoutsEl>,
    dynamic: NetworkConnectivityDestinationDynamic,
}
struct NetworkConnectivityDestination_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkConnectivityDestinationData>,
}
#[derive(Clone)]
pub struct NetworkConnectivityDestination(Rc<NetworkConnectivityDestination_>);
impl NetworkConnectivityDestination {
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
    #[doc = "Set the field `description`.\nA description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivityDestinationEndpointsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoints = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkConnectivityDestinationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when the 'Destination' resource was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this resource."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag is computed by the server, and might be sent with update and\ndelete requests so that the client has an up-to-date value before\nproceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_prefix` after provisioning.\nThe IP prefix that represents your workload on another CSP."]
    pub fn ip_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the destination."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicloud_data_transfer_config` after provisioning.\nThe multicloud data transfer config of the destination."]
    pub fn multicloud_data_transfer_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicloud_data_transfer_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the destination."]
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
    #[doc = "Get a reference to the value of field `state_timeline` after provisioning.\nThe timeline of the expected 'Destination' states or the current rest\nstate. If a state change is expected, the value is 'ADDING',\n'DELETING' or 'SUSPENDING', depending on the action specified."]
    pub fn state_timeline(&self) -> ListRef<NetworkConnectivityDestinationStateTimelineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state_timeline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe Google-generated unique ID for the 'Destination' resource.\nThis value is unique across all 'Destination' resources.\nIf a resource is deleted and another with the same name is\ncreated, the new resource is assigned a different and unique ID."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime when the 'Destination' resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityDestinationTimeoutsElRef {
        NetworkConnectivityDestinationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkConnectivityDestination {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkConnectivityDestination {}
impl ToListMappable for NetworkConnectivityDestination {
    type O = ListRef<NetworkConnectivityDestinationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkConnectivityDestination_ {
    fn extract_resource_type(&self) -> String {
        "google_network_connectivity_destination".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkConnectivityDestination {
    pub tf_id: String,
    #[doc = "The IP prefix that represents your workload on another CSP."]
    pub ip_prefix: PrimField<String>,
    #[doc = "The location of the destination."]
    pub location: PrimField<String>,
    #[doc = "The multicloud data transfer config of the destination."]
    pub multicloud_data_transfer_config: PrimField<String>,
    #[doc = "The name of the destination."]
    pub name: PrimField<String>,
}
impl BuildNetworkConnectivityDestination {
    pub fn build(self, stack: &mut Stack) -> NetworkConnectivityDestination {
        let out = NetworkConnectivityDestination(Rc::new(NetworkConnectivityDestination_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkConnectivityDestinationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_prefix: self.ip_prefix,
                labels: core::default::Default::default(),
                location: self.location,
                multicloud_data_transfer_config: self.multicloud_data_transfer_config,
                name: self.name,
                project: core::default::Default::default(),
                endpoints: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkConnectivityDestinationRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityDestinationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkConnectivityDestinationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when the 'Destination' resource was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of this resource."]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag is computed by the server, and might be sent with update and\ndelete requests so that the client has an up-to-date value before\nproceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ip_prefix` after provisioning.\nThe IP prefix that represents your workload on another CSP."]
    pub fn ip_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_prefix", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the destination."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicloud_data_transfer_config` after provisioning.\nThe multicloud data transfer config of the destination."]
    pub fn multicloud_data_transfer_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicloud_data_transfer_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the destination."]
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
    #[doc = "Get a reference to the value of field `state_timeline` after provisioning.\nThe timeline of the expected 'Destination' states or the current rest\nstate. If a state change is expected, the value is 'ADDING',\n'DELETING' or 'SUSPENDING', depending on the action specified."]
    pub fn state_timeline(&self) -> ListRef<NetworkConnectivityDestinationStateTimelineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.state_timeline", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nThe Google-generated unique ID for the 'Destination' resource.\nThis value is unique across all 'Destination' resources.\nIf a resource is deleted and another with the same name is\ncreated, the new resource is assigned a different and unique ID."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime when the 'Destination' resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityDestinationTimeoutsElRef {
        NetworkConnectivityDestinationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityDestinationStateTimelineElStatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl NetworkConnectivityDestinationStateTimelineElStatesEl {
    #[doc = "Set the field `effective_time`.\n"]
    pub fn set_effective_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effective_time = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivityDestinationStateTimelineElStatesEl {
    type O = BlockAssignable<NetworkConnectivityDestinationStateTimelineElStatesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityDestinationStateTimelineElStatesEl {}
impl BuildNetworkConnectivityDestinationStateTimelineElStatesEl {
    pub fn build(self) -> NetworkConnectivityDestinationStateTimelineElStatesEl {
        NetworkConnectivityDestinationStateTimelineElStatesEl {
            effective_time: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityDestinationStateTimelineElStatesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityDestinationStateTimelineElStatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivityDestinationStateTimelineElStatesElRef {
        NetworkConnectivityDestinationStateTimelineElStatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityDestinationStateTimelineElStatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `effective_time` after provisioning.\n"]
    pub fn effective_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.effective_time", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityDestinationStateTimelineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    states: Option<ListField<NetworkConnectivityDestinationStateTimelineElStatesEl>>,
}
impl NetworkConnectivityDestinationStateTimelineEl {
    #[doc = "Set the field `states`.\n"]
    pub fn set_states(
        mut self,
        v: impl Into<ListField<NetworkConnectivityDestinationStateTimelineElStatesEl>>,
    ) -> Self {
        self.states = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivityDestinationStateTimelineEl {
    type O = BlockAssignable<NetworkConnectivityDestinationStateTimelineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityDestinationStateTimelineEl {}
impl BuildNetworkConnectivityDestinationStateTimelineEl {
    pub fn build(self) -> NetworkConnectivityDestinationStateTimelineEl {
        NetworkConnectivityDestinationStateTimelineEl {
            states: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityDestinationStateTimelineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityDestinationStateTimelineElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivityDestinationStateTimelineElRef {
        NetworkConnectivityDestinationStateTimelineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityDestinationStateTimelineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `states` after provisioning.\n"]
    pub fn states(&self) -> ListRef<NetworkConnectivityDestinationStateTimelineElStatesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.states", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityDestinationEndpointsEl {
    asn: PrimField<String>,
    csp: PrimField<String>,
}
impl NetworkConnectivityDestinationEndpointsEl {}
impl ToListMappable for NetworkConnectivityDestinationEndpointsEl {
    type O = BlockAssignable<NetworkConnectivityDestinationEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityDestinationEndpointsEl {
    #[doc = "The ASN of the remote IP prefix."]
    pub asn: PrimField<String>,
    #[doc = "The CSP of the remote IP prefix."]
    pub csp: PrimField<String>,
}
impl BuildNetworkConnectivityDestinationEndpointsEl {
    pub fn build(self) -> NetworkConnectivityDestinationEndpointsEl {
        NetworkConnectivityDestinationEndpointsEl {
            asn: self.asn,
            csp: self.csp,
        }
    }
}
pub struct NetworkConnectivityDestinationEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityDestinationEndpointsElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivityDestinationEndpointsElRef {
        NetworkConnectivityDestinationEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityDestinationEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asn` after provisioning.\nThe ASN of the remote IP prefix."]
    pub fn asn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }
    #[doc = "Get a reference to the value of field `csp` after provisioning.\nThe CSP of the remote IP prefix."]
    pub fn csp(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.csp", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the DestinationEndpoint resource."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime when the DestinationEndpoint resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityDestinationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkConnectivityDestinationTimeoutsEl {
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
impl ToListMappable for NetworkConnectivityDestinationTimeoutsEl {
    type O = BlockAssignable<NetworkConnectivityDestinationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityDestinationTimeoutsEl {}
impl BuildNetworkConnectivityDestinationTimeoutsEl {
    pub fn build(self) -> NetworkConnectivityDestinationTimeoutsEl {
        NetworkConnectivityDestinationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityDestinationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityDestinationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivityDestinationTimeoutsElRef {
        NetworkConnectivityDestinationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityDestinationTimeoutsElRef {
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
struct NetworkConnectivityDestinationDynamic {
    endpoints: Option<DynamicBlock<NetworkConnectivityDestinationEndpointsEl>>,
}
