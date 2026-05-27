use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeAddressData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_collection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_endpoint_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeAddressTimeoutsEl>,
}
struct ComputeAddress_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeAddressData>,
}
#[derive(Clone)]
pub struct ComputeAddress(Rc<ComputeAddress_>);
impl ComputeAddress {
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
    #[doc = "Set the field `address`.\nThe static external IP address represented by this resource.\nThe IP address must be inside the specified subnetwork,\nif any. Set by the API if undefined."]
    pub fn set_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address = Some(v.into());
        self
    }
    #[doc = "Set the field `address_type`.\nThe type of address to reserve.\nNote: if you set this argument's value as 'INTERNAL' you need to leave the 'network_tier' argument unset in that resource block. Default value: \"EXTERNAL\" Possible values: [\"INTERNAL\", \"EXTERNAL\"]"]
    pub fn set_address_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address_type = Some(v.into());
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
    #[doc = "Set the field `ip_collection`.\nReference to the source of external IPv4 addresses, like a PublicDelegatedPrefix(PDP) for BYOIP.\nThe PDP must support enhanced IPv4 allocations.\nUse one of the following formats to specify a PDP when reserving an external IPv4 address using BYOIP.\nFull resource URL, as in:\n  * 'https://www.googleapis.com/compute/v1/projects/{{projectId}}/regions/{{region}}/publicDelegatedPrefixes/{{pdp-name}}'\nPartial URL, as in:\n  * 'projects/{{projectId}}/regions/region/publicDelegatedPrefixes/{{pdp-name}}'\n  * 'regions/{{region}}/publicDelegatedPrefixes/{{pdp-name}}'"]
    pub fn set_ip_collection(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_collection = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_version`.\nThe IP Version that will be used by this address. The default value is 'IPV4'. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn set_ip_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ip_version = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_endpoint_type`.\nThe endpoint type of this address, which should be VM or NETLB. This is\nused for deciding which type of endpoint this address can be used after\nthe external IPv6 address reservation. Possible values: [\"VM\", \"NETLB\"]"]
    pub fn set_ipv6_endpoint_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ipv6_endpoint_type = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to apply to this address.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe URL of the network in which to reserve the address. This field\ncan only be used with INTERNAL type with the VPC_PEERING and\nIPSEC_INTERCONNECT purposes."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tier`.\nThe networking tier used for configuring this address. If this field is not\nspecified, it is assumed to be PREMIUM.\nThis argument should not be used when configuring Internal addresses, because [network tier cannot be set for internal traffic; it's always Premium](https://cloud.google.com/network-tiers/docs/overview). Possible values: [\"PREMIUM\", \"STANDARD\"]"]
    pub fn set_network_tier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_length`.\nThe prefix length if the resource represents an IP range."]
    pub fn set_prefix_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().prefix_length = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `purpose`.\nThe purpose of this resource, which can be one of the following values.\n\n* GCE_ENDPOINT for addresses that are used by VM instances, alias IP\nranges, load balancers, and similar resources.\n\n* SHARED_LOADBALANCER_VIP for an address that can be used by multiple\ninternal load balancers.\n\n* VPC_PEERING for addresses that are reserved for VPC peer networks.\n\n* IPSEC_INTERCONNECT for addresses created from a private IP range that\nare reserved for a VLAN attachment in an HA VPN over Cloud Interconnect\nconfiguration. These addresses are regional resources.\n\n* PRIVATE_SERVICE_CONNECT for a private network address that is used to\nconfigure Private Service Connect. Only global internal addresses can use\nthis purpose.\n\nThis should only be set when using an Internal address."]
    pub fn set_purpose(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().purpose = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe Region in which the created address should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe URL of the subnetwork in which to reserve the address. If an IP\naddress is specified, it must be within the subnetwork's IP range.\nThis field can only be used with INTERNAL type with\nGCE_ENDPOINT/DNS_RESOLVER purposes."]
    pub fn set_subnetwork(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeAddressTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\nThe static external IP address represented by this resource.\nThe IP address must be inside the specified subnetwork,\nif any. Set by the API if undefined."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_id` after provisioning.\nThe unique numeric identifier for the resource. This identifier is defined by the server."]
    pub fn address_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_type` after provisioning.\nThe type of address to reserve.\nNote: if you set this argument's value as 'INTERNAL' you need to leave the 'network_tier' argument unset in that resource block. Default value: \"EXTERNAL\" Possible values: [\"INTERNAL\", \"EXTERNAL\"]"]
    pub fn address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ip_collection` after provisioning.\nReference to the source of external IPv4 addresses, like a PublicDelegatedPrefix(PDP) for BYOIP.\nThe PDP must support enhanced IPv4 allocations.\nUse one of the following formats to specify a PDP when reserving an external IPv4 address using BYOIP.\nFull resource URL, as in:\n  * 'https://www.googleapis.com/compute/v1/projects/{{projectId}}/regions/{{region}}/publicDelegatedPrefixes/{{pdp-name}}'\nPartial URL, as in:\n  * 'projects/{{projectId}}/regions/region/publicDelegatedPrefixes/{{pdp-name}}'\n  * 'regions/{{region}}/publicDelegatedPrefixes/{{pdp-name}}'"]
    pub fn ip_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_version` after provisioning.\nThe IP Version that will be used by this address. The default value is 'IPV4'. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn ip_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_endpoint_type` after provisioning.\nThe endpoint type of this address, which should be VM or NETLB. This is\nused for deciding which type of endpoint this address can be used after\nthe external IPv6 address reservation. Possible values: [\"VM\", \"NETLB\"]"]
    pub fn ipv6_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_endpoint_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this address.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe URL of the network in which to reserve the address. This field\ncan only be used with INTERNAL type with the VPC_PEERING and\nIPSEC_INTERCONNECT purposes."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\nThe networking tier used for configuring this address. If this field is not\nspecified, it is assumed to be PREMIUM.\nThis argument should not be used when configuring Internal addresses, because [network tier cannot be set for internal traffic; it's always Premium](https://cloud.google.com/network-tiers/docs/overview). Possible values: [\"PREMIUM\", \"STANDARD\"]"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_length` after provisioning.\nThe prefix length if the resource represents an IP range."]
    pub fn prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `purpose` after provisioning.\nThe purpose of this resource, which can be one of the following values.\n\n* GCE_ENDPOINT for addresses that are used by VM instances, alias IP\nranges, load balancers, and similar resources.\n\n* SHARED_LOADBALANCER_VIP for an address that can be used by multiple\ninternal load balancers.\n\n* VPC_PEERING for addresses that are reserved for VPC peer networks.\n\n* IPSEC_INTERCONNECT for addresses created from a private IP range that\nare reserved for a VLAN attachment in an HA VPN over Cloud Interconnect\nconfiguration. These addresses are regional resources.\n\n* PRIVATE_SERVICE_CONNECT for a private network address that is used to\nconfigure Private Service Connect. Only global internal addresses can use\nthis purpose.\n\nThis should only be set when using an Internal address."]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.purpose", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created address should reside.\nIf it is not provided, the provider region is used."]
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
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe URL of the subnetwork in which to reserve the address. If an IP\naddress is specified, it must be within the subnetwork's IP range.\nThis field can only be used with INTERNAL type with\nGCE_ENDPOINT/DNS_RESOLVER purposes."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `users` after provisioning.\nThe URLs of the resources that are using this address."]
    pub fn users(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeAddressTimeoutsElRef {
        ComputeAddressTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeAddress {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeAddress {}
impl ToListMappable for ComputeAddress {
    type O = ListRef<ComputeAddressRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeAddress_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_address".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeAddress {
    pub tf_id: String,
    #[doc = "Name of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeAddress {
    pub fn build(self, stack: &mut Stack) -> ComputeAddress {
        let out = ComputeAddress(Rc::new(ComputeAddress_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeAddressData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                address: core::default::Default::default(),
                address_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                ip_collection: core::default::Default::default(),
                ip_version: core::default::Default::default(),
                ipv6_endpoint_type: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                network_tier: core::default::Default::default(),
                prefix_length: core::default::Default::default(),
                project: core::default::Default::default(),
                purpose: core::default::Default::default(),
                region: core::default::Default::default(),
                subnetwork: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeAddressRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAddressRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeAddressRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address` after provisioning.\nThe static external IP address represented by this resource.\nThe IP address must be inside the specified subnetwork,\nif any. Set by the API if undefined."]
    pub fn address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_id` after provisioning.\nThe unique numeric identifier for the resource. This identifier is defined by the server."]
    pub fn address_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `address_type` after provisioning.\nThe type of address to reserve.\nNote: if you set this argument's value as 'INTERNAL' you need to leave the 'network_tier' argument unset in that resource block. Default value: \"EXTERNAL\" Possible values: [\"INTERNAL\", \"EXTERNAL\"]"]
    pub fn address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `ip_collection` after provisioning.\nReference to the source of external IPv4 addresses, like a PublicDelegatedPrefix(PDP) for BYOIP.\nThe PDP must support enhanced IPv4 allocations.\nUse one of the following formats to specify a PDP when reserving an external IPv4 address using BYOIP.\nFull resource URL, as in:\n  * 'https://www.googleapis.com/compute/v1/projects/{{projectId}}/regions/{{region}}/publicDelegatedPrefixes/{{pdp-name}}'\nPartial URL, as in:\n  * 'projects/{{projectId}}/regions/region/publicDelegatedPrefixes/{{pdp-name}}'\n  * 'regions/{{region}}/publicDelegatedPrefixes/{{pdp-name}}'"]
    pub fn ip_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_collection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ip_version` after provisioning.\nThe IP Version that will be used by this address. The default value is 'IPV4'. Possible values: [\"IPV4\", \"IPV6\"]"]
    pub fn ip_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_endpoint_type` after provisioning.\nThe endpoint type of this address, which should be VM or NETLB. This is\nused for deciding which type of endpoint this address can be used after\nthe external IPv6 address reservation. Possible values: [\"VM\", \"NETLB\"]"]
    pub fn ipv6_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_endpoint_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this address.  A list of key->value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe URL of the network in which to reserve the address. This field\ncan only be used with INTERNAL type with the VPC_PEERING and\nIPSEC_INTERCONNECT purposes."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\nThe networking tier used for configuring this address. If this field is not\nspecified, it is assumed to be PREMIUM.\nThis argument should not be used when configuring Internal addresses, because [network tier cannot be set for internal traffic; it's always Premium](https://cloud.google.com/network-tiers/docs/overview). Possible values: [\"PREMIUM\", \"STANDARD\"]"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_tier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_length` after provisioning.\nThe prefix length if the resource represents an IP range."]
    pub fn prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `purpose` after provisioning.\nThe purpose of this resource, which can be one of the following values.\n\n* GCE_ENDPOINT for addresses that are used by VM instances, alias IP\nranges, load balancers, and similar resources.\n\n* SHARED_LOADBALANCER_VIP for an address that can be used by multiple\ninternal load balancers.\n\n* VPC_PEERING for addresses that are reserved for VPC peer networks.\n\n* IPSEC_INTERCONNECT for addresses created from a private IP range that\nare reserved for a VLAN attachment in an HA VPN over Cloud Interconnect\nconfiguration. These addresses are regional resources.\n\n* PRIVATE_SERVICE_CONNECT for a private network address that is used to\nconfigure Private Service Connect. Only global internal addresses can use\nthis purpose.\n\nThis should only be set when using an Internal address."]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.purpose", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the created address should reside.\nIf it is not provided, the provider region is used."]
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
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe URL of the subnetwork in which to reserve the address. If an IP\naddress is specified, it must be within the subnetwork's IP range.\nThis field can only be used with INTERNAL type with\nGCE_ENDPOINT/DNS_RESOLVER purposes."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `users` after provisioning.\nThe URLs of the resources that are using this address."]
    pub fn users(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.users", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeAddressTimeoutsElRef {
        ComputeAddressTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeAddressTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeAddressTimeoutsEl {
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
impl ToListMappable for ComputeAddressTimeoutsEl {
    type O = BlockAssignable<ComputeAddressTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeAddressTimeoutsEl {}
impl BuildComputeAddressTimeoutsEl {
    pub fn build(self) -> ComputeAddressTimeoutsEl {
        ComputeAddressTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeAddressTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeAddressTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeAddressTimeoutsElRef {
        ComputeAddressTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeAddressTimeoutsElRef {
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
