use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesGrpcRouteData {
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
    gateways: Option<ListField<PrimField<String>>>,
    hostnames: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meshes: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<NetworkServicesGrpcRouteRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesGrpcRouteTimeoutsEl>,
    dynamic: NetworkServicesGrpcRouteDynamic,
}
struct NetworkServicesGrpcRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesGrpcRouteData>,
}
#[derive(Clone)]
pub struct NetworkServicesGrpcRoute(Rc<NetworkServicesGrpcRoute_>);
impl NetworkServicesGrpcRoute {
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
    #[doc = "Set the field `description`.\nA free-text description of the resource. Max length 1024 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `gateways`.\nList of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway."]
    pub fn set_gateways(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().gateways = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of label tags associated with the GrpcRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nLocation (region) of the GRPCRoute resource to be created. Only the value 'global' is currently allowed; defaults to 'global' if omitted."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `meshes`.\nList of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh."]
    pub fn set_meshes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().meshes = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<NetworkServicesGrpcRouteRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NetworkServicesGrpcRouteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the GrpcRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
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
    #[doc = "Get a reference to the value of field `gateways` after provisioning.\nList of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway."]
    pub fn gateways(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateways", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hostnames` after provisioning.\nRequired. Service hostnames with an optional port for which this route describes traffic."]
    pub fn hostnames(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hostnames", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the GrpcRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation (region) of the GRPCRoute resource to be created. Only the value 'global' is currently allowed; defaults to 'global' if omitted."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meshes` after provisioning.\nList of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh."]
    pub fn meshes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meshes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the GrpcRoute resource."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the GrpcRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<NetworkServicesGrpcRouteRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesGrpcRouteTimeoutsElRef {
        NetworkServicesGrpcRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesGrpcRoute {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesGrpcRoute {}
impl ToListMappable for NetworkServicesGrpcRoute {
    type O = ListRef<NetworkServicesGrpcRouteRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesGrpcRoute_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_grpc_route".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesGrpcRoute {
    pub tf_id: String,
    #[doc = "Required. Service hostnames with an optional port for which this route describes traffic."]
    pub hostnames: ListField<PrimField<String>>,
    #[doc = "Name of the GrpcRoute resource."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesGrpcRoute {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesGrpcRoute {
        let out = NetworkServicesGrpcRoute(Rc::new(NetworkServicesGrpcRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesGrpcRouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                gateways: core::default::Default::default(),
                hostnames: self.hostnames,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                meshes: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkServicesGrpcRouteRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesGrpcRouteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the GrpcRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA free-text description of the resource. Max length 1024 characters."]
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
    #[doc = "Get a reference to the value of field `gateways` after provisioning.\nList of gateways this GrpcRoute is attached to, as one of the routing rules to route the requests served by the gateway."]
    pub fn gateways(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateways", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hostnames` after provisioning.\nRequired. Service hostnames with an optional port for which this route describes traffic."]
    pub fn hostnames(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hostnames", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the GrpcRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation (region) of the GRPCRoute resource to be created. Only the value 'global' is currently allowed; defaults to 'global' if omitted."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meshes` after provisioning.\nList of meshes this GrpcRoute is attached to, as one of the routing rules to route the requests served by the mesh."]
    pub fn meshes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meshes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the GrpcRoute resource."]
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
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL of this resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the GrpcRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<NetworkServicesGrpcRouteRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesGrpcRouteTimeoutsElRef {
        NetworkServicesGrpcRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesElActionElDestinationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl NetworkServicesGrpcRouteRulesElActionElDestinationsEl {
    #[doc = "Set the field `service_name`.\nThe URL of a BackendService to route traffic to."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field."]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesElActionElDestinationsEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesElActionElDestinationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesElActionElDestinationsEl {}
impl BuildNetworkServicesGrpcRouteRulesElActionElDestinationsEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesElActionElDestinationsEl {
        NetworkServicesGrpcRouteRulesElActionElDestinationsEl {
            service_name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElActionElDestinationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElActionElDestinationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesGrpcRouteRulesElActionElDestinationsElRef {
        NetworkServicesGrpcRouteRulesElActionElDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElActionElDestinationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe URL of a BackendService to route traffic to."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_status: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}
impl NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl {
    #[doc = "Set the field `http_status`.\nThe HTTP status code used to abort the request."]
    pub fn set_http_status(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_status = Some(v.into());
        self
    }
    #[doc = "Set the field `percentage`.\nThe percentage of traffic which will be aborted."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl {}
impl BuildNetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl {
        NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
        NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request."]
    pub fn http_status(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic which will be aborted."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}
impl NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl {
    #[doc = "Set the field `fixed_delay`.\nSpecify a fixed delay before forwarding the request."]
    pub fn set_fixed_delay(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fixed_delay = Some(v.into());
        self
    }
    #[doc = "Set the field `percentage`.\nThe percentage of traffic on which delay will be injected."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl {}
impl BuildNetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl {
        NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl {
            fixed_delay: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
        NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed_delay` after provisioning.\nSpecify a fixed delay before forwarding the request."]
    pub fn fixed_delay(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic on which delay will be injected."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDynamic {
    abort:
        Option<DynamicBlock<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl>>,
    delay:
        Option<DynamicBlock<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<Vec<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<Vec<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDynamic,
}
impl NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.abort = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.abort = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `delay`.\n"]
    pub fn set_delay(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.delay = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.delay = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl {}
impl BuildNetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl {
        NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElRef {
        NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> ListRef<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElAbortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }
    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> ListRef<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesElActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
}
impl NetworkServicesGrpcRouteRulesElActionElRetryPolicyEl {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number of retries."]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry policy applies. Possible values: [\"connect-failure\", \"refused-stream\", \"cancelled\", \"deadline-exceeded\", \"resource-exhausted\", \"unavailable\"]"]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesElActionElRetryPolicyEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesElActionElRetryPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesElActionElRetryPolicyEl {}
impl BuildNetworkServicesGrpcRouteRulesElActionElRetryPolicyEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesElActionElRetryPolicyEl {
        NetworkServicesGrpcRouteRulesElActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElActionElRetryPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesGrpcRouteRulesElActionElRetryPolicyElRef {
        NetworkServicesGrpcRouteRulesElActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number of retries."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry policy applies. Possible values: [\"connect-failure\", \"refused-stream\", \"cancelled\", \"deadline-exceeded\", \"resource-exhausted\", \"unavailable\"]"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_conditions", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesGrpcRouteRulesElActionElDynamic {
    destinations: Option<DynamicBlock<NetworkServicesGrpcRouteRulesElActionElDestinationsEl>>,
    fault_injection_policy:
        Option<DynamicBlock<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl>>,
    retry_policy: Option<DynamicBlock<NetworkServicesGrpcRouteRulesElActionElRetryPolicyEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<Vec<NetworkServicesGrpcRouteRulesElActionElDestinationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy:
        Option<Vec<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<NetworkServicesGrpcRouteRulesElActionElRetryPolicyEl>>,
    dynamic: NetworkServicesGrpcRouteRulesElActionElDynamic,
}
impl NetworkServicesGrpcRouteRulesElActionEl {
    #[doc = "Set the field `timeout`.\nSpecifies the timeout for selected route."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesGrpcRouteRulesElActionElDestinationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destinations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fault_injection_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fault_injection_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesGrpcRouteRulesElActionElRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.retry_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesElActionEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesElActionEl {}
impl BuildNetworkServicesGrpcRouteRulesElActionEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesElActionEl {
        NetworkServicesGrpcRouteRulesElActionEl {
            timeout: core::default::Default::default(),
            destinations: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElActionElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesGrpcRouteRulesElActionElRef {
        NetworkServicesGrpcRouteRulesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nSpecifies the timeout for selected route."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(
        &self,
    ) -> ListRef<NetworkServicesGrpcRouteRulesElActionElDestinationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }
    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> ListRef<NetworkServicesGrpcRouteRulesElActionElFaultInjectionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fault_injection_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<NetworkServicesGrpcRouteRulesElActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesElMatchesElHeadersEl {
    key: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    value: PrimField<String>,
}
impl NetworkServicesGrpcRouteRulesElMatchesElHeadersEl {
    #[doc = "Set the field `type_`.\nThe type of match. Default value: \"EXACT\" Possible values: [\"TYPE_UNSPECIFIED\", \"EXACT\", \"REGULAR_EXPRESSION\"]"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesElMatchesElHeadersEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesElMatchesElHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesElMatchesElHeadersEl {
    #[doc = "Required. The key of the header."]
    pub key: PrimField<String>,
    #[doc = "Required. The value of the header."]
    pub value: PrimField<String>,
}
impl BuildNetworkServicesGrpcRouteRulesElMatchesElHeadersEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesElMatchesElHeadersEl {
        NetworkServicesGrpcRouteRulesElMatchesElHeadersEl {
            key: self.key,
            type_: core::default::Default::default(),
            value: self.value,
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElMatchesElHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElMatchesElHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesGrpcRouteRulesElMatchesElHeadersElRef {
        NetworkServicesGrpcRouteRulesElMatchesElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElMatchesElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\nRequired. The key of the header."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of match. Default value: \"EXACT\" Possible values: [\"TYPE_UNSPECIFIED\", \"EXACT\", \"REGULAR_EXPRESSION\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nRequired. The value of the header."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesElMatchesElMethodEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    case_sensitive: Option<PrimField<bool>>,
    grpc_method: PrimField<String>,
    grpc_service: PrimField<String>,
}
impl NetworkServicesGrpcRouteRulesElMatchesElMethodEl {
    #[doc = "Set the field `case_sensitive`.\nSpecifies that matches are case sensitive. The default value is true."]
    pub fn set_case_sensitive(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.case_sensitive = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesElMatchesElMethodEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesElMatchesElMethodEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesElMatchesElMethodEl {
    #[doc = "Required. Name of the method to match against."]
    pub grpc_method: PrimField<String>,
    #[doc = "Required. Name of the service to match against."]
    pub grpc_service: PrimField<String>,
}
impl BuildNetworkServicesGrpcRouteRulesElMatchesElMethodEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesElMatchesElMethodEl {
        NetworkServicesGrpcRouteRulesElMatchesElMethodEl {
            case_sensitive: core::default::Default::default(),
            grpc_method: self.grpc_method,
            grpc_service: self.grpc_service,
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElMatchesElMethodElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElMatchesElMethodElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesGrpcRouteRulesElMatchesElMethodElRef {
        NetworkServicesGrpcRouteRulesElMatchesElMethodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElMatchesElMethodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `case_sensitive` after provisioning.\nSpecifies that matches are case sensitive. The default value is true."]
    pub fn case_sensitive(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.case_sensitive", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_method` after provisioning.\nRequired. Name of the method to match against."]
    pub fn grpc_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grpc_method", self.base))
    }
    #[doc = "Get a reference to the value of field `grpc_service` after provisioning.\nRequired. Name of the service to match against."]
    pub fn grpc_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.grpc_service", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesGrpcRouteRulesElMatchesElDynamic {
    headers: Option<DynamicBlock<NetworkServicesGrpcRouteRulesElMatchesElHeadersEl>>,
    method: Option<DynamicBlock<NetworkServicesGrpcRouteRulesElMatchesElMethodEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesElMatchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<NetworkServicesGrpcRouteRulesElMatchesElHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<Vec<NetworkServicesGrpcRouteRulesElMatchesElMethodEl>>,
    dynamic: NetworkServicesGrpcRouteRulesElMatchesElDynamic,
}
impl NetworkServicesGrpcRouteRulesElMatchesEl {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesGrpcRouteRulesElMatchesElHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesGrpcRouteRulesElMatchesElMethodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.method = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.method = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesElMatchesEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesElMatchesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesElMatchesEl {}
impl BuildNetworkServicesGrpcRouteRulesElMatchesEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesElMatchesEl {
        NetworkServicesGrpcRouteRulesElMatchesEl {
            headers: core::default::Default::default(),
            method: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElMatchesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElMatchesElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesGrpcRouteRulesElMatchesElRef {
        NetworkServicesGrpcRouteRulesElMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElMatchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> ListRef<NetworkServicesGrpcRouteRulesElMatchesElHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> ListRef<NetworkServicesGrpcRouteRulesElMatchesElMethodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesGrpcRouteRulesElDynamic {
    action: Option<DynamicBlock<NetworkServicesGrpcRouteRulesElActionEl>>,
    matches: Option<DynamicBlock<NetworkServicesGrpcRouteRulesElMatchesEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<NetworkServicesGrpcRouteRulesElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matches: Option<Vec<NetworkServicesGrpcRouteRulesElMatchesEl>>,
    dynamic: NetworkServicesGrpcRouteRulesElDynamic,
}
impl NetworkServicesGrpcRouteRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesGrpcRouteRulesElActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `matches`.\n"]
    pub fn set_matches(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesGrpcRouteRulesElMatchesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.matches = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.matches = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesGrpcRouteRulesEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteRulesEl {}
impl BuildNetworkServicesGrpcRouteRulesEl {
    pub fn build(self) -> NetworkServicesGrpcRouteRulesEl {
        NetworkServicesGrpcRouteRulesEl {
            action: core::default::Default::default(),
            matches: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesGrpcRouteRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteRulesElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesGrpcRouteRulesElRef {
        NetworkServicesGrpcRouteRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<NetworkServicesGrpcRouteRulesElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `matches` after provisioning.\n"]
    pub fn matches(&self) -> ListRef<NetworkServicesGrpcRouteRulesElMatchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.matches", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesGrpcRouteTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesGrpcRouteTimeoutsEl {
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
impl ToListMappable for NetworkServicesGrpcRouteTimeoutsEl {
    type O = BlockAssignable<NetworkServicesGrpcRouteTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesGrpcRouteTimeoutsEl {}
impl BuildNetworkServicesGrpcRouteTimeoutsEl {
    pub fn build(self) -> NetworkServicesGrpcRouteTimeoutsEl {
        NetworkServicesGrpcRouteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesGrpcRouteTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesGrpcRouteTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesGrpcRouteTimeoutsElRef {
        NetworkServicesGrpcRouteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesGrpcRouteTimeoutsElRef {
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
struct NetworkServicesGrpcRouteDynamic {
    rules: Option<DynamicBlock<NetworkServicesGrpcRouteRulesEl>>,
}
