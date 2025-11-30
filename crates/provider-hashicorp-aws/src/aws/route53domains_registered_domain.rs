use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct Route53domainsRegisteredDomainData {
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
    billing_privacy: Option<PrimField<bool>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registrant_privacy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tech_privacy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lock: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_contact: Option<Vec<Route53domainsRegisteredDomainAdminContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_contact: Option<Vec<Route53domainsRegisteredDomainBillingContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_server: Option<Vec<Route53domainsRegisteredDomainNameServerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registrant_contact: Option<Vec<Route53domainsRegisteredDomainRegistrantContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tech_contact: Option<Vec<Route53domainsRegisteredDomainTechContactEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Route53domainsRegisteredDomainTimeoutsEl>,
    dynamic: Route53domainsRegisteredDomainDynamic,
}

struct Route53domainsRegisteredDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53domainsRegisteredDomainData>,
}

#[derive(Clone)]
pub struct Route53domainsRegisteredDomain(Rc<Route53domainsRegisteredDomain_>);

impl Route53domainsRegisteredDomain {
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

    #[doc = "Set the field `billing_privacy`.\n"]
    pub fn set_billing_privacy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().billing_privacy = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
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
        v: impl Into<BlockAssignable<Route53domainsRegisteredDomainAdminContactEl>>,
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

