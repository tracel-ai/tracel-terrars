use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkConnectivityPolicyBasedRouteData {
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
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_ilb_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_hop_other_routes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<NetworkConnectivityPolicyBasedRouteFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interconnect_attachment:
        Option<Vec<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkConnectivityPolicyBasedRouteTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_machine: Option<Vec<NetworkConnectivityPolicyBasedRouteVirtualMachineEl>>,
    dynamic: NetworkConnectivityPolicyBasedRouteDynamic,
}
struct NetworkConnectivityPolicyBasedRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkConnectivityPolicyBasedRouteData>,
}
#[derive(Clone)]
pub struct NetworkConnectivityPolicyBasedRoute(Rc<NetworkConnectivityPolicyBasedRoute_>);
impl NetworkConnectivityPolicyBasedRoute {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
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
    #[doc = "Set the field `next_hop_ilb_ip`.\nThe IP address of a global-access-enabled L4 ILB that is the next hop for matching packets."]
    pub fn set_next_hop_ilb_ip(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_ilb_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `next_hop_other_routes`.\nOther routes that will be referenced to determine the next hop of the packet. Possible values: [\"DEFAULT_ROUTING\"]"]
    pub fn set_next_hop_other_routes(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().next_hop_other_routes = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nThe priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivityPolicyBasedRouteFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `interconnect_attachment`.\n"]
    pub fn set_interconnect_attachment(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().interconnect_attachment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.interconnect_attachment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkConnectivityPolicyBasedRouteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_machine`.\n"]
    pub fn set_virtual_machine(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivityPolicyBasedRouteVirtualMachineEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().virtual_machine = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.virtual_machine = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when the policy-based route was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
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
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of this resource."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy based route."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nFully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_ilb_ip` after provisioning.\nThe IP address of a global-access-enabled L4 ILB that is the next hop for matching packets."]
    pub fn next_hop_ilb_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_ilb_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_other_routes` after provisioning.\nOther routes that will be referenced to determine the next hop of the packet. Possible values: [\"DEFAULT_ROUTING\"]"]
    pub fn next_hop_other_routes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_other_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime when the policy-based route was created."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `warnings` after provisioning.\nIf potential misconfigurations are detected for this route, this field will be populated with warning messages."]
    pub fn warnings(&self) -> ListRef<NetworkConnectivityPolicyBasedRouteWarningsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<NetworkConnectivityPolicyBasedRouteFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `interconnect_attachment` after provisioning.\n"]
    pub fn interconnect_attachment(
        &self,
    ) -> ListRef<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.interconnect_attachment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
        NetworkConnectivityPolicyBasedRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_machine` after provisioning.\n"]
    pub fn virtual_machine(
        &self,
    ) -> ListRef<NetworkConnectivityPolicyBasedRouteVirtualMachineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_machine", self.extract_ref()),
        )
    }
}
impl Referable for NetworkConnectivityPolicyBasedRoute {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkConnectivityPolicyBasedRoute {}
impl ToListMappable for NetworkConnectivityPolicyBasedRoute {
    type O = ListRef<NetworkConnectivityPolicyBasedRouteRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkConnectivityPolicyBasedRoute_ {
    fn extract_resource_type(&self) -> String {
        "google_network_connectivity_policy_based_route".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkConnectivityPolicyBasedRoute {
    pub tf_id: String,
    #[doc = "The name of the policy based route."]
    pub name: PrimField<String>,
    #[doc = "Fully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network."]
    pub network: PrimField<String>,
}
impl BuildNetworkConnectivityPolicyBasedRoute {
    pub fn build(self, stack: &mut Stack) -> NetworkConnectivityPolicyBasedRoute {
        let out =
            NetworkConnectivityPolicyBasedRoute(Rc::new(NetworkConnectivityPolicyBasedRoute_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(NetworkConnectivityPolicyBasedRouteData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    name: self.name,
                    network: self.network,
                    next_hop_ilb_ip: core::default::Default::default(),
                    next_hop_other_routes: core::default::Default::default(),
                    priority: core::default::Default::default(),
                    project: core::default::Default::default(),
                    filter: core::default::Default::default(),
                    interconnect_attachment: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    virtual_machine: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkConnectivityPolicyBasedRouteRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityPolicyBasedRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkConnectivityPolicyBasedRouteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime when the policy-based route was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
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
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of this resource."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kind", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy based route."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nFully-qualified URL of the network that this route applies to, for example: projects/my-project/global/networks/my-network."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_ilb_ip` after provisioning.\nThe IP address of a global-access-enabled L4 ILB that is the next hop for matching packets."]
    pub fn next_hop_ilb_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_ilb_ip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `next_hop_other_routes` after provisioning.\nOther routes that will be referenced to determine the next hop of the packet. Possible values: [\"DEFAULT_ROUTING\"]"]
    pub fn next_hop_other_routes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_hop_other_routes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe priority of this policy-based route. Priority is used to break ties in cases where there are more than one matching policy-based routes found. In cases where multiple policy-based routes are matched, the one with the lowest-numbered priority value wins. The default value is 1000. The priority value must be from 1 to 65535, inclusive."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.priority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime when the policy-based route was created."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `warnings` after provisioning.\nIf potential misconfigurations are detected for this route, this field will be populated with warning messages."]
    pub fn warnings(&self) -> ListRef<NetworkConnectivityPolicyBasedRouteWarningsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.warnings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<NetworkConnectivityPolicyBasedRouteFilterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `interconnect_attachment` after provisioning.\n"]
    pub fn interconnect_attachment(
        &self,
    ) -> ListRef<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.interconnect_attachment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
        NetworkConnectivityPolicyBasedRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_machine` after provisioning.\n"]
    pub fn virtual_machine(
        &self,
    ) -> ListRef<NetworkConnectivityPolicyBasedRouteVirtualMachineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_machine", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityPolicyBasedRouteWarningsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warning_message: Option<PrimField<String>>,
}
impl NetworkConnectivityPolicyBasedRouteWarningsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.data = Some(v.into());
        self
    }
    #[doc = "Set the field `warning_message`.\n"]
    pub fn set_warning_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.warning_message = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivityPolicyBasedRouteWarningsEl {
    type O = BlockAssignable<NetworkConnectivityPolicyBasedRouteWarningsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityPolicyBasedRouteWarningsEl {}
impl BuildNetworkConnectivityPolicyBasedRouteWarningsEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteWarningsEl {
        NetworkConnectivityPolicyBasedRouteWarningsEl {
            code: core::default::Default::default(),
            data: core::default::Default::default(),
            warning_message: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityPolicyBasedRouteWarningsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityPolicyBasedRouteWarningsElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivityPolicyBasedRouteWarningsElRef {
        NetworkConnectivityPolicyBasedRouteWarningsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityPolicyBasedRouteWarningsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.data", self.base))
    }
    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warning_message", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityPolicyBasedRouteFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_protocol: Option<PrimField<String>>,
    protocol_version: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_range: Option<PrimField<String>>,
}
impl NetworkConnectivityPolicyBasedRouteFilterEl {
    #[doc = "Set the field `dest_range`.\nThe destination IP range of outgoing packets that this policy-based route applies to. Default is \"0.0.0.0/0\"."]
    pub fn set_dest_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dest_range = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_protocol`.\nThe IP protocol that this policy-based route applies to. Valid values are 'TCP', 'UDP', and 'ALL'. Default is 'ALL'."]
    pub fn set_ip_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `src_range`.\nThe source IP range of outgoing packets that this policy-based route applies to. Default is \"0.0.0.0/0\"."]
    pub fn set_src_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.src_range = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivityPolicyBasedRouteFilterEl {
    type O = BlockAssignable<NetworkConnectivityPolicyBasedRouteFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityPolicyBasedRouteFilterEl {
    #[doc = "Internet protocol versions this policy-based route applies to. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub protocol_version: PrimField<String>,
}
impl BuildNetworkConnectivityPolicyBasedRouteFilterEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteFilterEl {
        NetworkConnectivityPolicyBasedRouteFilterEl {
            dest_range: core::default::Default::default(),
            ip_protocol: core::default::Default::default(),
            protocol_version: self.protocol_version,
            src_range: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityPolicyBasedRouteFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityPolicyBasedRouteFilterElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivityPolicyBasedRouteFilterElRef {
        NetworkConnectivityPolicyBasedRouteFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityPolicyBasedRouteFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dest_range` after provisioning.\nThe destination IP range of outgoing packets that this policy-based route applies to. Default is \"0.0.0.0/0\"."]
    pub fn dest_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dest_range", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol that this policy-based route applies to. Valid values are 'TCP', 'UDP', and 'ALL'. Default is 'ALL'."]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol_version` after provisioning.\nInternet protocol versions this policy-based route applies to. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn protocol_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `src_range` after provisioning.\nThe source IP range of outgoing packets that this policy-based route applies to. Default is \"0.0.0.0/0\"."]
    pub fn src_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.src_range", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
    region: PrimField<String>,
}
impl NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {}
impl ToListMappable for NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
    type O = BlockAssignable<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
    #[doc = "Cloud region to install this policy-based route on for Interconnect attachments. Use 'all' to install it on all Interconnect attachments."]
    pub region: PrimField<String>,
}
impl BuildNetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
        NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl {
            region: self.region,
        }
    }
}
pub struct NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
        NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityPolicyBasedRouteInterconnectAttachmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nCloud region to install this policy-based route on for Interconnect attachments. Use 'all' to install it on all Interconnect attachments."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityPolicyBasedRouteTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkConnectivityPolicyBasedRouteTimeoutsEl {
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
impl ToListMappable for NetworkConnectivityPolicyBasedRouteTimeoutsEl {
    type O = BlockAssignable<NetworkConnectivityPolicyBasedRouteTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityPolicyBasedRouteTimeoutsEl {}
impl BuildNetworkConnectivityPolicyBasedRouteTimeoutsEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteTimeoutsEl {
        NetworkConnectivityPolicyBasedRouteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
        NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityPolicyBasedRouteTimeoutsElRef {
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
pub struct NetworkConnectivityPolicyBasedRouteVirtualMachineEl {
    tags: ListField<PrimField<String>>,
}
impl NetworkConnectivityPolicyBasedRouteVirtualMachineEl {}
impl ToListMappable for NetworkConnectivityPolicyBasedRouteVirtualMachineEl {
    type O = BlockAssignable<NetworkConnectivityPolicyBasedRouteVirtualMachineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityPolicyBasedRouteVirtualMachineEl {
    #[doc = "A list of VM instance tags that this policy-based route applies to. VM instances that have ANY of tags specified here will install this PBR."]
    pub tags: ListField<PrimField<String>>,
}
impl BuildNetworkConnectivityPolicyBasedRouteVirtualMachineEl {
    pub fn build(self) -> NetworkConnectivityPolicyBasedRouteVirtualMachineEl {
        NetworkConnectivityPolicyBasedRouteVirtualMachineEl { tags: self.tags }
    }
}
pub struct NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
        NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityPolicyBasedRouteVirtualMachineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nA list of VM instance tags that this policy-based route applies to. VM instances that have ANY of tags specified here will install this PBR."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkConnectivityPolicyBasedRouteDynamic {
    filter: Option<DynamicBlock<NetworkConnectivityPolicyBasedRouteFilterEl>>,
    interconnect_attachment:
        Option<DynamicBlock<NetworkConnectivityPolicyBasedRouteInterconnectAttachmentEl>>,
    virtual_machine: Option<DynamicBlock<NetworkConnectivityPolicyBasedRouteVirtualMachineEl>>,
}
