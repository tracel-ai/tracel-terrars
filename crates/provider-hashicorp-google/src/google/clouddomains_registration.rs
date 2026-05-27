use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ClouddomainsRegistrationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_notices: Option<ListField<PrimField<String>>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_notices: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_settings: Option<Vec<ClouddomainsRegistrationContactSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_settings: Option<Vec<ClouddomainsRegistrationDnsSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    management_settings: Option<Vec<ClouddomainsRegistrationManagementSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ClouddomainsRegistrationTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    yearly_price: Option<Vec<ClouddomainsRegistrationYearlyPriceEl>>,
    dynamic: ClouddomainsRegistrationDynamic,
}
struct ClouddomainsRegistration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ClouddomainsRegistrationData>,
}
#[derive(Clone)]
pub struct ClouddomainsRegistration(Rc<ClouddomainsRegistration_>);
impl ClouddomainsRegistration {
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
    #[doc = "Set the field `contact_notices`.\nThe list of contact notices that the caller acknowledges. Possible value is PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT"]
    pub fn set_contact_notices(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().contact_notices = Some(v.into());
        self
    }
    #[doc = "Set the field `domain_notices`.\nThe list of domain notices that you acknowledge. Possible value is HSTS_PRELOADED"]
    pub fn set_domain_notices(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().domain_notices = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of labels associated with the Registration.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `contact_settings`.\n"]
    pub fn set_contact_settings(
        self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationContactSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().contact_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.contact_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dns_settings`.\n"]
    pub fn set_dns_settings(
        self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationDnsSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dns_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dns_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `management_settings`.\n"]
    pub fn set_management_settings(
        self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationManagementSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().management_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.management_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ClouddomainsRegistrationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `yearly_price`.\n"]
    pub fn set_yearly_price(
        self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationYearlyPriceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().yearly_price = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.yearly_price = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `contact_notices` after provisioning.\nThe list of contact notices that the caller acknowledges. Possible value is PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT"]
    pub fn contact_notices(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contact_notices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the automation was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\nRequired. The domain name. Unicode domain names must be expressed in Punycode format."]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_notices` after provisioning.\nThe list of domain notices that you acknowledge. Possible value is HSTS_PRELOADED"]
    pub fn domain_notices(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_notices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nOutput only. Time at which the automation was updated."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `issues` after provisioning.\nOutput only. The set of issues with the Registration that require attention."]
    pub fn issues(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.issues", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the Registration.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the Registration resource, in the format projects/*/locations/*/registrations/<domain_name>."]
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
    #[doc = "Get a reference to the value of field `register_failure_reason` after provisioning.\nOutput only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state."]
    pub fn register_failure_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.register_failure_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the Registration."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_privacy` after provisioning.\nOutput only. Set of options for the contactSettings.privacy field that this Registration supports."]
    pub fn supported_privacy(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `contact_settings` after provisioning.\n"]
    pub fn contact_settings(&self) -> ListRef<ClouddomainsRegistrationContactSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contact_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_settings` after provisioning.\n"]
    pub fn dns_settings(&self) -> ListRef<ClouddomainsRegistrationDnsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management_settings` after provisioning.\n"]
    pub fn management_settings(&self) -> ListRef<ClouddomainsRegistrationManagementSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddomainsRegistrationTimeoutsElRef {
        ClouddomainsRegistrationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `yearly_price` after provisioning.\n"]
    pub fn yearly_price(&self) -> ListRef<ClouddomainsRegistrationYearlyPriceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.yearly_price", self.extract_ref()),
        )
    }
}
impl Referable for ClouddomainsRegistration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ClouddomainsRegistration {}
impl ToListMappable for ClouddomainsRegistration {
    type O = ListRef<ClouddomainsRegistrationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ClouddomainsRegistration_ {
    fn extract_resource_type(&self) -> String {
        "google_clouddomains_registration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildClouddomainsRegistration {
    pub tf_id: String,
    #[doc = "Required. The domain name. Unicode domain names must be expressed in Punycode format."]
    pub domain_name: PrimField<String>,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
}
impl BuildClouddomainsRegistration {
    pub fn build(self, stack: &mut Stack) -> ClouddomainsRegistration {
        let out = ClouddomainsRegistration(Rc::new(ClouddomainsRegistration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ClouddomainsRegistrationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                contact_notices: core::default::Default::default(),
                domain_name: self.domain_name,
                domain_notices: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                contact_settings: core::default::Default::default(),
                dns_settings: core::default::Default::default(),
                management_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                yearly_price: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ClouddomainsRegistrationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ClouddomainsRegistrationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `contact_notices` after provisioning.\nThe list of contact notices that the caller acknowledges. Possible value is PUBLIC_CONTACT_DATA_ACKNOWLEDGEMENT"]
    pub fn contact_notices(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contact_notices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the automation was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\nRequired. The domain name. Unicode domain names must be expressed in Punycode format."]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_notices` after provisioning.\nThe list of domain notices that you acknowledge. Possible value is HSTS_PRELOADED"]
    pub fn domain_notices(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.domain_notices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\nOutput only. Time at which the automation was updated."]
    pub fn expire_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expire_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `issues` after provisioning.\nOutput only. The set of issues with the Registration that require attention."]
    pub fn issues(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.issues", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of labels associated with the Registration.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the Registration resource, in the format projects/*/locations/*/registrations/<domain_name>."]
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
    #[doc = "Get a reference to the value of field `register_failure_reason` after provisioning.\nOutput only. The reason the domain registration failed. Only set for domains in REGISTRATION_FAILED state."]
    pub fn register_failure_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.register_failure_reason", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the Registration."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_privacy` after provisioning.\nOutput only. Set of options for the contactSettings.privacy field that this Registration supports."]
    pub fn supported_privacy(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `contact_settings` after provisioning.\n"]
    pub fn contact_settings(&self) -> ListRef<ClouddomainsRegistrationContactSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contact_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_settings` after provisioning.\n"]
    pub fn dns_settings(&self) -> ListRef<ClouddomainsRegistrationDnsSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `management_settings` after provisioning.\n"]
    pub fn management_settings(&self) -> ListRef<ClouddomainsRegistrationManagementSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.management_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddomainsRegistrationTimeoutsElRef {
        ClouddomainsRegistrationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `yearly_price` after provisioning.\n"]
    pub fn yearly_price(&self) -> ListRef<ClouddomainsRegistrationYearlyPriceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.yearly_price", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_lines: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    administrative_area: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipients: Option<ListField<PrimField<String>>>,
    region_code: PrimField<String>,
}
impl ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl {
    #[doc = "Set the field `address_lines`.\nUnstructured address lines describing the lower levels of an address.\nBecause values in addressLines do not have type information and may sometimes contain multiple values in a single\nfield (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be\n\"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language\nis used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way,\nthe most specific line of an address can be selected based on the language."]
    pub fn set_address_lines(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.address_lines = Some(v.into());
        self
    }
    #[doc = "Set the field `administrative_area`.\nHighest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,\na province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community\n(e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland\nthis should be left unpopulated."]
    pub fn set_administrative_area(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.administrative_area = Some(v.into());
        self
    }
    #[doc = "Set the field `locality`.\nGenerally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world\nwhere localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines."]
    pub fn set_locality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locality = Some(v.into());
        self
    }
    #[doc = "Set the field `organization`.\nThe name of the organization at the address."]
    pub fn set_organization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization = Some(v.into());
        self
    }
    #[doc = "Set the field `postal_code`.\nPostal code of the address. Not all countries use or require postal codes to be present, but where they are used,\nthey may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
    pub fn set_postal_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.postal_code = Some(v.into());
        self
    }
    #[doc = "Set the field `recipients`.\nThe recipient at the address. This field may, under certain circumstances, contain multiline information. For example,\nit might contain \"care of\" information."]
    pub fn set_recipients(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.recipients = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl {
    type O =
        BlockAssignable<ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl {
    #[doc = "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to\nensure the value is correct. See https://cldr.unicode.org/ and\nhttps://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
    pub region_code: PrimField<String>,
}
impl BuildClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl {
    pub fn build(self) -> ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl {
        ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl {
            address_lines: core::default::Default::default(),
            administrative_area: core::default::Default::default(),
            locality: core::default::Default::default(),
            organization: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            recipients: core::default::Default::default(),
            region_code: self.region_code,
        }
    }
}
pub struct ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressElRef {
        ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_lines` after provisioning.\nUnstructured address lines describing the lower levels of an address.\nBecause values in addressLines do not have type information and may sometimes contain multiple values in a single\nfield (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be\n\"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language\nis used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way,\nthe most specific line of an address can be selected based on the language."]
    pub fn address_lines(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address_lines", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `administrative_area` after provisioning.\nHighest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,\na province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community\n(e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland\nthis should be left unpopulated."]
    pub fn administrative_area(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.administrative_area", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `locality` after provisioning.\nGenerally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world\nwhere localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines."]
    pub fn locality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe name of the organization at the address."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }
    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\nPostal code of the address. Not all countries use or require postal codes to be present, but where they are used,\nthey may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }
    #[doc = "Get a reference to the value of field `recipients` after provisioning.\nThe recipient at the address. This field may, under certain circumstances, contain multiline information. For example,\nit might contain \"care of\" information."]
    pub fn recipients(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.recipients", self.base))
    }
    #[doc = "Get a reference to the value of field `region_code` after provisioning.\nRequired. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to\nensure the value is correct. See https://cldr.unicode.org/ and\nhttps://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
    pub fn region_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_code", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddomainsRegistrationContactSettingsElAdminContactElDynamic {
    postal_address: Option<
        DynamicBlock<ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl>,
    >,
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationContactSettingsElAdminContactEl {
    email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax_number: Option<PrimField<String>>,
    phone_number: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_address:
        Option<Vec<ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl>>,
    dynamic: ClouddomainsRegistrationContactSettingsElAdminContactElDynamic,
}
impl ClouddomainsRegistrationContactSettingsElAdminContactEl {
    #[doc = "Set the field `fax_number`.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn set_fax_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fax_number = Some(v.into());
        self
    }
    #[doc = "Set the field `postal_address`.\n"]
    pub fn set_postal_address(
        mut self,
        v: impl Into<
            BlockAssignable<ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postal_address = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postal_address = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationContactSettingsElAdminContactEl {
    type O = BlockAssignable<ClouddomainsRegistrationContactSettingsElAdminContactEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationContactSettingsElAdminContactEl {
    #[doc = "Required. Email address of the contact."]
    pub email: PrimField<String>,
    #[doc = "Required. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub phone_number: PrimField<String>,
}
impl BuildClouddomainsRegistrationContactSettingsElAdminContactEl {
    pub fn build(self) -> ClouddomainsRegistrationContactSettingsElAdminContactEl {
        ClouddomainsRegistrationContactSettingsElAdminContactEl {
            email: self.email,
            fax_number: core::default::Default::default(),
            phone_number: self.phone_number,
            postal_address: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationContactSettingsElAdminContactElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationContactSettingsElAdminContactElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddomainsRegistrationContactSettingsElAdminContactElRef {
        ClouddomainsRegistrationContactSettingsElAdminContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationContactSettingsElAdminContactElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contact."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `fax_number` after provisioning.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn fax_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax_number", self.base))
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\nRequired. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
    #[doc = "Get a reference to the value of field `postal_address` after provisioning.\n"]
    pub fn postal_address(
        &self,
    ) -> ListRef<ClouddomainsRegistrationContactSettingsElAdminContactElPostalAddressElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postal_address", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_lines: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    administrative_area: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipients: Option<ListField<PrimField<String>>>,
    region_code: PrimField<String>,
}
impl ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl {
    #[doc = "Set the field `address_lines`.\nUnstructured address lines describing the lower levels of an address.\nBecause values in addressLines do not have type information and may sometimes contain multiple values in a single\nfield (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be\n\"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language\nis used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way,\nthe most specific line of an address can be selected based on the language."]
    pub fn set_address_lines(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.address_lines = Some(v.into());
        self
    }
    #[doc = "Set the field `administrative_area`.\nHighest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,\na province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community\n(e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland\nthis should be left unpopulated."]
    pub fn set_administrative_area(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.administrative_area = Some(v.into());
        self
    }
    #[doc = "Set the field `locality`.\nGenerally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world\nwhere localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines."]
    pub fn set_locality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locality = Some(v.into());
        self
    }
    #[doc = "Set the field `organization`.\nThe name of the organization at the address."]
    pub fn set_organization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization = Some(v.into());
        self
    }
    #[doc = "Set the field `postal_code`.\nPostal code of the address. Not all countries use or require postal codes to be present, but where they are used,\nthey may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
    pub fn set_postal_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.postal_code = Some(v.into());
        self
    }
    #[doc = "Set the field `recipients`.\nThe recipient at the address. This field may, under certain circumstances, contain multiline information. For example,\nit might contain \"care of\" information."]
    pub fn set_recipients(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.recipients = Some(v.into());
        self
    }
}
impl ToListMappable
    for ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl
{
    type O = BlockAssignable<
        ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl {
    #[doc = "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to\nensure the value is correct. See https://cldr.unicode.org/ and\nhttps://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
    pub region_code: PrimField<String>,
}
impl BuildClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl {
    pub fn build(
        self,
    ) -> ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl {
        ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl {
            address_lines: core::default::Default::default(),
            administrative_area: core::default::Default::default(),
            locality: core::default::Default::default(),
            organization: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            recipients: core::default::Default::default(),
            region_code: self.region_code,
        }
    }
}
pub struct ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressElRef {
        ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_lines` after provisioning.\nUnstructured address lines describing the lower levels of an address.\nBecause values in addressLines do not have type information and may sometimes contain multiple values in a single\nfield (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be\n\"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language\nis used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way,\nthe most specific line of an address can be selected based on the language."]
    pub fn address_lines(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address_lines", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `administrative_area` after provisioning.\nHighest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,\na province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community\n(e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland\nthis should be left unpopulated."]
    pub fn administrative_area(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.administrative_area", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `locality` after provisioning.\nGenerally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world\nwhere localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines."]
    pub fn locality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe name of the organization at the address."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }
    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\nPostal code of the address. Not all countries use or require postal codes to be present, but where they are used,\nthey may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }
    #[doc = "Get a reference to the value of field `recipients` after provisioning.\nThe recipient at the address. This field may, under certain circumstances, contain multiline information. For example,\nit might contain \"care of\" information."]
    pub fn recipients(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.recipients", self.base))
    }
    #[doc = "Get a reference to the value of field `region_code` after provisioning.\nRequired. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to\nensure the value is correct. See https://cldr.unicode.org/ and\nhttps://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
    pub fn region_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_code", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddomainsRegistrationContactSettingsElRegistrantContactElDynamic {
    postal_address: Option<
        DynamicBlock<ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl>,
    >,
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationContactSettingsElRegistrantContactEl {
    email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax_number: Option<PrimField<String>>,
    phone_number: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_address:
        Option<Vec<ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl>>,
    dynamic: ClouddomainsRegistrationContactSettingsElRegistrantContactElDynamic,
}
impl ClouddomainsRegistrationContactSettingsElRegistrantContactEl {
    #[doc = "Set the field `fax_number`.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn set_fax_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fax_number = Some(v.into());
        self
    }
    #[doc = "Set the field `postal_address`.\n"]
    pub fn set_postal_address(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postal_address = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postal_address = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationContactSettingsElRegistrantContactEl {
    type O = BlockAssignable<ClouddomainsRegistrationContactSettingsElRegistrantContactEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationContactSettingsElRegistrantContactEl {
    #[doc = "Required. Email address of the contact."]
    pub email: PrimField<String>,
    #[doc = "Required. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub phone_number: PrimField<String>,
}
impl BuildClouddomainsRegistrationContactSettingsElRegistrantContactEl {
    pub fn build(self) -> ClouddomainsRegistrationContactSettingsElRegistrantContactEl {
        ClouddomainsRegistrationContactSettingsElRegistrantContactEl {
            email: self.email,
            fax_number: core::default::Default::default(),
            phone_number: self.phone_number,
            postal_address: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationContactSettingsElRegistrantContactElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationContactSettingsElRegistrantContactElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddomainsRegistrationContactSettingsElRegistrantContactElRef {
        ClouddomainsRegistrationContactSettingsElRegistrantContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationContactSettingsElRegistrantContactElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contact."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `fax_number` after provisioning.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn fax_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax_number", self.base))
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\nRequired. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
    #[doc = "Get a reference to the value of field `postal_address` after provisioning.\n"]
    pub fn postal_address(
        &self,
    ) -> ListRef<ClouddomainsRegistrationContactSettingsElRegistrantContactElPostalAddressElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postal_address", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_lines: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    administrative_area: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locality: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recipients: Option<ListField<PrimField<String>>>,
    region_code: PrimField<String>,
}
impl ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl {
    #[doc = "Set the field `address_lines`.\nUnstructured address lines describing the lower levels of an address.\nBecause values in addressLines do not have type information and may sometimes contain multiple values in a single\nfield (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be\n\"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language\nis used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way,\nthe most specific line of an address can be selected based on the language."]
    pub fn set_address_lines(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.address_lines = Some(v.into());
        self
    }
    #[doc = "Set the field `administrative_area`.\nHighest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,\na province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community\n(e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland\nthis should be left unpopulated."]
    pub fn set_administrative_area(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.administrative_area = Some(v.into());
        self
    }
    #[doc = "Set the field `locality`.\nGenerally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world\nwhere localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines."]
    pub fn set_locality(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locality = Some(v.into());
        self
    }
    #[doc = "Set the field `organization`.\nThe name of the organization at the address."]
    pub fn set_organization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization = Some(v.into());
        self
    }
    #[doc = "Set the field `postal_code`.\nPostal code of the address. Not all countries use or require postal codes to be present, but where they are used,\nthey may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
    pub fn set_postal_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.postal_code = Some(v.into());
        self
    }
    #[doc = "Set the field `recipients`.\nThe recipient at the address. This field may, under certain circumstances, contain multiline information. For example,\nit might contain \"care of\" information."]
    pub fn set_recipients(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.recipients = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl {
    type O =
        BlockAssignable<ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl {
    #[doc = "Required. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to\nensure the value is correct. See https://cldr.unicode.org/ and\nhttps://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
    pub region_code: PrimField<String>,
}
impl BuildClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl {
    pub fn build(
        self,
    ) -> ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl {
        ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl {
            address_lines: core::default::Default::default(),
            administrative_area: core::default::Default::default(),
            locality: core::default::Default::default(),
            organization: core::default::Default::default(),
            postal_code: core::default::Default::default(),
            recipients: core::default::Default::default(),
            region_code: self.region_code,
        }
    }
}
pub struct ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressElRef {
        ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_lines` after provisioning.\nUnstructured address lines describing the lower levels of an address.\nBecause values in addressLines do not have type information and may sometimes contain multiple values in a single\nfield (e.g. \"Austin, TX\"), it is important that the line order is clear. The order of address lines should be\n\"envelope order\" for the country/region of the address. In places where this can vary (e.g. Japan), address_language\nis used to make it explicit (e.g. \"ja\" for large-to-small ordering and \"ja-Latn\" or \"en\" for small-to-large). This way,\nthe most specific line of an address can be selected based on the language."]
    pub fn address_lines(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.address_lines", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `administrative_area` after provisioning.\nHighest administrative subdivision which is used for postal addresses of a country or region. For example, this can be a state,\na province, an oblast, or a prefecture. Specifically, for Spain this is the province and not the autonomous community\n(e.g. \"Barcelona\" and not \"Catalonia\"). Many countries don't use an administrative area in postal addresses. E.g. in Switzerland\nthis should be left unpopulated."]
    pub fn administrative_area(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.administrative_area", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `locality` after provisioning.\nGenerally refers to the city/town portion of the address. Examples: US city, IT comune, UK post town. In regions of the world\nwhere localities are not well defined or do not fit into this structure well, leave locality empty and use addressLines."]
    pub fn locality(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\nThe name of the organization at the address."]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }
    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\nPostal code of the address. Not all countries use or require postal codes to be present, but where they are used,\nthey may trigger additional validation with other parts of the address (e.g. state/zip validation in the U.S.A.)."]
    pub fn postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.base))
    }
    #[doc = "Get a reference to the value of field `recipients` after provisioning.\nThe recipient at the address. This field may, under certain circumstances, contain multiline information. For example,\nit might contain \"care of\" information."]
    pub fn recipients(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.recipients", self.base))
    }
    #[doc = "Get a reference to the value of field `region_code` after provisioning.\nRequired. CLDR region code of the country/region of the address. This is never inferred and it is up to the user to\nensure the value is correct. See https://cldr.unicode.org/ and\nhttps://www.unicode.org/cldr/charts/30/supplemental/territory_information.html for details. Example: \"CH\" for Switzerland."]
    pub fn region_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_code", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddomainsRegistrationContactSettingsElTechnicalContactElDynamic {
    postal_address: Option<
        DynamicBlock<ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl>,
    >,
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationContactSettingsElTechnicalContactEl {
    email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax_number: Option<PrimField<String>>,
    phone_number: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    postal_address:
        Option<Vec<ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl>>,
    dynamic: ClouddomainsRegistrationContactSettingsElTechnicalContactElDynamic,
}
impl ClouddomainsRegistrationContactSettingsElTechnicalContactEl {
    #[doc = "Set the field `fax_number`.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn set_fax_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fax_number = Some(v.into());
        self
    }
    #[doc = "Set the field `postal_address`.\n"]
    pub fn set_postal_address(
        mut self,
        v: impl Into<
            BlockAssignable<
                ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.postal_address = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.postal_address = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationContactSettingsElTechnicalContactEl {
    type O = BlockAssignable<ClouddomainsRegistrationContactSettingsElTechnicalContactEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationContactSettingsElTechnicalContactEl {
    #[doc = "Required. Email address of the contact."]
    pub email: PrimField<String>,
    #[doc = "Required. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub phone_number: PrimField<String>,
}
impl BuildClouddomainsRegistrationContactSettingsElTechnicalContactEl {
    pub fn build(self) -> ClouddomainsRegistrationContactSettingsElTechnicalContactEl {
        ClouddomainsRegistrationContactSettingsElTechnicalContactEl {
            email: self.email,
            fax_number: core::default::Default::default(),
            phone_number: self.phone_number,
            postal_address: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationContactSettingsElTechnicalContactElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationContactSettingsElTechnicalContactElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddomainsRegistrationContactSettingsElTechnicalContactElRef {
        ClouddomainsRegistrationContactSettingsElTechnicalContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationContactSettingsElTechnicalContactElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contact."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `fax_number` after provisioning.\nFax number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn fax_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax_number", self.base))
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\nRequired. Phone number of the contact in international format. For example, \"+1-800-555-0123\"."]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
    #[doc = "Get a reference to the value of field `postal_address` after provisioning.\n"]
    pub fn postal_address(
        &self,
    ) -> ListRef<ClouddomainsRegistrationContactSettingsElTechnicalContactElPostalAddressElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.postal_address", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ClouddomainsRegistrationContactSettingsElDynamic {
    admin_contact: Option<DynamicBlock<ClouddomainsRegistrationContactSettingsElAdminContactEl>>,
    registrant_contact:
        Option<DynamicBlock<ClouddomainsRegistrationContactSettingsElRegistrantContactEl>>,
    technical_contact:
        Option<DynamicBlock<ClouddomainsRegistrationContactSettingsElTechnicalContactEl>>,
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationContactSettingsEl {
    privacy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_contact: Option<Vec<ClouddomainsRegistrationContactSettingsElAdminContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registrant_contact: Option<Vec<ClouddomainsRegistrationContactSettingsElRegistrantContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    technical_contact: Option<Vec<ClouddomainsRegistrationContactSettingsElTechnicalContactEl>>,
    dynamic: ClouddomainsRegistrationContactSettingsElDynamic,
}
impl ClouddomainsRegistrationContactSettingsEl {
    #[doc = "Set the field `admin_contact`.\n"]
    pub fn set_admin_contact(
        mut self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationContactSettingsElAdminContactEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.admin_contact = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.admin_contact = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `registrant_contact`.\n"]
    pub fn set_registrant_contact(
        mut self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationContactSettingsElRegistrantContactEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.registrant_contact = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.registrant_contact = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `technical_contact`.\n"]
    pub fn set_technical_contact(
        mut self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationContactSettingsElTechnicalContactEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.technical_contact = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.technical_contact = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationContactSettingsEl {
    type O = BlockAssignable<ClouddomainsRegistrationContactSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationContactSettingsEl {
    #[doc = "Required. Privacy setting for the contacts associated with the Registration.\nValues are PUBLIC_CONTACT_DATA, PRIVATE_CONTACT_DATA, and REDACTED_CONTACT_DATA"]
    pub privacy: PrimField<String>,
}
impl BuildClouddomainsRegistrationContactSettingsEl {
    pub fn build(self) -> ClouddomainsRegistrationContactSettingsEl {
        ClouddomainsRegistrationContactSettingsEl {
            privacy: self.privacy,
            admin_contact: core::default::Default::default(),
            registrant_contact: core::default::Default::default(),
            technical_contact: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationContactSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationContactSettingsElRef {
    fn new(shared: StackShared, base: String) -> ClouddomainsRegistrationContactSettingsElRef {
        ClouddomainsRegistrationContactSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationContactSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `privacy` after provisioning.\nRequired. Privacy setting for the contacts associated with the Registration.\nValues are PUBLIC_CONTACT_DATA, PRIVATE_CONTACT_DATA, and REDACTED_CONTACT_DATA"]
    pub fn privacy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.privacy", self.base))
    }
    #[doc = "Get a reference to the value of field `admin_contact` after provisioning.\n"]
    pub fn admin_contact(
        &self,
    ) -> ListRef<ClouddomainsRegistrationContactSettingsElAdminContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_contact", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(
        &self,
    ) -> ListRef<ClouddomainsRegistrationContactSettingsElRegistrantContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registrant_contact", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `technical_contact` after provisioning.\n"]
    pub fn technical_contact(
        &self,
    ) -> ListRef<ClouddomainsRegistrationContactSettingsElTechnicalContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.technical_contact", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digest: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    digest_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_tag: Option<PrimField<f64>>,
}
impl ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl {
    #[doc = "Set the field `algorithm`.\nThe algorithm used to generate the referenced DNSKEY."]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }
    #[doc = "Set the field `digest`.\nThe digest generated from the referenced DNSKEY."]
    pub fn set_digest(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.digest = Some(v.into());
        self
    }
    #[doc = "Set the field `digest_type`.\nThe hash function used to generate the digest of the referenced DNSKEY."]
    pub fn set_digest_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.digest_type = Some(v.into());
        self
    }
    #[doc = "Set the field `key_tag`.\nThe key tag of the record. Must be set in range 0 -- 65535."]
    pub fn set_key_tag(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.key_tag = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl {
    type O = BlockAssignable<ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl {}
impl BuildClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl {
    pub fn build(self) -> ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl {
        ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl {
            algorithm: core::default::Default::default(),
            digest: core::default::Default::default(),
            digest_type: core::default::Default::default(),
            key_tag: core::default::Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsElRef {
        ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\nThe algorithm used to generate the referenced DNSKEY."]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }
    #[doc = "Get a reference to the value of field `digest` after provisioning.\nThe digest generated from the referenced DNSKEY."]
    pub fn digest(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest", self.base))
    }
    #[doc = "Get a reference to the value of field `digest_type` after provisioning.\nThe hash function used to generate the digest of the referenced DNSKEY."]
    pub fn digest_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.digest_type", self.base))
    }
    #[doc = "Get a reference to the value of field `key_tag` after provisioning.\nThe key tag of the record. Must be set in range 0 -- 65535."]
    pub fn key_tag(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddomainsRegistrationDnsSettingsElCustomDnsElDynamic {
    ds_records: Option<DynamicBlock<ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl>>,
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationDnsSettingsElCustomDnsEl {
    name_servers: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ds_records: Option<Vec<ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl>>,
    dynamic: ClouddomainsRegistrationDnsSettingsElCustomDnsElDynamic,
}
impl ClouddomainsRegistrationDnsSettingsElCustomDnsEl {
    #[doc = "Set the field `ds_records`.\n"]
    pub fn set_ds_records(
        mut self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ds_records = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ds_records = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationDnsSettingsElCustomDnsEl {
    type O = BlockAssignable<ClouddomainsRegistrationDnsSettingsElCustomDnsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationDnsSettingsElCustomDnsEl {
    #[doc = "Required. A list of name servers that store the DNS zone for this domain. Each name server is a domain\nname, with Unicode domain names expressed in Punycode format."]
    pub name_servers: ListField<PrimField<String>>,
}
impl BuildClouddomainsRegistrationDnsSettingsElCustomDnsEl {
    pub fn build(self) -> ClouddomainsRegistrationDnsSettingsElCustomDnsEl {
        ClouddomainsRegistrationDnsSettingsElCustomDnsEl {
            name_servers: self.name_servers,
            ds_records: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationDnsSettingsElCustomDnsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationDnsSettingsElCustomDnsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddomainsRegistrationDnsSettingsElCustomDnsElRef {
        ClouddomainsRegistrationDnsSettingsElCustomDnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationDnsSettingsElCustomDnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name_servers` after provisioning.\nRequired. A list of name servers that store the DNS zone for this domain. Each name server is a domain\nname, with Unicode domain names expressed in Punycode format."]
    pub fn name_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.name_servers", self.base))
    }
    #[doc = "Get a reference to the value of field `ds_records` after provisioning.\n"]
    pub fn ds_records(
        &self,
    ) -> ListRef<ClouddomainsRegistrationDnsSettingsElCustomDnsElDsRecordsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ds_records", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationDnsSettingsElGlueRecordsEl {
    host_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv4_addresses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_addresses: Option<ListField<PrimField<String>>>,
}
impl ClouddomainsRegistrationDnsSettingsElGlueRecordsEl {
    #[doc = "Set the field `ipv4_addresses`.\nList of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).\nAt least one of ipv4_address and ipv6_address must be set."]
    pub fn set_ipv4_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ipv4_addresses = Some(v.into());
        self
    }
    #[doc = "Set the field `ipv6_addresses`.\nList of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).\nAt least one of ipv4_address and ipv6_address must be set."]
    pub fn set_ipv6_addresses(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ipv6_addresses = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationDnsSettingsElGlueRecordsEl {
    type O = BlockAssignable<ClouddomainsRegistrationDnsSettingsElGlueRecordsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationDnsSettingsElGlueRecordsEl {
    #[doc = "Required. Domain name of the host in Punycode format."]
    pub host_name: PrimField<String>,
}
impl BuildClouddomainsRegistrationDnsSettingsElGlueRecordsEl {
    pub fn build(self) -> ClouddomainsRegistrationDnsSettingsElGlueRecordsEl {
        ClouddomainsRegistrationDnsSettingsElGlueRecordsEl {
            host_name: self.host_name,
            ipv4_addresses: core::default::Default::default(),
            ipv6_addresses: core::default::Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationDnsSettingsElGlueRecordsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationDnsSettingsElGlueRecordsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ClouddomainsRegistrationDnsSettingsElGlueRecordsElRef {
        ClouddomainsRegistrationDnsSettingsElGlueRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationDnsSettingsElGlueRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_name` after provisioning.\nRequired. Domain name of the host in Punycode format."]
    pub fn host_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }
    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\nList of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).\nAt least one of ipv4_address and ipv6_address must be set."]
    pub fn ipv4_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv4_addresses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\nList of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).\nAt least one of ipv4_address and ipv6_address must be set."]
    pub fn ipv6_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipv6_addresses", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ClouddomainsRegistrationDnsSettingsElDynamic {
    custom_dns: Option<DynamicBlock<ClouddomainsRegistrationDnsSettingsElCustomDnsEl>>,
    glue_records: Option<DynamicBlock<ClouddomainsRegistrationDnsSettingsElGlueRecordsEl>>,
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationDnsSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_dns: Option<Vec<ClouddomainsRegistrationDnsSettingsElCustomDnsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    glue_records: Option<Vec<ClouddomainsRegistrationDnsSettingsElGlueRecordsEl>>,
    dynamic: ClouddomainsRegistrationDnsSettingsElDynamic,
}
impl ClouddomainsRegistrationDnsSettingsEl {
    #[doc = "Set the field `custom_dns`.\n"]
    pub fn set_custom_dns(
        mut self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationDnsSettingsElCustomDnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_dns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_dns = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `glue_records`.\n"]
    pub fn set_glue_records(
        mut self,
        v: impl Into<BlockAssignable<ClouddomainsRegistrationDnsSettingsElGlueRecordsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.glue_records = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.glue_records = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationDnsSettingsEl {
    type O = BlockAssignable<ClouddomainsRegistrationDnsSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationDnsSettingsEl {}
impl BuildClouddomainsRegistrationDnsSettingsEl {
    pub fn build(self) -> ClouddomainsRegistrationDnsSettingsEl {
        ClouddomainsRegistrationDnsSettingsEl {
            custom_dns: core::default::Default::default(),
            glue_records: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationDnsSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationDnsSettingsElRef {
    fn new(shared: StackShared, base: String) -> ClouddomainsRegistrationDnsSettingsElRef {
        ClouddomainsRegistrationDnsSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationDnsSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_dns` after provisioning.\n"]
    pub fn custom_dns(&self) -> ListRef<ClouddomainsRegistrationDnsSettingsElCustomDnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_dns", self.base))
    }
    #[doc = "Get a reference to the value of field `glue_records` after provisioning.\n"]
    pub fn glue_records(&self) -> ListRef<ClouddomainsRegistrationDnsSettingsElGlueRecordsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.glue_records", self.base))
    }
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationManagementSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_renewal_method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lock_state: Option<PrimField<String>>,
}
impl ClouddomainsRegistrationManagementSettingsEl {
    #[doc = "Set the field `preferred_renewal_method`.\nThe desired renewal method for this Registration. The actual renewalMethod is automatically updated to reflect this choice.\nIf unset or equal to RENEWAL_METHOD_UNSPECIFIED, the actual renewalMethod is treated as if it were set to AUTOMATIC_RENEWAL.\nYou cannot use RENEWAL_DISABLED during resource creation, and you can update the renewal status only when the Registration\nresource has state ACTIVE or SUSPENDED.\n\nWhen preferredRenewalMethod is set to AUTOMATIC_RENEWAL, the actual renewalMethod can be set to RENEWAL_DISABLED in case of\nproblems with the billing account or reported domain abuse. In such cases, check the issues field on the Registration. After\nthe problem is resolved, the renewalMethod is automatically updated to preferredRenewalMethod in a few hours."]
    pub fn set_preferred_renewal_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preferred_renewal_method = Some(v.into());
        self
    }
    #[doc = "Set the field `transfer_lock_state`.\nControls whether the domain can be transferred to another registrar. Values are UNLOCKED or LOCKED."]
    pub fn set_transfer_lock_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transfer_lock_state = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationManagementSettingsEl {
    type O = BlockAssignable<ClouddomainsRegistrationManagementSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationManagementSettingsEl {}
impl BuildClouddomainsRegistrationManagementSettingsEl {
    pub fn build(self) -> ClouddomainsRegistrationManagementSettingsEl {
        ClouddomainsRegistrationManagementSettingsEl {
            preferred_renewal_method: core::default::Default::default(),
            transfer_lock_state: core::default::Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationManagementSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationManagementSettingsElRef {
    fn new(shared: StackShared, base: String) -> ClouddomainsRegistrationManagementSettingsElRef {
        ClouddomainsRegistrationManagementSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationManagementSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `preferred_renewal_method` after provisioning.\nThe desired renewal method for this Registration. The actual renewalMethod is automatically updated to reflect this choice.\nIf unset or equal to RENEWAL_METHOD_UNSPECIFIED, the actual renewalMethod is treated as if it were set to AUTOMATIC_RENEWAL.\nYou cannot use RENEWAL_DISABLED during resource creation, and you can update the renewal status only when the Registration\nresource has state ACTIVE or SUSPENDED.\n\nWhen preferredRenewalMethod is set to AUTOMATIC_RENEWAL, the actual renewalMethod can be set to RENEWAL_DISABLED in case of\nproblems with the billing account or reported domain abuse. In such cases, check the issues field on the Registration. After\nthe problem is resolved, the renewalMethod is automatically updated to preferredRenewalMethod in a few hours."]
    pub fn preferred_renewal_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preferred_renewal_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `renewal_method` after provisioning.\nOutput only. The actual renewal method for this Registration. When preferredRenewalMethod is set to AUTOMATIC_RENEWAL,\nthe actual renewalMethod can be equal to RENEWAL_DISABLED—for example, when there are problems with the billing account\nor reported domain abuse. In such cases, check the issues field on the Registration. After the problem is resolved, the\nrenewalMethod is automatically updated to preferredRenewalMethod in a few hours."]
    pub fn renewal_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.renewal_method", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_lock_state` after provisioning.\nControls whether the domain can be transferred to another registrar. Values are UNLOCKED or LOCKED."]
    pub fn transfer_lock_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transfer_lock_state", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ClouddomainsRegistrationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ClouddomainsRegistrationTimeoutsEl {
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
impl ToListMappable for ClouddomainsRegistrationTimeoutsEl {
    type O = BlockAssignable<ClouddomainsRegistrationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationTimeoutsEl {}
impl BuildClouddomainsRegistrationTimeoutsEl {
    pub fn build(self) -> ClouddomainsRegistrationTimeoutsEl {
        ClouddomainsRegistrationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ClouddomainsRegistrationTimeoutsElRef {
        ClouddomainsRegistrationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationTimeoutsElRef {
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
pub struct ClouddomainsRegistrationYearlyPriceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<PrimField<String>>,
}
impl ClouddomainsRegistrationYearlyPriceEl {
    #[doc = "Set the field `currency_code`.\nThe three-letter currency code defined in ISO 4217."]
    pub fn set_currency_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.currency_code = Some(v.into());
        self
    }
    #[doc = "Set the field `units`.\nThe whole units of the amount. For example if currencyCode is \"USD\", then 1 unit is one US dollar."]
    pub fn set_units(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.units = Some(v.into());
        self
    }
}
impl ToListMappable for ClouddomainsRegistrationYearlyPriceEl {
    type O = BlockAssignable<ClouddomainsRegistrationYearlyPriceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildClouddomainsRegistrationYearlyPriceEl {}
impl BuildClouddomainsRegistrationYearlyPriceEl {
    pub fn build(self) -> ClouddomainsRegistrationYearlyPriceEl {
        ClouddomainsRegistrationYearlyPriceEl {
            currency_code: core::default::Default::default(),
            units: core::default::Default::default(),
        }
    }
}
pub struct ClouddomainsRegistrationYearlyPriceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ClouddomainsRegistrationYearlyPriceElRef {
    fn new(shared: StackShared, base: String) -> ClouddomainsRegistrationYearlyPriceElRef {
        ClouddomainsRegistrationYearlyPriceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ClouddomainsRegistrationYearlyPriceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `currency_code` after provisioning.\nThe three-letter currency code defined in ISO 4217."]
    pub fn currency_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.currency_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `units` after provisioning.\nThe whole units of the amount. For example if currencyCode is \"USD\", then 1 unit is one US dollar."]
    pub fn units(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.units", self.base))
    }
}
#[derive(Serialize, Default)]
struct ClouddomainsRegistrationDynamic {
    contact_settings: Option<DynamicBlock<ClouddomainsRegistrationContactSettingsEl>>,
    dns_settings: Option<DynamicBlock<ClouddomainsRegistrationDnsSettingsEl>>,
    management_settings: Option<DynamicBlock<ClouddomainsRegistrationManagementSettingsEl>>,
    yearly_price: Option<DynamicBlock<ClouddomainsRegistrationYearlyPriceEl>>,
}
