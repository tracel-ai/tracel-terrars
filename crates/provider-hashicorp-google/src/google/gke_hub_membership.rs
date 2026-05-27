use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GkeHubMembershipData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    membership_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authority: Option<Vec<GkeHubMembershipAuthorityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<Vec<GkeHubMembershipEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<GkeHubMembershipTimeoutsEl>,
    dynamic: GkeHubMembershipDynamic,
}
struct GkeHubMembership_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GkeHubMembershipData>,
}
#[derive(Clone)]
pub struct GkeHubMembership(Rc<GkeHubMembership_>);
impl GkeHubMembership {
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
    #[doc = "Set the field `labels`.\nLabels to apply to this membership.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nLocation of the membership.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `authority`.\n"]
    pub fn set_authority(self, v: impl Into<BlockAssignable<GkeHubMembershipAuthorityEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authority = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authority = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(self, v: impl Into<BlockAssignable<GkeHubMembershipEndpointEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<GkeHubMembershipTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this membership.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the membership.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `membership_id` after provisioning.\nThe client-provided identifier of the membership."]
    pub fn membership_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.membership_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the membership."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authority` after provisioning.\n"]
    pub fn authority(&self) -> ListRef<GkeHubMembershipAuthorityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<GkeHubMembershipEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubMembershipTimeoutsElRef {
        GkeHubMembershipTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for GkeHubMembership {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GkeHubMembership {}
impl ToListMappable for GkeHubMembership {
    type O = ListRef<GkeHubMembershipRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GkeHubMembership_ {
    fn extract_resource_type(&self) -> String {
        "google_gke_hub_membership".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGkeHubMembership {
    pub tf_id: String,
    #[doc = "The client-provided identifier of the membership."]
    pub membership_id: PrimField<String>,
}
impl BuildGkeHubMembership {
    pub fn build(self, stack: &mut Stack) -> GkeHubMembership {
        let out = GkeHubMembership(Rc::new(GkeHubMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GkeHubMembershipData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                membership_id: self.membership_id,
                project: core::default::Default::default(),
                authority: core::default::Default::default(),
                endpoint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GkeHubMembershipRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubMembershipRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GkeHubMembershipRef {
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this membership.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation of the membership.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `membership_id` after provisioning.\nThe client-provided identifier of the membership."]
    pub fn membership_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.membership_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the membership."]
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
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authority` after provisioning.\n"]
    pub fn authority(&self) -> ListRef<GkeHubMembershipAuthorityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authority", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<GkeHubMembershipEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubMembershipTimeoutsElRef {
        GkeHubMembershipTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GkeHubMembershipAuthorityEl {
    issuer: PrimField<String>,
}
impl GkeHubMembershipAuthorityEl {}
impl ToListMappable for GkeHubMembershipAuthorityEl {
    type O = BlockAssignable<GkeHubMembershipAuthorityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubMembershipAuthorityEl {
    #[doc = "A JSON Web Token (JWT) issuer URI. 'issuer' must start with 'https://' and // be a valid\nwith length <2000 characters. For example: 'https://container.googleapis.com/v1/projects/my-project/locations/us-west1/clusters/my-cluster'. If the cluster is provisioned with Terraform, this is '\"https://container.googleapis.com/v1/${google_container_cluster.my-cluster.id}\"'."]
    pub issuer: PrimField<String>,
}
impl BuildGkeHubMembershipAuthorityEl {
    pub fn build(self) -> GkeHubMembershipAuthorityEl {
        GkeHubMembershipAuthorityEl {
            issuer: self.issuer,
        }
    }
}
pub struct GkeHubMembershipAuthorityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubMembershipAuthorityElRef {
    fn new(shared: StackShared, base: String) -> GkeHubMembershipAuthorityElRef {
        GkeHubMembershipAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubMembershipAuthorityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\nA JSON Web Token (JWT) issuer URI. 'issuer' must start with 'https://' and // be a valid\nwith length <2000 characters. For example: 'https://container.googleapis.com/v1/projects/my-project/locations/us-west1/clusters/my-cluster'. If the cluster is provisioned with Terraform, this is '\"https://container.googleapis.com/v1/${google_container_cluster.my-cluster.id}\"'."]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubMembershipEndpointElGkeClusterEl {
    resource_link: PrimField<String>,
}
impl GkeHubMembershipEndpointElGkeClusterEl {}
impl ToListMappable for GkeHubMembershipEndpointElGkeClusterEl {
    type O = BlockAssignable<GkeHubMembershipEndpointElGkeClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubMembershipEndpointElGkeClusterEl {
    #[doc = "Self-link of the GCP resource for the GKE cluster.\nFor example: '//container.googleapis.com/projects/my-project/locations/us-west1-a/clusters/my-cluster'.\nIt can be at the most 1000 characters in length. If the cluster is provisioned with Terraform,\nthis can be '\"//container.googleapis.com/${google_container_cluster.my-cluster.id}\"' or\n'google_container_cluster.my-cluster.id'."]
    pub resource_link: PrimField<String>,
}
impl BuildGkeHubMembershipEndpointElGkeClusterEl {
    pub fn build(self) -> GkeHubMembershipEndpointElGkeClusterEl {
        GkeHubMembershipEndpointElGkeClusterEl {
            resource_link: self.resource_link,
        }
    }
}
pub struct GkeHubMembershipEndpointElGkeClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubMembershipEndpointElGkeClusterElRef {
    fn new(shared: StackShared, base: String) -> GkeHubMembershipEndpointElGkeClusterElRef {
        GkeHubMembershipEndpointElGkeClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubMembershipEndpointElGkeClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_link` after provisioning.\nSelf-link of the GCP resource for the GKE cluster.\nFor example: '//container.googleapis.com/projects/my-project/locations/us-west1-a/clusters/my-cluster'.\nIt can be at the most 1000 characters in length. If the cluster is provisioned with Terraform,\nthis can be '\"//container.googleapis.com/${google_container_cluster.my-cluster.id}\"' or\n'google_container_cluster.my-cluster.id'."]
    pub fn resource_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_link", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GkeHubMembershipEndpointElDynamic {
    gke_cluster: Option<DynamicBlock<GkeHubMembershipEndpointElGkeClusterEl>>,
}
#[derive(Serialize)]
pub struct GkeHubMembershipEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_cluster: Option<Vec<GkeHubMembershipEndpointElGkeClusterEl>>,
    dynamic: GkeHubMembershipEndpointElDynamic,
}
impl GkeHubMembershipEndpointEl {
    #[doc = "Set the field `gke_cluster`.\n"]
    pub fn set_gke_cluster(
        mut self,
        v: impl Into<BlockAssignable<GkeHubMembershipEndpointElGkeClusterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.gke_cluster = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.gke_cluster = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GkeHubMembershipEndpointEl {
    type O = BlockAssignable<GkeHubMembershipEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubMembershipEndpointEl {}
impl BuildGkeHubMembershipEndpointEl {
    pub fn build(self) -> GkeHubMembershipEndpointEl {
        GkeHubMembershipEndpointEl {
            gke_cluster: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GkeHubMembershipEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubMembershipEndpointElRef {
    fn new(shared: StackShared, base: String) -> GkeHubMembershipEndpointElRef {
        GkeHubMembershipEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubMembershipEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gke_cluster` after provisioning.\n"]
    pub fn gke_cluster(&self) -> ListRef<GkeHubMembershipEndpointElGkeClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gke_cluster", self.base))
    }
}
#[derive(Serialize)]
pub struct GkeHubMembershipTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl GkeHubMembershipTimeoutsEl {
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
impl ToListMappable for GkeHubMembershipTimeoutsEl {
    type O = BlockAssignable<GkeHubMembershipTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGkeHubMembershipTimeoutsEl {}
impl BuildGkeHubMembershipTimeoutsEl {
    pub fn build(self) -> GkeHubMembershipTimeoutsEl {
        GkeHubMembershipTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct GkeHubMembershipTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GkeHubMembershipTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> GkeHubMembershipTimeoutsElRef {
        GkeHubMembershipTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GkeHubMembershipTimeoutsElRef {
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
struct GkeHubMembershipDynamic {
    authority: Option<DynamicBlock<GkeHubMembershipAuthorityEl>>,
    endpoint: Option<DynamicBlock<GkeHubMembershipEndpointEl>>,
}