    #[doc = "Set the field `billing_contact`.\n"]
    pub fn set_billing_contact(
        self,
        v: impl Into<BlockAssignable<Route53domainsRegisteredDomainBillingContactEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().billing_contact = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.billing_contact = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `name_server`.\n"]
    pub fn set_name_server(
        self,
        v: impl Into<BlockAssignable<Route53domainsRegisteredDomainNameServerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().name_server = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.name_server = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `registrant_contact`.\n"]
    pub fn set_registrant_contact(
        self,
        v: impl Into<BlockAssignable<Route53domainsRegisteredDomainRegistrantContactEl>>,
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
        v: impl Into<BlockAssignable<Route53domainsRegisteredDomainTechContactEl>>,
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
    pub fn set_timeouts(self, v: impl Into<Route53domainsRegisteredDomainTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `expiration_date` after provisioning.\n"]
    pub fn expiration_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `reseller` after provisioning.\n"]
    pub fn reseller(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reseller", self.extract_ref()),
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
    pub fn admin_contact(&self) -> ListRef<Route53domainsRegisteredDomainAdminContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_contact", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `billing_contact` after provisioning.\n"]
    pub fn billing_contact(&self) -> ListRef<Route53domainsRegisteredDomainBillingContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_contact", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name_server` after provisioning.\n"]
    pub fn name_server(&self) -> ListRef<Route53domainsRegisteredDomainNameServerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.name_server", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(
        &self,
    ) -> ListRef<Route53domainsRegisteredDomainRegistrantContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registrant_contact", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tech_contact` after provisioning.\n"]
    pub fn tech_contact(&self) -> ListRef<Route53domainsRegisteredDomainTechContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tech_contact", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsRegisteredDomainTimeoutsElRef {
        Route53domainsRegisteredDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Route53domainsRegisteredDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for Route53domainsRegisteredDomain {}

impl ToListMappable for Route53domainsRegisteredDomain {
    type O = ListRef<Route53domainsRegisteredDomainRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53domainsRegisteredDomain_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53domains_registered_domain".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53domainsRegisteredDomain {
    pub tf_id: String,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildRoute53domainsRegisteredDomain {
    pub fn build(self, stack: &mut Stack) -> Route53domainsRegisteredDomain {
        let out = Route53domainsRegisteredDomain(Rc::new(Route53domainsRegisteredDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Route53domainsRegisteredDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_privacy: core::default::Default::default(),
                auto_renew: core::default::Default::default(),
                billing_privacy: core::default::Default::default(),
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                registrant_privacy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tech_privacy: core::default::Default::default(),
                transfer_lock: core::default::Default::default(),
                admin_contact: core::default::Default::default(),
                billing_contact: core::default::Default::default(),
                name_server: core::default::Default::default(),
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

pub struct Route53domainsRegisteredDomainRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsRegisteredDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl Route53domainsRegisteredDomainRef {
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

    #[doc = "Get a reference to the value of field `expiration_date` after provisioning.\n"]
    pub fn expiration_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.expiration_date", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `reseller` after provisioning.\n"]
    pub fn reseller(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reseller", self.extract_ref()),
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
    pub fn admin_contact(&self) -> ListRef<Route53domainsRegisteredDomainAdminContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_contact", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `billing_contact` after provisioning.\n"]
    pub fn billing_contact(&self) -> ListRef<Route53domainsRegisteredDomainBillingContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.billing_contact", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name_server` after provisioning.\n"]
    pub fn name_server(&self) -> ListRef<Route53domainsRegisteredDomainNameServerElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.name_server", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `registrant_contact` after provisioning.\n"]
    pub fn registrant_contact(
        &self,
    ) -> ListRef<Route53domainsRegisteredDomainRegistrantContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registrant_contact", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tech_contact` after provisioning.\n"]
    pub fn tech_contact(&self) -> ListRef<Route53domainsRegisteredDomainTechContactElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tech_contact", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Route53domainsRegisteredDomainTimeoutsElRef {
        Route53domainsRegisteredDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Route53domainsRegisteredDomainAdminContactEl {
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
    extra_params: Option<RecField<PrimField<String>>>,
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

impl Route53domainsRegisteredDomainAdminContactEl {
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

    #[doc = "Set the field `extra_params`.\n"]
    pub fn set_extra_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.extra_params = Some(v.into());
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

impl ToListMappable for Route53domainsRegisteredDomainAdminContactEl {
    type O = BlockAssignable<Route53domainsRegisteredDomainAdminContactEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainAdminContactEl {}

impl BuildRoute53domainsRegisteredDomainAdminContactEl {
    pub fn build(self) -> Route53domainsRegisteredDomainAdminContactEl {
        Route53domainsRegisteredDomainAdminContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            extra_params: core::default::Default::default(),
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

pub struct Route53domainsRegisteredDomainAdminContactElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsRegisteredDomainAdminContactElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsRegisteredDomainAdminContactElRef {
        Route53domainsRegisteredDomainAdminContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainAdminContactElRef {
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

    #[doc = "Get a reference to the value of field `extra_params` after provisioning.\n"]
    pub fn extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.extra_params", self.base))
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
pub struct Route53domainsRegisteredDomainBillingContactEl {
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
    extra_params: Option<RecField<PrimField<String>>>,
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

impl Route53domainsRegisteredDomainBillingContactEl {
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

    #[doc = "Set the field `extra_params`.\n"]
    pub fn set_extra_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.extra_params = Some(v.into());
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

impl ToListMappable for Route53domainsRegisteredDomainBillingContactEl {
    type O = BlockAssignable<Route53domainsRegisteredDomainBillingContactEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainBillingContactEl {}

impl BuildRoute53domainsRegisteredDomainBillingContactEl {
    pub fn build(self) -> Route53domainsRegisteredDomainBillingContactEl {
        Route53domainsRegisteredDomainBillingContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            extra_params: core::default::Default::default(),
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

pub struct Route53domainsRegisteredDomainBillingContactElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsRegisteredDomainBillingContactElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsRegisteredDomainBillingContactElRef {
        Route53domainsRegisteredDomainBillingContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainBillingContactElRef {
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

    #[doc = "Get a reference to the value of field `extra_params` after provisioning.\n"]
    pub fn extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.extra_params", self.base))
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
pub struct Route53domainsRegisteredDomainNameServerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    glue_ips: Option<SetField<PrimField<String>>>,
    name: PrimField<String>,
}

impl Route53domainsRegisteredDomainNameServerEl {
    #[doc = "Set the field `glue_ips`.\n"]
    pub fn set_glue_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.glue_ips = Some(v.into());
        self
    }
}

impl ToListMappable for Route53domainsRegisteredDomainNameServerEl {
    type O = BlockAssignable<Route53domainsRegisteredDomainNameServerEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainNameServerEl {
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildRoute53domainsRegisteredDomainNameServerEl {
    pub fn build(self) -> Route53domainsRegisteredDomainNameServerEl {
        Route53domainsRegisteredDomainNameServerEl {
            glue_ips: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct Route53domainsRegisteredDomainNameServerElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsRegisteredDomainNameServerElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsRegisteredDomainNameServerElRef {
        Route53domainsRegisteredDomainNameServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainNameServerElRef {
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
pub struct Route53domainsRegisteredDomainRegistrantContactEl {
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
    extra_params: Option<RecField<PrimField<String>>>,
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

impl Route53domainsRegisteredDomainRegistrantContactEl {
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

    #[doc = "Set the field `extra_params`.\n"]
    pub fn set_extra_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.extra_params = Some(v.into());
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

impl ToListMappable for Route53domainsRegisteredDomainRegistrantContactEl {
    type O = BlockAssignable<Route53domainsRegisteredDomainRegistrantContactEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainRegistrantContactEl {}

impl BuildRoute53domainsRegisteredDomainRegistrantContactEl {
    pub fn build(self) -> Route53domainsRegisteredDomainRegistrantContactEl {
        Route53domainsRegisteredDomainRegistrantContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            extra_params: core::default::Default::default(),
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

pub struct Route53domainsRegisteredDomainRegistrantContactElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsRegisteredDomainRegistrantContactElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> Route53domainsRegisteredDomainRegistrantContactElRef {
        Route53domainsRegisteredDomainRegistrantContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainRegistrantContactElRef {
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

    #[doc = "Get a reference to the value of field `extra_params` after provisioning.\n"]
    pub fn extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.extra_params", self.base))
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
pub struct Route53domainsRegisteredDomainTechContactEl {
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
    extra_params: Option<RecField<PrimField<String>>>,
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

impl Route53domainsRegisteredDomainTechContactEl {
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

    #[doc = "Set the field `extra_params`.\n"]
    pub fn set_extra_params(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.extra_params = Some(v.into());
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

impl ToListMappable for Route53domainsRegisteredDomainTechContactEl {
    type O = BlockAssignable<Route53domainsRegisteredDomainTechContactEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainTechContactEl {}

impl BuildRoute53domainsRegisteredDomainTechContactEl {
    pub fn build(self) -> Route53domainsRegisteredDomainTechContactEl {
        Route53domainsRegisteredDomainTechContactEl {
            address_line_1: core::default::Default::default(),
            address_line_2: core::default::Default::default(),
            city: core::default::Default::default(),
            contact_type: core::default::Default::default(),
            country_code: core::default::Default::default(),
            email: core::default::Default::default(),
            extra_params: core::default::Default::default(),
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

pub struct Route53domainsRegisteredDomainTechContactElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsRegisteredDomainTechContactElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsRegisteredDomainTechContactElRef {
        Route53domainsRegisteredDomainTechContactElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainTechContactElRef {
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

    #[doc = "Get a reference to the value of field `extra_params` after provisioning.\n"]
    pub fn extra_params(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.extra_params", self.base))
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
pub struct Route53domainsRegisteredDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl Route53domainsRegisteredDomainTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for Route53domainsRegisteredDomainTimeoutsEl {
    type O = BlockAssignable<Route53domainsRegisteredDomainTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53domainsRegisteredDomainTimeoutsEl {}

impl BuildRoute53domainsRegisteredDomainTimeoutsEl {
    pub fn build(self) -> Route53domainsRegisteredDomainTimeoutsEl {
        Route53domainsRegisteredDomainTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Route53domainsRegisteredDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53domainsRegisteredDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Route53domainsRegisteredDomainTimeoutsElRef {
        Route53domainsRegisteredDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53domainsRegisteredDomainTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct Route53domainsRegisteredDomainDynamic {
    admin_contact: Option<DynamicBlock<Route53domainsRegisteredDomainAdminContactEl>>,
    billing_contact: Option<DynamicBlock<Route53domainsRegisteredDomainBillingContactEl>>,
    name_server: Option<DynamicBlock<Route53domainsRegisteredDomainNameServerEl>>,
    registrant_contact: Option<DynamicBlock<Route53domainsRegisteredDomainRegistrantContactEl>>,
    tech_contact: Option<DynamicBlock<Route53domainsRegisteredDomainTechContactEl>>,
}
