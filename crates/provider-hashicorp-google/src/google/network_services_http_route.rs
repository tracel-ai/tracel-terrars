use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkServicesHttpRouteData {
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
    meshes: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<NetworkServicesHttpRouteRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkServicesHttpRouteTimeoutsEl>,
    dynamic: NetworkServicesHttpRouteDynamic,
}
struct NetworkServicesHttpRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkServicesHttpRouteData>,
}
#[derive(Clone)]
pub struct NetworkServicesHttpRoute(Rc<NetworkServicesHttpRoute_>);
impl NetworkServicesHttpRoute {
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
    #[doc = "Set the field `gateways`.\nGateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/global/gateways/<gateway_name>"]
    pub fn set_gateways(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().gateways = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSet of label tags associated with the HttpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `meshes`.\nMeshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>.\nThe attached Mesh should be of a type SIDECAR."]
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
    pub fn set_rules(self, v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<NetworkServicesHttpRouteTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the HttpRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `gateways` after provisioning.\nGateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/global/gateways/<gateway_name>"]
    pub fn gateways(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateways", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hostnames` after provisioning.\nSet of hosts that should match against the HTTP host header to select a HttpRoute to process the request."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the HttpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meshes` after provisioning.\nMeshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>.\nThe attached Mesh should be of a type SIDECAR."]
    pub fn meshes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meshes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the HttpRoute resource."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the HttpRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<NetworkServicesHttpRouteRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesHttpRouteTimeoutsElRef {
        NetworkServicesHttpRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkServicesHttpRoute {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkServicesHttpRoute {}
impl ToListMappable for NetworkServicesHttpRoute {
    type O = ListRef<NetworkServicesHttpRouteRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkServicesHttpRoute_ {
    fn extract_resource_type(&self) -> String {
        "google_network_services_http_route".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkServicesHttpRoute {
    pub tf_id: String,
    #[doc = "Set of hosts that should match against the HTTP host header to select a HttpRoute to process the request."]
    pub hostnames: ListField<PrimField<String>>,
    #[doc = "Name of the HttpRoute resource."]
    pub name: PrimField<String>,
}
impl BuildNetworkServicesHttpRoute {
    pub fn build(self, stack: &mut Stack) -> NetworkServicesHttpRoute {
        let out = NetworkServicesHttpRoute(Rc::new(NetworkServicesHttpRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkServicesHttpRouteData {
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
pub struct NetworkServicesHttpRouteRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkServicesHttpRouteRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the HttpRoute was created in UTC."]
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
    #[doc = "Get a reference to the value of field `gateways` after provisioning.\nGateways defines a list of gateways this HttpRoute is attached to, as one of the routing rules to route the requests served by the gateway.\nEach gateway reference should match the pattern: projects/*/locations/global/gateways/<gateway_name>"]
    pub fn gateways(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.gateways", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hostnames` after provisioning.\nSet of hosts that should match against the HTTP host header to select a HttpRoute to process the request."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the HttpRoute resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `meshes` after provisioning.\nMeshes defines a list of meshes this HttpRoute is attached to, as one of the routing rules to route the requests served by the mesh.\nEach mesh reference should match the pattern: projects/*/locations/global/meshes/<mesh_name>.\nThe attached Mesh should be of a type SIDECAR."]
    pub fn meshes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.meshes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the HttpRoute resource."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the HttpRoute was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> ListRef<NetworkServicesHttpRouteRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesHttpRouteTimeoutsElRef {
        NetworkServicesHttpRouteTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElCorsPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_methods: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origin_regexes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_origins: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<String>>,
}
impl NetworkServicesHttpRouteRulesElActionElCorsPolicyEl {
    #[doc = "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_headers`.\nSpecifies the content for Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_methods`.\nSpecifies the content for Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins."]
    pub fn set_allow_origin_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origin_regexes = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nIf true, the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `expose_headers`.\nSpecifies the content for Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `max_age`.\nSpecifies how long result of a preflight request can be cached in seconds."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElCorsPolicyEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElCorsPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElCorsPolicyEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElCorsPolicyEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElCorsPolicyEl {
        NetworkServicesHttpRouteRulesElActionElCorsPolicyEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origin_regexes: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            disabled: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElCorsPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElCorsPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElCorsPolicyElRef {
        NetworkServicesHttpRouteRulesElActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins."]
    pub fn allow_origin_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origin_regexes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf true, the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expose_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long result of a preflight request can be cached in seconds."]
    pub fn max_age(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElDestinationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl NetworkServicesHttpRouteRulesElActionElDestinationsEl {
    #[doc = "Set the field `service_name`.\nThe URL of a BackendService to route traffic to."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElDestinationsEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElDestinationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElDestinationsEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElDestinationsEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElDestinationsEl {
        NetworkServicesHttpRouteRulesElActionElDestinationsEl {
            service_name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElDestinationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElDestinationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElDestinationsElRef {
        NetworkServicesHttpRouteRulesElActionElDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElDestinationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe URL of a BackendService to route traffic to."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_status: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}
impl NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl {
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
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl {
        NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
        NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortElRef {
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
pub struct NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_delay: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}
impl NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl {
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
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl {
        NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl {
            fixed_delay: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
        NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayElRef {
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
struct NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDynamic {
    abort:
        Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl>>,
    delay:
        Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<Vec<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<Vec<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDynamic,
}
impl NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v: impl Into<
            BlockAssignable<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortEl>,
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
            BlockAssignable<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayEl>,
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
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl {
        NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElRef {
        NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElAbortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }
    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_redirect: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strip_query: Option<PrimField<bool>>,
}
impl NetworkServicesHttpRouteRulesElActionElRedirectEl {
    #[doc = "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was supplied in the request."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was supplied in the request. pathRedirect can not be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `port_redirect`.\nThe port that will be used in the redirected request instead of the one that was supplied in the request."]
    pub fn set_port_redirect(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_rewrite`.\nIndicates that during redirection, the matched prefix (or path) should be swapped with this value."]
    pub fn set_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `response_code`.\nThe HTTP Status code to use for the redirect."]
    pub fn set_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.response_code = Some(v.into());
        self
    }
    #[doc = "Set the field `strip_query`.\nIf set to true, any accompanying query portion of the original URL is removed prior to redirecting the request."]
    pub fn set_strip_query(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strip_query = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElRedirectEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElRedirectEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElRedirectEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElRedirectEl {
        NetworkServicesHttpRouteRulesElActionElRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            port_redirect: core::default::Default::default(),
            prefix_rewrite: core::default::Default::default(),
            response_code: core::default::Default::default(),
            strip_query: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElRedirectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElRedirectElRef {
        NetworkServicesHttpRouteRulesElActionElRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was supplied in the request."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was supplied in the request. pathRedirect can not be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request will be used for the redirect."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `port_redirect` after provisioning.\nThe port that will be used in the redirected request instead of the one that was supplied in the request."]
    pub fn port_redirect(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_rewrite` after provisioning.\nIndicates that during redirection, the matched prefix (or path) should be swapped with this value."]
    pub fn prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_rewrite", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_code` after provisioning.\nThe HTTP Status code to use for the redirect."]
    pub fn response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.response_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior to redirecting the request."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    add: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set: Option<RecField<PrimField<String>>>,
}
impl NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl {
    #[doc = "Set the field `add`.\nAdd the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set_add(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.add = Some(v.into());
        self
    }
    #[doc = "Set the field `remove`.\nRemove headers (matching by header names) specified in the list."]
    pub fn set_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.remove = Some(v.into());
        self
    }
    #[doc = "Set the field `set`.\nCompletely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set_set(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.set = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl {
        NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl {
            add: core::default::Default::default(),
            remove: core::default::Default::default(),
            set: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierElRef {
        NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `add` after provisioning.\nAdd the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn add(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.add", self.base))
    }
    #[doc = "Get a reference to the value of field `remove` after provisioning.\nRemove headers (matching by header names) specified in the list."]
    pub fn remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.remove", self.base))
    }
    #[doc = "Get a reference to the value of field `set` after provisioning.\nCompletely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.set", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl {
    #[doc = "Set the field `service_name`.\nThe URL of a BackendService to route traffic to."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl {
    type O =
        BlockAssignable<NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl {
    pub fn build(
        self,
    ) -> NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl {
        NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl {
            service_name: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationElRef {
        NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe URL of a BackendService to route traffic to."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nSpecifies the proportion of requests forwarded to the backend referenced by the serviceName field. This is computed as: weight/Sum(weights in this destination list). For non-zero values, there may be some epsilon from the exact proportion defined here depending on the precision an implementation supports.\nIf only one serviceName is specified and it has a weight greater than 0, 100% of the traffic is forwarded to that backend.\nIf weights are specified for any one service name, they need to be specified for all of them.\nIf weights are unspecified for all services, then, traffic is distributed in equal proportions to all of them."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDynamic {
    destination: Option<
        DynamicBlock<NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl>,
    >,
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination:
        Option<Vec<NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl>>,
    dynamic: NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDynamic,
}
impl NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl Into<
            BlockAssignable<
                NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl {
        NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl {
            destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElRef {
        NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    add: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    set: Option<RecField<PrimField<String>>>,
}
impl NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl {
    #[doc = "Set the field `add`.\nAdd the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set_add(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.add = Some(v.into());
        self
    }
    #[doc = "Set the field `remove`.\nRemove headers (matching by header names) specified in the list."]
    pub fn set_remove(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.remove = Some(v.into());
        self
    }
    #[doc = "Set the field `set`.\nCompletely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set_set(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.set = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl {
        NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl {
            add: core::default::Default::default(),
            remove: core::default::Default::default(),
            set: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierElRef {
        NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `add` after provisioning.\nAdd the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn add(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.add", self.base))
    }
    #[doc = "Get a reference to the value of field `remove` after provisioning.\nRemove headers (matching by header names) specified in the list."]
    pub fn remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.remove", self.base))
    }
    #[doc = "Get a reference to the value of field `set` after provisioning.\nCompletely overwrite/replace the headers with given map where key is the name of the header, value is the value of the header."]
    pub fn set(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.set", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_try_timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
}
impl NetworkServicesHttpRouteRulesElActionElRetryPolicyEl {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number of retries."]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `per_try_timeout`.\nSpecifies a non-zero timeout per retry attempt. A duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_per_try_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.per_try_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry policy applies."]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElRetryPolicyEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElRetryPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElRetryPolicyEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElRetryPolicyEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElRetryPolicyEl {
        NetworkServicesHttpRouteRulesElActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElRetryPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElRetryPolicyElRef {
        NetworkServicesHttpRouteRulesElActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number of retries."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\nSpecifies a non-zero timeout per retry attempt. A duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn per_try_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.per_try_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry policy applies."]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_conditions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
}
impl NetworkServicesHttpRouteRulesElActionElUrlRewriteEl {
    #[doc = "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected destination, the requests host header is replaced by this value."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected destination, the matching portion of the requests path is replaced by this value."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionElUrlRewriteEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionElUrlRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionElUrlRewriteEl {}
impl BuildNetworkServicesHttpRouteRulesElActionElUrlRewriteEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionElUrlRewriteEl {
        NetworkServicesHttpRouteRulesElActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElUrlRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElActionElUrlRewriteElRef {
        NetworkServicesHttpRouteRulesElActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected destination, the requests host header is replaced by this value."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected destination, the matching portion of the requests path is replaced by this value."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_prefix_rewrite", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesHttpRouteRulesElActionElDynamic {
    cors_policy: Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElCorsPolicyEl>>,
    destinations: Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElDestinationsEl>>,
    fault_injection_policy:
        Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl>>,
    redirect: Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElRedirectEl>>,
    request_header_modifier:
        Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl>>,
    request_mirror_policy:
        Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl>>,
    response_header_modifier:
        Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl>>,
    retry_policy: Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElRetryPolicyEl>>,
    url_rewrite: Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionElUrlRewriteEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_policy: Option<Vec<NetworkServicesHttpRouteRulesElActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destinations: Option<Vec<NetworkServicesHttpRouteRulesElActionElDestinationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy:
        Option<Vec<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect: Option<Vec<NetworkServicesHttpRouteRulesElActionElRedirectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_header_modifier:
        Option<Vec<NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mirror_policy:
        Option<Vec<NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_header_modifier:
        Option<Vec<NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<NetworkServicesHttpRouteRulesElActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<NetworkServicesHttpRouteRulesElActionElUrlRewriteEl>>,
    dynamic: NetworkServicesHttpRouteRulesElActionElDynamic,
}
impl NetworkServicesHttpRouteRulesElActionEl {
    #[doc = "Set the field `timeout`.\nSpecifies the timeout for selected route."]
    pub fn set_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionElCorsPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cors_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cors_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionElDestinationsEl>>,
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
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyEl>>,
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
    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionElRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_header_modifier`.\n"]
    pub fn set_request_header_modifier(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_header_modifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header_modifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_mirror_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_mirror_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_header_modifier`.\n"]
    pub fn set_response_header_modifier(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_header_modifier = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_header_modifier = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionElRetryPolicyEl>>,
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
    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionElUrlRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_rewrite = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElActionEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElActionEl {}
impl BuildNetworkServicesHttpRouteRulesElActionEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElActionEl {
        NetworkServicesHttpRouteRulesElActionEl {
            timeout: core::default::Default::default(),
            cors_policy: core::default::Default::default(),
            destinations: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            redirect: core::default::Default::default(),
            request_header_modifier: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            response_header_modifier: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElActionElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesHttpRouteRulesElActionElRef {
        NetworkServicesHttpRouteRulesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\nSpecifies the timeout for selected route."]
    pub fn timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> ListRef<NetworkServicesHttpRouteRulesElActionElCorsPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElActionElDestinationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }
    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElActionElFaultInjectionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fault_injection_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> ListRef<NetworkServicesHttpRouteRulesElActionElRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
    #[doc = "Get a reference to the value of field `request_header_modifier` after provisioning.\n"]
    pub fn request_header_modifier(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElActionElRequestHeaderModifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_header_modifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElActionElRequestMirrorPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_mirror_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_header_modifier` after provisioning.\n"]
    pub fn response_header_modifier(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElActionElResponseHeaderModifierElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_header_modifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<NetworkServicesHttpRouteRulesElActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> ListRef<NetworkServicesHttpRouteRulesElActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl {
    end: PrimField<f64>,
    start: PrimField<f64>,
}
impl NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl {}
impl ToListMappable for NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl {
    #[doc = "End of the range (exclusive)."]
    pub end: PrimField<f64>,
    #[doc = "Start of the range (inclusive)."]
    pub start: PrimField<f64>,
}
impl BuildNetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl {
        NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl {
            end: self.end,
            start: self.start,
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchElRef {
        NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `end` after provisioning.\nEnd of the range (exclusive)."]
    pub fn end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\nStart of the range (inclusive)."]
    pub fn start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesHttpRouteRulesElMatchesElHeadersElDynamic {
    range_match:
        Option<DynamicBlock<NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElMatchesElHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invert_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    present_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_match: Option<Vec<NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl>>,
    dynamic: NetworkServicesHttpRouteRulesElMatchesElHeadersElDynamic,
}
impl NetworkServicesHttpRouteRulesElMatchesElHeadersEl {
    #[doc = "Set the field `exact_match`.\nThe value of the header should match exactly the content of exactMatch."]
    pub fn set_exact_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact_match = Some(v.into());
        self
    }
    #[doc = "Set the field `header`.\nThe name of the HTTP header to match against."]
    pub fn set_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_match`.\nIf specified, the match result will be inverted before checking. Default value is set to false."]
    pub fn set_invert_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_match = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_match`.\nThe value of the header must start with the contents of prefixMatch."]
    pub fn set_prefix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_match = Some(v.into());
        self
    }
    #[doc = "Set the field `present_match`.\nA header with headerName must exist. The match takes place whether or not the header has a value."]
    pub fn set_present_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.present_match = Some(v.into());
        self
    }
    #[doc = "Set the field `regex_match`.\nThe value of the header must match the regular expression specified in regexMatch."]
    pub fn set_regex_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_match = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix_match`.\nThe value of the header must end with the contents of suffixMatch."]
    pub fn set_suffix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix_match = Some(v.into());
        self
    }
    #[doc = "Set the field `range_match`.\n"]
    pub fn set_range_match(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.range_match = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.range_match = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElMatchesElHeadersEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElMatchesElHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElMatchesElHeadersEl {}
impl BuildNetworkServicesHttpRouteRulesElMatchesElHeadersEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElMatchesElHeadersEl {
        NetworkServicesHttpRouteRulesElMatchesElHeadersEl {
            exact_match: core::default::Default::default(),
            header: core::default::Default::default(),
            invert_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            present_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            suffix_match: core::default::Default::default(),
            range_match: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElMatchesElHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElMatchesElHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElMatchesElHeadersElRef {
        NetworkServicesHttpRouteRulesElMatchesElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElMatchesElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact_match` after provisioning.\nThe value of the header should match exactly the content of exactMatch."]
    pub fn exact_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\nThe name of the HTTP header to match against."]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_match` after provisioning.\nIf specified, the match result will be inverted before checking. Default value is set to false."]
    pub fn invert_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_match", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_match` after provisioning.\nThe value of the header must start with the contents of prefixMatch."]
    pub fn prefix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }
    #[doc = "Get a reference to the value of field `present_match` after provisioning.\nA header with headerName must exist. The match takes place whether or not the header has a value."]
    pub fn present_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.present_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regex_match` after provisioning.\nThe value of the header must match the regular expression specified in regexMatch."]
    pub fn regex_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix_match` after provisioning.\nThe value of the header must end with the contents of suffixMatch."]
    pub fn suffix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix_match", self.base))
    }
    #[doc = "Get a reference to the value of field `range_match` after provisioning.\n"]
    pub fn range_match(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElMatchesElHeadersElRangeMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.range_match", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElMatchesElQueryParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    present_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_match: Option<PrimField<String>>,
}
impl NetworkServicesHttpRouteRulesElMatchesElQueryParametersEl {
    #[doc = "Set the field `exact_match`.\nThe value of the query parameter must exactly match the contents of exactMatch."]
    pub fn set_exact_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact_match = Some(v.into());
        self
    }
    #[doc = "Set the field `present_match`.\nSpecifies that the QueryParameterMatcher matches if request contains query parameter, irrespective of whether the parameter has a value or not."]
    pub fn set_present_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.present_match = Some(v.into());
        self
    }
    #[doc = "Set the field `query_parameter`.\nThe name of the query parameter to match."]
    pub fn set_query_parameter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_parameter = Some(v.into());
        self
    }
    #[doc = "Set the field `regex_match`.\nThe value of the query parameter must match the regular expression specified by regexMatch.For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax"]
    pub fn set_regex_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_match = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElMatchesElQueryParametersEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElMatchesElQueryParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElMatchesElQueryParametersEl {}
impl BuildNetworkServicesHttpRouteRulesElMatchesElQueryParametersEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElMatchesElQueryParametersEl {
        NetworkServicesHttpRouteRulesElMatchesElQueryParametersEl {
            exact_match: core::default::Default::default(),
            present_match: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
            regex_match: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElMatchesElQueryParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElMatchesElQueryParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkServicesHttpRouteRulesElMatchesElQueryParametersElRef {
        NetworkServicesHttpRouteRulesElMatchesElQueryParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElMatchesElQueryParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact_match` after provisioning.\nThe value of the query parameter must exactly match the contents of exactMatch."]
    pub fn exact_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }
    #[doc = "Get a reference to the value of field `present_match` after provisioning.\nSpecifies that the QueryParameterMatcher matches if request contains query parameter, irrespective of whether the parameter has a value or not."]
    pub fn present_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.present_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_parameter` after provisioning.\nThe name of the query parameter to match."]
    pub fn query_parameter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_parameter", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regex_match` after provisioning.\nThe value of the query parameter must match the regular expression specified by regexMatch.For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax"]
    pub fn regex_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesHttpRouteRulesElMatchesElDynamic {
    headers: Option<DynamicBlock<NetworkServicesHttpRouteRulesElMatchesElHeadersEl>>,
    query_parameters:
        Option<DynamicBlock<NetworkServicesHttpRouteRulesElMatchesElQueryParametersEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesElMatchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    full_path_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<NetworkServicesHttpRouteRulesElMatchesElHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameters: Option<Vec<NetworkServicesHttpRouteRulesElMatchesElQueryParametersEl>>,
    dynamic: NetworkServicesHttpRouteRulesElMatchesElDynamic,
}
impl NetworkServicesHttpRouteRulesElMatchesEl {
    #[doc = "Set the field `full_path_match`.\nThe HTTP request path value should exactly match this value."]
    pub fn set_full_path_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.full_path_match = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nSpecifies if prefixMatch and fullPathMatch matches are case sensitive. The default value is false."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_match`.\nThe HTTP request path value must begin with specified prefixMatch. prefixMatch must begin with a /."]
    pub fn set_prefix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_match = Some(v.into());
        self
    }
    #[doc = "Set the field `regex_match`.\nThe HTTP request path value must satisfy the regular expression specified by regexMatch after removing any query parameters and anchor supplied with the original URL. For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax"]
    pub fn set_regex_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_match = Some(v.into());
        self
    }
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElMatchesElHeadersEl>>,
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
    #[doc = "Set the field `query_parameters`.\n"]
    pub fn set_query_parameters(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElMatchesElQueryParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NetworkServicesHttpRouteRulesElMatchesEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesElMatchesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesElMatchesEl {}
impl BuildNetworkServicesHttpRouteRulesElMatchesEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesElMatchesEl {
        NetworkServicesHttpRouteRulesElMatchesEl {
            full_path_match: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            headers: core::default::Default::default(),
            query_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElMatchesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElMatchesElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesHttpRouteRulesElMatchesElRef {
        NetworkServicesHttpRouteRulesElMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElMatchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `full_path_match` after provisioning.\nThe HTTP request path value should exactly match this value."]
    pub fn full_path_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_path_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nSpecifies if prefixMatch and fullPathMatch matches are case sensitive. The default value is false."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_match` after provisioning.\nThe HTTP request path value must begin with specified prefixMatch. prefixMatch must begin with a /."]
    pub fn prefix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }
    #[doc = "Get a reference to the value of field `regex_match` after provisioning.\nThe HTTP request path value must satisfy the regular expression specified by regexMatch after removing any query parameters and anchor supplied with the original URL. For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax"]
    pub fn regex_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }
    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> ListRef<NetworkServicesHttpRouteRulesElMatchesElHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
    #[doc = "Get a reference to the value of field `query_parameters` after provisioning.\n"]
    pub fn query_parameters(
        &self,
    ) -> ListRef<NetworkServicesHttpRouteRulesElMatchesElQueryParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_parameters", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NetworkServicesHttpRouteRulesElDynamic {
    action: Option<DynamicBlock<NetworkServicesHttpRouteRulesElActionEl>>,
    matches: Option<DynamicBlock<NetworkServicesHttpRouteRulesElMatchesEl>>,
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<NetworkServicesHttpRouteRulesElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matches: Option<Vec<NetworkServicesHttpRouteRulesElMatchesEl>>,
    dynamic: NetworkServicesHttpRouteRulesElDynamic,
}
impl NetworkServicesHttpRouteRulesEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElActionEl>>,
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
        v: impl Into<BlockAssignable<NetworkServicesHttpRouteRulesElMatchesEl>>,
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
impl ToListMappable for NetworkServicesHttpRouteRulesEl {
    type O = BlockAssignable<NetworkServicesHttpRouteRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteRulesEl {}
impl BuildNetworkServicesHttpRouteRulesEl {
    pub fn build(self) -> NetworkServicesHttpRouteRulesEl {
        NetworkServicesHttpRouteRulesEl {
            action: core::default::Default::default(),
            matches: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteRulesElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesHttpRouteRulesElRef {
        NetworkServicesHttpRouteRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<NetworkServicesHttpRouteRulesElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `matches` after provisioning.\n"]
    pub fn matches(&self) -> ListRef<NetworkServicesHttpRouteRulesElMatchesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.matches", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkServicesHttpRouteTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkServicesHttpRouteTimeoutsEl {
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
impl ToListMappable for NetworkServicesHttpRouteTimeoutsEl {
    type O = BlockAssignable<NetworkServicesHttpRouteTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkServicesHttpRouteTimeoutsEl {}
impl BuildNetworkServicesHttpRouteTimeoutsEl {
    pub fn build(self) -> NetworkServicesHttpRouteTimeoutsEl {
        NetworkServicesHttpRouteTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkServicesHttpRouteTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkServicesHttpRouteTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NetworkServicesHttpRouteTimeoutsElRef {
        NetworkServicesHttpRouteTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkServicesHttpRouteTimeoutsElRef {
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
struct NetworkServicesHttpRouteDynamic {
    rules: Option<DynamicBlock<NetworkServicesHttpRouteRulesEl>>,
}
