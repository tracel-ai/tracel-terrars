use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRegionCompositeHealthCheckData {
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
    health_destination: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_sources: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionCompositeHealthCheckTimeoutsEl>,
}
struct ComputeRegionCompositeHealthCheck_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionCompositeHealthCheckData>,
}
#[derive(Clone)]
pub struct ComputeRegionCompositeHealthCheck(Rc<ComputeRegionCompositeHealthCheck_>);
impl ComputeRegionCompositeHealthCheck {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when you\ncreate the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `health_sources`.\nURLs to the HealthSource resources whose results are AND'ed.\nI.e. he aggregated result is is HEALTHY only if all sources\nare HEALTHY. Must have at least 1. Must not have more than 10.\nMust be regional and in the same region as the\nCompositeHealthCheck. Can be mutated."]
    pub fn set_health_sources(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().health_sources = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionCompositeHealthCheckTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you\ncreate the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this object.\nThis field is used in optimistic locking. This field will be ignored when\ninserting a 'CompositeHealthCheck'. An up-to-date fingerprint\nmust be provided in order to patch the CompositeHealthCheck; Otherwise,\nthe request will fail with error '412 conditionNotMet'. To see\nthe latest fingerprint, make a 'get()' request to retrieve the\nCompositeHealthCheck."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_destination` after provisioning.\nURL to the destination resource. Must be set. Must be a\nForwardingRule. The ForwardingRule must have\nload balancing scheme INTERNAL or\nINTERNAL_MANAGED and must be regional and in the same region\nas the CompositeHealthCheck (cross-region deployment for\nINTERNAL_MANAGED is not supported). Can be mutated."]
    pub fn health_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_sources` after provisioning.\nURLs to the HealthSource resources whose results are AND'ed.\nI.e. he aggregated result is is HEALTHY only if all sources\nare HEALTHY. Must have at least 1. Must not have more than 10.\nMust be regional and in the same region as the\nCompositeHealthCheck. Can be mutated."]
    pub fn health_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_sources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular\nexpression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit, except the last character, which\ncannot be a dash."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nURL of the region where the composite health check resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL with id for the resource."]
    pub fn self_link_with_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionCompositeHealthCheckTimeoutsElRef {
        ComputeRegionCompositeHealthCheckTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRegionCompositeHealthCheck {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRegionCompositeHealthCheck {}
impl ToListMappable for ComputeRegionCompositeHealthCheck {
    type O = ListRef<ComputeRegionCompositeHealthCheckRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRegionCompositeHealthCheck_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_composite_health_check".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRegionCompositeHealthCheck {
    pub tf_id: String,
    #[doc = "URL to the destination resource. Must be set. Must be a\nForwardingRule. The ForwardingRule must have\nload balancing scheme INTERNAL or\nINTERNAL_MANAGED and must be regional and in the same region\nas the CompositeHealthCheck (cross-region deployment for\nINTERNAL_MANAGED is not supported). Can be mutated."]
    pub health_destination: PrimField<String>,
    #[doc = "Name of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular\nexpression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit, except the last character, which\ncannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "URL of the region where the composite health check resides."]
    pub region: PrimField<String>,
}
impl BuildComputeRegionCompositeHealthCheck {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionCompositeHealthCheck {
        let out = ComputeRegionCompositeHealthCheck(Rc::new(ComputeRegionCompositeHealthCheck_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionCompositeHealthCheckData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                health_destination: self.health_destination,
                health_sources: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: self.region,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRegionCompositeHealthCheckRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionCompositeHealthCheckRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRegionCompositeHealthCheckRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when you\ncreate the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. A hash of the contents stored in this object.\nThis field is used in optimistic locking. This field will be ignored when\ninserting a 'CompositeHealthCheck'. An up-to-date fingerprint\nmust be provided in order to patch the CompositeHealthCheck; Otherwise,\nthe request will fail with error '412 conditionNotMet'. To see\nthe latest fingerprint, make a 'get()' request to retrieve the\nCompositeHealthCheck."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_destination` after provisioning.\nURL to the destination resource. Must be set. Must be a\nForwardingRule. The ForwardingRule must have\nload balancing scheme INTERNAL or\nINTERNAL_MANAGED and must be regional and in the same region\nas the CompositeHealthCheck (cross-region deployment for\nINTERNAL_MANAGED is not supported). Can be mutated."]
    pub fn health_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_destination", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_sources` after provisioning.\nURLs to the HealthSource resources whose results are AND'ed.\nI.e. he aggregated result is is HEALTHY only if all sources\nare HEALTHY. Must have at least 1. Must not have more than 10.\nMust be regional and in the same region as the\nCompositeHealthCheck. Can be mutated."]
    pub fn health_sources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_sources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is created.\nThe name must be 1-63 characters long, and comply with RFC1035.\nSpecifically, the name must be 1-63 characters long and match the regular\nexpression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the first\ncharacter must be a lowercase letter, and all following characters must\nbe a dash, lowercase letter, or digit, except the last character, which\ncannot be a dash."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nURL of the region where the composite health check resides."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL with id for the resource."]
    pub fn self_link_with_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link_with_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionCompositeHealthCheckTimeoutsElRef {
        ComputeRegionCompositeHealthCheckTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionCompositeHealthCheckTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRegionCompositeHealthCheckTimeoutsEl {
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
impl ToListMappable for ComputeRegionCompositeHealthCheckTimeoutsEl {
    type O = BlockAssignable<ComputeRegionCompositeHealthCheckTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionCompositeHealthCheckTimeoutsEl {}
impl BuildComputeRegionCompositeHealthCheckTimeoutsEl {
    pub fn build(self) -> ComputeRegionCompositeHealthCheckTimeoutsEl {
        ComputeRegionCompositeHealthCheckTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionCompositeHealthCheckTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionCompositeHealthCheckTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionCompositeHealthCheckTimeoutsElRef {
        ComputeRegionCompositeHealthCheckTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionCompositeHealthCheckTimeoutsElRef {
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
