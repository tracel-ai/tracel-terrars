use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeInterconnectData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    interconnect_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    link_type: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    macsec_enabled: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    noc_contact_email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_features: Option<ListField<PrimField<String>>>,
    requested_link_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    macsec: Option<Vec<ComputeInterconnectMacsecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeInterconnectParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeInterconnectTimeoutsEl>,
    dynamic: ComputeInterconnectDynamic,
}
struct ComputeInterconnect_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeInterconnectData>,
}
#[derive(Clone)]
pub struct ComputeInterconnect(Rc<ComputeInterconnect_>);
impl ComputeInterconnect {
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
    #[doc = "Set the field `admin_enabled`.\nAdministrative status of the interconnect. When this is set to true, the Interconnect is\nfunctional and can carry traffic. When set to false, no packets can be carried over the\ninterconnect and no BGP routes are exchanged over it. By default, the status is set to true."]
    pub fn set_admin_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().admin_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `customer_name`.\nCustomer name, to put in the Letter of Authorization as the party authorized to request a\ncrossconnect. This field is required for Dedicated and Partner Interconnect, should not be specified\nfor cross-cloud interconnect."]
    pub fn set_customer_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_name = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when you create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels for this resource. These can only be added or modified by the setLabels\nmethod. Each label key/value pair must comply with RFC1035. Label values may be empty.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `macsec_enabled`.\nEnable or disable MACsec on this Interconnect connection.\nMACsec enablement fails if the MACsec object is not specified."]
    pub fn set_macsec_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().macsec_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `noc_contact_email`.\nEmail address to contact the customer NOC for operations and maintenance notifications\nregarding this Interconnect. If specified, this will be used for notifications in addition to\nall other forms described, such as Cloud Monitoring logs alerting and Cloud Notifications.\nThis field is required for users who sign up for Cloud Interconnect using workforce identity\nfederation."]
    pub fn set_noc_contact_email(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().noc_contact_email = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `remote_location`.\nIndicates that this is a Cross-Cloud Interconnect. This field specifies the location outside\nof Google's network that the interconnect is connected to."]
    pub fn set_remote_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().remote_location = Some(v.into());
        self
    }
    #[doc = "Set the field `requested_features`.\nList of features to request for this Interconnect connection. This field is only applicable during Interconnect creation and cannot be modified later.\nPossible values include:\n- 'IF_MACSEC': Provisions the connection on hardware ports that support MACsec (Media Access Control Security). If not specified, the system may allocate non-MACsec capable ports if available.\n- 'IF_L2_FORWARDING': Provisions the connection for Layer 2 (L2) traffic forwarding. If not specified, the connection defaults to Layer 3 (L3) traffic forwarding.\n- 'IF_CROSS_SITE_NETWORK': Provisions the connection exclusively for Cross-Site Networking.\nNote: 'MACSEC' is a legacy value for compatibility reasons and has the same effect as 'IF_MACSEC'. 'IF_MACSEC' is preferred. Possible values: [\"MACSEC\", \"CROSS_SITE_NETWORK\", \"IF_MACSEC\", \"IF_L2_FORWARDING\"]"]
    pub fn set_requested_features(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().requested_features = Some(v.into());
        self
    }
    #[doc = "Set the field `macsec`.\n"]
    pub fn set_macsec(self, v: impl Into<BlockAssignable<ComputeInterconnectMacsecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().macsec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.macsec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl Into<BlockAssignable<ComputeInterconnectParamsEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<ComputeInterconnectTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `admin_enabled` after provisioning.\nAdministrative status of the interconnect. When this is set to true, the Interconnect is\nfunctional and can carry traffic. When set to false, no packets can be carried over the\ninterconnect and no BGP routes are exchanged over it. By default, the status is set to true."]
    pub fn admin_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_features` after provisioning.\n[Output Only] List of features that are available on this Interconnect connection based on the provisioned hardware and configuration.\nPossible values include:\n- 'IF_MACSEC': Indicates the Interconnect connection is provisioned on MACsec capable hardware ports. If this feature is not present, the connection does not support MACsec, and any attempt to enable it will fail.\n- 'IF_L2_FORWARDING': Indicates the Interconnect connection can be used for Layer 2 (L2) traffic forwarding. If not present, the connection cannot be used with L2 forwarding attachments.\n- 'IF_CROSS_SITE_NETWORK': Indicates the Interconnect connection is provisioned for Cross-Site Networking.\nNote: 'MACSEC' is a legacy value and has the same meaning as 'IF_MACSEC'."]
    pub fn available_features(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_infos` after provisioning.\nA list of CircuitInfo objects, that describe the individual circuits in this LAG."]
    pub fn circuit_infos(&self) -> ListRef<ComputeInterconnectCircuitInfosElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_infos", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_name` after provisioning.\nCustomer name, to put in the Letter of Authorization as the party authorized to request a\ncrossconnect. This field is required for Dedicated and Partner Interconnect, should not be specified\nfor cross-cloud interconnect."]
    pub fn customer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
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
    #[doc = "Get a reference to the value of field `expected_outages` after provisioning.\nA list of outages expected for this Interconnect."]
    pub fn expected_outages(&self) -> ListRef<ComputeInterconnectExpectedOutagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expected_outages", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `google_ip_address` after provisioning.\nIP address configured on the Google side of the Interconnect link.\nThis can be used only for ping tests."]
    pub fn google_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.google_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `google_reference_id` after provisioning.\nGoogle reference ID to be used when raising support tickets with Google or otherwise to debug\nbackend connectivity issues."]
    pub fn google_reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.google_reference_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interconnect_attachments` after provisioning.\nA list of the URLs of all InterconnectAttachments configured to use this Interconnect."]
    pub fn interconnect_attachments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.interconnect_attachments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `interconnect_groups` after provisioning.\nURLs of InterconnectGroups that include this Interconnect.\nOrder is arbitrary and items are unique."]
    pub fn interconnect_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.interconnect_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `interconnect_type` after provisioning.\nType of interconnect. Note that a value IT_PRIVATE has been deprecated in favor of DEDICATED.\nCan take one of the following values:\n  - PARTNER: A partner-managed interconnection shared between customers though a partner.\n  - DEDICATED: A dedicated physical interconnection with the customer. Possible values: [\"DEDICATED\", \"PARTNER\", \"IT_PRIVATE\"]"]
    pub fn interconnect_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interconnect_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nA fingerprint for the labels being applied to this Interconnect, which is essentially a hash\nof the labels set used for optimistic locking. The fingerprint is initially generated by\nCompute Engine and changes after every request to modify or update labels.\nYou must always provide an up-to-date fingerprint hash in order to update or change labels,\notherwise the request will fail with error 412 conditionNotMet."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels for this resource. These can only be added or modified by the setLabels\nmethod. Each label key/value pair must comply with RFC1035. Label values may be empty.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `link_type` after provisioning.\nType of link requested. Note that this field indicates the speed of each of the links in the\nbundle, not the speed of the entire bundle. Can take one of the following values:\n  - LINK_TYPE_ETHERNET_10G_LR: A 10G Ethernet with LR optics.\n  - LINK_TYPE_ETHERNET_100G_LR: A 100G Ethernet with LR optics.\n  - LINK_TYPE_ETHERNET_400G_LR4: A 400G Ethernet with LR4 optics Possible values: [\"LINK_TYPE_ETHERNET_10G_LR\", \"LINK_TYPE_ETHERNET_100G_LR\", \"LINK_TYPE_ETHERNET_400G_LR4\"]"]
    pub fn link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nURL of the InterconnectLocation object that represents where this connection is to be provisioned.\nSpecifies the location inside Google's Networks."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `macsec_enabled` after provisioning.\nEnable or disable MACsec on this Interconnect connection.\nMACsec enablement fails if the MACsec object is not specified."]
    pub fn macsec_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.macsec_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `noc_contact_email` after provisioning.\nEmail address to contact the customer NOC for operations and maintenance notifications\nregarding this Interconnect. If specified, this will be used for notifications in addition to\nall other forms described, such as Cloud Monitoring logs alerting and Cloud Notifications.\nThis field is required for users who sign up for Cloud Interconnect using workforce identity\nfederation."]
    pub fn noc_contact_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.noc_contact_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operational_status` after provisioning.\nThe current status of this Interconnect's functionality, which can take one of the following:\n  - OS_ACTIVE: A valid Interconnect, which is turned up and is ready to use. Attachments may\n  be provisioned on this Interconnect.\n  - OS_UNPROVISIONED: An Interconnect that has not completed turnup. No attachments may be\n  provisioned on this Interconnect.\n  - OS_UNDER_MAINTENANCE: An Interconnect that is undergoing internal maintenance. No\n  attachments may be provisioned or updated on this Interconnect."]
    pub fn operational_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operational_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ip_address` after provisioning.\nIP address configured on the customer side of the Interconnect link.\nThe customer should configure this IP address during turnup when prompted by Google NOC.\nThis can be used only for ping tests."]
    pub fn peer_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_link_count` after provisioning.\nNumber of links actually provisioned in this interconnect."]
    pub fn provisioned_link_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_link_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remote_location` after provisioning.\nIndicates that this is a Cross-Cloud Interconnect. This field specifies the location outside\nof Google's network that the interconnect is connected to."]
    pub fn remote_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remote_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requested_features` after provisioning.\nList of features to request for this Interconnect connection. This field is only applicable during Interconnect creation and cannot be modified later.\nPossible values include:\n- 'IF_MACSEC': Provisions the connection on hardware ports that support MACsec (Media Access Control Security). If not specified, the system may allocate non-MACsec capable ports if available.\n- 'IF_L2_FORWARDING': Provisions the connection for Layer 2 (L2) traffic forwarding. If not specified, the connection defaults to Layer 3 (L3) traffic forwarding.\n- 'IF_CROSS_SITE_NETWORK': Provisions the connection exclusively for Cross-Site Networking.\nNote: 'MACSEC' is a legacy value for compatibility reasons and has the same effect as 'IF_MACSEC'. 'IF_MACSEC' is preferred. Possible values: [\"MACSEC\", \"CROSS_SITE_NETWORK\", \"IF_MACSEC\", \"IF_L2_FORWARDING\"]"]
    pub fn requested_features(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requested_features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requested_link_count` after provisioning.\nTarget number of physical links in the link bundle, as requested by the customer."]
    pub fn requested_link_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requested_link_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.satisfies_pzs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of Interconnect functionality, which can take one of the following values:\n  - ACTIVE: The Interconnect is valid, turned up and ready to use.\n  Attachments may be provisioned on this Interconnect.\n  - UNPROVISIONED: The Interconnect has not completed turnup. No attachments may b\n   provisioned on this Interconnect.\n  - UNDER_MAINTENANCE: The Interconnect is undergoing internal maintenance. No attachments may\n   be provisioned or updated on this Interconnect."]
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
    #[doc = "Get a reference to the value of field `wire_groups` after provisioning.\nA list of the URLs of all CrossSiteNetwork WireGroups configured to use this Interconnect. The Interconnect cannot be deleted if this list is non-empty."]
    pub fn wire_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.wire_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `macsec` after provisioning.\n"]
    pub fn macsec(&self) -> ListRef<ComputeInterconnectMacsecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.macsec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeInterconnectParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectTimeoutsElRef {
        ComputeInterconnectTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeInterconnect {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeInterconnect {}
impl ToListMappable for ComputeInterconnect {
    type O = ListRef<ComputeInterconnectRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeInterconnect_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_interconnect".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeInterconnect {
    pub tf_id: String,
    #[doc = "Type of interconnect. Note that a value IT_PRIVATE has been deprecated in favor of DEDICATED.\nCan take one of the following values:\n  - PARTNER: A partner-managed interconnection shared between customers though a partner.\n  - DEDICATED: A dedicated physical interconnection with the customer. Possible values: [\"DEDICATED\", \"PARTNER\", \"IT_PRIVATE\"]"]
    pub interconnect_type: PrimField<String>,
    #[doc = "Type of link requested. Note that this field indicates the speed of each of the links in the\nbundle, not the speed of the entire bundle. Can take one of the following values:\n  - LINK_TYPE_ETHERNET_10G_LR: A 10G Ethernet with LR optics.\n  - LINK_TYPE_ETHERNET_100G_LR: A 100G Ethernet with LR optics.\n  - LINK_TYPE_ETHERNET_400G_LR4: A 400G Ethernet with LR4 optics Possible values: [\"LINK_TYPE_ETHERNET_10G_LR\", \"LINK_TYPE_ETHERNET_100G_LR\", \"LINK_TYPE_ETHERNET_400G_LR4\"]"]
    pub link_type: PrimField<String>,
    #[doc = "URL of the InterconnectLocation object that represents where this connection is to be provisioned.\nSpecifies the location inside Google's Networks."]
    pub location: PrimField<String>,
    #[doc = "Name of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "Target number of physical links in the link bundle, as requested by the customer."]
    pub requested_link_count: PrimField<f64>,
}
impl BuildComputeInterconnect {
    pub fn build(self, stack: &mut Stack) -> ComputeInterconnect {
        let out = ComputeInterconnect(Rc::new(ComputeInterconnect_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeInterconnectData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_enabled: core::default::Default::default(),
                customer_name: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                interconnect_type: self.interconnect_type,
                labels: core::default::Default::default(),
                link_type: self.link_type,
                location: self.location,
                macsec_enabled: core::default::Default::default(),
                name: self.name,
                noc_contact_email: core::default::Default::default(),
                project: core::default::Default::default(),
                remote_location: core::default::Default::default(),
                requested_features: core::default::Default::default(),
                requested_link_count: self.requested_link_count,
                macsec: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeInterconnectRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeInterconnectRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_enabled` after provisioning.\nAdministrative status of the interconnect. When this is set to true, the Interconnect is\nfunctional and can carry traffic. When set to false, no packets can be carried over the\ninterconnect and no BGP routes are exchanged over it. By default, the status is set to true."]
    pub fn admin_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `available_features` after provisioning.\n[Output Only] List of features that are available on this Interconnect connection based on the provisioned hardware and configuration.\nPossible values include:\n- 'IF_MACSEC': Indicates the Interconnect connection is provisioned on MACsec capable hardware ports. If this feature is not present, the connection does not support MACsec, and any attempt to enable it will fail.\n- 'IF_L2_FORWARDING': Indicates the Interconnect connection can be used for Layer 2 (L2) traffic forwarding. If not present, the connection cannot be used with L2 forwarding attachments.\n- 'IF_CROSS_SITE_NETWORK': Indicates the Interconnect connection is provisioned for Cross-Site Networking.\nNote: 'MACSEC' is a legacy value and has the same meaning as 'IF_MACSEC'."]
    pub fn available_features(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.available_features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `circuit_infos` after provisioning.\nA list of CircuitInfo objects, that describe the individual circuits in this LAG."]
    pub fn circuit_infos(&self) -> ListRef<ComputeInterconnectCircuitInfosElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.circuit_infos", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `customer_name` after provisioning.\nCustomer name, to put in the Letter of Authorization as the party authorized to request a\ncrossconnect. This field is required for Dedicated and Partner Interconnect, should not be specified\nfor cross-cloud interconnect."]
    pub fn customer_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you create the resource."]
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
    #[doc = "Get a reference to the value of field `expected_outages` after provisioning.\nA list of outages expected for this Interconnect."]
    pub fn expected_outages(&self) -> ListRef<ComputeInterconnectExpectedOutagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expected_outages", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `google_ip_address` after provisioning.\nIP address configured on the Google side of the Interconnect link.\nThis can be used only for ping tests."]
    pub fn google_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.google_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `google_reference_id` after provisioning.\nGoogle reference ID to be used when raising support tickets with Google or otherwise to debug\nbackend connectivity issues."]
    pub fn google_reference_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.google_reference_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `interconnect_attachments` after provisioning.\nA list of the URLs of all InterconnectAttachments configured to use this Interconnect."]
    pub fn interconnect_attachments(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.interconnect_attachments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `interconnect_groups` after provisioning.\nURLs of InterconnectGroups that include this Interconnect.\nOrder is arbitrary and items are unique."]
    pub fn interconnect_groups(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.interconnect_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `interconnect_type` after provisioning.\nType of interconnect. Note that a value IT_PRIVATE has been deprecated in favor of DEDICATED.\nCan take one of the following values:\n  - PARTNER: A partner-managed interconnection shared between customers though a partner.\n  - DEDICATED: A dedicated physical interconnection with the customer. Possible values: [\"DEDICATED\", \"PARTNER\", \"IT_PRIVATE\"]"]
    pub fn interconnect_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interconnect_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\nA fingerprint for the labels being applied to this Interconnect, which is essentially a hash\nof the labels set used for optimistic locking. The fingerprint is initially generated by\nCompute Engine and changes after every request to modify or update labels.\nYou must always provide an up-to-date fingerprint hash in order to update or change labels,\notherwise the request will fail with error 412 conditionNotMet."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels for this resource. These can only be added or modified by the setLabels\nmethod. Each label key/value pair must comply with RFC1035. Label values may be empty.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `link_type` after provisioning.\nType of link requested. Note that this field indicates the speed of each of the links in the\nbundle, not the speed of the entire bundle. Can take one of the following values:\n  - LINK_TYPE_ETHERNET_10G_LR: A 10G Ethernet with LR optics.\n  - LINK_TYPE_ETHERNET_100G_LR: A 100G Ethernet with LR optics.\n  - LINK_TYPE_ETHERNET_400G_LR4: A 400G Ethernet with LR4 optics Possible values: [\"LINK_TYPE_ETHERNET_10G_LR\", \"LINK_TYPE_ETHERNET_100G_LR\", \"LINK_TYPE_ETHERNET_400G_LR4\"]"]
    pub fn link_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.link_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nURL of the InterconnectLocation object that represents where this connection is to be provisioned.\nSpecifies the location inside Google's Networks."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `macsec_enabled` after provisioning.\nEnable or disable MACsec on this Interconnect connection.\nMACsec enablement fails if the MACsec object is not specified."]
    pub fn macsec_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.macsec_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created. The name must be\n1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters\nlong and match the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `noc_contact_email` after provisioning.\nEmail address to contact the customer NOC for operations and maintenance notifications\nregarding this Interconnect. If specified, this will be used for notifications in addition to\nall other forms described, such as Cloud Monitoring logs alerting and Cloud Notifications.\nThis field is required for users who sign up for Cloud Interconnect using workforce identity\nfederation."]
    pub fn noc_contact_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.noc_contact_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operational_status` after provisioning.\nThe current status of this Interconnect's functionality, which can take one of the following:\n  - OS_ACTIVE: A valid Interconnect, which is turned up and is ready to use. Attachments may\n  be provisioned on this Interconnect.\n  - OS_UNPROVISIONED: An Interconnect that has not completed turnup. No attachments may be\n  provisioned on this Interconnect.\n  - OS_UNDER_MAINTENANCE: An Interconnect that is undergoing internal maintenance. No\n  attachments may be provisioned or updated on this Interconnect."]
    pub fn operational_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operational_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `peer_ip_address` after provisioning.\nIP address configured on the customer side of the Interconnect link.\nThe customer should configure this IP address during turnup when prompted by Google NOC.\nThis can be used only for ping tests."]
    pub fn peer_ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peer_ip_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `provisioned_link_count` after provisioning.\nNumber of links actually provisioned in this interconnect."]
    pub fn provisioned_link_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_link_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remote_location` after provisioning.\nIndicates that this is a Cross-Cloud Interconnect. This field specifies the location outside\nof Google's network that the interconnect is connected to."]
    pub fn remote_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remote_location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requested_features` after provisioning.\nList of features to request for this Interconnect connection. This field is only applicable during Interconnect creation and cannot be modified later.\nPossible values include:\n- 'IF_MACSEC': Provisions the connection on hardware ports that support MACsec (Media Access Control Security). If not specified, the system may allocate non-MACsec capable ports if available.\n- 'IF_L2_FORWARDING': Provisions the connection for Layer 2 (L2) traffic forwarding. If not specified, the connection defaults to Layer 3 (L3) traffic forwarding.\n- 'IF_CROSS_SITE_NETWORK': Provisions the connection exclusively for Cross-Site Networking.\nNote: 'MACSEC' is a legacy value for compatibility reasons and has the same effect as 'IF_MACSEC'. 'IF_MACSEC' is preferred. Possible values: [\"MACSEC\", \"CROSS_SITE_NETWORK\", \"IF_MACSEC\", \"IF_L2_FORWARDING\"]"]
    pub fn requested_features(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requested_features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requested_link_count` after provisioning.\nTarget number of physical links in the link bundle, as requested by the customer."]
    pub fn requested_link_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.requested_link_count", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `satisfies_pzs` after provisioning.\nReserved for future use."]
    pub fn satisfies_pzs(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.satisfies_pzs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of Interconnect functionality, which can take one of the following values:\n  - ACTIVE: The Interconnect is valid, turned up and ready to use.\n  Attachments may be provisioned on this Interconnect.\n  - UNPROVISIONED: The Interconnect has not completed turnup. No attachments may b\n   provisioned on this Interconnect.\n  - UNDER_MAINTENANCE: The Interconnect is undergoing internal maintenance. No attachments may\n   be provisioned or updated on this Interconnect."]
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
    #[doc = "Get a reference to the value of field `wire_groups` after provisioning.\nA list of the URLs of all CrossSiteNetwork WireGroups configured to use this Interconnect. The Interconnect cannot be deleted if this list is non-empty."]
    pub fn wire_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.wire_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `macsec` after provisioning.\n"]
    pub fn macsec(&self) -> ListRef<ComputeInterconnectMacsecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.macsec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeInterconnectParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInterconnectTimeoutsElRef {
        ComputeInterconnectTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectCircuitInfosEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_demarc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_circuit_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_demarc_id: Option<PrimField<String>>,
}
impl ComputeInterconnectCircuitInfosEl {
    #[doc = "Set the field `customer_demarc_id`.\n"]
    pub fn set_customer_demarc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.customer_demarc_id = Some(v.into());
        self
    }
    #[doc = "Set the field `google_circuit_id`.\n"]
    pub fn set_google_circuit_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.google_circuit_id = Some(v.into());
        self
    }
    #[doc = "Set the field `google_demarc_id`.\n"]
    pub fn set_google_demarc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.google_demarc_id = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectCircuitInfosEl {
    type O = BlockAssignable<ComputeInterconnectCircuitInfosEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectCircuitInfosEl {}
impl BuildComputeInterconnectCircuitInfosEl {
    pub fn build(self) -> ComputeInterconnectCircuitInfosEl {
        ComputeInterconnectCircuitInfosEl {
            customer_demarc_id: core::default::Default::default(),
            google_circuit_id: core::default::Default::default(),
            google_demarc_id: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectCircuitInfosElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectCircuitInfosElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectCircuitInfosElRef {
        ComputeInterconnectCircuitInfosElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectCircuitInfosElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `customer_demarc_id` after provisioning.\n"]
    pub fn customer_demarc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_demarc_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_circuit_id` after provisioning.\n"]
    pub fn google_circuit_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.google_circuit_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `google_demarc_id` after provisioning.\n"]
    pub fn google_demarc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.google_demarc_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectExpectedOutagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    affected_circuits: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issue_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl ComputeInterconnectExpectedOutagesEl {
    #[doc = "Set the field `affected_circuits`.\n"]
    pub fn set_affected_circuits(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.affected_circuits = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.end_time = Some(v.into());
        self
    }
    #[doc = "Set the field `issue_type`.\n"]
    pub fn set_issue_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issue_type = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectExpectedOutagesEl {
    type O = BlockAssignable<ComputeInterconnectExpectedOutagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectExpectedOutagesEl {}
impl BuildComputeInterconnectExpectedOutagesEl {
    pub fn build(self) -> ComputeInterconnectExpectedOutagesEl {
        ComputeInterconnectExpectedOutagesEl {
            affected_circuits: core::default::Default::default(),
            description: core::default::Default::default(),
            end_time: core::default::Default::default(),
            issue_type: core::default::Default::default(),
            name: core::default::Default::default(),
            source: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectExpectedOutagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectExpectedOutagesElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectExpectedOutagesElRef {
        ComputeInterconnectExpectedOutagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectExpectedOutagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `affected_circuits` after provisioning.\n"]
    pub fn affected_circuits(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.affected_circuits", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }
    #[doc = "Get a reference to the value of field `issue_type` after provisioning.\n"]
    pub fn issue_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issue_type", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectMacsecElPreSharedKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<PrimField<String>>,
}
impl ComputeInterconnectMacsecElPreSharedKeysEl {
    #[doc = "Set the field `fail_open`.\nIf set to true, the Interconnect connection is configured with a should-secure\nMACsec security policy, that allows the Google router to fallback to cleartext\ntraffic if the MKA session cannot be established. By default, the Interconnect\nconnection is configured with a must-secure security policy that drops all traffic\nif the MKA session cannot be established with your router."]
    pub fn set_fail_open(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_open = Some(v.into());
        self
    }
    #[doc = "Set the field `start_time`.\nA RFC3339 timestamp on or after which the key is valid. startTime can be in the\nfuture. If the keychain has a single key, startTime can be omitted. If the keychain\nhas multiple keys, startTime is mandatory for each key. The start times of keys must\nbe in increasing order. The start times of two consecutive keys must be at least 6\nhours apart."]
    pub fn set_start_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_time = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectMacsecElPreSharedKeysEl {
    type O = BlockAssignable<ComputeInterconnectMacsecElPreSharedKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectMacsecElPreSharedKeysEl {
    #[doc = "A name for this pre-shared key. The name must be 1-63 characters long, and\n comply with RFC1035. Specifically, the name must be 1-63 characters long and match\n the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character\n must be a lowercase letter, and all following characters must be a dash, lowercase\n letter, or digit, except the last character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeInterconnectMacsecElPreSharedKeysEl {
    pub fn build(self) -> ComputeInterconnectMacsecElPreSharedKeysEl {
        ComputeInterconnectMacsecElPreSharedKeysEl {
            fail_open: core::default::Default::default(),
            name: self.name,
            start_time: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectMacsecElPreSharedKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectMacsecElPreSharedKeysElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectMacsecElPreSharedKeysElRef {
        ComputeInterconnectMacsecElPreSharedKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectMacsecElPreSharedKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\nIf set to true, the Interconnect connection is configured with a should-secure\nMACsec security policy, that allows the Google router to fallback to cleartext\ntraffic if the MKA session cannot be established. By default, the Interconnect\nconnection is configured with a must-secure security policy that drops all traffic\nif the MKA session cannot be established with your router."]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA name for this pre-shared key. The name must be 1-63 characters long, and\n comply with RFC1035. Specifically, the name must be 1-63 characters long and match\n the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first character\n must be a lowercase letter, and all following characters must be a dash, lowercase\n letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nA RFC3339 timestamp on or after which the key is valid. startTime can be in the\nfuture. If the keychain has a single key, startTime can be omitted. If the keychain\nhas multiple keys, startTime is mandatory for each key. The start times of keys must\nbe in increasing order. The start times of two consecutive keys must be at least 6\nhours apart."]
    pub fn start_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeInterconnectMacsecElDynamic {
    pre_shared_keys: Option<DynamicBlock<ComputeInterconnectMacsecElPreSharedKeysEl>>,
}
#[derive(Serialize)]
pub struct ComputeInterconnectMacsecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_open: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_shared_keys: Option<Vec<ComputeInterconnectMacsecElPreSharedKeysEl>>,
    dynamic: ComputeInterconnectMacsecElDynamic,
}
impl ComputeInterconnectMacsecEl {
    #[doc = "Set the field `fail_open`.\nIf set to true, the Interconnect connection is configured with a should-secure\nMACsec security policy, that allows the Google router to fallback to cleartext\ntraffic if the MKA session cannot be established. By default, the Interconnect\nconnection is configured with a must-secure security policy that drops all traffic\nif the MKA session cannot be established with your router."]
    pub fn set_fail_open(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.fail_open = Some(v.into());
        self
    }
    #[doc = "Set the field `pre_shared_keys`.\n"]
    pub fn set_pre_shared_keys(
        mut self,
        v: impl Into<BlockAssignable<ComputeInterconnectMacsecElPreSharedKeysEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pre_shared_keys = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_shared_keys = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeInterconnectMacsecEl {
    type O = BlockAssignable<ComputeInterconnectMacsecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectMacsecEl {}
impl BuildComputeInterconnectMacsecEl {
    pub fn build(self) -> ComputeInterconnectMacsecEl {
        ComputeInterconnectMacsecEl {
            fail_open: core::default::Default::default(),
            pre_shared_keys: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeInterconnectMacsecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectMacsecElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectMacsecElRef {
        ComputeInterconnectMacsecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectMacsecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\nIf set to true, the Interconnect connection is configured with a should-secure\nMACsec security policy, that allows the Google router to fallback to cleartext\ntraffic if the MKA session cannot be established. By default, the Interconnect\nconnection is configured with a must-secure security policy that drops all traffic\nif the MKA session cannot be established with your router."]
    pub fn fail_open(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }
    #[doc = "Get a reference to the value of field `pre_shared_keys` after provisioning.\n"]
    pub fn pre_shared_keys(&self) -> ListRef<ComputeInterconnectMacsecElPreSharedKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pre_shared_keys", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeInterconnectParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the interconnect. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeInterconnectParamsEl {
    type O = BlockAssignable<ComputeInterconnectParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectParamsEl {}
impl BuildComputeInterconnectParamsEl {
    pub fn build(self) -> ComputeInterconnectParamsEl {
        ComputeInterconnectParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectParamsElRef {
        ComputeInterconnectParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the interconnect. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeInterconnectTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeInterconnectTimeoutsEl {
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
impl ToListMappable for ComputeInterconnectTimeoutsEl {
    type O = BlockAssignable<ComputeInterconnectTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeInterconnectTimeoutsEl {}
impl BuildComputeInterconnectTimeoutsEl {
    pub fn build(self) -> ComputeInterconnectTimeoutsEl {
        ComputeInterconnectTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeInterconnectTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeInterconnectTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeInterconnectTimeoutsElRef {
        ComputeInterconnectTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeInterconnectTimeoutsElRef {
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
struct ComputeInterconnectDynamic {
    macsec: Option<DynamicBlock<ComputeInterconnectMacsecEl>>,
    params: Option<DynamicBlock<ComputeInterconnectParamsEl>>,
}
