use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ActiveDirectoryDomainData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorized_networks: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection: Option<PrimField<bool>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    locations: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    reserved_ip_range: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ActiveDirectoryDomainTimeoutsEl>,
}
struct ActiveDirectoryDomain_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ActiveDirectoryDomainData>,
}
#[derive(Clone)]
pub struct ActiveDirectoryDomain(Rc<ActiveDirectoryDomain_>);
impl ActiveDirectoryDomain {
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
    #[doc = "Set the field `admin`.\nThe name of delegated administrator account used to perform Active Directory operations.\nIf not specified, setupadmin will be used."]
    pub fn set_admin(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().admin = Some(v.into());
        self
    }
    #[doc = "Set the field `authorized_networks`.\nThe full names of the Google Compute Engine networks the domain instance is connected to. The domain is only available on networks listed in authorizedNetworks.\nIf CIDR subnets overlap between networks, domain creation will fail."]
    pub fn set_authorized_networks(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().authorized_networks = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the domain. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the domain,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the domain will fail.\nWhen the field is set to false, deleting the domain is allowed."]
    pub fn set_deletion_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nResource labels that can contain user-provided metadata\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ActiveDirectoryDomainTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `admin` after provisioning.\nThe name of delegated administrator account used to perform Active Directory operations.\nIf not specified, setupadmin will be used."]
    pub fn admin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_networks` after provisioning.\nThe full names of the Google Compute Engine networks the domain instance is connected to. The domain is only available on networks listed in authorizedNetworks.\nIf CIDR subnets overlap between networks, domain creation will fail."]
    pub fn authorized_networks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.authorized_networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the domain. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the domain,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the domain will fail.\nWhen the field is set to false, deleting the domain is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\nThe fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions\nof https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains."]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\nThe fully-qualified domain name of the exposed domain used by clients to connect to the service.\nSimilar to what would be chosen for an Active Directory set up on an internal network."]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fqdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels that can contain user-provided metadata\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nLocations where domain needs to be provisioned. [regions][compute/docs/regions-zones/]\ne.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique name of the domain using the format: 'projects/{project}/locations/global/domains/{domainName}'."]
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
    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\nThe CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger.\nRanges must be unique and non-overlapping with existing subnets in authorizedNetworks"]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ActiveDirectoryDomainTimeoutsElRef {
        ActiveDirectoryDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ActiveDirectoryDomain {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ActiveDirectoryDomain {}
impl ToListMappable for ActiveDirectoryDomain {
    type O = ListRef<ActiveDirectoryDomainRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ActiveDirectoryDomain_ {
    fn extract_resource_type(&self) -> String {
        "google_active_directory_domain".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildActiveDirectoryDomain {
    pub tf_id: String,
    #[doc = "The fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions\nof https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains."]
    pub domain_name: PrimField<String>,
    #[doc = "Locations where domain needs to be provisioned. [regions][compute/docs/regions-zones/]\ne.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block."]
    pub locations: ListField<PrimField<String>>,
    #[doc = "The CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger.\nRanges must be unique and non-overlapping with existing subnets in authorizedNetworks"]
    pub reserved_ip_range: PrimField<String>,
}
impl BuildActiveDirectoryDomain {
    pub fn build(self, stack: &mut Stack) -> ActiveDirectoryDomain {
        let out = ActiveDirectoryDomain(Rc::new(ActiveDirectoryDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ActiveDirectoryDomainData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                admin: core::default::Default::default(),
                authorized_networks: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                locations: self.locations,
                project: core::default::Default::default(),
                reserved_ip_range: self.reserved_ip_range,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ActiveDirectoryDomainRef {
    shared: StackShared,
    base: String,
}
impl Ref for ActiveDirectoryDomainRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ActiveDirectoryDomainRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `admin` after provisioning.\nThe name of delegated administrator account used to perform Active Directory operations.\nIf not specified, setupadmin will be used."]
    pub fn admin(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.admin", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authorized_networks` after provisioning.\nThe full names of the Google Compute Engine networks the domain instance is connected to. The domain is only available on networks listed in authorizedNetworks.\nIf CIDR subnets overlap between networks, domain creation will fail."]
    pub fn authorized_networks(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.authorized_networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the domain. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the domain,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the domain will fail.\nWhen the field is set to false, deleting the domain is allowed."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\nThe fully qualified domain name. e.g. mydomain.myorganization.com, with the restrictions\nof https://cloud.google.com/managed-microsoft-ad/reference/rest/v1/projects.locations.global.domains."]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fqdn` after provisioning.\nThe fully-qualified domain name of the exposed domain used by clients to connect to the service.\nSimilar to what would be chosen for an Active Directory set up on an internal network."]
    pub fn fqdn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fqdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nResource labels that can contain user-provided metadata\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `locations` after provisioning.\nLocations where domain needs to be provisioned. [regions][compute/docs/regions-zones/]\ne.g. us-west1 or us-east4 Service supports up to 4 locations at once. Each location will use a /26 block."]
    pub fn locations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.locations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique name of the domain using the format: 'projects/{project}/locations/global/domains/{domainName}'."]
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
    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\nThe CIDR range of internal addresses that are reserved for this domain. Reserved networks must be /24 or larger.\nRanges must be unique and non-overlapping with existing subnets in authorizedNetworks"]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ActiveDirectoryDomainTimeoutsElRef {
        ActiveDirectoryDomainTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ActiveDirectoryDomainTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ActiveDirectoryDomainTimeoutsEl {
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
impl ToListMappable for ActiveDirectoryDomainTimeoutsEl {
    type O = BlockAssignable<ActiveDirectoryDomainTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildActiveDirectoryDomainTimeoutsEl {}
impl BuildActiveDirectoryDomainTimeoutsEl {
    pub fn build(self) -> ActiveDirectoryDomainTimeoutsEl {
        ActiveDirectoryDomainTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ActiveDirectoryDomainTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ActiveDirectoryDomainTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ActiveDirectoryDomainTimeoutsElRef {
        ActiveDirectoryDomainTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ActiveDirectoryDomainTimeoutsElRef {
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
