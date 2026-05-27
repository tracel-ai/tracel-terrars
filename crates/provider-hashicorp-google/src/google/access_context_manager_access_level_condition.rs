use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccessContextManagerAccessLevelConditionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_level: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_subnetworks: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_access_levels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_policy: Option<Vec<AccessContextManagerAccessLevelConditionDevicePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerAccessLevelConditionTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_network_sources: Option<Vec<AccessContextManagerAccessLevelConditionVpcNetworkSourcesEl>>,
    dynamic: AccessContextManagerAccessLevelConditionDynamic,
}
struct AccessContextManagerAccessLevelCondition_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerAccessLevelConditionData>,
}
#[derive(Clone)]
pub struct AccessContextManagerAccessLevelCondition(Rc<AccessContextManagerAccessLevelCondition_>);
impl AccessContextManagerAccessLevelCondition {
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
    #[doc = "Set the field `ip_subnetworks`.\nA list of CIDR block IP subnetwork specification. May be IPv4\nor IPv6.\nNote that for a CIDR IP address block, the specified IP address\nportion must be properly truncated (i.e. all the host bits must\nbe zero) or the input is considered malformed. For example,\n\"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly,\nfor IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\"\nis not. The originating IP of a request must be in one of the\nlisted subnets in order for this Condition to be true.\nIf empty, all IP addresses are allowed."]
    pub fn set_ip_subnetworks(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ip_subnetworks = Some(v.into());
        self
    }
    #[doc = "Set the field `members`.\nAn allowed list of members (users, service accounts).\nUsing groups is not supported yet.\n\nThe signed-in user originating the request must be a part of one\nof the provided members. If not specified, a request may come\nfrom any user (logged in/not logged in, not present in any\ngroups, etc.).\nFormats: 'user:{emailid}', 'serviceAccount:{emailid}'"]
    pub fn set_members(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().members = Some(v.into());
        self
    }
    #[doc = "Set the field `negate`.\nWhether to negate the Condition. If true, the Condition becomes\na NAND over its non-empty fields, each field must be false for\nthe Condition overall to be satisfied. Defaults to false."]
    pub fn set_negate(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().negate = Some(v.into());
        self
    }
    #[doc = "Set the field `regions`.\nThe request must originate from one of the provided\ncountries/regions.\nFormat: A valid ISO 3166-1 alpha-2 code."]
    pub fn set_regions(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().regions = Some(v.into());
        self
    }
    #[doc = "Set the field `required_access_levels`.\nA list of other access levels defined in the same Policy,\nreferenced by resource name. Referencing an AccessLevel which\ndoes not exist is an error. All access levels listed must be\ngranted for the Condition to be true.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn set_required_access_levels(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().required_access_levels = Some(v.into());
        self
    }
    #[doc = "Set the field `device_policy`.\n"]
    pub fn set_device_policy(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerAccessLevelConditionDevicePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().device_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.device_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<AccessContextManagerAccessLevelConditionTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_network_sources`.\n"]
    pub fn set_vpc_network_sources(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerAccessLevelConditionVpcNetworkSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_network_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_network_sources = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `access_level` after provisioning.\nThe name of the Access Level to add this condition to."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_policy_id` after provisioning.\nThe name of the Access Policy this resource belongs to."]
    pub fn access_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_policy_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `ip_subnetworks` after provisioning.\nA list of CIDR block IP subnetwork specification. May be IPv4\nor IPv6.\nNote that for a CIDR IP address block, the specified IP address\nportion must be properly truncated (i.e. all the host bits must\nbe zero) or the input is considered malformed. For example,\n\"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly,\nfor IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\"\nis not. The originating IP of a request must be in one of the\nlisted subnets in order for this Condition to be true.\nIf empty, all IP addresses are allowed."]
    pub fn ip_subnetworks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_subnetworks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `members` after provisioning.\nAn allowed list of members (users, service accounts).\nUsing groups is not supported yet.\n\nThe signed-in user originating the request must be a part of one\nof the provided members. If not specified, a request may come\nfrom any user (logged in/not logged in, not present in any\ngroups, etc.).\nFormats: 'user:{emailid}', 'serviceAccount:{emailid}'"]
    pub fn members(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.members", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `negate` after provisioning.\nWhether to negate the Condition. If true, the Condition becomes\na NAND over its non-empty fields, each field must be false for\nthe Condition overall to be satisfied. Defaults to false."]
    pub fn negate(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.negate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\nThe request must originate from one of the provided\ncountries/regions.\nFormat: A valid ISO 3166-1 alpha-2 code."]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `required_access_levels` after provisioning.\nA list of other access levels defined in the same Policy,\nreferenced by resource name. Referencing an AccessLevel which\ndoes not exist is an error. All access levels listed must be\ngranted for the Condition to be true.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn required_access_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.required_access_levels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `device_policy` after provisioning.\n"]
    pub fn device_policy(
        &self,
    ) -> ListRef<AccessContextManagerAccessLevelConditionDevicePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.device_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAccessLevelConditionTimeoutsElRef {
        AccessContextManagerAccessLevelConditionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_network_sources` after provisioning.\n"]
    pub fn vpc_network_sources(
        &self,
    ) -> ListRef<AccessContextManagerAccessLevelConditionVpcNetworkSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_network_sources", self.extract_ref()),
        )
    }
}
impl Referable for AccessContextManagerAccessLevelCondition {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccessContextManagerAccessLevelCondition {}
impl ToListMappable for AccessContextManagerAccessLevelCondition {
    type O = ListRef<AccessContextManagerAccessLevelConditionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccessContextManagerAccessLevelCondition_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_access_level_condition".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccessContextManagerAccessLevelCondition {
    pub tf_id: String,
    #[doc = "The name of the Access Level to add this condition to."]
    pub access_level: PrimField<String>,
}
impl BuildAccessContextManagerAccessLevelCondition {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerAccessLevelCondition {
        let out = AccessContextManagerAccessLevelCondition(Rc::new(
            AccessContextManagerAccessLevelCondition_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(AccessContextManagerAccessLevelConditionData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    access_level: self.access_level,
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    ip_subnetworks: core::default::Default::default(),
                    members: core::default::Default::default(),
                    negate: core::default::Default::default(),
                    regions: core::default::Default::default(),
                    required_access_levels: core::default::Default::default(),
                    device_policy: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    vpc_network_sources: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccessContextManagerAccessLevelConditionRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelConditionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccessContextManagerAccessLevelConditionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_level` after provisioning.\nThe name of the Access Level to add this condition to."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_policy_id` after provisioning.\nThe name of the Access Policy this resource belongs to."]
    pub fn access_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_policy_id", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `ip_subnetworks` after provisioning.\nA list of CIDR block IP subnetwork specification. May be IPv4\nor IPv6.\nNote that for a CIDR IP address block, the specified IP address\nportion must be properly truncated (i.e. all the host bits must\nbe zero) or the input is considered malformed. For example,\n\"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly,\nfor IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\"\nis not. The originating IP of a request must be in one of the\nlisted subnets in order for this Condition to be true.\nIf empty, all IP addresses are allowed."]
    pub fn ip_subnetworks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_subnetworks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `members` after provisioning.\nAn allowed list of members (users, service accounts).\nUsing groups is not supported yet.\n\nThe signed-in user originating the request must be a part of one\nof the provided members. If not specified, a request may come\nfrom any user (logged in/not logged in, not present in any\ngroups, etc.).\nFormats: 'user:{emailid}', 'serviceAccount:{emailid}'"]
    pub fn members(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.members", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `negate` after provisioning.\nWhether to negate the Condition. If true, the Condition becomes\na NAND over its non-empty fields, each field must be false for\nthe Condition overall to be satisfied. Defaults to false."]
    pub fn negate(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.negate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\nThe request must originate from one of the provided\ncountries/regions.\nFormat: A valid ISO 3166-1 alpha-2 code."]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `required_access_levels` after provisioning.\nA list of other access levels defined in the same Policy,\nreferenced by resource name. Referencing an AccessLevel which\ndoes not exist is an error. All access levels listed must be\ngranted for the Condition to be true.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn required_access_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.required_access_levels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `device_policy` after provisioning.\n"]
    pub fn device_policy(
        &self,
    ) -> ListRef<AccessContextManagerAccessLevelConditionDevicePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.device_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAccessLevelConditionTimeoutsElRef {
        AccessContextManagerAccessLevelConditionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_network_sources` after provisioning.\n"]
    pub fn vpc_network_sources(
        &self,
    ) -> ListRef<AccessContextManagerAccessLevelConditionVpcNetworkSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_network_sources", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_version: Option<PrimField<String>>,
    os_type: PrimField<String>,
}
impl AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl {
    #[doc = "Set the field `minimum_version`.\nThe minimum allowed OS version. If not set, any version\nof this OS satisfies the constraint.\nFormat: \"major.minor.patch\" such as \"10.5.301\", \"9.2.1\"."]
    pub fn set_minimum_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_version = Some(v.into());
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl {
    #[doc = "The operating system type of the device. Possible values: [\"OS_UNSPECIFIED\", \"DESKTOP_MAC\", \"DESKTOP_WINDOWS\", \"DESKTOP_LINUX\", \"DESKTOP_CHROME_OS\", \"ANDROID\", \"IOS\"]"]
    pub os_type: PrimField<String>,
}
impl BuildAccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl {
    pub fn build(self) -> AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl {
        AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl {
            minimum_version: core::default::Default::default(),
            os_type: self.os_type,
        }
    }
}
pub struct AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsElRef {
        AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `minimum_version` after provisioning.\nThe minimum allowed OS version. If not set, any version\nof this OS satisfies the constraint.\nFormat: \"major.minor.patch\" such as \"10.5.301\", \"9.2.1\"."]
    pub fn minimum_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `os_type` after provisioning.\nThe operating system type of the device. Possible values: [\"OS_UNSPECIFIED\", \"DESKTOP_MAC\", \"DESKTOP_WINDOWS\", \"DESKTOP_LINUX\", \"DESKTOP_CHROME_OS\", \"ANDROID\", \"IOS\"]"]
    pub fn os_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelConditionDevicePolicyElDynamic {
    os_constraints:
        Option<DynamicBlock<AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl>>,
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelConditionDevicePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_device_management_levels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_encryption_statuses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_admin_approval: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_corp_owned: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_screen_lock: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_constraints:
        Option<Vec<AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl>>,
    dynamic: AccessContextManagerAccessLevelConditionDevicePolicyElDynamic,
}
impl AccessContextManagerAccessLevelConditionDevicePolicyEl {
    #[doc = "Set the field `allowed_device_management_levels`.\nA list of allowed device management levels.\nAn empty list allows all management levels. Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"NONE\", \"BASIC\", \"COMPLETE\"]"]
    pub fn set_allowed_device_management_levels(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_device_management_levels = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_encryption_statuses`.\nA list of allowed encryptions statuses.\nAn empty list allows all statuses. Possible values: [\"ENCRYPTION_UNSPECIFIED\", \"ENCRYPTION_UNSUPPORTED\", \"UNENCRYPTED\", \"ENCRYPTED\"]"]
    pub fn set_allowed_encryption_statuses(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_encryption_statuses = Some(v.into());
        self
    }
    #[doc = "Set the field `require_admin_approval`.\nWhether the device needs to be approved by the customer admin."]
    pub fn set_require_admin_approval(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_admin_approval = Some(v.into());
        self
    }
    #[doc = "Set the field `require_corp_owned`.\nWhether the device needs to be corp owned."]
    pub fn set_require_corp_owned(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_corp_owned = Some(v.into());
        self
    }
    #[doc = "Set the field `require_screen_lock`.\nWhether or not screenlock is required for the DevicePolicy\nto be true. Defaults to false."]
    pub fn set_require_screen_lock(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_screen_lock = Some(v.into());
        self
    }
    #[doc = "Set the field `os_constraints`.\n"]
    pub fn set_os_constraints(
        mut self,
        v: impl Into<
            BlockAssignable<AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.os_constraints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.os_constraints = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelConditionDevicePolicyEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelConditionDevicePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelConditionDevicePolicyEl {}
impl BuildAccessContextManagerAccessLevelConditionDevicePolicyEl {
    pub fn build(self) -> AccessContextManagerAccessLevelConditionDevicePolicyEl {
        AccessContextManagerAccessLevelConditionDevicePolicyEl {
            allowed_device_management_levels: core::default::Default::default(),
            allowed_encryption_statuses: core::default::Default::default(),
            require_admin_approval: core::default::Default::default(),
            require_corp_owned: core::default::Default::default(),
            require_screen_lock: core::default::Default::default(),
            os_constraints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelConditionDevicePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelConditionDevicePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelConditionDevicePolicyElRef {
        AccessContextManagerAccessLevelConditionDevicePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelConditionDevicePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_device_management_levels` after provisioning.\nA list of allowed device management levels.\nAn empty list allows all management levels. Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"NONE\", \"BASIC\", \"COMPLETE\"]"]
    pub fn allowed_device_management_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_device_management_levels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_encryption_statuses` after provisioning.\nA list of allowed encryptions statuses.\nAn empty list allows all statuses. Possible values: [\"ENCRYPTION_UNSPECIFIED\", \"ENCRYPTION_UNSUPPORTED\", \"UNENCRYPTED\", \"ENCRYPTED\"]"]
    pub fn allowed_encryption_statuses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_encryption_statuses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `require_admin_approval` after provisioning.\nWhether the device needs to be approved by the customer admin."]
    pub fn require_admin_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_admin_approval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `require_corp_owned` after provisioning.\nWhether the device needs to be corp owned."]
    pub fn require_corp_owned(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_corp_owned", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `require_screen_lock` after provisioning.\nWhether or not screenlock is required for the DevicePolicy\nto be true. Defaults to false."]
    pub fn require_screen_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_screen_lock", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `os_constraints` after provisioning.\n"]
    pub fn os_constraints(
        &self,
    ) -> ListRef<AccessContextManagerAccessLevelConditionDevicePolicyElOsConstraintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.os_constraints", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelConditionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl AccessContextManagerAccessLevelConditionTimeoutsEl {
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
}
impl ToListMappable for AccessContextManagerAccessLevelConditionTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelConditionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelConditionTimeoutsEl {}
impl BuildAccessContextManagerAccessLevelConditionTimeoutsEl {
    pub fn build(self) -> AccessContextManagerAccessLevelConditionTimeoutsEl {
        AccessContextManagerAccessLevelConditionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelConditionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelConditionTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelConditionTimeoutsElRef {
        AccessContextManagerAccessLevelConditionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelConditionTimeoutsElRef {
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
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl {
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_ip_subnetworks: Option<ListField<PrimField<String>>>,
}
impl AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl {
    #[doc = "Set the field `vpc_ip_subnetworks`.\nCIDR block IP subnetwork specification. Must be IPv4."]
    pub fn set_vpc_ip_subnetworks(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.vpc_ip_subnetworks = Some(v.into());
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl {
    type O =
        BlockAssignable<AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl {
    #[doc = "Required. Network name to be allowed by this Access Level. Networks of foreign organizations requires 'compute.network.get' permission to be granted to caller."]
    pub network: PrimField<String>,
}
impl BuildAccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl {
    pub fn build(
        self,
    ) -> AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl {
        AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl {
            network: self.network,
            vpc_ip_subnetworks: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkElRef {
        AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nRequired. Network name to be allowed by this Access Level. Networks of foreign organizations requires 'compute.network.get' permission to be granted to caller."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_ip_subnetworks` after provisioning.\nCIDR block IP subnetwork specification. Must be IPv4."]
    pub fn vpc_ip_subnetworks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_ip_subnetworks", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelConditionVpcNetworkSourcesElDynamic {
    vpc_subnetwork: Option<
        DynamicBlock<AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl>,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelConditionVpcNetworkSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_subnetwork:
        Option<Vec<AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl>>,
    dynamic: AccessContextManagerAccessLevelConditionVpcNetworkSourcesElDynamic,
}
impl AccessContextManagerAccessLevelConditionVpcNetworkSourcesEl {
    #[doc = "Set the field `vpc_subnetwork`.\n"]
    pub fn set_vpc_subnetwork(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_subnetwork = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_subnetwork = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelConditionVpcNetworkSourcesEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelConditionVpcNetworkSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelConditionVpcNetworkSourcesEl {}
impl BuildAccessContextManagerAccessLevelConditionVpcNetworkSourcesEl {
    pub fn build(self) -> AccessContextManagerAccessLevelConditionVpcNetworkSourcesEl {
        AccessContextManagerAccessLevelConditionVpcNetworkSourcesEl {
            vpc_subnetwork: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelConditionVpcNetworkSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelConditionVpcNetworkSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelConditionVpcNetworkSourcesElRef {
        AccessContextManagerAccessLevelConditionVpcNetworkSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelConditionVpcNetworkSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `vpc_subnetwork` after provisioning.\n"]
    pub fn vpc_subnetwork(
        &self,
    ) -> ListRef<AccessContextManagerAccessLevelConditionVpcNetworkSourcesElVpcSubnetworkElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_subnetwork", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelConditionDynamic {
    device_policy: Option<DynamicBlock<AccessContextManagerAccessLevelConditionDevicePolicyEl>>,
    vpc_network_sources:
        Option<DynamicBlock<AccessContextManagerAccessLevelConditionVpcNetworkSourcesEl>>,
}
