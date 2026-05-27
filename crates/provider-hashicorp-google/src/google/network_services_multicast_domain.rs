use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesMulticastDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    admin_network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multicast_domain_group: Option<PrimField<String>>,
    multicast_domain_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_config: Option<Vec<NetworkServicesMulticastDomainConnectionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesMulticastDomainTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ull_multicast_domain: Option<Vec<NetworkServicesMulticastDomainUllMulticastDomainEl>>,
    dynamic: NetworkServicesMulticastDomainDynamic,
}
struct NetworkServicesMulticastDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesMulticastDomainData>,
}
#[derive(Clone)]
pub struct NetworkServicesMulticastDomain(Rc<NetworkServicesMulticastDomain_>);
impl NetworkServicesMulticastDomain {
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
    #[doc = "Set the field `description`.\nAn optional text description of the multicast domain."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels as key-value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `multicast_domain_group`.\nThe multicast domain group this domain should be associated with.\nUse the following format:\n'projects/{project}/locations/global/multicastDomainGroups/{multicast_domain_group}'."]
    pub fn set_multicast_domain_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().multicast_domain_group = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_config`.\n"]
    pub fn set_connection_config(
        self,
        v: impl Into<BlockAssignable<NetworkServicesMulticastDomainConnectionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesMulticastDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `ull_multicast_domain`.\n"]
    pub fn set_ull_multicast_domain(
        self,
        v: impl Into<BlockAssignable<NetworkServicesMulticastDomainUllMulticastDomainEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ull_multicast_domain = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ull_multicast_domain = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `admin_network` after provisioning.\nThe resource name of the multicast admin VPC network.\nUse the following format:\n'projects/{project}/locations/global/networks/{network}'."]
    pub fn admin_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the multicast domain was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional text description of the multicast domain."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key-value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_domain_group` after provisioning.\nThe multicast domain group this domain should be associated with.\nUse the following format:\n'projects/{project}/locations/global/multicastDomainGroups/{multicast_domain_group}'."]
    pub fn multicast_domain_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_domain_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_domain_id` after provisioning.\nA unique name for the multicast domain.\nThe name is restricted to letters, numbers, and hyphen, with the first\ncharacter a letter, and the last a letter or a number. The name must not\nexceed 48 characters."]
    pub fn multicast_domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the multicast domain.\nUse the following format:\n'projects/*/locations/global/multicastDomains/*'"]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe multicast resource's state."]
    pub fn state(&self) -> ListRef<NetworkServicesMulticastDomainStateElRef> {
        ListRef::new(
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
    #[doc = "Get a reference to the value of field `unique_id` after provisioning.\nThe Google-generated UUID for the resource. This value is\nunique across all multicast domain resources. If a domain is deleted and\nanother with the same name is created, the new domain is assigned a\ndifferent unique_id."]
    pub fn unique_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the multicast domain was most recently\nupdated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_config` after provisioning.\n"]
    pub fn connection_config(
        &self,
    ) -> ListRef<NetworkServicesMulticastDomainConnectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesMulticastDomainTimeoutsElRef {
        NetworkServicesMulticastDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ull_multicast_domain` after provisioning.\n"]
    pub fn ull_multicast_domain(
        &self,
    ) -> ListRef<NetworkServicesMulticastDomainUllMulticastDomainElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ull_multicast_domain", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesMulticastDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesMulticastDomain {}
impl ToListMappable for NetworkServicesMulticastDomain {
    type O = ListRef<NetworkServicesMulticastDomainRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesMulticastDomain_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_multicast_domain".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesMulticastDomain {
    pub tf_id: String,
    #[doc = "The resource name of the multicast admin VPC network.\nUse the following format:\n'projects/{project}/locations/global/networks/{network}'."]
    pub admin_network: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "A unique name for the multicast domain.\nThe name is restricted to letters, numbers, and hyphen, with the first\ncharacter a letter, and the last a letter or a number. The name must not\nexceed 48 characters."]
    pub multicast_domain_id: PrimField<String>,
}
impl BuildNetworkServicesMulticastDomain {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesMulticastDomain {
        let out = NetworkServicesMulticastDomain(Rc::new(NetworkServicesMulticastDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesMulticastDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin_network: self.admin_network,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                multicast_domain_group: core::default::Default::default(),
                multicast_domain_id: self.multicast_domain_id,
                project: core::default::Default::default(),
                connection_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                ull_multicast_domain: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesMulticastDomainRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesMulticastDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesMulticastDomainRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin_network` after provisioning.\nThe resource name of the multicast admin VPC network.\nUse the following format:\n'projects/{project}/locations/global/networks/{network}'."]
    pub fn admin_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin_network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the multicast domain was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional text description of the multicast domain."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key-value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_domain_group` after provisioning.\nThe multicast domain group this domain should be associated with.\nUse the following format:\n'projects/{project}/locations/global/multicastDomainGroups/{multicast_domain_group}'."]
    pub fn multicast_domain_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_domain_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `multicast_domain_id` after provisioning.\nA unique name for the multicast domain.\nThe name is restricted to letters, numbers, and hyphen, with the first\ncharacter a letter, and the last a letter or a number. The name must not\nexceed 48 characters."]
    pub fn multicast_domain_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.multicast_domain_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the multicast domain.\nUse the following format:\n'projects/*/locations/global/multicastDomains/*'"]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe multicast resource's state."]
    pub fn state(&self) -> ListRef<NetworkServicesMulticastDomainStateElRef> {
        ListRef::new(
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
    #[doc = "Get a reference to the value of field `unique_id` after provisioning.\nThe Google-generated UUID for the resource. This value is\nunique across all multicast domain resources. If a domain is deleted and\nanother with the same name is created, the new domain is assigned a\ndifferent unique_id."]
    pub fn unique_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unique_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the multicast domain was most recently\nupdated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_config` after provisioning.\n"]
    pub fn connection_config(
        &self,
    ) -> ListRef<NetworkServicesMulticastDomainConnectionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesMulticastDomainTimeoutsElRef {
        NetworkServicesMulticastDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ull_multicast_domain` after provisioning.\n"]
    pub fn ull_multicast_domain(
        &self,
    ) -> ListRef<NetworkServicesMulticastDomainUllMulticastDomainElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ull_multicast_domain", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesMulticastDomainStateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl NetworkServicesMulticastDomainStateEl {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesMulticastDomainStateEl {
    type O = BlockAssignable<NetworkServicesMulticastDomainStateEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesMulticastDomainStateEl {}
impl BuildNetworkServicesMulticastDomainStateEl {
    pub fn build(self) -> NetworkServicesMulticastDomainStateEl {
        NetworkServicesMulticastDomainStateEl {
            state: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesMulticastDomainStateElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesMulticastDomainStateElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesMulticastDomainStateElRef {
        NetworkServicesMulticastDomainStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesMulticastDomainStateElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesMulticastDomainConnectionConfigEl {
    connection_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ncc_hub: Option<PrimField<String>>,
}
impl NetworkServicesMulticastDomainConnectionConfigEl {
    #[doc = "Set the field `ncc_hub`.\nThe resource name of the\n[NCC](https://cloud.google.com/network-connectivity-center) hub.\nUse the following format:\n'projects/{project}/locations/global/hubs/{hub}'."]
    pub fn set_ncc_hub(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ncc_hub = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesMulticastDomainConnectionConfigEl {
    type O = BlockAssignable<NetworkServicesMulticastDomainConnectionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesMulticastDomainConnectionConfigEl {
    #[doc = "The VPC connection type.\nPossible values:\nNCC\nSAME_VPC"]
    pub connection_type: PrimField<String>,
}
impl BuildNetworkServicesMulticastDomainConnectionConfigEl {
    pub fn build(self) -> NetworkServicesMulticastDomainConnectionConfigEl {
        NetworkServicesMulticastDomainConnectionConfigEl {
            connection_type: self.connection_type,
            ncc_hub: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesMulticastDomainConnectionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesMulticastDomainConnectionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesMulticastDomainConnectionConfigElRef {
        NetworkServicesMulticastDomainConnectionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesMulticastDomainConnectionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\nThe VPC connection type.\nPossible values:\nNCC\nSAME_VPC"]
    pub fn connection_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ncc_hub` after provisioning.\nThe resource name of the\n[NCC](https://cloud.google.com/network-connectivity-center) hub.\nUse the following format:\n'projects/{project}/locations/global/hubs/{hub}'."]
    pub fn ncc_hub(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ncc_hub", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesMulticastDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesMulticastDomainTimeoutsEl {
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
impl ToListMappable for NetworkServicesMulticastDomainTimeoutsEl {
    type O = BlockAssignable<NetworkServicesMulticastDomainTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesMulticastDomainTimeoutsEl {}
impl BuildNetworkServicesMulticastDomainTimeoutsEl {
    pub fn build(self) -> NetworkServicesMulticastDomainTimeoutsEl {
        NetworkServicesMulticastDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesMulticastDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesMulticastDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesMulticastDomainTimeoutsElRef {
        NetworkServicesMulticastDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesMulticastDomainTimeoutsElRef {
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
pub struct NetworkServicesMulticastDomainUllMulticastDomainEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    preconfigured_ull_domain: Option<PrimField<String>>,
}
impl NetworkServicesMulticastDomainUllMulticastDomainEl {
    #[doc = "Set the field `preconfigured_ull_domain`.\nThe preconfigured Ultra-Low-Latency domain name."]
    pub fn set_preconfigured_ull_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preconfigured_ull_domain = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesMulticastDomainUllMulticastDomainEl {
    type O = BlockAssignable<NetworkServicesMulticastDomainUllMulticastDomainEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesMulticastDomainUllMulticastDomainEl {}
impl BuildNetworkServicesMulticastDomainUllMulticastDomainEl {
    pub fn build(self) -> NetworkServicesMulticastDomainUllMulticastDomainEl {
        NetworkServicesMulticastDomainUllMulticastDomainEl {
            preconfigured_ull_domain: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesMulticastDomainUllMulticastDomainElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesMulticastDomainUllMulticastDomainElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesMulticastDomainUllMulticastDomainElRef {
        NetworkServicesMulticastDomainUllMulticastDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesMulticastDomainUllMulticastDomainElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `preconfigured_ull_domain` after provisioning.\nThe preconfigured Ultra-Low-Latency domain name."]
    pub fn preconfigured_ull_domain(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preconfigured_ull_domain", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesMulticastDomainDynamic {
    connection_config: Option<DynamicBlock<NetworkServicesMulticastDomainConnectionConfigEl>>,
    ull_multicast_domain: Option<DynamicBlock<NetworkServicesMulticastDomainUllMulticastDomainEl>>,
}
