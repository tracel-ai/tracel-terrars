use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRouterData {
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
    encrypted_interconnect_router: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bgp: Option<Vec<ComputeRouterBgpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    md5_authentication_keys: Option<Vec<ComputeRouterMd5AuthenticationKeysEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeRouterParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRouterTimeoutsEl>,
    dynamic: ComputeRouterDynamic,
}
struct ComputeRouter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRouterData>,
}
#[derive(Clone)]
pub struct ComputeRouter(Rc<ComputeRouter_>);
impl ComputeRouter {
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
    #[doc = "Set the field `encrypted_interconnect_router`.\nIndicates if a router is dedicated for use with encrypted VLAN\nattachments (interconnectAttachments)."]
    pub fn set_encrypted_interconnect_router(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().encrypted_interconnect_router = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nA reference to the network to which this router belongs."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where the router resides."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `bgp`.\n"]
    pub fn set_bgp(self, v: impl Into<BlockAssignable<ComputeRouterBgpEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bgp = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bgp = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `md5_authentication_keys`.\n"]
    pub fn set_md5_authentication_keys(
        self,
        v: impl Into<BlockAssignable<ComputeRouterMd5AuthenticationKeysEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().md5_authentication_keys = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.md5_authentication_keys = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl Into<BlockAssignable<ComputeRouterParamsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRouterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `encrypted_interconnect_router` after provisioning.\nIndicates if a router is dedicated for use with encrypted VLAN\nattachments (interconnectAttachments)."]
    pub fn encrypted_interconnect_router(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted_interconnect_router", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `network` after provisioning.\nA reference to the network to which this router belongs."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router resides."]
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
    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> ListRef<ComputeRouterBgpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `md5_authentication_keys` after provisioning.\n"]
    pub fn md5_authentication_keys(&self) -> ListRef<ComputeRouterMd5AuthenticationKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.md5_authentication_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeRouterParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterTimeoutsElRef {
        ComputeRouterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRouter {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRouter {}
impl ToListMappable for ComputeRouter {
    type O = ListRef<ComputeRouterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRouter_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_router".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRouter {
    pub tf_id: String,
    #[doc = "Name of the resource. The name must be 1-63 characters long, and\ncomply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?'\nwhich means the first character must be a lowercase letter, and all\nfollowing characters must be a dash, lowercase letter, or digit,\nexcept the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeRouter {
    pub fn build(self, stack: &mut Stack) -> ComputeRouter {
        let out = ComputeRouter(Rc::new(ComputeRouter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRouterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                encrypted_interconnect_router: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                bgp: core::default::Default::default(),
                md5_authentication_keys: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRouterRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRouterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `encrypted_interconnect_router` after provisioning.\nIndicates if a router is dedicated for use with encrypted VLAN\nattachments (interconnectAttachments)."]
    pub fn encrypted_interconnect_router(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted_interconnect_router", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `network` after provisioning.\nA reference to the network to which this router belongs."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router resides."]
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
    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> ListRef<ComputeRouterBgpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `md5_authentication_keys` after provisioning.\n"]
    pub fn md5_authentication_keys(&self) -> ListRef<ComputeRouterMd5AuthenticationKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.md5_authentication_keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeRouterParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterTimeoutsElRef {
        ComputeRouterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouterBgpElAdvertisedIpRangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    range: PrimField<String>,
}
impl ComputeRouterBgpElAdvertisedIpRangesEl {
    #[doc = "Set the field `description`.\nUser-specified description for the IP range."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouterBgpElAdvertisedIpRangesEl {
    type O = BlockAssignable<ComputeRouterBgpElAdvertisedIpRangesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterBgpElAdvertisedIpRangesEl {
    #[doc = "The IP range to advertise. The value must be a\nCIDR-formatted string."]
    pub range: PrimField<String>,
}
impl BuildComputeRouterBgpElAdvertisedIpRangesEl {
    pub fn build(self) -> ComputeRouterBgpElAdvertisedIpRangesEl {
        ComputeRouterBgpElAdvertisedIpRangesEl {
            description: core::default::Default::default(),
            range: self.range,
        }
    }
}
pub struct ComputeRouterBgpElAdvertisedIpRangesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterBgpElAdvertisedIpRangesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterBgpElAdvertisedIpRangesElRef {
        ComputeRouterBgpElAdvertisedIpRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterBgpElAdvertisedIpRangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-specified description for the IP range."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `range` after provisioning.\nThe IP range to advertise. The value must be a\nCIDR-formatted string."]
    pub fn range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.range", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRouterBgpElDynamic {
    advertised_ip_ranges: Option<DynamicBlock<ComputeRouterBgpElAdvertisedIpRangesEl>>,
}
#[derive(Serialize)]
pub struct ComputeRouterBgpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advertise_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_groups: Option<ListField<PrimField<String>>>,
    asn: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identifier_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keepalive_interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advertised_ip_ranges: Option<Vec<ComputeRouterBgpElAdvertisedIpRangesEl>>,
    dynamic: ComputeRouterBgpElDynamic,
}
impl ComputeRouterBgpEl {
    #[doc = "Set the field `advertise_mode`.\nUser-specified flag to indicate which mode to use for advertisement. Default value: \"DEFAULT\" Possible values: [\"DEFAULT\", \"CUSTOM\"]"]
    pub fn set_advertise_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.advertise_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_groups`.\nUser-specified list of prefix groups to advertise in custom mode.\nThis field can only be populated if advertiseMode is CUSTOM and\nis advertised to all peers of the router. These groups will be\nadvertised in addition to any specified prefixes. Leave this field\nblank to advertise no custom groups.\n\nThis enum field has the one valid value: ALL_SUBNETS"]
    pub fn set_advertised_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.advertised_groups = Some(v.into());
        self
    }
    #[doc = "Set the field `identifier_range`.\nExplicitly specifies a range of valid BGP Identifiers for this Router.\nIt is provided as a link-local IPv4 range (from 169.254.0.0/16), of\nsize at least /30, even if the BGP sessions are over IPv6. It must\nnot overlap with any IPv4 BGP session ranges. Other vendors commonly\ncall this router ID."]
    pub fn set_identifier_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identifier_range = Some(v.into());
        self
    }
    #[doc = "Set the field `keepalive_interval`.\nThe interval in seconds between BGP keepalive messages that are sent\nto the peer. Hold time is three times the interval at which keepalive\nmessages are sent, and the hold time is the maximum number of seconds\nallowed to elapse between successive keepalive messages that BGP\nreceives from a peer.\n\nBGP will use the smaller of either the local hold time value or the\npeer's hold time value as the hold time for the BGP connection\nbetween the two peers. If set, this value must be between 20 and 60.\nThe default is 20."]
    pub fn set_keepalive_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.keepalive_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `advertised_ip_ranges`.\n"]
    pub fn set_advertised_ip_ranges(
        mut self,
        v: impl Into<BlockAssignable<ComputeRouterBgpElAdvertisedIpRangesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advertised_ip_ranges = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advertised_ip_ranges = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRouterBgpEl {
    type O = BlockAssignable<ComputeRouterBgpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterBgpEl {
    #[doc = "Local BGP Autonomous System Number (ASN). Must be an RFC6996\nprivate ASN, either 16-bit or 32-bit. The value will be fixed for\nthis router resource. All VPN tunnels that link to this router\nwill have the same local ASN."]
    pub asn: PrimField<f64>,
}
impl BuildComputeRouterBgpEl {
    pub fn build(self) -> ComputeRouterBgpEl {
        ComputeRouterBgpEl {
            advertise_mode: core::default::Default::default(),
            advertised_groups: core::default::Default::default(),
            asn: self.asn,
            identifier_range: core::default::Default::default(),
            keepalive_interval: core::default::Default::default(),
            advertised_ip_ranges: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRouterBgpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterBgpElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterBgpElRef {
        ComputeRouterBgpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterBgpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advertise_mode` after provisioning.\nUser-specified flag to indicate which mode to use for advertisement. Default value: \"DEFAULT\" Possible values: [\"DEFAULT\", \"CUSTOM\"]"]
    pub fn advertise_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.advertise_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `advertised_groups` after provisioning.\nUser-specified list of prefix groups to advertise in custom mode.\nThis field can only be populated if advertiseMode is CUSTOM and\nis advertised to all peers of the router. These groups will be\nadvertised in addition to any specified prefixes. Leave this field\nblank to advertise no custom groups.\n\nThis enum field has the one valid value: ALL_SUBNETS"]
    pub fn advertised_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advertised_groups", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `asn` after provisioning.\nLocal BGP Autonomous System Number (ASN). Must be an RFC6996\nprivate ASN, either 16-bit or 32-bit. The value will be fixed for\nthis router resource. All VPN tunnels that link to this router\nwill have the same local ASN."]
    pub fn asn(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }
    #[doc = "Get a reference to the value of field `identifier_range` after provisioning.\nExplicitly specifies a range of valid BGP Identifiers for this Router.\nIt is provided as a link-local IPv4 range (from 169.254.0.0/16), of\nsize at least /30, even if the BGP sessions are over IPv6. It must\nnot overlap with any IPv4 BGP session ranges. Other vendors commonly\ncall this router ID."]
    pub fn identifier_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identifier_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `keepalive_interval` after provisioning.\nThe interval in seconds between BGP keepalive messages that are sent\nto the peer. Hold time is three times the interval at which keepalive\nmessages are sent, and the hold time is the maximum number of seconds\nallowed to elapse between successive keepalive messages that BGP\nreceives from a peer.\n\nBGP will use the smaller of either the local hold time value or the\npeer's hold time value as the hold time for the BGP connection\nbetween the two peers. If set, this value must be between 20 and 60.\nThe default is 20."]
    pub fn keepalive_interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.keepalive_interval", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouterMd5AuthenticationKeysEl {
    key: PrimField<String>,
    name: PrimField<String>,
}
impl ComputeRouterMd5AuthenticationKeysEl {}
impl ToListMappable for ComputeRouterMd5AuthenticationKeysEl {
    type O = BlockAssignable<ComputeRouterMd5AuthenticationKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterMd5AuthenticationKeysEl {
    #[doc = "Value of the key used for MD5 authentication."]
    pub key: PrimField<String>,
    #[doc = "Name used to identify the key. Must be unique within a router.\nMust be referenced by exactly one bgpPeer. Must comply with RFC1035."]
    pub name: PrimField<String>,
}
impl BuildComputeRouterMd5AuthenticationKeysEl {
    pub fn build(self) -> ComputeRouterMd5AuthenticationKeysEl {
        ComputeRouterMd5AuthenticationKeysEl {
            key: self.key,
            name: self.name,
        }
    }
}
pub struct ComputeRouterMd5AuthenticationKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterMd5AuthenticationKeysElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterMd5AuthenticationKeysElRef {
        ComputeRouterMd5AuthenticationKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterMd5AuthenticationKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nValue of the key used for MD5 authentication."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName used to identify the key. Must be unique within a router.\nMust be referenced by exactly one bgpPeer. Must comply with RFC1035."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouterParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeRouterParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the router. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouterParamsEl {
    type O = BlockAssignable<ComputeRouterParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterParamsEl {}
impl BuildComputeRouterParamsEl {
    pub fn build(self) -> ComputeRouterParamsEl {
        ComputeRouterParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouterParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterParamsElRef {
        ComputeRouterParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the router. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRouterTimeoutsEl {
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
impl ToListMappable for ComputeRouterTimeoutsEl {
    type O = BlockAssignable<ComputeRouterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterTimeoutsEl {}
impl BuildComputeRouterTimeoutsEl {
    pub fn build(self) -> ComputeRouterTimeoutsEl {
        ComputeRouterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterTimeoutsElRef {
        ComputeRouterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterTimeoutsElRef {
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
struct ComputeRouterDynamic {
    bgp: Option<DynamicBlock<ComputeRouterBgpEl>>,
    md5_authentication_keys: Option<DynamicBlock<ComputeRouterMd5AuthenticationKeysEl>>,
    params: Option<DynamicBlock<ComputeRouterParamsEl>>,
}
