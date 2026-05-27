use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkManagementConnectivityTestData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_firewall_checks: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_projects: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    round_trip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<Vec<NetworkManagementConnectivityTestDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<NetworkManagementConnectivityTestSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkManagementConnectivityTestTimeoutsEl>,
    dynamic: NetworkManagementConnectivityTestDynamic,
}
struct NetworkManagementConnectivityTest_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkManagementConnectivityTestData>,
}
#[derive(Clone)]
pub struct NetworkManagementConnectivityTest(Rc<NetworkManagementConnectivityTest_>);
impl NetworkManagementConnectivityTest {
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
    #[doc = "Set the field `bypass_firewall_checks`.\nWhether the analysis should skip firewall checking. Default value is false."]
    pub fn set_bypass_firewall_checks(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().bypass_firewall_checks = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe user-supplied description of the Connectivity Test.\nMaximum of 512 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\nIP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub fn set_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `related_projects`.\nOther projects that may be relevant for reachability analysis.\nThis is applicable to scenarios where a test can cross project\nboundaries."]
    pub fn set_related_projects(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().related_projects = Some(v.into());
        self
    }
    #[doc = "Set the field `round_trip`.\nWhether run analysis for the return path from destination to source.\nDefault value is false."]
    pub fn set_round_trip(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().round_trip = Some(v.into());
        self
    }
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        self,
        v: impl Into<BlockAssignable<NetworkManagementConnectivityTestDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        self,
        v: impl Into<BlockAssignable<NetworkManagementConnectivityTestSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkManagementConnectivityTestTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bypass_firewall_checks` after provisioning.\nWhether the analysis should skip firewall checking. Default value is false."]
    pub fn bypass_firewall_checks(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bypass_firewall_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe user-supplied description of the Connectivity Test.\nMaximum of 512 characters."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
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
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nIP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `related_projects` after provisioning.\nOther projects that may be relevant for reachability analysis.\nThis is applicable to scenarios where a test can cross project\nboundaries."]
    pub fn related_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.related_projects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `round_trip` after provisioning.\nWhether run analysis for the return path from destination to source.\nDefault value is false."]
    pub fn round_trip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.round_trip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<NetworkManagementConnectivityTestDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<NetworkManagementConnectivityTestSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkManagementConnectivityTestTimeoutsElRef {
        NetworkManagementConnectivityTestTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkManagementConnectivityTest {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkManagementConnectivityTest {}
impl ToListMappable for NetworkManagementConnectivityTest {
    type O = ListRef<NetworkManagementConnectivityTestRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkManagementConnectivityTest_ {
    fn extract_resource_type(&self) -> String {
        "google_network_management_connectivity_test".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkManagementConnectivityTest {
    pub tf_id: String,
    #[doc = "Unique name for the connectivity test."]
    pub name: PrimField<String>,
}
impl BuildNetworkManagementConnectivityTest {
    pub fn build(self, stack: &mut Stack) -> NetworkManagementConnectivityTest {
        let out = NetworkManagementConnectivityTest(Rc::new(NetworkManagementConnectivityTest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkManagementConnectivityTestData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bypass_firewall_checks: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                related_projects: core::default::Default::default(),
                round_trip: core::default::Default::default(),
                destination: core::default::Default::default(),
                source: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkManagementConnectivityTestRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkManagementConnectivityTestRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkManagementConnectivityTestRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bypass_firewall_checks` after provisioning.\nWhether the analysis should skip firewall checking. Default value is false."]
    pub fn bypass_firewall_checks(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bypass_firewall_checks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe user-supplied description of the Connectivity Test.\nMaximum of 512 characters."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user-provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the connectivity test."]
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
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nIP Protocol of the test. When not provided, \"TCP\" is assumed."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `related_projects` after provisioning.\nOther projects that may be relevant for reachability analysis.\nThis is applicable to scenarios where a test can cross project\nboundaries."]
    pub fn related_projects(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.related_projects", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `round_trip` after provisioning.\nWhether run analysis for the return path from destination to source.\nDefault value is false."]
    pub fn round_trip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.round_trip", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> ListRef<NetworkManagementConnectivityTestDestinationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> ListRef<NetworkManagementConnectivityTestSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkManagementConnectivityTestTimeoutsElRef {
        NetworkManagementConnectivityTestTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkManagementConnectivityTestDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fqdn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_master_cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redis_instance: Option<PrimField<String>>,
}
impl NetworkManagementConnectivityTestDestinationEl {
    #[doc = "Set the field `cloud_sql_instance`.\nA Cloud SQL instance URI."]
    pub fn set_cloud_sql_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_sql_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_rule`.\nForwarding rule URI. Forwarding rules are frontends for load balancers,\nPSC endpoints, and Protocol Forwarding."]
    pub fn set_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `fqdn`.\nA DNS endpoint of Google Kubernetes Engine cluster control plane.\nRequires gke_master_cluster to be set, can't be used simultaneoulsly with\nip_address or network. Applicable only to destination endpoint."]
    pub fn set_fqdn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fqdn = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_master_cluster`.\nA cluster URI for Google Kubernetes Engine cluster control plane."]
    pub fn set_gke_master_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gke_master_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `instance`.\nA Compute Engine instance URI."]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\nThe IP address of the endpoint, which can be an external or internal IP."]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nA VPC network URI."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe IP protocol port of the endpoint. Only applicable when protocol is\nTCP or UDP."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nProject ID where the endpoint is located.\nThe project ID can be derived from the URI if you provide a endpoint or\nnetwork URI.\nThe following are two cases where you may need to provide the project ID:\n1. Only the IP address is specified, and the IP address is within a Google\nCloud project.\n2. When you are using Shared VPC and the IP address that you provide is\nfrom the service project. In this case, the network that the IP address\nresides in is defined in the host project."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `redis_cluster`.\nA Redis Cluster URI."]
    pub fn set_redis_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redis_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `redis_instance`.\nA Redis Instance URI."]
    pub fn set_redis_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redis_instance = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkManagementConnectivityTestDestinationEl {
    type O = BlockAssignable<NetworkManagementConnectivityTestDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkManagementConnectivityTestDestinationEl {}
impl BuildNetworkManagementConnectivityTestDestinationEl {
    pub fn build(self) -> NetworkManagementConnectivityTestDestinationEl {
        NetworkManagementConnectivityTestDestinationEl {
            cloud_sql_instance: core::default::Default::default(),
            forwarding_rule: core::default::Default::default(),
            fqdn: core::default::Default::default(),
            gke_master_cluster: core::default::Default::default(),
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            redis_cluster: core::default::Default::default(),
            redis_instance: core::default::Default::default(),
        }
    }
}
pub struct NetworkManagementConnectivityTestDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkManagementConnectivityTestDestinationElRef {
    fn new(shared: StackShared, base: String) -> NetworkManagementConnectivityTestDestinationElRef {
        NetworkManagementConnectivityTestDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkManagementConnectivityTestDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\nA Cloud SQL instance URI."]
    pub fn cloud_sql_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\nForwarding rule URI. Forwarding rules are frontends for load balancers,\nPSC endpoints, and Protocol Forwarding."]
    pub fn forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\nA DNS endpoint of Google Kubernetes Engine cluster control plane.\nRequires gke_master_cluster to be set, can't be used simultaneoulsly with\nip_address or network. Applicable only to destination endpoint."]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fqdn", self.base))
    }
    #[doc = "Get a reference to the value of field `gke_master_cluster` after provisioning.\nA cluster URI for Google Kubernetes Engine cluster control plane."]
    pub fn gke_master_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gke_master_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nA Compute Engine instance URI."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the endpoint, which can be an external or internal IP."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nA VPC network URI."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe IP protocol port of the endpoint. Only applicable when protocol is\nTCP or UDP."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject ID where the endpoint is located.\nThe project ID can be derived from the URI if you provide a endpoint or\nnetwork URI.\nThe following are two cases where you may need to provide the project ID:\n1. Only the IP address is specified, and the IP address is within a Google\nCloud project.\n2. When you are using Shared VPC and the IP address that you provide is\nfrom the service project. In this case, the network that the IP address\nresides in is defined in the host project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `redis_cluster` after provisioning.\nA Redis Cluster URI."]
    pub fn redis_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redis_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redis_instance` after provisioning.\nA Redis Instance URI."]
    pub fn redis_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redis_instance", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkManagementConnectivityTestSourceElAppEngineVersionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl NetworkManagementConnectivityTestSourceElAppEngineVersionEl {
    #[doc = "Set the field `uri`.\nAn App Engine service version name."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkManagementConnectivityTestSourceElAppEngineVersionEl {
    type O = BlockAssignable<NetworkManagementConnectivityTestSourceElAppEngineVersionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkManagementConnectivityTestSourceElAppEngineVersionEl {}
impl BuildNetworkManagementConnectivityTestSourceElAppEngineVersionEl {
    pub fn build(self) -> NetworkManagementConnectivityTestSourceElAppEngineVersionEl {
        NetworkManagementConnectivityTestSourceElAppEngineVersionEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct NetworkManagementConnectivityTestSourceElAppEngineVersionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkManagementConnectivityTestSourceElAppEngineVersionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkManagementConnectivityTestSourceElAppEngineVersionElRef {
        NetworkManagementConnectivityTestSourceElAppEngineVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkManagementConnectivityTestSourceElAppEngineVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nAn App Engine service version name."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkManagementConnectivityTestSourceElCloudFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl NetworkManagementConnectivityTestSourceElCloudFunctionEl {
    #[doc = "Set the field `uri`.\nA Cloud Function name."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkManagementConnectivityTestSourceElCloudFunctionEl {
    type O = BlockAssignable<NetworkManagementConnectivityTestSourceElCloudFunctionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkManagementConnectivityTestSourceElCloudFunctionEl {}
impl BuildNetworkManagementConnectivityTestSourceElCloudFunctionEl {
    pub fn build(self) -> NetworkManagementConnectivityTestSourceElCloudFunctionEl {
        NetworkManagementConnectivityTestSourceElCloudFunctionEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct NetworkManagementConnectivityTestSourceElCloudFunctionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkManagementConnectivityTestSourceElCloudFunctionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkManagementConnectivityTestSourceElCloudFunctionElRef {
        NetworkManagementConnectivityTestSourceElCloudFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkManagementConnectivityTestSourceElCloudFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nA Cloud Function name."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkManagementConnectivityTestSourceElCloudRunRevisionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl NetworkManagementConnectivityTestSourceElCloudRunRevisionEl {
    #[doc = "Set the field `uri`.\nA Cloud Run revision URI."]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkManagementConnectivityTestSourceElCloudRunRevisionEl {
    type O = BlockAssignable<NetworkManagementConnectivityTestSourceElCloudRunRevisionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkManagementConnectivityTestSourceElCloudRunRevisionEl {}
impl BuildNetworkManagementConnectivityTestSourceElCloudRunRevisionEl {
    pub fn build(self) -> NetworkManagementConnectivityTestSourceElCloudRunRevisionEl {
        NetworkManagementConnectivityTestSourceElCloudRunRevisionEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct NetworkManagementConnectivityTestSourceElCloudRunRevisionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkManagementConnectivityTestSourceElCloudRunRevisionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkManagementConnectivityTestSourceElCloudRunRevisionElRef {
        NetworkManagementConnectivityTestSourceElCloudRunRevisionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkManagementConnectivityTestSourceElCloudRunRevisionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nA Cloud Run revision URI."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkManagementConnectivityTestSourceElDynamic {
    app_engine_version:
        Option<DynamicBlock<NetworkManagementConnectivityTestSourceElAppEngineVersionEl>>,
    cloud_function: Option<DynamicBlock<NetworkManagementConnectivityTestSourceElCloudFunctionEl>>,
    cloud_run_revision:
        Option<DynamicBlock<NetworkManagementConnectivityTestSourceElCloudRunRevisionEl>>,
}
#[derive(Serialize)]
pub struct NetworkManagementConnectivityTestSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_sql_instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_master_cluster: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_engine_version: Option<Vec<NetworkManagementConnectivityTestSourceElAppEngineVersionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_function: Option<Vec<NetworkManagementConnectivityTestSourceElCloudFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_run_revision: Option<Vec<NetworkManagementConnectivityTestSourceElCloudRunRevisionEl>>,
    dynamic: NetworkManagementConnectivityTestSourceElDynamic,
}
impl NetworkManagementConnectivityTestSourceEl {
    #[doc = "Set the field `cloud_sql_instance`.\nA Cloud SQL instance URI."]
    pub fn set_cloud_sql_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloud_sql_instance = Some(v.into());
        self
    }
    #[doc = "Set the field `gke_master_cluster`.\nA cluster URI for Google Kubernetes Engine cluster control plane."]
    pub fn set_gke_master_cluster(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gke_master_cluster = Some(v.into());
        self
    }
    #[doc = "Set the field `instance`.\nA Compute Engine instance URI."]
    pub fn set_instance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\nThe IP address of the endpoint, which can be an external or internal IP."]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nA VPC network URI."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_type`.\nType of the network where the endpoint is located. Possible values: [\"GCP_NETWORK\", \"NON_GCP_NETWORK\"]"]
    pub fn set_network_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_type = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\nThe IP protocol port of the endpoint. Only applicable when protocol is\nTCP or UDP."]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\nProject ID where the endpoint is located.\nThe project ID can be derived from the URI if you provide a endpoint or\nnetwork URI.\nThe following are two cases where you may need to provide the project ID:\n1. Only the IP address is specified, and the IP address is within a Google\nCloud project.\n2. When you are using Shared VPC and the IP address that you provide is\nfrom the service project. In this case, the network that the IP address\nresides in is defined in the host project."]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
    #[doc = "Set the field `app_engine_version`.\n"]
    pub fn set_app_engine_version(
        mut self,
        v: impl Into<BlockAssignable<NetworkManagementConnectivityTestSourceElAppEngineVersionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.app_engine_version = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.app_engine_version = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_function`.\n"]
    pub fn set_cloud_function(
        mut self,
        v: impl Into<BlockAssignable<NetworkManagementConnectivityTestSourceElCloudFunctionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_function = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_function = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_run_revision`.\n"]
    pub fn set_cloud_run_revision(
        mut self,
        v: impl Into<BlockAssignable<NetworkManagementConnectivityTestSourceElCloudRunRevisionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_run_revision = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_run_revision = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkManagementConnectivityTestSourceEl {
    type O = BlockAssignable<NetworkManagementConnectivityTestSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkManagementConnectivityTestSourceEl {}
impl BuildNetworkManagementConnectivityTestSourceEl {
    pub fn build(self) -> NetworkManagementConnectivityTestSourceEl {
        NetworkManagementConnectivityTestSourceEl {
            cloud_sql_instance: core::default::Default::default(),
            gke_master_cluster: core::default::Default::default(),
            instance: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_type: core::default::Default::default(),
            port: core::default::Default::default(),
            project_id: core::default::Default::default(),
            app_engine_version: core::default::Default::default(),
            cloud_function: core::default::Default::default(),
            cloud_run_revision: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkManagementConnectivityTestSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkManagementConnectivityTestSourceElRef {
    fn new(shared: StackShared, base: String) -> NetworkManagementConnectivityTestSourceElRef {
        NetworkManagementConnectivityTestSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkManagementConnectivityTestSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_sql_instance` after provisioning.\nA Cloud SQL instance URI."]
    pub fn cloud_sql_instance(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloud_sql_instance", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gke_master_cluster` after provisioning.\nA cluster URI for Google Kubernetes Engine cluster control plane."]
    pub fn gke_master_cluster(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gke_master_cluster", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `instance` after provisioning.\nA Compute Engine instance URI."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP address of the endpoint, which can be an external or internal IP."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nA VPC network URI."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_type` after provisioning.\nType of the network where the endpoint is located. Possible values: [\"GCP_NETWORK\", \"NON_GCP_NETWORK\"]"]
    pub fn network_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_type", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe IP protocol port of the endpoint. Only applicable when protocol is\nTCP or UDP."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject ID where the endpoint is located.\nThe project ID can be derived from the URI if you provide a endpoint or\nnetwork URI.\nThe following are two cases where you may need to provide the project ID:\n1. Only the IP address is specified, and the IP address is within a Google\nCloud project.\n2. When you are using Shared VPC and the IP address that you provide is\nfrom the service project. In this case, the network that the IP address\nresides in is defined in the host project."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `app_engine_version` after provisioning.\n"]
    pub fn app_engine_version(
        &self,
    ) -> ListRef<NetworkManagementConnectivityTestSourceElAppEngineVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_engine_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\n"]
    pub fn cloud_function(
        &self,
    ) -> ListRef<NetworkManagementConnectivityTestSourceElCloudFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_function", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_run_revision` after provisioning.\n"]
    pub fn cloud_run_revision(
        &self,
    ) -> ListRef<NetworkManagementConnectivityTestSourceElCloudRunRevisionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_run_revision", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkManagementConnectivityTestTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkManagementConnectivityTestTimeoutsEl {
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
impl ToListMappable for NetworkManagementConnectivityTestTimeoutsEl {
    type O = BlockAssignable<NetworkManagementConnectivityTestTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkManagementConnectivityTestTimeoutsEl {}
impl BuildNetworkManagementConnectivityTestTimeoutsEl {
    pub fn build(self) -> NetworkManagementConnectivityTestTimeoutsEl {
        NetworkManagementConnectivityTestTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkManagementConnectivityTestTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkManagementConnectivityTestTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkManagementConnectivityTestTimeoutsElRef {
        NetworkManagementConnectivityTestTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkManagementConnectivityTestTimeoutsElRef {
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
struct NetworkManagementConnectivityTestDynamic {
    destination: Option<DynamicBlock<NetworkManagementConnectivityTestDestinationEl>>,
    source: Option<DynamicBlock<NetworkManagementConnectivityTestSourceEl>>,
}
