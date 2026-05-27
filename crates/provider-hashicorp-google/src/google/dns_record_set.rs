use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DnsRecordSetData {
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
    id: Option<PrimField<String>>,
    managed_zone: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rrdatas: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_policy: Option<Vec<DnsRecordSetRoutingPolicyEl>>,
    dynamic: DnsRecordSetDynamic,
}
struct DnsRecordSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DnsRecordSetData>,
}
#[derive(Clone)]
pub struct DnsRecordSet(Rc<DnsRecordSet_>);
impl DnsRecordSet {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `rrdatas`.\nThe string data for the records in this record set whose meaning depends on the DNS type. For TXT record, if the string data contains spaces, add surrounding \\\" if you don't want your string to get split on spaces. To specify a single record value longer than 255 characters such as a TXT record for DKIM, add \\\"\\\" inside the Terraform configuration string (e.g. \"first255characters\\\"\\\"morecharacters\")."]
    pub fn set_rrdatas(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().rrdatas = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nThe time-to-live of this record set (seconds)."]
    pub fn set_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `routing_policy`.\n"]
    pub fn set_routing_policy(
        self,
        v: impl Into<BlockAssignable<DnsRecordSetRoutingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `managed_zone` after provisioning.\nThe name of the zone in which this record set will reside."]
    pub fn managed_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe DNS name this record set will apply to."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rrdatas` after provisioning.\nThe string data for the records in this record set whose meaning depends on the DNS type. For TXT record, if the string data contains spaces, add surrounding \\\" if you don't want your string to get split on spaces. To specify a single record value longer than 255 characters such as a TXT record for DKIM, add \\\"\\\" inside the Terraform configuration string (e.g. \"first255characters\\\"\\\"morecharacters\")."]
    pub fn rrdatas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rrdatas", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe time-to-live of this record set (seconds)."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe DNS record set type."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing_policy` after provisioning.\n"]
    pub fn routing_policy(&self) -> ListRef<DnsRecordSetRoutingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routing_policy", self.extract_ref()),
        )
    }
}
impl Referable for DnsRecordSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DnsRecordSet {}
impl ToListMappable for DnsRecordSet {
    type O = ListRef<DnsRecordSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DnsRecordSet_ {
    fn extract_resource_type(&self) -> String {
        "google_dns_record_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDnsRecordSet {
    pub tf_id: String,
    #[doc = "The name of the zone in which this record set will reside."]
    pub managed_zone: PrimField<String>,
    #[doc = "The DNS name this record set will apply to."]
    pub name: PrimField<String>,
    #[doc = "The DNS record set type."]
    pub type_: PrimField<String>,
}
impl BuildDnsRecordSet {
    pub fn build(self, stack: &mut Stack) -> DnsRecordSet {
        let out = DnsRecordSet(Rc::new(DnsRecordSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DnsRecordSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                managed_zone: self.managed_zone,
                name: self.name,
                project: core::default::Default::default(),
                rrdatas: core::default::Default::default(),
                ttl: core::default::Default::default(),
                type_: self.type_,
                routing_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DnsRecordSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DnsRecordSetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `managed_zone` after provisioning.\nThe name of the zone in which this record set will reside."]
    pub fn managed_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.managed_zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe DNS name this record set will apply to."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rrdatas` after provisioning.\nThe string data for the records in this record set whose meaning depends on the DNS type. For TXT record, if the string data contains spaces, add surrounding \\\" if you don't want your string to get split on spaces. To specify a single record value longer than 255 characters such as a TXT record for DKIM, add \\\"\\\" inside the Terraform configuration string (e.g. \"first255characters\\\"\\\"morecharacters\")."]
    pub fn rrdatas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rrdatas", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe time-to-live of this record set (seconds)."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe DNS record set type."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routing_policy` after provisioning.\n"]
    pub fn routing_policy(&self) -> ListRef<DnsRecordSetRoutingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.routing_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    ip_address: PrimField<String>,
    ip_protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_type: Option<PrimField<String>>,
    network_url: PrimField<String>,
    port: PrimField<String>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "Set the field `load_balancer_type`.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn set_load_balancer_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancer_type = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable
    for DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl
{
    type O = BlockAssignable<
        DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "The frontend IP address of the load balancer."]
    pub ip_address: PrimField<String>,
    #[doc = "The configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub ip_protocol: PrimField<String>,
    #[doc = "The fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub network_url: PrimField<String>,
    #[doc = "The configured port of the load balancer."]
    pub port: PrimField<String>,
    #[doc = "The ID of the project in which the load balancer belongs."]
    pub project: PrimField<String>,
}
impl BuildDnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
    pub fn build(
        self,
    ) -> DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
        DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl {
            ip_address: self.ip_address,
            ip_protocol: self.ip_protocol,
            load_balancer_type: core::default::Default::default(),
            network_url: self.network_url,
            port: self.port,
            project: self.project,
            region: core::default::Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
        DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe frontend IP address of the load balancer."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn load_balancer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub fn network_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe configured port of the load balancer."]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the load balancer belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElDynamic {
    internal_load_balancers: Option<
        DynamicBlock<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl>,
    >,
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_endpoints: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_load_balancers:
        Option<Vec<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl>>,
    dynamic: DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElDynamic,
}
impl DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
    #[doc = "Set the field `external_endpoints`.\nThe Internet IP addresses to be health checked."]
    pub fn set_external_endpoints(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.external_endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `internal_load_balancers`.\n"]
    pub fn set_internal_load_balancers(
        mut self,
        v: impl Into<
            BlockAssignable<
                DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.internal_load_balancers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_load_balancers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
    type O = BlockAssignable<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {}
impl BuildDnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
        DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl {
            external_endpoints: core::default::Default::default(),
            internal_load_balancers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
        DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_endpoints` after provisioning.\nThe Internet IP addresses to be health checked."]
    pub fn external_endpoints(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_endpoints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `internal_load_balancers` after provisioning.\n"]
    pub fn internal_load_balancers(
        &self,
    ) -> ListRef<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElInternalLoadBalancersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.internal_load_balancers", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElGeoElDynamic {
    health_checked_targets:
        Option<DynamicBlock<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl>>,
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElGeoEl {
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rrdatas: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_checked_targets: Option<Vec<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl>>,
    dynamic: DnsRecordSetRoutingPolicyElGeoElDynamic,
}
impl DnsRecordSetRoutingPolicyElGeoEl {
    #[doc = "Set the field `rrdatas`.\n"]
    pub fn set_rrdatas(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.rrdatas = Some(v.into());
        self
    }
    #[doc = "Set the field `health_checked_targets`.\n"]
    pub fn set_health_checked_targets(
        mut self,
        v: impl Into<BlockAssignable<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.health_checked_targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.health_checked_targets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsRecordSetRoutingPolicyElGeoEl {
    type O = BlockAssignable<DnsRecordSetRoutingPolicyElGeoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElGeoEl {
    #[doc = "The location name defined in Google Cloud."]
    pub location: PrimField<String>,
}
impl BuildDnsRecordSetRoutingPolicyElGeoEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElGeoEl {
        DnsRecordSetRoutingPolicyElGeoEl {
            location: self.location,
            rrdatas: core::default::Default::default(),
            health_checked_targets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElGeoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElGeoElRef {
    fn new(shared: StackShared, base: String) -> DnsRecordSetRoutingPolicyElGeoElRef {
        DnsRecordSetRoutingPolicyElGeoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElGeoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location name defined in Google Cloud."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `rrdatas` after provisioning.\n"]
    pub fn rrdatas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.base))
    }
    #[doc = "Get a reference to the value of field `health_checked_targets` after provisioning.\n"]
    pub fn health_checked_targets(
        &self,
    ) -> ListRef<DnsRecordSetRoutingPolicyElGeoElHealthCheckedTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_checked_targets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl
{
    ip_address: PrimField<String>,
    ip_protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_type: Option<PrimField<String>>,
    network_url: PrimField<String>,
    port: PrimField<String>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl { # [doc = "Set the field `load_balancer_type`.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"] pub fn set_load_balancer_type (mut self , v : impl Into < PrimField < String > >) -> Self { self . load_balancer_type = Some (v . into ()) ; self } # [doc = "Set the field `region`.\nThe region of the load balancer. Only needed for regional load balancers."] pub fn set_region (mut self , v : impl Into < PrimField < String > >) -> Self { self . region = Some (v . into ()) ; self } }
impl ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl { type O = BlockAssignable < DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl
{
    #[doc = "The frontend IP address of the load balancer."]
    pub ip_address: PrimField<String>,
    #[doc = "The configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub ip_protocol: PrimField<String>,
    #[doc = "The fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub network_url: PrimField<String>,
    #[doc = "The configured port of the load balancer."]
    pub port: PrimField<String>,
    #[doc = "The ID of the project in which the load balancer belongs."]
    pub project: PrimField<String>,
}
impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl { pub fn build (self) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl { DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl { ip_address : self . ip_address , ip_protocol : self . ip_protocol , load_balancer_type : core :: default :: Default :: default () , network_url : self . network_url , port : self . port , project : self . project , region : core :: default :: Default :: default () , } } }
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef { fn new (shared : StackShared , base : String) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef { DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef { shared : shared , base : base . to_string () , } } }
impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe frontend IP address of the load balancer."] pub fn ip_address (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ip_address" , self . base)) } # [doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"] pub fn ip_protocol (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ip_protocol" , self . base)) } # [doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"] pub fn load_balancer_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.load_balancer_type" , self . base)) } # [doc = "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."] pub fn network_url (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.network_url" , self . base)) } # [doc = "Get a reference to the value of field `port` after provisioning.\nThe configured port of the load balancer."] pub fn port (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.port" , self . base)) } # [doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the load balancer belongs."] pub fn project (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.project" , self . base)) } # [doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the load balancer. Only needed for regional load balancers."] pub fn region (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.region" , self . base)) } }
#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElDynamic { internal_load_balancers : Option < DynamicBlock < DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl >> , }
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl { # [serde (skip_serializing_if = "Option::is_none")] external_endpoints : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] internal_load_balancers : Option < Vec < DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl > > , dynamic : DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElDynamic , }
impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
    #[doc = "Set the field `external_endpoints`.\nThe Internet IP addresses to be health checked."]
    pub fn set_external_endpoints(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.external_endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `internal_load_balancers`.\n"]
    pub fn set_internal_load_balancers(
        mut self,
        v : impl Into < BlockAssignable < DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.internal_load_balancers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_load_balancers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl
{
    type O = BlockAssignable<
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {}
impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
    pub fn build(
        self,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl {
            external_endpoints: core::default::Default::default(),
            internal_load_balancers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_endpoints` after provisioning.\nThe Internet IP addresses to be health checked."]
    pub fn external_endpoints(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_endpoints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `internal_load_balancers` after provisioning.\n"]    pub fn internal_load_balancers (& self) -> ListRef < DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElInternalLoadBalancersElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.internal_load_balancers", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElDynamic {
    health_checked_targets: Option<
        DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl>,
    >,
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rrdatas: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_checked_targets:
        Option<Vec<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl>>,
    dynamic: DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElDynamic,
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    #[doc = "Set the field `rrdatas`.\n"]
    pub fn set_rrdatas(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.rrdatas = Some(v.into());
        self
    }
    #[doc = "Set the field `health_checked_targets`.\n"]
    pub fn set_health_checked_targets(
        mut self,
        v: impl Into<
            BlockAssignable<
                DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.health_checked_targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.health_checked_targets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    type O = BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    #[doc = "The location name defined in Google Cloud."]
    pub location: PrimField<String>,
}
impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl {
            location: self.location,
            rrdatas: core::default::Default::default(),
            health_checked_targets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location name defined in Google Cloud."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `rrdatas` after provisioning.\n"]
    pub fn rrdatas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.base))
    }
    #[doc = "Get a reference to the value of field `health_checked_targets` after provisioning.\n"]
    pub fn health_checked_targets(
        &self,
    ) -> ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElHealthCheckedTargetsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_checked_targets", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    ip_address: PrimField<String>,
    ip_protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_type: Option<PrimField<String>>,
    network_url: PrimField<String>,
    port: PrimField<String>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    #[doc = "Set the field `load_balancer_type`.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn set_load_balancer_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancer_type = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    type O =
        BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    #[doc = "The frontend IP address of the load balancer."]
    pub ip_address: PrimField<String>,
    #[doc = "The configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub ip_protocol: PrimField<String>,
    #[doc = "The fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub network_url: PrimField<String>,
    #[doc = "The configured port of the load balancer."]
    pub port: PrimField<String>,
    #[doc = "The ID of the project in which the load balancer belongs."]
    pub project: PrimField<String>,
}
impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
    pub fn build(
        self,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl {
            ip_address: self.ip_address,
            ip_protocol: self.ip_protocol,
            load_balancer_type: core::default::Default::default(),
            network_url: self.network_url,
            port: self.port,
            project: self.project,
            region: core::default::Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe frontend IP address of the load balancer."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn load_balancer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub fn network_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe configured port of the load balancer."]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the load balancer belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElDynamic {
    internal_load_balancers: Option<
        DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl>,
    >,
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_endpoints: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_load_balancers:
        Option<Vec<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl>>,
    dynamic: DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElDynamic,
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
    #[doc = "Set the field `external_endpoints`.\nThe Internet IP addresses to be health checked."]
    pub fn set_external_endpoints(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.external_endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `internal_load_balancers`.\n"]
    pub fn set_internal_load_balancers(
        mut self,
        v: impl Into<
            BlockAssignable<
                DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.internal_load_balancers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_load_balancers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
    type O = BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {}
impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl {
            external_endpoints: core::default::Default::default(),
            internal_load_balancers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_endpoints` after provisioning.\nThe Internet IP addresses to be health checked."]
    pub fn external_endpoints(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_endpoints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `internal_load_balancers` after provisioning.\n"]
    pub fn internal_load_balancers(
        &self,
    ) -> ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElInternalLoadBalancersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.internal_load_balancers", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElPrimaryBackupElDynamic {
    backup_geo: Option<DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl>>,
    primary: Option<DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl>>,
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_geo_fencing_for_backups: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trickle_ratio: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backup_geo: Option<Vec<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary: Option<Vec<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl>>,
    dynamic: DnsRecordSetRoutingPolicyElPrimaryBackupElDynamic,
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupEl {
    #[doc = "Set the field `enable_geo_fencing_for_backups`.\nSpecifies whether to enable fencing for backup geo queries."]
    pub fn set_enable_geo_fencing_for_backups(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_geo_fencing_for_backups = Some(v.into());
        self
    }
    #[doc = "Set the field `trickle_ratio`.\nSpecifies the percentage of traffic to send to the backup targets even when the primary targets are healthy."]
    pub fn set_trickle_ratio(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.trickle_ratio = Some(v.into());
        self
    }
    #[doc = "Set the field `backup_geo`.\n"]
    pub fn set_backup_geo(
        mut self,
        v: impl Into<BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.backup_geo = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_geo = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `primary`.\n"]
    pub fn set_primary(
        mut self,
        v: impl Into<BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.primary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.primary = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsRecordSetRoutingPolicyElPrimaryBackupEl {
    type O = BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElPrimaryBackupEl {}
impl BuildDnsRecordSetRoutingPolicyElPrimaryBackupEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElPrimaryBackupEl {
        DnsRecordSetRoutingPolicyElPrimaryBackupEl {
            enable_geo_fencing_for_backups: core::default::Default::default(),
            trickle_ratio: core::default::Default::default(),
            backup_geo: core::default::Default::default(),
            primary: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
    fn new(shared: StackShared, base: String) -> DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
        DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElPrimaryBackupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_geo_fencing_for_backups` after provisioning.\nSpecifies whether to enable fencing for backup geo queries."]
    pub fn enable_geo_fencing_for_backups(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_geo_fencing_for_backups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `trickle_ratio` after provisioning.\nSpecifies the percentage of traffic to send to the backup targets even when the primary targets are healthy."]
    pub fn trickle_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.trickle_ratio", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backup_geo` after provisioning.\n"]
    pub fn backup_geo(&self) -> ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElBackupGeoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.backup_geo", self.base))
    }
    #[doc = "Get a reference to the value of field `primary` after provisioning.\n"]
    pub fn primary(&self) -> ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElPrimaryElRef> {
        ListRef::new(self.shared().clone(), format!("{}.primary", self.base))
    }
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
    ip_address: PrimField<String>,
    ip_protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancer_type: Option<PrimField<String>>,
    network_url: PrimField<String>,
    port: PrimField<String>,
    project: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "Set the field `load_balancer_type`.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn set_load_balancer_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancer_type = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable
    for DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl
{
    type O = BlockAssignable<
        DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
    #[doc = "The frontend IP address of the load balancer."]
    pub ip_address: PrimField<String>,
    #[doc = "The configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub ip_protocol: PrimField<String>,
    #[doc = "The fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub network_url: PrimField<String>,
    #[doc = "The configured port of the load balancer."]
    pub port: PrimField<String>,
    #[doc = "The ID of the project in which the load balancer belongs."]
    pub project: PrimField<String>,
}
impl BuildDnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
    pub fn build(
        self,
    ) -> DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
        DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl {
            ip_address: self.ip_address,
            ip_protocol: self.ip_protocol,
            load_balancer_type: core::default::Default::default(),
            network_url: self.network_url,
            port: self.port,
            project: self.project,
            region: core::default::Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
        DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe frontend IP address of the load balancer."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\nThe configured IP protocol of the load balancer. This value is case-sensitive. Possible values: [\"tcp\", \"udp\"]"]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `load_balancer_type` after provisioning.\nThe type of load balancer. This value is case-sensitive. Possible values: [\"regionalL4ilb\", \"regionalL7ilb\", \"globalL7ilb\"]"]
    pub fn load_balancer_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancer_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified url of the network in which the load balancer belongs. This should be formatted like `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`."]
    pub fn network_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe configured port of the load balancer."]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the load balancer belongs."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the load balancer. Only needed for regional load balancers."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElDynamic {
    internal_load_balancers: Option<
        DynamicBlock<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl>,
    >,
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_endpoints: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_load_balancers:
        Option<Vec<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl>>,
    dynamic: DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElDynamic,
}
impl DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
    #[doc = "Set the field `external_endpoints`.\nThe Internet IP addresses to be health checked."]
    pub fn set_external_endpoints(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.external_endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `internal_load_balancers`.\n"]
    pub fn set_internal_load_balancers(
        mut self,
        v: impl Into<
            BlockAssignable<
                DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.internal_load_balancers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.internal_load_balancers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
    type O = BlockAssignable<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {}
impl BuildDnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
        DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl {
            external_endpoints: core::default::Default::default(),
            internal_load_balancers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
        DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_endpoints` after provisioning.\nThe Internet IP addresses to be health checked."]
    pub fn external_endpoints(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_endpoints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `internal_load_balancers` after provisioning.\n"]
    pub fn internal_load_balancers(
        &self,
    ) -> ListRef<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElInternalLoadBalancersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.internal_load_balancers", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElWrrElDynamic {
    health_checked_targets:
        Option<DynamicBlock<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl>>,
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyElWrrEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rrdatas: Option<ListField<PrimField<String>>>,
    weight: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_checked_targets: Option<Vec<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl>>,
    dynamic: DnsRecordSetRoutingPolicyElWrrElDynamic,
}
impl DnsRecordSetRoutingPolicyElWrrEl {
    #[doc = "Set the field `rrdatas`.\n"]
    pub fn set_rrdatas(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.rrdatas = Some(v.into());
        self
    }
    #[doc = "Set the field `health_checked_targets`.\n"]
    pub fn set_health_checked_targets(
        mut self,
        v: impl Into<BlockAssignable<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.health_checked_targets = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.health_checked_targets = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsRecordSetRoutingPolicyElWrrEl {
    type O = BlockAssignable<DnsRecordSetRoutingPolicyElWrrEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyElWrrEl {
    #[doc = "The ratio of traffic routed to the target."]
    pub weight: PrimField<f64>,
}
impl BuildDnsRecordSetRoutingPolicyElWrrEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyElWrrEl {
        DnsRecordSetRoutingPolicyElWrrEl {
            rrdatas: core::default::Default::default(),
            weight: self.weight,
            health_checked_targets: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElWrrElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElWrrElRef {
    fn new(shared: StackShared, base: String) -> DnsRecordSetRoutingPolicyElWrrElRef {
        DnsRecordSetRoutingPolicyElWrrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElWrrElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rrdatas` after provisioning.\n"]
    pub fn rrdatas(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.base))
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nThe ratio of traffic routed to the target."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
    #[doc = "Get a reference to the value of field `health_checked_targets` after provisioning.\n"]
    pub fn health_checked_targets(
        &self,
    ) -> ListRef<DnsRecordSetRoutingPolicyElWrrElHealthCheckedTargetsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_checked_targets", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DnsRecordSetRoutingPolicyElDynamic {
    geo: Option<DynamicBlock<DnsRecordSetRoutingPolicyElGeoEl>>,
    primary_backup: Option<DynamicBlock<DnsRecordSetRoutingPolicyElPrimaryBackupEl>>,
    wrr: Option<DynamicBlock<DnsRecordSetRoutingPolicyElWrrEl>>,
}
#[derive(Serialize)]
pub struct DnsRecordSetRoutingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_geo_fencing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<Vec<DnsRecordSetRoutingPolicyElGeoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_backup: Option<Vec<DnsRecordSetRoutingPolicyElPrimaryBackupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wrr: Option<Vec<DnsRecordSetRoutingPolicyElWrrEl>>,
    dynamic: DnsRecordSetRoutingPolicyElDynamic,
}
impl DnsRecordSetRoutingPolicyEl {
    #[doc = "Set the field `enable_geo_fencing`.\nSpecifies whether to enable fencing for geo queries."]
    pub fn set_enable_geo_fencing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_geo_fencing = Some(v.into());
        self
    }
    #[doc = "Set the field `health_check`.\nSpecifies the health check."]
    pub fn set_health_check(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.health_check = Some(v.into());
        self
    }
    #[doc = "Set the field `geo`.\n"]
    pub fn set_geo(
        mut self,
        v: impl Into<BlockAssignable<DnsRecordSetRoutingPolicyElGeoEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.geo = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.geo = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `primary_backup`.\n"]
    pub fn set_primary_backup(
        mut self,
        v: impl Into<BlockAssignable<DnsRecordSetRoutingPolicyElPrimaryBackupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.primary_backup = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.primary_backup = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `wrr`.\n"]
    pub fn set_wrr(
        mut self,
        v: impl Into<BlockAssignable<DnsRecordSetRoutingPolicyElWrrEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.wrr = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.wrr = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DnsRecordSetRoutingPolicyEl {
    type O = BlockAssignable<DnsRecordSetRoutingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsRecordSetRoutingPolicyEl {}
impl BuildDnsRecordSetRoutingPolicyEl {
    pub fn build(self) -> DnsRecordSetRoutingPolicyEl {
        DnsRecordSetRoutingPolicyEl {
            enable_geo_fencing: core::default::Default::default(),
            health_check: core::default::Default::default(),
            geo: core::default::Default::default(),
            primary_backup: core::default::Default::default(),
            wrr: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DnsRecordSetRoutingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsRecordSetRoutingPolicyElRef {
    fn new(shared: StackShared, base: String) -> DnsRecordSetRoutingPolicyElRef {
        DnsRecordSetRoutingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsRecordSetRoutingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_geo_fencing` after provisioning.\nSpecifies whether to enable fencing for geo queries."]
    pub fn enable_geo_fencing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_geo_fencing", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_check` after provisioning.\nSpecifies the health check."]
    pub fn health_check(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
    #[doc = "Get a reference to the value of field `geo` after provisioning.\n"]
    pub fn geo(&self) -> ListRef<DnsRecordSetRoutingPolicyElGeoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.geo", self.base))
    }
    #[doc = "Get a reference to the value of field `primary_backup` after provisioning.\n"]
    pub fn primary_backup(&self) -> ListRef<DnsRecordSetRoutingPolicyElPrimaryBackupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.primary_backup", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `wrr` after provisioning.\n"]
    pub fn wrr(&self) -> ListRef<DnsRecordSetRoutingPolicyElWrrElRef> {
        ListRef::new(self.shared().clone(), format!("{}.wrr", self.base))
    }
}
#[derive(Serialize, Default)]
struct DnsRecordSetDynamic {
    routing_policy: Option<DynamicBlock<DnsRecordSetRoutingPolicyEl>>,
}
