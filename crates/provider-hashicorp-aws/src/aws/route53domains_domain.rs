use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Route53domainsDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_privacy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_renew: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_contact: Option<ListField<Route53domainsDomainBillingContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_privacy: Option<PrimField<bool>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_in_years: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_server: Option<ListField<Route53domainsDomainNameServerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registrant_privacy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tech_privacy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lock: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_contact: Option<Vec<Route53domainsDomainAdminContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registrant_contact: Option<Vec<Route53domainsDomainRegistrantContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tech_contact: Option<Vec<Route53domainsDomainTechContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Route53domainsDomainTimeoutsEl>,
    dynamic: Route53domainsDomainDynamic,
}
struct Route53domainsDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53domainsDomainData>,
}
#[derive(Clone)]
pub struct Route53domainsDomain(Rc<Route53domainsDomain_>);
impl Route53domainsDomain {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `admin_privacy`.\n"]
    pub fn set_admin_privacy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().admin_privacy = Some(v.into());
        self
    }
    #[doc = "Set the field `auto_renew`.\n"]
    pub fn set_auto_renew(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_renew = Some(v.into());
        self
    }
    #[doc = "Set the field `billing_contact`.\n"]
    pub fn set_billing_contact(
        self,
        v: impl Into<ListField<Route53domainsDomainBillingContactEl>>,
    ) -> Self {
        self.0.data.borrow_mut().billing_contact = Some(v.into());
        self
    }
    #[doc = "Set the field `billing_privacy`.\n"]
    pub fn set_billing_privacy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().billing_privacy = Some(v.into());
        self
    }
    #[doc = "Set the field `duration_in_years`.\n"]
    pub fn set_duration_in_years(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().duration_in_years = Some(v.into());
        self
    }
    #[doc = "Set the field `name_server`.\n"]
    pub fn set_name_server(
        self,
        v: impl Into<ListField<Route53domainsDomainNameServerEl>>,
    ) -> Self {
        self.0.data.borrow_mut().name_server = Some(v.into());
        self
    }
    #[doc = "Set the field `registrant_privacy`.\n"]
    pub fn set_registrant_privacy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().registrant_privacy = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tech_privacy`.\n"]
    pub fn set_tech_privacy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tech_privacy = Some(v.into());
        self
    }
    #[doc = "Set the field `transfer_lock`.\n"]
    pub fn set_transfer_lock(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().transfer_lock = Some(v.into());
        self
    }
    #[doc = "Set the field `admin_contact`.\n"]
    pub fn set_admin_contact(
        self,
        v: impl Into<BlockAssignable<Route53domainsDomainAdminContactEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().admin_contact = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.admin_contact = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `registrant_contact`.\n"]
    pub fn set_registrant_contact(
        self,
        v: impl Into<BlockAssignable<Route53domainsDomainRegistrantContactEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().registrant_contact = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.registrant_contact = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tech_contact`.\n"]
    pub fn set_tech_contact(
        self,
        v: impl Into<BlockAssignable<Route53domainsDomainTechContactEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tech_contact = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tech_contact = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Route53domainsDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `abuse_contact_email` after provisioning.\n"]
    pub fn abuse_contact_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.abuse_contact_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `abuse_contact_phone` after provisioning.\n"]
    pub fn abuse_contact_phone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.abuse_contact_phone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_privacy` after provisioning.\n"]
    pub fn admin_privacy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_renew` after provisioning.\n"]
    pub fn auto_renew(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_renew", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_contact` after provisioning.\n"]
    pub fn billing_contact(&self) -> ListRef<Route53domainsDomainBillingContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_privacy` after provisioning.\n"]
    pub fn billing_privacy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `duration_in_years` after provisioning.\n"]
    pub fn duration_in_years(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.duration_in_years", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration_date` after provisioning.\n"]
    pub fn expiration_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_server` after provisioning.\n"]
    pub fn name_server(&self) -> ListRef<Route53domainsDomainNameServerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.name_server", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registrant_privacy` after provisioning.\n"]
    pub fn registrant_privacy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registrant_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registrar_name` after provisioning.\n"]
    pub fn registrar_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registrar_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registrar_url` after provisioning.\n"]
    pub fn registrar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registrar_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_list` after provisioning.\n"]
    pub fn status_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tech_privacy` after provisioning.\n"]
    pub fn tech_privacy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tech_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_lock` after provisioning.\n"]
    pub fn transfer_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transfer_lock", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `updated_date` after provisioning.\n"]
    pub fn updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `whois_server` after provisioning.\n"]
    pub fn whois_server(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.whois_server", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_contact` after provisioning.\n"]
    pub fn admin_contact(&self) -> ListRef<Route53domainsDomainAdminContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(&self) -> ListRef<Route53domainsDomainRegistrantContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registrant_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tech_contact` after provisioning.\n"]
    pub fn tech_contact(&self) -> ListRef<Route53domainsDomainTechContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tech_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsDomainTimeoutsElRef {
        Route53domainsDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for Route53domainsDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Route53domainsDomain {}
impl ToListMappable for Route53domainsDomain {
    type O = ListRef<Route53domainsDomainRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Route53domainsDomain_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53domains_domain".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildRoute53domainsDomain {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}
impl BuildRoute53domainsDomain {
    pub fn build(self, stack: &mut Stack) -> Route53domainsDomain {
        let out = Route53domainsDomain(Rc::new(Route53domainsDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Route53domainsDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_privacy: core::default::Default::default(),
                auto_renew: core::default::Default::default(),
                billing_contact: core::default::Default::default(),
                billing_privacy: core::default::Default::default(),
                domain_name: self.domain_name,
                duration_in_years: core::default::Default::default(),
                name_server: core::default::Default::default(),
                registrant_privacy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tech_privacy: core::default::Default::default(),
                transfer_lock: core::default::Default::default(),
                admin_contact: core::default::Default::default(),
                registrant_contact: core::default::Default::default(),
                tech_contact: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Route53domainsDomainRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Route53domainsDomainRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `abuse_contact_email` after provisioning.\n"]
    pub fn abuse_contact_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.abuse_contact_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `abuse_contact_phone` after provisioning.\n"]
    pub fn abuse_contact_phone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.abuse_contact_phone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_privacy` after provisioning.\n"]
    pub fn admin_privacy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auto_renew` after provisioning.\n"]
    pub fn auto_renew(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_renew", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_contact` after provisioning.\n"]
    pub fn billing_contact(&self) -> ListRef<Route53domainsDomainBillingContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `billing_privacy` after provisioning.\n"]
    pub fn billing_privacy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `duration_in_years` after provisioning.\n"]
    pub fn duration_in_years(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.duration_in_years", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `expiration_date` after provisioning.\n"]
    pub fn expiration_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hosted_zone_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_server` after provisioning.\n"]
    pub fn name_server(&self) -> ListRef<Route53domainsDomainNameServerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.name_server", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registrant_privacy` after provisioning.\n"]
    pub fn registrant_privacy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registrant_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registrar_name` after provisioning.\n"]
    pub fn registrar_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registrar_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registrar_url` after provisioning.\n"]
    pub fn registrar_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registrar_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status_list` after provisioning.\n"]
    pub fn status_list(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status_list", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tech_privacy` after provisioning.\n"]
    pub fn tech_privacy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tech_privacy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transfer_lock` after provisioning.\n"]
    pub fn transfer_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transfer_lock", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `updated_date` after provisioning.\n"]
    pub fn updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `whois_server` after provisioning.\n"]
    pub fn whois_server(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.whois_server", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_contact` after provisioning.\n"]
    pub fn admin_contact(&self) -> ListRef<Route53domainsDomainAdminContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(&self) -> ListRef<Route53domainsDomainRegistrantContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registrant_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tech_contact` after provisioning.\n"]
    pub fn tech_contact(&self) -> ListRef<Route53domainsDomainTechContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tech_contact", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsDomainTimeoutsElRef {
        Route53domainsDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Route53domainsDomainBillingContactElExtraParamEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl Route53domainsDomainBillingContactElExtraParamEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for Route53domainsDomainBillingContactElExtraParamEl {
    type O = BlockAssignable<Route53domainsDomainBillingContactElExtraParamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainBillingContactElExtraParamEl {}
impl BuildRoute53domainsDomainBillingContactElExtraParamEl {
    pub fn build(self) -> Route53domainsDomainBillingContactElExtraParamEl {
        Route53domainsDomainBillingContactElExtraParamEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct Route53domainsDomainBillingContactElExtraParamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainBillingContactElExtraParamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Route53domainsDomainBillingContactElExtraParamElRef {
        Route53domainsDomainBillingContactElExtraParamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainBillingContactElExtraParamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct Route53domainsDomainBillingContactEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_param: Option<ListField<Route53domainsDomainBillingContactElExtraParamEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip_code: Option<PrimField<String>>,
}
impl Route53domainsDomainBillingContactEl {
    #[doc = "Set the field `address_line_1`.\n"]
    pub fn set_address_line_1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_line_1 = Some(v.into());
        self
    }
    #[doc = "Set the field `address_line_2`.\n"]
    pub fn set_address_line_2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_line_2 = Some(v.into());
        self
    }
    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }
    #[doc = "Set the field `contact_type`.\n"]
    pub fn set_contact_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_type = Some(v.into());
        self
    }
    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country_code = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `extra_param`.\n"]
    pub fn set_extra_param(
        mut self,
        v: impl Into<ListField<Route53domainsDomainBillingContactElExtraParamEl>>,
    ) -> Self {
        self.extra_param = Some(v.into());
        self
    }
    #[doc = "Set the field `fax`.\n"]
    pub fn set_fax(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fax = Some(v.into());
        self
    }
    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.first_name = Some(v.into());
        self
    }
    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_name = Some(v.into());
        self
    }
    #[doc = "Set the field `organization_name`.\n"]
    pub fn set_organization_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization_name = Some(v.into());
        self
    }
    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phone_number = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `zip_code`.\n"]
    pub fn set_zip_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zip_code = Some(v.into());
        self
    }
}
impl ToListMappable for Route53domainsDomainBillingContactEl {
    type O = BlockAssignable<Route53domainsDomainBillingContactEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainBillingContactEl {}
impl BuildRoute53domainsDomainBillingContactEl {
    pub fn build(self) -> Route53domainsDomainBillingContactEl {
        Route53domainsDomainBillingContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            extra_param: core::default::Default::default(),
            fax: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            organization_name: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            state: core::default::Default::default(),
            zip_code: core::default::Default::default(),
        }
    }
}
pub struct Route53domainsDomainBillingContactElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainBillingContactElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDomainBillingContactElRef {
        Route53domainsDomainBillingContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainBillingContactElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_1", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_2", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }
    #[doc = "Get a reference to the value of field `contact_type` after provisioning.\n"]
    pub fn contact_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_type", self.base))
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `extra_param` after provisioning.\n"]
    pub fn extra_param(&self) -> ListRef<Route53domainsDomainBillingContactElExtraParamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extra_param", self.base))
    }
    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax", self.base))
    }
    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }
    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }
    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\n"]
    pub fn zip_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.base))
    }
}
#[derive(Serialize)]
pub struct Route53domainsDomainNameServerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    glue_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl Route53domainsDomainNameServerEl {
    #[doc = "Set the field `glue_ips`.\n"]
    pub fn set_glue_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.glue_ips = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for Route53domainsDomainNameServerEl {
    type O = BlockAssignable<Route53domainsDomainNameServerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainNameServerEl {}
impl BuildRoute53domainsDomainNameServerEl {
    pub fn build(self) -> Route53domainsDomainNameServerEl {
        Route53domainsDomainNameServerEl {
            glue_ips: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct Route53domainsDomainNameServerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainNameServerElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDomainNameServerElRef {
        Route53domainsDomainNameServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainNameServerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `glue_ips` after provisioning.\n"]
    pub fn glue_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.glue_ips", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct Route53domainsDomainAdminContactElExtraParamEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Route53domainsDomainAdminContactElExtraParamEl {}
impl ToListMappable for Route53domainsDomainAdminContactElExtraParamEl {
    type O = BlockAssignable<Route53domainsDomainAdminContactElExtraParamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainAdminContactElExtraParamEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildRoute53domainsDomainAdminContactElExtraParamEl {
    pub fn build(self) -> Route53domainsDomainAdminContactElExtraParamEl {
        Route53domainsDomainAdminContactElExtraParamEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct Route53domainsDomainAdminContactElExtraParamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainAdminContactElExtraParamElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDomainAdminContactElExtraParamElRef {
        Route53domainsDomainAdminContactElExtraParamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainAdminContactElExtraParamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct Route53domainsDomainAdminContactElDynamic {
    extra_param: Option<DynamicBlock<Route53domainsDomainAdminContactElExtraParamEl>>,
}
#[derive(Serialize)]
pub struct Route53domainsDomainAdminContactEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_param: Option<Vec<Route53domainsDomainAdminContactElExtraParamEl>>,
    dynamic: Route53domainsDomainAdminContactElDynamic,
}
impl Route53domainsDomainAdminContactEl {
    #[doc = "Set the field `address_line_1`.\n"]
    pub fn set_address_line_1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_line_1 = Some(v.into());
        self
    }
    #[doc = "Set the field `address_line_2`.\n"]
    pub fn set_address_line_2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_line_2 = Some(v.into());
        self
    }
    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }
    #[doc = "Set the field `contact_type`.\n"]
    pub fn set_contact_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_type = Some(v.into());
        self
    }
    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country_code = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `fax`.\n"]
    pub fn set_fax(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fax = Some(v.into());
        self
    }
    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.first_name = Some(v.into());
        self
    }
    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_name = Some(v.into());
        self
    }
    #[doc = "Set the field `organization_name`.\n"]
    pub fn set_organization_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization_name = Some(v.into());
        self
    }
    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phone_number = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `zip_code`.\n"]
    pub fn set_zip_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zip_code = Some(v.into());
        self
    }
    #[doc = "Set the field `extra_param`.\n"]
    pub fn set_extra_param(
        mut self,
        v: impl Into<BlockAssignable<Route53domainsDomainAdminContactElExtraParamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.extra_param = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.extra_param = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Route53domainsDomainAdminContactEl {
    type O = BlockAssignable<Route53domainsDomainAdminContactEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainAdminContactEl {}
impl BuildRoute53domainsDomainAdminContactEl {
    pub fn build(self) -> Route53domainsDomainAdminContactEl {
        Route53domainsDomainAdminContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            fax: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            organization_name: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            state: core::default::Default::default(),
            zip_code: core::default::Default::default(),
            extra_param: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Route53domainsDomainAdminContactElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainAdminContactElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDomainAdminContactElRef {
        Route53domainsDomainAdminContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainAdminContactElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_1", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_2", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }
    #[doc = "Get a reference to the value of field `contact_type` after provisioning.\n"]
    pub fn contact_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_type", self.base))
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax", self.base))
    }
    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }
    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }
    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\n"]
    pub fn zip_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.base))
    }
    #[doc = "Get a reference to the value of field `extra_param` after provisioning.\n"]
    pub fn extra_param(&self) -> ListRef<Route53domainsDomainAdminContactElExtraParamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extra_param", self.base))
    }
}
#[derive(Serialize)]
pub struct Route53domainsDomainRegistrantContactElExtraParamEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Route53domainsDomainRegistrantContactElExtraParamEl {}
impl ToListMappable for Route53domainsDomainRegistrantContactElExtraParamEl {
    type O = BlockAssignable<Route53domainsDomainRegistrantContactElExtraParamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainRegistrantContactElExtraParamEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildRoute53domainsDomainRegistrantContactElExtraParamEl {
    pub fn build(self) -> Route53domainsDomainRegistrantContactElExtraParamEl {
        Route53domainsDomainRegistrantContactElExtraParamEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct Route53domainsDomainRegistrantContactElExtraParamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainRegistrantContactElExtraParamElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Route53domainsDomainRegistrantContactElExtraParamElRef {
        Route53domainsDomainRegistrantContactElExtraParamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainRegistrantContactElExtraParamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct Route53domainsDomainRegistrantContactElDynamic {
    extra_param: Option<DynamicBlock<Route53domainsDomainRegistrantContactElExtraParamEl>>,
}
#[derive(Serialize)]
pub struct Route53domainsDomainRegistrantContactEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_param: Option<Vec<Route53domainsDomainRegistrantContactElExtraParamEl>>,
    dynamic: Route53domainsDomainRegistrantContactElDynamic,
}
impl Route53domainsDomainRegistrantContactEl {
    #[doc = "Set the field `address_line_1`.\n"]
    pub fn set_address_line_1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_line_1 = Some(v.into());
        self
    }
    #[doc = "Set the field `address_line_2`.\n"]
    pub fn set_address_line_2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_line_2 = Some(v.into());
        self
    }
    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }
    #[doc = "Set the field `contact_type`.\n"]
    pub fn set_contact_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_type = Some(v.into());
        self
    }
    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country_code = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `fax`.\n"]
    pub fn set_fax(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fax = Some(v.into());
        self
    }
    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.first_name = Some(v.into());
        self
    }
    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_name = Some(v.into());
        self
    }
    #[doc = "Set the field `organization_name`.\n"]
    pub fn set_organization_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization_name = Some(v.into());
        self
    }
    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phone_number = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `zip_code`.\n"]
    pub fn set_zip_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zip_code = Some(v.into());
        self
    }
    #[doc = "Set the field `extra_param`.\n"]
    pub fn set_extra_param(
        mut self,
        v: impl Into<BlockAssignable<Route53domainsDomainRegistrantContactElExtraParamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.extra_param = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.extra_param = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Route53domainsDomainRegistrantContactEl {
    type O = BlockAssignable<Route53domainsDomainRegistrantContactEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainRegistrantContactEl {}
impl BuildRoute53domainsDomainRegistrantContactEl {
    pub fn build(self) -> Route53domainsDomainRegistrantContactEl {
        Route53domainsDomainRegistrantContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            fax: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            organization_name: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            state: core::default::Default::default(),
            zip_code: core::default::Default::default(),
            extra_param: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Route53domainsDomainRegistrantContactElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainRegistrantContactElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDomainRegistrantContactElRef {
        Route53domainsDomainRegistrantContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainRegistrantContactElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_1", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_2", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }
    #[doc = "Get a reference to the value of field `contact_type` after provisioning.\n"]
    pub fn contact_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_type", self.base))
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax", self.base))
    }
    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }
    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }
    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\n"]
    pub fn zip_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.base))
    }
    #[doc = "Get a reference to the value of field `extra_param` after provisioning.\n"]
    pub fn extra_param(&self) -> ListRef<Route53domainsDomainRegistrantContactElExtraParamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extra_param", self.base))
    }
}
#[derive(Serialize)]
pub struct Route53domainsDomainTechContactElExtraParamEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl Route53domainsDomainTechContactElExtraParamEl {}
impl ToListMappable for Route53domainsDomainTechContactElExtraParamEl {
    type O = BlockAssignable<Route53domainsDomainTechContactElExtraParamEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainTechContactElExtraParamEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildRoute53domainsDomainTechContactElExtraParamEl {
    pub fn build(self) -> Route53domainsDomainTechContactElExtraParamEl {
        Route53domainsDomainTechContactElExtraParamEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct Route53domainsDomainTechContactElExtraParamElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainTechContactElExtraParamElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDomainTechContactElExtraParamElRef {
        Route53domainsDomainTechContactElExtraParamElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainTechContactElExtraParamElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct Route53domainsDomainTechContactElDynamic {
    extra_param: Option<DynamicBlock<Route53domainsDomainTechContactElExtraParamEl>>,
}
#[derive(Serialize)]
pub struct Route53domainsDomainTechContactEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line_2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    city: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fax: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zip_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra_param: Option<Vec<Route53domainsDomainTechContactElExtraParamEl>>,
    dynamic: Route53domainsDomainTechContactElDynamic,
}
impl Route53domainsDomainTechContactEl {
    #[doc = "Set the field `address_line_1`.\n"]
    pub fn set_address_line_1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_line_1 = Some(v.into());
        self
    }
    #[doc = "Set the field `address_line_2`.\n"]
    pub fn set_address_line_2(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_line_2 = Some(v.into());
        self
    }
    #[doc = "Set the field `city`.\n"]
    pub fn set_city(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.city = Some(v.into());
        self
    }
    #[doc = "Set the field `contact_type`.\n"]
    pub fn set_contact_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_type = Some(v.into());
        self
    }
    #[doc = "Set the field `country_code`.\n"]
    pub fn set_country_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.country_code = Some(v.into());
        self
    }
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
    #[doc = "Set the field `fax`.\n"]
    pub fn set_fax(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fax = Some(v.into());
        self
    }
    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.first_name = Some(v.into());
        self
    }
    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_name = Some(v.into());
        self
    }
    #[doc = "Set the field `organization_name`.\n"]
    pub fn set_organization_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization_name = Some(v.into());
        self
    }
    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phone_number = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `zip_code`.\n"]
    pub fn set_zip_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zip_code = Some(v.into());
        self
    }
    #[doc = "Set the field `extra_param`.\n"]
    pub fn set_extra_param(
        mut self,
        v: impl Into<BlockAssignable<Route53domainsDomainTechContactElExtraParamEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.extra_param = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.extra_param = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for Route53domainsDomainTechContactEl {
    type O = BlockAssignable<Route53domainsDomainTechContactEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainTechContactEl {}
impl BuildRoute53domainsDomainTechContactEl {
    pub fn build(self) -> Route53domainsDomainTechContactEl {
        Route53domainsDomainTechContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            fax: core::default::Default::default(),
            first_name: core::default::Default::default(),
            last_name: core::default::Default::default(),
            organization_name: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            state: core::default::Default::default(),
            zip_code: core::default::Default::default(),
            extra_param: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct Route53domainsDomainTechContactElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainTechContactElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDomainTechContactElRef {
        Route53domainsDomainTechContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainTechContactElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_1", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.address_line_2", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.city", self.base))
    }
    #[doc = "Get a reference to the value of field `contact_type` after provisioning.\n"]
    pub fn contact_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_type", self.base))
    }
    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
    #[doc = "Get a reference to the value of field `fax` after provisioning.\n"]
    pub fn fax(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fax", self.base))
    }
    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.first_name", self.base))
    }
    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_name", self.base))
    }
    #[doc = "Get a reference to the value of field `organization_name` after provisioning.\n"]
    pub fn organization_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.organization_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `zip_code` after provisioning.\n"]
    pub fn zip_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zip_code", self.base))
    }
    #[doc = "Get a reference to the value of field `extra_param` after provisioning.\n"]
    pub fn extra_param(&self) -> ListRef<Route53domainsDomainTechContactElExtraParamElRef> {
        ListRef::new(self.shared().clone(), format!("{}.extra_param", self.base))
    }
}
#[derive(Serialize)]
pub struct Route53domainsDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Route53domainsDomainTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for Route53domainsDomainTimeoutsEl {
    type O = BlockAssignable<Route53domainsDomainTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRoute53domainsDomainTimeoutsEl {}
impl BuildRoute53domainsDomainTimeoutsEl {
    pub fn build(self) -> Route53domainsDomainTimeoutsEl {
        Route53domainsDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Route53domainsDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Route53domainsDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsDomainTimeoutsElRef {
        Route53domainsDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Route53domainsDomainTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct Route53domainsDomainDynamic {
    admin_contact: Option<DynamicBlock<Route53domainsDomainAdminContactEl>>,
    registrant_contact: Option<DynamicBlock<Route53domainsDomainRegistrantContactEl>>,
    tech_contact: Option<DynamicBlock<Route53domainsDomainTechContactEl>>,
}
