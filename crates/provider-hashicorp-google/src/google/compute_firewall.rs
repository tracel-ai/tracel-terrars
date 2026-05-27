use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeFirewallData {
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
    destination_ranges: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ranges: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_service_accounts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_service_accounts: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow: Option<Vec<ComputeFirewallAllowEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny: Option<Vec<ComputeFirewallDenyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_config: Option<Vec<ComputeFirewallLogConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeFirewallParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeFirewallTimeoutsEl>,
    dynamic: ComputeFirewallDynamic,
}
struct ComputeFirewall_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeFirewallData>,
}
#[derive(Clone)]
pub struct ComputeFirewall(Rc<ComputeFirewall_>);
impl ComputeFirewall {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `destination_ranges`.\nIf destination ranges are specified, the firewall will apply only to\ntraffic that has destination IP address in these ranges. These ranges\nmust be expressed in CIDR format. IPv4 or IPv6 ranges are supported."]
    pub fn set_destination_ranges(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().destination_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `direction`.\nDirection of traffic to which this firewall applies; default is\nINGRESS. Note: For INGRESS traffic, one of 'source_ranges',\n'source_tags' or 'source_service_accounts' is required. Possible values: [\"INGRESS\", \"EGRESS\"]"]
    pub fn set_direction(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().direction = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nDenotes whether the firewall rule is disabled, i.e not applied to the\nnetwork it is associated with. When set to true, the firewall rule is\nnot enforced and the network behaves as if it did not exist. If this\nis unspecified, the firewall rule will be enabled."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_logging`.\nThis field denotes whether to enable logging for a particular firewall rule. If logging is enabled, logs will be exported to Stackdriver."]
    pub fn set_enable_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `priority`.\nPriority for this rule. This is an integer between 0 and 65535, both\ninclusive. When not specified, the value assumed is 1000. Relative\npriorities determine precedence of conflicting rules. Lower value of\npriority implies higher precedence (eg, a rule with priority 0 has\nhigher precedence than a rule with priority 1). DENY rules take\nprecedence over ALLOW rules having equal priority."]
    pub fn set_priority(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().priority = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `source_ranges`.\nIf source ranges are specified, the firewall will apply only to\ntraffic that has source IP address in these ranges. These ranges must\nbe expressed in CIDR format. One or both of sourceRanges and\nsourceTags may be set. If both properties are set, the firewall will\napply to traffic that has source IP address within sourceRanges OR the\nsource IP that belongs to a tag listed in the sourceTags property. The\nconnection does not need to match both properties for the firewall to\napply. IPv4 or IPv6 ranges are supported. For INGRESS traffic, one of\n'source_ranges', 'source_tags' or 'source_service_accounts' is required."]
    pub fn set_source_ranges(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().source_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `source_service_accounts`.\nIf source service accounts are specified, the firewall will apply only\nto traffic originating from an instance with a service account in this\nlist. Source service accounts cannot be used to control traffic to an\ninstance's external IP address because service accounts are associated\nwith an instance, not an IP address. sourceRanges can be set at the\nsame time as sourceServiceAccounts. If both are set, the firewall will\napply to traffic that has source IP address within sourceRanges OR the\nsource IP belongs to an instance with service account listed in\nsourceServiceAccount. The connection does not need to match both\nproperties for the firewall to apply. sourceServiceAccounts cannot be\nused at the same time as sourceTags or targetTags. For INGRESS traffic,\none of 'source_ranges', 'source_tags' or 'source_service_accounts' is required."]
    pub fn set_source_service_accounts(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().source_service_accounts = Some(v.into());
        self
    }
    #[doc = "Set the field `source_tags`.\nIf source tags are specified, the firewall will apply only to traffic\nwith source IP that belongs to a tag listed in source tags. Source\ntags cannot be used to control traffic to an instance's external IP\naddress. Because tags are associated with an instance, not an IP\naddress. One or both of sourceRanges and sourceTags may be set. If\nboth properties are set, the firewall will apply to traffic that has\nsource IP address within sourceRanges OR the source IP that belongs to\na tag listed in the sourceTags property. The connection does not need\nto match both properties for the firewall to apply. For INGRESS traffic,\none of 'source_ranges', 'source_tags' or 'source_service_accounts' is required."]
    pub fn set_source_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().source_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `target_service_accounts`.\nA list of service accounts indicating sets of instances located in the\nnetwork that may make network connections as specified in allowed[].\ntargetServiceAccounts cannot be used at the same time as targetTags or\nsourceTags. If neither targetServiceAccounts nor targetTags are\nspecified, the firewall rule applies to all instances on the specified\nnetwork."]
    pub fn set_target_service_accounts(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_service_accounts = Some(v.into());
        self
    }
    #[doc = "Set the field `target_tags`.\nA list of instance tags indicating sets of instances located in the\nnetwork that may make network connections as specified in allowed[].\nIf no targetTags are specified, the firewall rule applies to all\ninstances on the specified network."]
    pub fn set_target_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(self, v: impl Into<BlockAssignable<ComputeFirewallAllowEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allow = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allow = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `deny`.\n"]
    pub fn set_deny(self, v: impl Into<BlockAssignable<ComputeFirewallDenyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deny = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deny = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(self, v: impl Into<BlockAssignable<ComputeFirewallLogConfigEl>>) -> Self {
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
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl Into<BlockAssignable<ComputeFirewallParamsEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<ComputeFirewallTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_ranges` after provisioning.\nIf destination ranges are specified, the firewall will apply only to\ntraffic that has destination IP address in these ranges. These ranges\nmust be expressed in CIDR format. IPv4 or IPv6 ranges are supported."]
    pub fn destination_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.destination_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `direction` after provisioning.\nDirection of traffic to which this firewall applies; default is\nINGRESS. Note: For INGRESS traffic, one of 'source_ranges',\n'source_tags' or 'source_service_accounts' is required. Possible values: [\"INGRESS\", \"EGRESS\"]"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.direction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nDenotes whether the firewall rule is disabled, i.e not applied to the\nnetwork it is associated with. When set to true, the firewall rule is\nnot enforced and the network behaves as if it did not exist. If this\nis unspecified, the firewall rule will be enabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\nThis field denotes whether to enable logging for a particular firewall rule. If logging is enabled, logs will be exported to Stackdriver."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the network to attach this firewall to."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority for this rule. This is an integer between 0 and 65535, both\ninclusive. When not specified, the value assumed is 1000. Relative\npriorities determine precedence of conflicting rules. Lower value of\npriority implies higher precedence (eg, a rule with priority 0 has\nhigher precedence than a rule with priority 1). DENY rules take\nprecedence over ALLOW rules having equal priority."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ranges` after provisioning.\nIf source ranges are specified, the firewall will apply only to\ntraffic that has source IP address in these ranges. These ranges must\nbe expressed in CIDR format. One or both of sourceRanges and\nsourceTags may be set. If both properties are set, the firewall will\napply to traffic that has source IP address within sourceRanges OR the\nsource IP that belongs to a tag listed in the sourceTags property. The\nconnection does not need to match both properties for the firewall to\napply. IPv4 or IPv6 ranges are supported. For INGRESS traffic, one of\n'source_ranges', 'source_tags' or 'source_service_accounts' is required."]
    pub fn source_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_service_accounts` after provisioning.\nIf source service accounts are specified, the firewall will apply only\nto traffic originating from an instance with a service account in this\nlist. Source service accounts cannot be used to control traffic to an\ninstance's external IP address because service accounts are associated\nwith an instance, not an IP address. sourceRanges can be set at the\nsame time as sourceServiceAccounts. If both are set, the firewall will\napply to traffic that has source IP address within sourceRanges OR the\nsource IP belongs to an instance with service account listed in\nsourceServiceAccount. The connection does not need to match both\nproperties for the firewall to apply. sourceServiceAccounts cannot be\nused at the same time as sourceTags or targetTags. For INGRESS traffic,\none of 'source_ranges', 'source_tags' or 'source_service_accounts' is required."]
    pub fn source_service_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_service_accounts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_tags` after provisioning.\nIf source tags are specified, the firewall will apply only to traffic\nwith source IP that belongs to a tag listed in source tags. Source\ntags cannot be used to control traffic to an instance's external IP\naddress. Because tags are associated with an instance, not an IP\naddress. One or both of sourceRanges and sourceTags may be set. If\nboth properties are set, the firewall will apply to traffic that has\nsource IP address within sourceRanges OR the source IP that belongs to\na tag listed in the sourceTags property. The connection does not need\nto match both properties for the firewall to apply. For INGRESS traffic,\none of 'source_ranges', 'source_tags' or 'source_service_accounts' is required."]
    pub fn source_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_service_accounts` after provisioning.\nA list of service accounts indicating sets of instances located in the\nnetwork that may make network connections as specified in allowed[].\ntargetServiceAccounts cannot be used at the same time as targetTags or\nsourceTags. If neither targetServiceAccounts nor targetTags are\nspecified, the firewall rule applies to all instances on the specified\nnetwork."]
    pub fn target_service_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_service_accounts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_tags` after provisioning.\nA list of instance tags indicating sets of instances located in the\nnetwork that may make network connections as specified in allowed[].\nIf no targetTags are specified, the firewall rule applies to all\ninstances on the specified network."]
    pub fn target_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeFirewallLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeFirewallParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeFirewallTimeoutsElRef {
        ComputeFirewallTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeFirewall {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeFirewall {}
impl ToListMappable for ComputeFirewall {
    type O = ListRef<ComputeFirewallRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeFirewall_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_firewall".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeFirewall {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "The name or self_link of the network to attach this firewall to."]
    pub network: PrimField<String>,
}
impl BuildComputeFirewall {
    pub fn build(self, stack: &mut Stack) -> ComputeFirewall {
        let out = ComputeFirewall(Rc::new(ComputeFirewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeFirewallData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                destination_ranges: core::default::Default::default(),
                direction: core::default::Default::default(),
                disabled: core::default::Default::default(),
                enable_logging: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                network: self.network,
                priority: core::default::Default::default(),
                project: core::default::Default::default(),
                source_ranges: core::default::Default::default(),
                source_service_accounts: core::default::Default::default(),
                source_tags: core::default::Default::default(),
                target_service_accounts: core::default::Default::default(),
                target_tags: core::default::Default::default(),
                allow: core::default::Default::default(),
                deny: core::default::Default::default(),
                log_config: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeFirewallRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeFirewallRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destination_ranges` after provisioning.\nIf destination ranges are specified, the firewall will apply only to\ntraffic that has destination IP address in these ranges. These ranges\nmust be expressed in CIDR format. IPv4 or IPv6 ranges are supported."]
    pub fn destination_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.destination_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `direction` after provisioning.\nDirection of traffic to which this firewall applies; default is\nINGRESS. Note: For INGRESS traffic, one of 'source_ranges',\n'source_tags' or 'source_service_accounts' is required. Possible values: [\"INGRESS\", \"EGRESS\"]"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.direction", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nDenotes whether the firewall rule is disabled, i.e not applied to the\nnetwork it is associated with. When set to true, the firewall rule is\nnot enforced and the network behaves as if it did not exist. If this\nis unspecified, the firewall rule will be enabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\nThis field denotes whether to enable logging for a particular firewall rule. If logging is enabled, logs will be exported to Stackdriver."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the network to attach this firewall to."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nPriority for this rule. This is an integer between 0 and 65535, both\ninclusive. When not specified, the value assumed is 1000. Relative\npriorities determine precedence of conflicting rules. Lower value of\npriority implies higher precedence (eg, a rule with priority 0 has\nhigher precedence than a rule with priority 1). DENY rules take\nprecedence over ALLOW rules having equal priority."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_ranges` after provisioning.\nIf source ranges are specified, the firewall will apply only to\ntraffic that has source IP address in these ranges. These ranges must\nbe expressed in CIDR format. One or both of sourceRanges and\nsourceTags may be set. If both properties are set, the firewall will\napply to traffic that has source IP address within sourceRanges OR the\nsource IP that belongs to a tag listed in the sourceTags property. The\nconnection does not need to match both properties for the firewall to\napply. IPv4 or IPv6 ranges are supported. For INGRESS traffic, one of\n'source_ranges', 'source_tags' or 'source_service_accounts' is required."]
    pub fn source_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_service_accounts` after provisioning.\nIf source service accounts are specified, the firewall will apply only\nto traffic originating from an instance with a service account in this\nlist. Source service accounts cannot be used to control traffic to an\ninstance's external IP address because service accounts are associated\nwith an instance, not an IP address. sourceRanges can be set at the\nsame time as sourceServiceAccounts. If both are set, the firewall will\napply to traffic that has source IP address within sourceRanges OR the\nsource IP belongs to an instance with service account listed in\nsourceServiceAccount. The connection does not need to match both\nproperties for the firewall to apply. sourceServiceAccounts cannot be\nused at the same time as sourceTags or targetTags. For INGRESS traffic,\none of 'source_ranges', 'source_tags' or 'source_service_accounts' is required."]
    pub fn source_service_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_service_accounts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_tags` after provisioning.\nIf source tags are specified, the firewall will apply only to traffic\nwith source IP that belongs to a tag listed in source tags. Source\ntags cannot be used to control traffic to an instance's external IP\naddress. Because tags are associated with an instance, not an IP\naddress. One or both of sourceRanges and sourceTags may be set. If\nboth properties are set, the firewall will apply to traffic that has\nsource IP address within sourceRanges OR the source IP that belongs to\na tag listed in the sourceTags property. The connection does not need\nto match both properties for the firewall to apply. For INGRESS traffic,\none of 'source_ranges', 'source_tags' or 'source_service_accounts' is required."]
    pub fn source_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_service_accounts` after provisioning.\nA list of service accounts indicating sets of instances located in the\nnetwork that may make network connections as specified in allowed[].\ntargetServiceAccounts cannot be used at the same time as targetTags or\nsourceTags. If neither targetServiceAccounts nor targetTags are\nspecified, the firewall rule applies to all instances on the specified\nnetwork."]
    pub fn target_service_accounts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_service_accounts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_tags` after provisioning.\nA list of instance tags indicating sets of instances located in the\nnetwork that may make network connections as specified in allowed[].\nIf no targetTags are specified, the firewall rule applies to all\ninstances on the specified network."]
    pub fn target_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.target_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> ListRef<ComputeFirewallLogConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.log_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeFirewallParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeFirewallTimeoutsElRef {
        ComputeFirewallTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallAllowEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<PrimField<String>>>,
    protocol: PrimField<String>,
}
impl ComputeFirewallAllowEl {
    #[doc = "Set the field `ports`.\nAn optional list of ports to which this rule applies. This field\nis only applicable for UDP or TCP protocol. Each entry must be\neither an integer or a range. If not specified, this rule\napplies to connections through any port.\n\nExample inputs include: [22], [80, 443], and\n[\"12345-12349\"]."]
    pub fn set_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ports = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallAllowEl {
    type O = BlockAssignable<ComputeFirewallAllowEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallAllowEl {
    #[doc = "The IP protocol to which this rule applies. The protocol type is\nrequired when creating a firewall rule. This value can either be\none of the following well known protocol strings (tcp, udp,\nicmp, esp, ah, sctp, ipip, all), or the IP protocol number."]
    pub protocol: PrimField<String>,
}
impl BuildComputeFirewallAllowEl {
    pub fn build(self) -> ComputeFirewallAllowEl {
        ComputeFirewallAllowEl {
            ports: core::default::Default::default(),
            protocol: self.protocol,
        }
    }
}
pub struct ComputeFirewallAllowElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallAllowElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallAllowElRef {
        ComputeFirewallAllowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallAllowElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\nAn optional list of ports to which this rule applies. This field\nis only applicable for UDP or TCP protocol. Each entry must be\neither an integer or a range. If not specified, this rule\napplies to connections through any port.\n\nExample inputs include: [22], [80, 443], and\n[\"12345-12349\"]."]
    pub fn ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nThe IP protocol to which this rule applies. The protocol type is\nrequired when creating a firewall rule. This value can either be\none of the following well known protocol strings (tcp, udp,\nicmp, esp, ah, sctp, ipip, all), or the IP protocol number."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallDenyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<PrimField<String>>>,
    protocol: PrimField<String>,
}
impl ComputeFirewallDenyEl {
    #[doc = "Set the field `ports`.\nAn optional list of ports to which this rule applies. This field\nis only applicable for UDP or TCP protocol. Each entry must be\neither an integer or a range. If not specified, this rule\napplies to connections through any port.\n\nExample inputs include: [22], [80, 443], and\n[\"12345-12349\"]."]
    pub fn set_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ports = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallDenyEl {
    type O = BlockAssignable<ComputeFirewallDenyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallDenyEl {
    #[doc = "The IP protocol to which this rule applies. The protocol type is\nrequired when creating a firewall rule. This value can either be\none of the following well known protocol strings (tcp, udp,\nicmp, esp, ah, sctp, ipip, all), or the IP protocol number."]
    pub protocol: PrimField<String>,
}
impl BuildComputeFirewallDenyEl {
    pub fn build(self) -> ComputeFirewallDenyEl {
        ComputeFirewallDenyEl {
            ports: core::default::Default::default(),
            protocol: self.protocol,
        }
    }
}
pub struct ComputeFirewallDenyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallDenyElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallDenyElRef {
        ComputeFirewallDenyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallDenyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\nAn optional list of ports to which this rule applies. This field\nis only applicable for UDP or TCP protocol. Each entry must be\neither an integer or a range. If not specified, this rule\napplies to connections through any port.\n\nExample inputs include: [22], [80, 443], and\n[\"12345-12349\"]."]
    pub fn ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\nThe IP protocol to which this rule applies. The protocol type is\nrequired when creating a firewall rule. This value can either be\none of the following well known protocol strings (tcp, udp,\nicmp, esp, ah, sctp, ipip, all), or the IP protocol number."]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallLogConfigEl {
    metadata: PrimField<String>,
}
impl ComputeFirewallLogConfigEl {}
impl ToListMappable for ComputeFirewallLogConfigEl {
    type O = BlockAssignable<ComputeFirewallLogConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallLogConfigEl {
    #[doc = "This field denotes whether to include or exclude metadata for firewall logs. Possible values: [\"EXCLUDE_ALL_METADATA\", \"INCLUDE_ALL_METADATA\"]"]
    pub metadata: PrimField<String>,
}
impl BuildComputeFirewallLogConfigEl {
    pub fn build(self) -> ComputeFirewallLogConfigEl {
        ComputeFirewallLogConfigEl {
            metadata: self.metadata,
        }
    }
}
pub struct ComputeFirewallLogConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallLogConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallLogConfigElRef {
        ComputeFirewallLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallLogConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nThis field denotes whether to include or exclude metadata for firewall logs. Possible values: [\"EXCLUDE_ALL_METADATA\", \"INCLUDE_ALL_METADATA\"]"]
    pub fn metadata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeFirewallParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the firewall. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeFirewallParamsEl {
    type O = BlockAssignable<ComputeFirewallParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallParamsEl {}
impl BuildComputeFirewallParamsEl {
    pub fn build(self) -> ComputeFirewallParamsEl {
        ComputeFirewallParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallParamsElRef {
        ComputeFirewallParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the firewall. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456. The field is ignored when empty.\nThe field is immutable and causes resource replacement when mutated. This field is only\nset at create time and modifying this field after creation will trigger recreation.\nTo apply tags to an existing resource, see the google_tags_tag_binding resource."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeFirewallTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeFirewallTimeoutsEl {
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
impl ToListMappable for ComputeFirewallTimeoutsEl {
    type O = BlockAssignable<ComputeFirewallTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeFirewallTimeoutsEl {}
impl BuildComputeFirewallTimeoutsEl {
    pub fn build(self) -> ComputeFirewallTimeoutsEl {
        ComputeFirewallTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeFirewallTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeFirewallTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallTimeoutsElRef {
        ComputeFirewallTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeFirewallTimeoutsElRef {
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
struct ComputeFirewallDynamic {
    allow: Option<DynamicBlock<ComputeFirewallAllowEl>>,
    deny: Option<DynamicBlock<ComputeFirewallDenyEl>>,
    log_config: Option<DynamicBlock<ComputeFirewallLogConfigEl>>,
    params: Option<DynamicBlock<ComputeFirewallParamsEl>>,
}
