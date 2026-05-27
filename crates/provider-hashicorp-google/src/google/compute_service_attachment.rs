use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeServiceAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    connection_preference: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_reject_lists: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_names: Option<ListField<PrimField<String>>>,
    enable_proxy_protocol: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    nat_subnets: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagated_connection_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reconcile_connections: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_propagated_connection_limit_if_zero: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_nat_ips: Option<PrimField<bool>>,
    target_service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_accept_lists: Option<Vec<ComputeServiceAttachmentConsumerAcceptListsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeServiceAttachmentTimeoutsEl>,
    dynamic: ComputeServiceAttachmentDynamic,
}
struct ComputeServiceAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeServiceAttachmentData>,
}
#[derive(Clone)]
pub struct ComputeServiceAttachment(Rc<ComputeServiceAttachment_>);
impl ComputeServiceAttachment {
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
    #[doc = "Set the field `consumer_reject_lists`.\nAn array of projects that are not allowed to connect to this service\nattachment."]
    pub fn set_consumer_reject_lists(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().consumer_reject_lists = Some(v.into());
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
    #[doc = "Set the field `domain_names`.\nIf specified, the domain name will be used during the integration between\nthe PSC connected endpoints and the Cloud DNS. For example, this is a\nvalid domain name: \"p.mycompany.com.\". Current max number of domain names\nsupported is 1."]
    pub fn set_domain_names(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().domain_names = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `propagated_connection_limit`.\nThe number of consumer spokes that connected Private Service Connect endpoints can be propagated to through Network Connectivity Center.\nThis limit lets the service producer limit how many propagated Private Service Connect connections can be established to this service attachment from a single consumer.\n\nIf the connection preference of the service attachment is ACCEPT_MANUAL, the limit applies to each project or network that is listed in the consumer accept list.\nIf the connection preference of the service attachment is ACCEPT_AUTOMATIC, the limit applies to each project that contains a connected endpoint.\n\nIf unspecified, the default propagated connection limit is 250. To explicitly send a zero value, set 'send_propagated_connection_limit_if_zero = true'."]
    pub fn set_propagated_connection_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().propagated_connection_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `reconcile_connections`.\nThis flag determines whether a consumer accept/reject list change can reconcile the statuses of existing ACCEPTED or REJECTED PSC endpoints.\n\nIf false, connection policy update will only affect existing PENDING PSC endpoints. Existing ACCEPTED/REJECTED endpoints will remain untouched regardless how the connection policy is modified .\nIf true, update will affect both PENDING and ACCEPTED/REJECTED PSC endpoints. For example, an ACCEPTED PSC endpoint will be moved to REJECTED if its project is added to the reject list."]
    pub fn set_reconcile_connections(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().reconcile_connections = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nURL of the region where the resource resides."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `send_propagated_connection_limit_if_zero`.\nControls the behavior of propagated_connection_limit.\nWhen false, setting propagated_connection_limit to zero causes the provider to use to the API's default value.\nWhen true, the provider will set propagated_connection_limit to zero.\nDefaults to false."]
    pub fn set_send_propagated_connection_limit_if_zero(
        self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.0
            .data
            .borrow_mut()
            .send_propagated_connection_limit_if_zero = Some(v.into());
        self
    }
    #[doc = "Set the field `show_nat_ips`.\nNOTE: This field is temporarily non-functional due to an underlying API issue.\nAny value provided here will be ignored until the API issue is resolved, expected around 2026-03.\n[If true, show NAT IPs of all connected endpoints.]"]
    pub fn set_show_nat_ips(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().show_nat_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_accept_lists`.\n"]
    pub fn set_consumer_accept_lists(
        self,
        v: impl Into<BlockAssignable<ComputeServiceAttachmentConsumerAcceptListsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().consumer_accept_lists = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.consumer_accept_lists = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeServiceAttachmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `connected_endpoints` after provisioning.\nAn array of the consumer forwarding rules connected to this service\nattachment."]
    pub fn connected_endpoints(&self) -> ListRef<ComputeServiceAttachmentConnectedEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connected_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_preference` after provisioning.\nThe connection preference to use for this service attachment. Valid\nvalues include \"ACCEPT_AUTOMATIC\", \"ACCEPT_MANUAL\"."]
    pub fn connection_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_preference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_reject_lists` after provisioning.\nAn array of projects that are not allowed to connect to this service\nattachment."]
    pub fn consumer_reject_lists(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumer_reject_lists", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `domain_names` after provisioning.\nIf specified, the domain name will be used during the integration between\nthe PSC connected endpoints and the Cloud DNS. For example, this is a\nvalid domain name: \"p.mycompany.com.\". Current max number of domain names\nsupported is 1."]
    pub fn domain_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\nIf true, enable the proxy protocol which is for supplying client TCP/IP\naddress data in TCP connections that traverse proxies on their way to\ndestination servers."]
    pub fn enable_proxy_protocol(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_proxy_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nat_subnets` after provisioning.\nAn array of subnets that is provided for NAT in this service attachment."]
    pub fn nat_subnets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nat_subnets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `propagated_connection_limit` after provisioning.\nThe number of consumer spokes that connected Private Service Connect endpoints can be propagated to through Network Connectivity Center.\nThis limit lets the service producer limit how many propagated Private Service Connect connections can be established to this service attachment from a single consumer.\n\nIf the connection preference of the service attachment is ACCEPT_MANUAL, the limit applies to each project or network that is listed in the consumer accept list.\nIf the connection preference of the service attachment is ACCEPT_AUTOMATIC, the limit applies to each project that contains a connected endpoint.\n\nIf unspecified, the default propagated connection limit is 250. To explicitly send a zero value, set 'send_propagated_connection_limit_if_zero = true'."]
    pub fn propagated_connection_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagated_connection_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_service_attachment_id` after provisioning.\nAn 128-bit global unique ID of the PSC service attachment."]
    pub fn psc_service_attachment_id(
        &self,
    ) -> ListRef<ComputeServiceAttachmentPscServiceAttachmentIdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_service_attachment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconcile_connections` after provisioning.\nThis flag determines whether a consumer accept/reject list change can reconcile the statuses of existing ACCEPTED or REJECTED PSC endpoints.\n\nIf false, connection policy update will only affect existing PENDING PSC endpoints. Existing ACCEPTED/REJECTED endpoints will remain untouched regardless how the connection policy is modified .\nIf true, update will affect both PENDING and ACCEPTED/REJECTED PSC endpoints. For example, an ACCEPTED PSC endpoint will be moved to REJECTED if its project is added to the reject list."]
    pub fn reconcile_connections(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconcile_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nURL of the region where the resource resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `send_propagated_connection_limit_if_zero` after provisioning.\nControls the behavior of propagated_connection_limit.\nWhen false, setting propagated_connection_limit to zero causes the provider to use to the API's default value.\nWhen true, the provider will set propagated_connection_limit to zero.\nDefaults to false."]
    pub fn send_propagated_connection_limit_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.send_propagated_connection_limit_if_zero",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `show_nat_ips` after provisioning.\nNOTE: This field is temporarily non-functional due to an underlying API issue.\nAny value provided here will be ignored until the API issue is resolved, expected around 2026-03.\n[If true, show NAT IPs of all connected endpoints.]"]
    pub fn show_nat_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.show_nat_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_service` after provisioning.\nThe URL of a service serving the endpoint identified by this service attachment."]
    pub fn target_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeServiceAttachmentTimeoutsElRef {
        ComputeServiceAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeServiceAttachment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeServiceAttachment {}
impl ToListMappable for ComputeServiceAttachment {
    type O = ListRef<ComputeServiceAttachmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeServiceAttachment_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_service_attachment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeServiceAttachment {
    pub tf_id: String,
    #[doc = "The connection preference to use for this service attachment. Valid\nvalues include \"ACCEPT_AUTOMATIC\", \"ACCEPT_MANUAL\"."]
    pub connection_preference: PrimField<String>,
    #[doc = "If true, enable the proxy protocol which is for supplying client TCP/IP\naddress data in TCP connections that traverse proxies on their way to\ndestination servers."]
    pub enable_proxy_protocol: PrimField<bool>,
    #[doc = "Name of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "An array of subnets that is provided for NAT in this service attachment."]
    pub nat_subnets: ListField<PrimField<String>>,
    #[doc = "The URL of a service serving the endpoint identified by this service attachment."]
    pub target_service: PrimField<String>,
}
impl BuildComputeServiceAttachment {
    pub fn build(self, stack: &mut Stack) -> ComputeServiceAttachment {
        let out = ComputeServiceAttachment(Rc::new(ComputeServiceAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeServiceAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                connection_preference: self.connection_preference,
                consumer_reject_lists: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                domain_names: core::default::Default::default(),
                enable_proxy_protocol: self.enable_proxy_protocol,
                id: core::default::Default::default(),
                name: self.name,
                nat_subnets: self.nat_subnets,
                project: core::default::Default::default(),
                propagated_connection_limit: core::default::Default::default(),
                reconcile_connections: core::default::Default::default(),
                region: core::default::Default::default(),
                send_propagated_connection_limit_if_zero: core::default::Default::default(),
                show_nat_ips: core::default::Default::default(),
                target_service: self.target_service,
                consumer_accept_lists: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeServiceAttachmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeServiceAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeServiceAttachmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connected_endpoints` after provisioning.\nAn array of the consumer forwarding rules connected to this service\nattachment."]
    pub fn connected_endpoints(&self) -> ListRef<ComputeServiceAttachmentConnectedEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connected_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_preference` after provisioning.\nThe connection preference to use for this service attachment. Valid\nvalues include \"ACCEPT_AUTOMATIC\", \"ACCEPT_MANUAL\"."]
    pub fn connection_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_preference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_reject_lists` after provisioning.\nAn array of projects that are not allowed to connect to this service\nattachment."]
    pub fn consumer_reject_lists(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.consumer_reject_lists", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `domain_names` after provisioning.\nIf specified, the domain name will be used during the integration between\nthe PSC connected endpoints and the Cloud DNS. For example, this is a\nvalid domain name: \"p.mycompany.com.\". Current max number of domain names\nsupported is 1."]
    pub fn domain_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\nIf true, enable the proxy protocol which is for supplying client TCP/IP\naddress data in TCP connections that traverse proxies on their way to\ndestination servers."]
    pub fn enable_proxy_protocol(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_proxy_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `nat_subnets` after provisioning.\nAn array of subnets that is provided for NAT in this service attachment."]
    pub fn nat_subnets(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.nat_subnets", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `propagated_connection_limit` after provisioning.\nThe number of consumer spokes that connected Private Service Connect endpoints can be propagated to through Network Connectivity Center.\nThis limit lets the service producer limit how many propagated Private Service Connect connections can be established to this service attachment from a single consumer.\n\nIf the connection preference of the service attachment is ACCEPT_MANUAL, the limit applies to each project or network that is listed in the consumer accept list.\nIf the connection preference of the service attachment is ACCEPT_AUTOMATIC, the limit applies to each project that contains a connected endpoint.\n\nIf unspecified, the default propagated connection limit is 250. To explicitly send a zero value, set 'send_propagated_connection_limit_if_zero = true'."]
    pub fn propagated_connection_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagated_connection_limit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_service_attachment_id` after provisioning.\nAn 128-bit global unique ID of the PSC service attachment."]
    pub fn psc_service_attachment_id(
        &self,
    ) -> ListRef<ComputeServiceAttachmentPscServiceAttachmentIdElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_service_attachment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconcile_connections` after provisioning.\nThis flag determines whether a consumer accept/reject list change can reconcile the statuses of existing ACCEPTED or REJECTED PSC endpoints.\n\nIf false, connection policy update will only affect existing PENDING PSC endpoints. Existing ACCEPTED/REJECTED endpoints will remain untouched regardless how the connection policy is modified .\nIf true, update will affect both PENDING and ACCEPTED/REJECTED PSC endpoints. For example, an ACCEPTED PSC endpoint will be moved to REJECTED if its project is added to the reject list."]
    pub fn reconcile_connections(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconcile_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nURL of the region where the resource resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `send_propagated_connection_limit_if_zero` after provisioning.\nControls the behavior of propagated_connection_limit.\nWhen false, setting propagated_connection_limit to zero causes the provider to use to the API's default value.\nWhen true, the provider will set propagated_connection_limit to zero.\nDefaults to false."]
    pub fn send_propagated_connection_limit_if_zero(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.send_propagated_connection_limit_if_zero",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `show_nat_ips` after provisioning.\nNOTE: This field is temporarily non-functional due to an underlying API issue.\nAny value provided here will be ignored until the API issue is resolved, expected around 2026-03.\n[If true, show NAT IPs of all connected endpoints.]"]
    pub fn show_nat_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.show_nat_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_service` after provisioning.\nThe URL of a service serving the endpoint identified by this service attachment."]
    pub fn target_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeServiceAttachmentTimeoutsElRef {
        ComputeServiceAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeServiceAttachmentConnectedEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_ips: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    propagated_connection_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}
impl ComputeServiceAttachmentConnectedEndpointsEl {
    #[doc = "Set the field `consumer_network`.\n"]
    pub fn set_consumer_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_network = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `nat_ips`.\n"]
    pub fn set_nat_ips(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.nat_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `propagated_connection_count`.\n"]
    pub fn set_propagated_connection_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.propagated_connection_count = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_connection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeServiceAttachmentConnectedEndpointsEl {
    type O = BlockAssignable<ComputeServiceAttachmentConnectedEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeServiceAttachmentConnectedEndpointsEl {}
impl BuildComputeServiceAttachmentConnectedEndpointsEl {
    pub fn build(self) -> ComputeServiceAttachmentConnectedEndpointsEl {
        ComputeServiceAttachmentConnectedEndpointsEl {
            consumer_network: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            nat_ips: core::default::Default::default(),
            propagated_connection_count: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}
pub struct ComputeServiceAttachmentConnectedEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeServiceAttachmentConnectedEndpointsElRef {
    fn new(shared: StackShared, base: String) -> ComputeServiceAttachmentConnectedEndpointsElRef {
        ComputeServiceAttachmentConnectedEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeServiceAttachmentConnectedEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consumer_network` after provisioning.\n"]
    pub fn consumer_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `nat_ips` after provisioning.\n"]
    pub fn nat_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.nat_ips", self.base))
    }
    #[doc = "Get a reference to the value of field `propagated_connection_count` after provisioning.\n"]
    pub fn propagated_connection_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagated_connection_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeServiceAttachmentPscServiceAttachmentIdEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    high: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low: Option<PrimField<String>>,
}
impl ComputeServiceAttachmentPscServiceAttachmentIdEl {
    #[doc = "Set the field `high`.\n"]
    pub fn set_high(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.high = Some(v.into());
        self
    }
    #[doc = "Set the field `low`.\n"]
    pub fn set_low(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.low = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeServiceAttachmentPscServiceAttachmentIdEl {
    type O = BlockAssignable<ComputeServiceAttachmentPscServiceAttachmentIdEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeServiceAttachmentPscServiceAttachmentIdEl {}
impl BuildComputeServiceAttachmentPscServiceAttachmentIdEl {
    pub fn build(self) -> ComputeServiceAttachmentPscServiceAttachmentIdEl {
        ComputeServiceAttachmentPscServiceAttachmentIdEl {
            high: core::default::Default::default(),
            low: core::default::Default::default(),
        }
    }
}
pub struct ComputeServiceAttachmentPscServiceAttachmentIdElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeServiceAttachmentPscServiceAttachmentIdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeServiceAttachmentPscServiceAttachmentIdElRef {
        ComputeServiceAttachmentPscServiceAttachmentIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeServiceAttachmentPscServiceAttachmentIdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `high` after provisioning.\n"]
    pub fn high(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.high", self.base))
    }
    #[doc = "Get a reference to the value of field `low` after provisioning.\n"]
    pub fn low(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.low", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeServiceAttachmentConsumerAcceptListsEl {
    connection_limit: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id_or_num: Option<PrimField<String>>,
}
impl ComputeServiceAttachmentConsumerAcceptListsEl {
    #[doc = "Set the field `endpoint_url`.\nThe endpoint that is allowed to connect to this service attachment.\nOnly one of project_id_or_num, network_url and endpoint_url may be set."]
    pub fn set_endpoint_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_url = Some(v.into());
        self
    }
    #[doc = "Set the field `network_url`.\nThe network that is allowed to connect to this service attachment.\nOnly one of project_id_or_num and network_url may be set."]
    pub fn set_network_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_url = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id_or_num`.\nA project that is allowed to connect to this service attachment.\nOnly one of project_id_or_num and network_url may be set."]
    pub fn set_project_id_or_num(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id_or_num = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeServiceAttachmentConsumerAcceptListsEl {
    type O = BlockAssignable<ComputeServiceAttachmentConsumerAcceptListsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeServiceAttachmentConsumerAcceptListsEl {
    #[doc = "The number of consumer forwarding rules the consumer project can\ncreate."]
    pub connection_limit: PrimField<f64>,
}
impl BuildComputeServiceAttachmentConsumerAcceptListsEl {
    pub fn build(self) -> ComputeServiceAttachmentConsumerAcceptListsEl {
        ComputeServiceAttachmentConsumerAcceptListsEl {
            connection_limit: self.connection_limit,
            endpoint_url: core::default::Default::default(),
            network_url: core::default::Default::default(),
            project_id_or_num: core::default::Default::default(),
        }
    }
}
pub struct ComputeServiceAttachmentConsumerAcceptListsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeServiceAttachmentConsumerAcceptListsElRef {
    fn new(shared: StackShared, base: String) -> ComputeServiceAttachmentConsumerAcceptListsElRef {
        ComputeServiceAttachmentConsumerAcceptListsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeServiceAttachmentConsumerAcceptListsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_limit` after provisioning.\nThe number of consumer forwarding rules the consumer project can\ncreate."]
    pub fn connection_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_limit", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_url` after provisioning.\nThe endpoint that is allowed to connect to this service attachment.\nOnly one of project_id_or_num, network_url and endpoint_url may be set."]
    pub fn endpoint_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_url", self.base))
    }
    #[doc = "Get a reference to the value of field `network_url` after provisioning.\nThe network that is allowed to connect to this service attachment.\nOnly one of project_id_or_num and network_url may be set."]
    pub fn network_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id_or_num` after provisioning.\nA project that is allowed to connect to this service attachment.\nOnly one of project_id_or_num and network_url may be set."]
    pub fn project_id_or_num(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project_id_or_num", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeServiceAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeServiceAttachmentTimeoutsEl {
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
impl ToListMappable for ComputeServiceAttachmentTimeoutsEl {
    type O = BlockAssignable<ComputeServiceAttachmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeServiceAttachmentTimeoutsEl {}
impl BuildComputeServiceAttachmentTimeoutsEl {
    pub fn build(self) -> ComputeServiceAttachmentTimeoutsEl {
        ComputeServiceAttachmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeServiceAttachmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeServiceAttachmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeServiceAttachmentTimeoutsElRef {
        ComputeServiceAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeServiceAttachmentTimeoutsElRef {
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
struct ComputeServiceAttachmentDynamic {
    consumer_accept_lists: Option<DynamicBlock<ComputeServiceAttachmentConsumerAcceptListsEl>>,
}
