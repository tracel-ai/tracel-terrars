use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DnsResponsePolicyData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    response_policy_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_clusters: Option<Vec<DnsResponsePolicyGkeClustersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<DnsResponsePolicyNetworksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DnsResponsePolicyTimeoutsEl>,
    dynamic: DnsResponsePolicyDynamic,
}
struct DnsResponsePolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DnsResponsePolicyData>,
}
#[derive(Clone)]
pub struct DnsResponsePolicy(Rc<DnsResponsePolicy_>);
impl DnsResponsePolicy {
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
    #[doc = "Set the field `description`.\nThe description of the response policy, such as 'My new response policy'."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `gke_clusters`.\n"]
    pub fn set_gke_clusters(
        self,
        v: impl Into<BlockAssignable<DnsResponsePolicyGkeClustersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gke_clusters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gke_clusters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `networks`.\n"]
    pub fn set_networks(self, v: impl Into<BlockAssignable<DnsResponsePolicyNetworksEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().networks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.networks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DnsResponsePolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the response policy, such as 'My new response policy'."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `response_policy_name` after provisioning.\nThe user assigned name for this Response Policy, such as 'myresponsepolicy'."]
    pub fn response_policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_policy_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gke_clusters` after provisioning.\n"]
    pub fn gke_clusters(&self) -> ListRef<DnsResponsePolicyGkeClustersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gke_clusters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> ListRef<DnsResponsePolicyNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyTimeoutsElRef {
        DnsResponsePolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DnsResponsePolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DnsResponsePolicy {}
impl ToListMappable for DnsResponsePolicy {
    type O = ListRef<DnsResponsePolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DnsResponsePolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_dns_response_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDnsResponsePolicy {
    pub tf_id: String,
    #[doc = "The user assigned name for this Response Policy, such as 'myresponsepolicy'."]
    pub response_policy_name: PrimField<String>,
}
impl BuildDnsResponsePolicy {
    pub fn build(self, stack: &mut Stack) -> DnsResponsePolicy {
        let out = DnsResponsePolicy(Rc::new(DnsResponsePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DnsResponsePolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                response_policy_name: self.response_policy_name,
                gke_clusters: core::default::Default::default(),
                networks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DnsResponsePolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsResponsePolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DnsResponsePolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the response policy, such as 'My new response policy'."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `response_policy_name` after provisioning.\nThe user assigned name for this Response Policy, such as 'myresponsepolicy'."]
    pub fn response_policy_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_policy_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `gke_clusters` after provisioning.\n"]
    pub fn gke_clusters(&self) -> ListRef<DnsResponsePolicyGkeClustersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gke_clusters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `networks` after provisioning.\n"]
    pub fn networks(&self) -> ListRef<DnsResponsePolicyNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.networks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyTimeoutsElRef {
        DnsResponsePolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DnsResponsePolicyGkeClustersEl {
    gke_cluster_name: PrimField<String>,
}
impl DnsResponsePolicyGkeClustersEl {}
impl ToListMappable for DnsResponsePolicyGkeClustersEl {
    type O = BlockAssignable<DnsResponsePolicyGkeClustersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsResponsePolicyGkeClustersEl {
    #[doc = "The resource name of the cluster to bind this ManagedZone to.\nThis should be specified in the format like\n'projects/*/locations/*/clusters/*'"]
    pub gke_cluster_name: PrimField<String>,
}
impl BuildDnsResponsePolicyGkeClustersEl {
    pub fn build(self) -> DnsResponsePolicyGkeClustersEl {
        DnsResponsePolicyGkeClustersEl {
            gke_cluster_name: self.gke_cluster_name,
        }
    }
}
pub struct DnsResponsePolicyGkeClustersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsResponsePolicyGkeClustersElRef {
    fn new(shared: StackShared, base: String) -> DnsResponsePolicyGkeClustersElRef {
        DnsResponsePolicyGkeClustersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsResponsePolicyGkeClustersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gke_cluster_name` after provisioning.\nThe resource name of the cluster to bind this ManagedZone to.\nThis should be specified in the format like\n'projects/*/locations/*/clusters/*'"]
    pub fn gke_cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gke_cluster_name", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DnsResponsePolicyNetworksEl {
    network_url: PrimField<String>,
}
impl DnsResponsePolicyNetworksEl {}
impl ToListMappable for DnsResponsePolicyNetworksEl {
    type O = BlockAssignable<DnsResponsePolicyNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsResponsePolicyNetworksEl {
    #[doc = "The fully qualified URL of the VPC network to bind to.\nThis should be formatted like\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub network_url: PrimField<String>,
}
impl BuildDnsResponsePolicyNetworksEl {
    pub fn build(self) -> DnsResponsePolicyNetworksEl {
        DnsResponsePolicyNetworksEl {
            network_url: self.network_url,
        }
    }
}
pub struct DnsResponsePolicyNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsResponsePolicyNetworksElRef {
    fn new(shared: StackShared, base: String) -> DnsResponsePolicyNetworksElRef {
        DnsResponsePolicyNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsResponsePolicyNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_url` after provisioning.\nThe fully qualified URL of the VPC network to bind to.\nThis should be formatted like\n'https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}'"]
    pub fn network_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_url", self.base))
    }
}
#[derive(Serialize)]
pub struct DnsResponsePolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DnsResponsePolicyTimeoutsEl {
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
impl ToListMappable for DnsResponsePolicyTimeoutsEl {
    type O = BlockAssignable<DnsResponsePolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDnsResponsePolicyTimeoutsEl {}
impl BuildDnsResponsePolicyTimeoutsEl {
    pub fn build(self) -> DnsResponsePolicyTimeoutsEl {
        DnsResponsePolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DnsResponsePolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DnsResponsePolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DnsResponsePolicyTimeoutsElRef {
        DnsResponsePolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DnsResponsePolicyTimeoutsElRef {
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
struct DnsResponsePolicyDynamic {
    gke_clusters: Option<DynamicBlock<DnsResponsePolicyGkeClustersEl>>,
    networks: Option<DynamicBlock<DnsResponsePolicyNetworksEl>>,
}
