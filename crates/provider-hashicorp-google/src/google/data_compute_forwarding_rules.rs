use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeForwardingRulesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComputeForwardingRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeForwardingRulesData>,
}
#[derive(Clone)]
pub struct DataComputeForwardingRules(Rc<DataComputeForwardingRules_>);
impl DataComputeForwardingRules {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<DataComputeForwardingRulesRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeForwardingRules {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeForwardingRules {}
impl ToListMappable for DataComputeForwardingRules {
    type O = ListRef<DataComputeForwardingRulesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeForwardingRules_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_forwarding_rules".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeForwardingRules {
    pub tf_id: String,
}
impl BuildDataComputeForwardingRules {
    pub fn build(self, stack: &mut Stack) -> DataComputeForwardingRules {
        let out = DataComputeForwardingRules(Rc::new(DataComputeForwardingRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeForwardingRulesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeForwardingRulesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeForwardingRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeForwardingRulesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<DataComputeForwardingRulesRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl {
    type O = BlockAssignable<DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl {}
impl BuildDataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl {
    pub fn build(self) -> DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl {
        DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl {
            namespace: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}
pub struct DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsElRef {
        DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeForwardingRulesRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all_ports: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_global_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_psc_global_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effective_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_collection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_mirroring_collector: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label_fingerprint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_scheme: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_automate_dns_zone: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recreate_closed_psc: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_registrations:
        Option<ListField<DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terraform_labels: Option<RecField<PrimField<String>>>,
}
impl DataComputeForwardingRulesRulesEl {
    #[doc = "Set the field `all_ports`.\n"]
    pub fn set_all_ports(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all_ports = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_global_access`.\n"]
    pub fn set_allow_global_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_global_access = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_psc_global_access`.\n"]
    pub fn set_allow_psc_global_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_psc_global_access = Some(v.into());
        self
    }
    #[doc = "Set the field `backend_service`.\n"]
    pub fn set_backend_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backend_service = Some(v.into());
        self
    }
    #[doc = "Set the field `base_forwarding_rule`.\n"]
    pub fn set_base_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.base_forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `creation_timestamp`.\n"]
    pub fn set_creation_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_timestamp = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.effective_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `forwarding_rule_id`.\n"]
    pub fn set_forwarding_rule_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.forwarding_rule_id = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_address`.\n"]
    pub fn set_ip_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_collection`.\n"]
    pub fn set_ip_collection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_collection = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_protocol`.\n"]
    pub fn set_ip_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `ip_version`.\n"]
    pub fn set_ip_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_version = Some(v.into());
        self
    }
    #[doc = "Set the field `is_mirroring_collector`.\n"]
    pub fn set_is_mirroring_collector(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_mirroring_collector = Some(v.into());
        self
    }
    #[doc = "Set the field `label_fingerprint`.\n"]
    pub fn set_label_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.label_fingerprint = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `load_balancing_scheme`.\n"]
    pub fn set_load_balancing_scheme(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.load_balancing_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tier`.\n"]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `no_automate_dns_zone`.\n"]
    pub fn set_no_automate_dns_zone(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_automate_dns_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_range = Some(v.into());
        self
    }
    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ports = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_connection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_connection_status`.\n"]
    pub fn set_psc_connection_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_connection_status = Some(v.into());
        self
    }
    #[doc = "Set the field `recreate_closed_psc`.\n"]
    pub fn set_recreate_closed_psc(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.recreate_closed_psc = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `service_directory_registrations`.\n"]
    pub fn set_service_directory_registrations(
        mut self,
        v: impl Into<ListField<DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsEl>>,
    ) -> Self {
        self.service_directory_registrations = Some(v.into());
        self
    }
    #[doc = "Set the field `service_label`.\n"]
    pub fn set_service_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_label = Some(v.into());
        self
    }
    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `source_ip_ranges`.\n"]
    pub fn set_source_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.source_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.terraform_labels = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeForwardingRulesRulesEl {
    type O = BlockAssignable<DataComputeForwardingRulesRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeForwardingRulesRulesEl {}
impl BuildDataComputeForwardingRulesRulesEl {
    pub fn build(self) -> DataComputeForwardingRulesRulesEl {
        DataComputeForwardingRulesRulesEl {
            all_ports: core::default::Default::default(),
            allow_global_access: core::default::Default::default(),
            allow_psc_global_access: core::default::Default::default(),
            backend_service: core::default::Default::default(),
            base_forwarding_rule: core::default::Default::default(),
            creation_timestamp: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            forwarding_rule_id: core::default::Default::default(),
            ip_address: core::default::Default::default(),
            ip_collection: core::default::Default::default(),
            ip_protocol: core::default::Default::default(),
            ip_version: core::default::Default::default(),
            is_mirroring_collector: core::default::Default::default(),
            label_fingerprint: core::default::Default::default(),
            labels: core::default::Default::default(),
            load_balancing_scheme: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            no_automate_dns_zone: core::default::Default::default(),
            port_range: core::default::Default::default(),
            ports: core::default::Default::default(),
            project: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            psc_connection_status: core::default::Default::default(),
            recreate_closed_psc: core::default::Default::default(),
            region: core::default::Default::default(),
            self_link: core::default::Default::default(),
            service_directory_registrations: core::default::Default::default(),
            service_label: core::default::Default::default(),
            service_name: core::default::Default::default(),
            source_ip_ranges: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            target: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
        }
    }
}
pub struct DataComputeForwardingRulesRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeForwardingRulesRulesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeForwardingRulesRulesElRef {
        DataComputeForwardingRulesRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeForwardingRulesRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `all_ports` after provisioning.\n"]
    pub fn all_ports(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all_ports", self.base))
    }
    #[doc = "Get a reference to the value of field `allow_global_access` after provisioning.\n"]
    pub fn allow_global_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_global_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_psc_global_access` after provisioning.\n"]
    pub fn allow_psc_global_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_psc_global_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\n"]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `base_forwarding_rule` after provisioning.\n"]
    pub fn base_forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.base_forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\n"]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forwarding_rule_id` after provisioning.\n"]
    pub fn forwarding_rule_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forwarding_rule_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_collection` after provisioning.\n"]
    pub fn ip_collection(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_collection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\n"]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `ip_version` after provisioning.\n"]
    pub fn ip_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_version", self.base))
    }
    #[doc = "Get a reference to the value of field `is_mirroring_collector` after provisioning.\n"]
    pub fn is_mirroring_collector(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_mirroring_collector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `label_fingerprint` after provisioning.\n"]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_fingerprint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\n"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\n"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }
    #[doc = "Get a reference to the value of field `no_automate_dns_zone` after provisioning.\n"]
    pub fn no_automate_dns_zone(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_automate_dns_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_range", self.base))
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connection_status` after provisioning.\n"]
    pub fn psc_connection_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_status", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `recreate_closed_psc` after provisioning.\n"]
    pub fn recreate_closed_psc(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recreate_closed_psc", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `service_directory_registrations` after provisioning.\n"]
    pub fn service_directory_registrations(
        &self,
    ) -> ListRef<DataComputeForwardingRulesRulesElServiceDirectoryRegistrationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_registrations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_label` after provisioning.\n"]
    pub fn service_label(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_label", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `source_ip_ranges` after provisioning.\n"]
    pub fn source_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_ip_ranges", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
}
