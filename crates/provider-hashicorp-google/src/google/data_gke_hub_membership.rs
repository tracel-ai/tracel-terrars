use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataGkeHubMembershipData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    membership_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataGkeHubMembership_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataGkeHubMembershipData>,
}
#[derive(Clone)]
pub struct DataGkeHubMembership(Rc<DataGkeHubMembership_>);
impl DataGkeHubMembership {
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
    #[doc = "Get a reference to the value of field `authority` after provisioning.\nAuthority encodes how Google will recognize identities from this Membership.\nSee the workload identity documentation for more details:\nhttps://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity"]
    pub fn authority(&self) -> ListRef<DataGkeHubMembershipAuthorityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authority", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nIf this Membership is a Kubernetes API server hosted on GKE, this is a self link to its GCP resource."]
    pub fn endpoint(&self) -> ListRef<DataGkeHubMembershipEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
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
}
impl Referable for DataGkeHubMembership {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataGkeHubMembership {}
impl ToListMappable for DataGkeHubMembership {
    type O = ListRef<DataGkeHubMembershipRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataGkeHubMembership_ {
    fn extract_datasource_type(&self) -> String {
        "google_gke_hub_membership".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataGkeHubMembership {
    pub tf_id: String,
    #[doc = "Location of the membership.\nThe default value is 'global'."]
    pub location: PrimField<String>,
    #[doc = "The client-provided identifier of the membership."]
    pub membership_id: PrimField<String>,
}
impl BuildDataGkeHubMembership {
    pub fn build(self, stack: &mut Stack) -> DataGkeHubMembership {
        let out = DataGkeHubMembership(Rc::new(DataGkeHubMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataGkeHubMembershipData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                membership_id: self.membership_id,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataGkeHubMembershipRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubMembershipRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataGkeHubMembershipRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `authority` after provisioning.\nAuthority encodes how Google will recognize identities from this Membership.\nSee the workload identity documentation for more details:\nhttps://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity"]
    pub fn authority(&self) -> ListRef<DataGkeHubMembershipAuthorityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authority", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nIf this Membership is a Kubernetes API server hosted on GKE, this is a self link to its GCP resource."]
    pub fn endpoint(&self) -> ListRef<DataGkeHubMembershipEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataGkeHubMembershipAuthorityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
}
impl DataGkeHubMembershipAuthorityEl {
    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubMembershipAuthorityEl {
    type O = BlockAssignable<DataGkeHubMembershipAuthorityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubMembershipAuthorityEl {}
impl BuildDataGkeHubMembershipAuthorityEl {
    pub fn build(self) -> DataGkeHubMembershipAuthorityEl {
        DataGkeHubMembershipAuthorityEl {
            issuer: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubMembershipAuthorityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubMembershipAuthorityElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubMembershipAuthorityElRef {
        DataGkeHubMembershipAuthorityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubMembershipAuthorityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }
}
#[derive(Serialize)]
pub struct DataGkeHubMembershipEndpointElGkeClusterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_link: Option<PrimField<String>>,
}
impl DataGkeHubMembershipEndpointElGkeClusterEl {
    #[doc = "Set the field `resource_link`.\n"]
    pub fn set_resource_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_link = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubMembershipEndpointElGkeClusterEl {
    type O = BlockAssignable<DataGkeHubMembershipEndpointElGkeClusterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubMembershipEndpointElGkeClusterEl {}
impl BuildDataGkeHubMembershipEndpointElGkeClusterEl {
    pub fn build(self) -> DataGkeHubMembershipEndpointElGkeClusterEl {
        DataGkeHubMembershipEndpointElGkeClusterEl {
            resource_link: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubMembershipEndpointElGkeClusterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubMembershipEndpointElGkeClusterElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubMembershipEndpointElGkeClusterElRef {
        DataGkeHubMembershipEndpointElGkeClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubMembershipEndpointElGkeClusterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_link` after provisioning.\n"]
    pub fn resource_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_link", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataGkeHubMembershipEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    gke_cluster: Option<ListField<DataGkeHubMembershipEndpointElGkeClusterEl>>,
}
impl DataGkeHubMembershipEndpointEl {
    #[doc = "Set the field `gke_cluster`.\n"]
    pub fn set_gke_cluster(
        mut self,
        v: impl Into<ListField<DataGkeHubMembershipEndpointElGkeClusterEl>>,
    ) -> Self {
        self.gke_cluster = Some(v.into());
        self
    }
}
impl ToListMappable for DataGkeHubMembershipEndpointEl {
    type O = BlockAssignable<DataGkeHubMembershipEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataGkeHubMembershipEndpointEl {}
impl BuildDataGkeHubMembershipEndpointEl {
    pub fn build(self) -> DataGkeHubMembershipEndpointEl {
        DataGkeHubMembershipEndpointEl {
            gke_cluster: core::default::Default::default(),
        }
    }
}
pub struct DataGkeHubMembershipEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataGkeHubMembershipEndpointElRef {
    fn new(shared: StackShared, base: String) -> DataGkeHubMembershipEndpointElRef {
        DataGkeHubMembershipEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataGkeHubMembershipEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `gke_cluster` after provisioning.\n"]
    pub fn gke_cluster(&self) -> ListRef<DataGkeHubMembershipEndpointElGkeClusterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.gke_cluster", self.base))
    }
}
