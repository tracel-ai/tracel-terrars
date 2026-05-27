use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRegionUrlMapData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_route_action: Option<Vec<ComputeRegionUrlMapDefaultRouteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_url_redirect: Option<Vec<ComputeRegionUrlMapDefaultUrlRedirectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeRegionUrlMapHeaderActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rule: Option<Vec<ComputeRegionUrlMapHostRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_matcher: Option<Vec<ComputeRegionUrlMapPathMatcherEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test: Option<Vec<ComputeRegionUrlMapTestEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionUrlMapTimeoutsEl>,
    dynamic: ComputeRegionUrlMapDynamic,
}
struct ComputeRegionUrlMap_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionUrlMapData>,
}
#[derive(Clone)]
pub struct ComputeRegionUrlMap(Rc<ComputeRegionUrlMap_>);
impl ComputeRegionUrlMap {
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
    #[doc = "Set the field `default_service`.\nThe full or partial URL of the defaultService resource to which traffic is directed if\nnone of the hostRules match. If defaultRouteAction is additionally specified, advanced\nrouting actions like URL Rewrites, etc. take effect prior to sending the request to the\nbackend. However, if defaultService is specified, defaultRouteAction cannot contain any\nweightedBackendServices. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified.  Only one of defaultService,\ndefaultUrlRedirect or defaultRouteAction.weightedBackendService must be set."]
    pub fn set_default_service(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_service = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
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
    #[doc = "Set the field `region`.\nThe Region in which the url map should reside.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `default_route_action`.\n"]
    pub fn set_default_route_action(
        self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapDefaultRouteActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_route_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_route_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_url_redirect`.\n"]
    pub fn set_default_url_redirect(
        self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapDefaultUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_url_redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_url_redirect = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.header_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `host_rule`.\n"]
    pub fn set_host_rule(
        self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapHostRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().host_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.host_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `path_matcher`.\n"]
    pub fn set_path_matcher(
        self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().path_matcher = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.path_matcher = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `test`.\n"]
    pub fn set_test(self, v: impl Into<BlockAssignable<ComputeRegionUrlMapTestEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().test = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.test = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionUrlMapTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `default_service` after provisioning.\nThe full or partial URL of the defaultService resource to which traffic is directed if\nnone of the hostRules match. If defaultRouteAction is additionally specified, advanced\nrouting actions like URL Rewrites, etc. take effect prior to sending the request to the\nbackend. However, if defaultService is specified, defaultRouteAction cannot contain any\nweightedBackendServices. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified.  Only one of defaultService,\ndefaultUrlRedirect or defaultRouteAction.weightedBackendService must be set."]
    pub fn default_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `map_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn map_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the url map should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_route_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> ListRef<ComputeRegionUrlMapDefaultUrlRedirectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_url_redirect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeRegionUrlMapHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> ListRef<ComputeRegionUrlMapPathMatcherElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.path_matcher", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> ListRef<ComputeRegionUrlMapTestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.test", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionUrlMapTimeoutsElRef {
        ComputeRegionUrlMapTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRegionUrlMap {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRegionUrlMap {}
impl ToListMappable for ComputeRegionUrlMap {
    type O = ListRef<ComputeRegionUrlMapRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRegionUrlMap_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_url_map".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRegionUrlMap {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeRegionUrlMap {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionUrlMap {
        let out = ComputeRegionUrlMap(Rc::new(ComputeRegionUrlMap_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionUrlMapData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_service: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                default_route_action: core::default::Default::default(),
                default_url_redirect: core::default::Default::default(),
                header_action: core::default::Default::default(),
                host_rule: core::default::Default::default(),
                path_matcher: core::default::Default::default(),
                test: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRegionUrlMapRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRegionUrlMapRef {
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
    #[doc = "Get a reference to the value of field `default_service` after provisioning.\nThe full or partial URL of the defaultService resource to which traffic is directed if\nnone of the hostRules match. If defaultRouteAction is additionally specified, advanced\nrouting actions like URL Rewrites, etc. take effect prior to sending the request to the\nbackend. However, if defaultService is specified, defaultRouteAction cannot contain any\nweightedBackendServices. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified.  Only one of defaultService,\ndefaultUrlRedirect or defaultRouteAction.weightedBackendService must be set."]
    pub fn default_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource. This field is used internally during\nupdates of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fingerprint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `map_id` after provisioning.\nThe unique identifier for the resource."]
    pub fn map_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe Region in which the url map should reside.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(&self) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_route_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(&self) -> ListRef<ComputeRegionUrlMapDefaultUrlRedirectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_url_redirect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeRegionUrlMapHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\n"]
    pub fn path_matcher(&self) -> ListRef<ComputeRegionUrlMapPathMatcherElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.path_matcher", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `test` after provisioning.\n"]
    pub fn test(&self) -> ListRef<ComputeRegionUrlMapTestElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.test", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionUrlMapTimeoutsElRef {
        ComputeRegionUrlMapTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
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
    max_age: Option<PrimField<f64>>,
}
impl ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
    #[doc = "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials. This field translates to the Access-Control-Allow-Credentials header.\nDefault is false."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origin_regexes`.\nSpecifies the regualar expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origin_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origin_regexes = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nIf true, the setting specifies the CORS policy is disabled. The default value of false, which indicates that the CORS policy is in effect."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `max_age`.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
        ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl {
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
pub struct ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
        ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials. This field translates to the Access-Control-Allow-Credentials header.\nDefault is false."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regualar expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origin_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origin_regexes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf true, the setting specifies the CORS policy is disabled. The default value of false, which indicates that the CORS policy is in effect."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expose_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_status: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}
impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc = "Set the field `http_status`.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn set_http_status(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_status = Some(v.into());
        self
    }
    #[doc = "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn http_status(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
}
impl BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic {
    fixed_delay: Option<
        DynamicBlock<
            ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_delay: Option<
        Vec<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl>,
    >,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic,
}
impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc = "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_delay = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
    #[doc = "Get a reference to the value of field `fixed_delay` after provisioning.\n"]
    pub fn fixed_delay(
        &self,
    ) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDynamic {
    abort:
        Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    delay:
        Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<Vec<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<Vec<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>>,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDynamic,
}
impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortEl>,
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
            BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayEl>,
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
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
        ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElAbortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }
    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElDelayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_service: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    #[doc = "Set the field `backend_service`.\nThe full or partial URL to the RegionBackendService resource being mirrored to.\nThe backend service configured for a mirroring policy must reference backends that are of the same type as the original backend service matched in the URL map.\nServerless NEG backends are not currently supported as a mirrored backend service."]
    pub fn set_backend_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backend_service = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
        ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl {
            backend_service: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
        ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the RegionBackendService resource being mirrored to.\nThe backend service configured for a mirroring policy must reference backends that are of the same type as the original backend service matched in the URL map.\nServerless NEG backends are not currently supported as a mirrored backend service."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
        ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElDynamic {
    per_try_timeout:
        Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_try_timeout:
        Option<Vec<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>>,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElDynamic,
}
impl ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry policy applies.\nValid values are listed below. Only the following codes are supported when the URL map is bound to target gRPC proxy that has validateForProxyless field set to true: cancelled, deadline-exceeded, internal, resource-exhausted, unavailable.\n  - 5xx : retry is attempted if the instance or endpoint responds with any 5xx response code, or if the instance or endpoint does not respond at all. For example, disconnects, reset, read timeout, connection failure, and refused streams.\n  - gateway-error : Similar to 5xx, but only applies to response codes 502, 503 or 504.\n  - connect-failure : a retry is attempted on failures connecting to the instance or endpoint. For example, connection timeouts.\n  - retriable-4xx : a retry is attempted if the instance or endpoint responds with a 4xx response code. The only error that you can retry is error code 409.\n  - refused-stream : a retry is attempted if the instance or endpoint resets the stream with a REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n  - cancelled : a retry is attempted if the gRPC status code in the response header is set to cancelled.\n  - deadline-exceeded : a retry is attempted if the gRPC status code in the response header is set to deadline-exceeded.\n  - internal :  a retry is attempted if the gRPC status code in the response header is set to internal.\n  - resource-exhausted : a retry is attempted if the gRPC status code in the response header is set to resource-exhausted.\n  - unavailable : a retry is attempted if the gRPC status code in the response header is set to unavailable."]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_try_timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
        ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
        ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry policy applies.\nValid values are listed below. Only the following codes are supported when the URL map is bound to target gRPC proxy that has validateForProxyless field set to true: cancelled, deadline-exceeded, internal, resource-exhausted, unavailable.\n  - 5xx : retry is attempted if the instance or endpoint responds with any 5xx response code, or if the instance or endpoint does not respond at all. For example, disconnects, reset, read timeout, connection failure, and refused streams.\n  - gateway-error : Similar to 5xx, but only applies to response codes 502, 503 or 504.\n  - connect-failure : a retry is attempted on failures connecting to the instance or endpoint. For example, connection timeouts.\n  - retriable-4xx : a retry is attempted if the instance or endpoint responds with a 4xx response code. The only error that you can retry is error code 409.\n  - refused-stream : a retry is attempted if the instance or endpoint resets the stream with a REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n  - cancelled : a retry is attempted if the gRPC status code in the response header is set to cancelled.\n  - deadline-exceeded : a retry is attempted if the gRPC status code in the response header is set to deadline-exceeded.\n  - internal :  a retry is attempted if the gRPC status code in the response header is set to internal.\n  - resource-exhausted : a retry is attempted if the gRPC status code in the response header is set to resource-exhausted.\n  - unavailable : a retry is attempted if the gRPC status code in the response header is set to unavailable."]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.per_try_timeout", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive. Note: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElTimeoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElTimeoutEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
        ComputeRegionUrlMapDefaultRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
        ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive. Note: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
    #[doc = "Set the field `host_rewrite`.\nBefore forwarding the request to the selected service, the request's host header is replaced with contents of hostRewrite.\nThe value must be from 1 to 255 characters."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `path_prefix_rewrite`.\nBefore forwarding the request to the selected backend service, the matching portion of the request's path is replaced by pathPrefixRewrite.\nThe value must be from 1 to 1024 characters."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
        ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
        ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_rewrite` after provisioning.\nBefore forwarding the request to the selected service, the request's host header is replaced with contents of hostRewrite.\nThe value must be from 1 to 255 characters."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nBefore forwarding the request to the selected backend service, the matching portion of the request's path is replaced by pathPrefixRewrite.\nThe value must be from 1 to 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_prefix_rewrite", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { # [doc = "Set the field `header_name`.\nThe name of the header."] pub fn set_header_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . header_name = Some (v . into ()) ; self } # [doc = "Set the field `header_value`.\nThe value of the header to add."] pub fn set_header_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . header_value = Some (v . into ()) ; self } # [doc = "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."] pub fn set_replace (mut self , v : impl Into < PrimField < bool > >) -> Self { self . replace = Some (v . into ()) ; self } }
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { type O = BlockAssignable < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl
{}
impl BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { pub fn build (self) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { header_name : core :: default :: Default :: default () , header_value : core :: default :: Default :: default () , replace : core :: default :: Default :: default () , } } }
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { # [doc = "Set the field `header_name`.\nThe name of the header."] pub fn set_header_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . header_name = Some (v . into ()) ; self } # [doc = "Set the field `header_value`.\nThe value of the header to add."] pub fn set_header_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . header_value = Some (v . into ()) ; self } # [doc = "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."] pub fn set_replace (mut self , v : impl Into < PrimField < bool > >) -> Self { self . replace = Some (v . into ()) ; self } }
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { type O = BlockAssignable < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl
{}
impl BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { pub fn build (self) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { header_name : core :: default :: Default :: default () , header_value : core :: default :: Default :: default () , replace : core :: default :: Default :: default () , } } }
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic { request_headers_to_add : Option < DynamicBlock < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl >> , response_headers_to_add : Option < DynamicBlock < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl { # [serde (skip_serializing_if = "Option::is_none")] request_headers_to_remove : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] response_headers_to_remove : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] request_headers_to_add : Option < Vec < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl > > , # [serde (skip_serializing_if = "Option::is_none")] response_headers_to_add : Option < Vec < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl > > , dynamic : ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic , }
impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[doc = "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request before forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response before sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request before forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response before sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]    pub fn request_headers_to_add (& self) -> ListRef < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_add", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]    pub fn response_headers_to_add (& self) -> ListRef < ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_add", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElDynamic {
    header_action: Option<
        DynamicBlock<
            ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action:
        Option<Vec<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl>>,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElDynamic,
}
impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    #[doc = "Set the field `backend_service`.\nThe full or partial URL to the default BackendService resource. Before forwarding the request to backendService, the load balancer applies any relevant headerActions specified as part of this backendServiceWeight."]
    pub fn set_backend_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backend_service = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\nSpecifies the fraction of traffic sent to a backend service, computed as weight / (sum of all weightedBackendService weights in routeAction) .\nThe selection of a backend service is determined only for new traffic. Once a user's request has been directed to a backend service, subsequent requests are sent to the same backend service as determined by the backend service's session affinity policy.\nThe value must be from 0 to 1000."]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl {
            backend_service: core::default::Default::default(),
            weight: core::default::Default::default(),
            header_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
        ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the default BackendService resource. Before forwarding the request to backendService, the load balancer applies any relevant headerActions specified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to a backend service, computed as weight / (sum of all weightedBackendService weights in routeAction) .\nThe selection of a backend service is determined only for new traffic. Once a user's request has been directed to a backend service, subsequent requests are sent to the same backend service as determined by the backend service's session affinity policy.\nThe value must be from 0 to 1000."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapDefaultRouteActionElDynamic {
    cors_policy: Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl>>,
    fault_injection_policy:
        Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl>>,
    request_mirror_policy:
        Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl>>,
    retry_policy: Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElTimeoutEl>>,
    url_rewrite: Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl>>,
    weighted_backend_services:
        Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultRouteActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_policy: Option<Vec<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy:
        Option<Vec<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mirror_policy:
        Option<Vec<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<ComputeRegionUrlMapDefaultRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_backend_services:
        Option<Vec<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl>>,
    dynamic: ComputeRegionUrlMapDefaultRouteActionElDynamic,
}
impl ComputeRegionUrlMapDefaultRouteActionEl {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyEl>>,
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
    #[doc = "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyEl>>,
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
    #[doc = "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyEl>>,
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
    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyEl>>,
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
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteEl>>,
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
    #[doc = "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_backend_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultRouteActionEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultRouteActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultRouteActionEl {}
impl BuildComputeRegionUrlMapDefaultRouteActionEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultRouteActionEl {
        ComputeRegionUrlMapDefaultRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapDefaultRouteActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultRouteActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapDefaultRouteActionElRef {
        ComputeRegionUrlMapDefaultRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultRouteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(&self) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElCorsPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElFaultInjectionPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fault_injection_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElRequestMirrorPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_mirror_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> ListRef<ComputeRegionUrlMapDefaultRouteActionElWeightedBackendServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weighted_backend_services", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapDefaultUrlRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_response_code: Option<PrimField<String>>,
    strip_query: PrimField<bool>,
}
impl ComputeRegionUrlMapDefaultUrlRedirectEl {
    #[doc = "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_response_code = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapDefaultUrlRedirectEl {
    type O = BlockAssignable<ComputeRegionUrlMapDefaultUrlRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapDefaultUrlRedirectEl {
    #[doc = "If set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: PrimField<bool>,
}
impl BuildComputeRegionUrlMapDefaultUrlRedirectEl {
    pub fn build(self) -> ComputeRegionUrlMapDefaultUrlRedirectEl {
        ComputeRegionUrlMapDefaultUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}
pub struct ComputeRegionUrlMapDefaultUrlRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapDefaultUrlRedirectElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapDefaultUrlRedirectElRef {
        ComputeRegionUrlMapDefaultUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapDefaultUrlRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redirect_response_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl ComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl {
    #[doc = "Set the field `header_name`.\nThe name of the header."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn set_replace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replace = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl {
    type O = BlockAssignable<ComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl {}
impl BuildComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl {
    pub fn build(self) -> ComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl {
        ComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapHeaderActionElRequestHeadersToAddElRef {
        ComputeRegionUrlMapHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
    #[doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl ComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl {
    #[doc = "Set the field `header_name`.\nThe name of the header."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn set_replace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replace = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl {
    type O = BlockAssignable<ComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl {}
impl BuildComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl {
    pub fn build(self) -> ComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl {
        ComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapHeaderActionElResponseHeadersToAddElRef {
        ComputeRegionUrlMapHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
    #[doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapHeaderActionElDynamic {
    request_headers_to_add:
        Option<DynamicBlock<ComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl>>,
    response_headers_to_add:
        Option<DynamicBlock<ComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<Vec<ComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add: Option<Vec<ComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl>>,
    dynamic: ComputeRegionUrlMapHeaderActionElDynamic,
}
impl ComputeRegionUrlMapHeaderActionEl {
    #[doc = "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request before forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response before sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapHeaderActionElRequestHeadersToAddEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapHeaderActionElResponseHeadersToAddEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapHeaderActionEl {
    type O = BlockAssignable<ComputeRegionUrlMapHeaderActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapHeaderActionEl {}
impl BuildComputeRegionUrlMapHeaderActionEl {
    pub fn build(self) -> ComputeRegionUrlMapHeaderActionEl {
        ComputeRegionUrlMapHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapHeaderActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapHeaderActionElRef {
        ComputeRegionUrlMapHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request before forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response before sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> ListRef<ComputeRegionUrlMapHeaderActionElRequestHeadersToAddElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_add", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> ListRef<ComputeRegionUrlMapHeaderActionElResponseHeadersToAddElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_add", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapHostRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    hosts: SetField<PrimField<String>>,
    path_matcher: PrimField<String>,
}
impl ComputeRegionUrlMapHostRuleEl {
    #[doc = "Set the field `description`.\nAn optional description of this HostRule. Provide this property\nwhen you create the resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapHostRuleEl {
    type O = BlockAssignable<ComputeRegionUrlMapHostRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapHostRuleEl {
    #[doc = "The list of host patterns to match. They must be valid\nhostnames, except * will match any string of ([a-z0-9-.]*). In\nthat case, * must be the first character and must be followed in\nthe pattern by either - or .."]
    pub hosts: SetField<PrimField<String>>,
    #[doc = "The name of the PathMatcher to use to match the path portion of\nthe URL if the hostRule matches the URL's host portion."]
    pub path_matcher: PrimField<String>,
}
impl BuildComputeRegionUrlMapHostRuleEl {
    pub fn build(self) -> ComputeRegionUrlMapHostRuleEl {
        ComputeRegionUrlMapHostRuleEl {
            description: core::default::Default::default(),
            hosts: self.hosts,
            path_matcher: self.path_matcher,
        }
    }
}
pub struct ComputeRegionUrlMapHostRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapHostRuleElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapHostRuleElRef {
        ComputeRegionUrlMapHostRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapHostRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this HostRule. Provide this property\nwhen you create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `hosts` after provisioning.\nThe list of host patterns to match. They must be valid\nhostnames, except * will match any string of ([a-z0-9-.]*). In\nthat case, * must be the first character and must be followed in\nthe pattern by either - or .."]
    pub fn hosts(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.hosts", self.base))
    }
    #[doc = "Get a reference to the value of field `path_matcher` after provisioning.\nThe name of the PathMatcher to use to match the path portion of\nthe URL if the hostRule matches the URL's host portion."]
    pub fn path_matcher(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path_matcher", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
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
    max_age: Option<PrimField<f64>>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
    #[doc = "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\nThis translates to the Access-Control-Allow-Credentials header."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origin_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origin_regexes = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `max_age`.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl {
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
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the actual request can include user credentials.\nThis translates to the Access-Control-Allow-Credentials header."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For regular expression grammar\nplease see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origin_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origin_regexes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests.\nAn origin is allowed if it matches either an item in allowOrigins or an item in allowOriginRegexes."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled. The default value is false, which indicates that the CORS policy is in effect."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expose_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long results of a preflight request can be cached in seconds.\nThis translates to the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_status: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc = "Set the field `http_status`.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn set_http_status(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_status = Some(v.into());
        self
    }
    #[doc = "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request.\nThe value must be between 200 and 599 inclusive."]
    pub fn http_status(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl
{}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { nanos : core :: default :: Default :: default () , seconds : core :: default :: Default :: default () , } } }
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { shared : shared , base : base . to_string () , } } }
impl
    ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic { fixed_delay : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl { # [serde (skip_serializing_if = "Option::is_none")] percentage : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] fixed_delay : Option < Vec < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl > > , dynamic : ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElDynamic , }
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc = "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_delay = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.\nThe value must be between 0.0 and 100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
    #[doc = "Get a reference to the value of field `fixed_delay` after provisioning.\n"]    pub fn fixed_delay (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef >{
        ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDynamic {
    abort: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl,
        >,
    >,
    delay: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<
        Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<
        Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortEl,
            >,
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
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayEl,
            >,
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
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
    type O =
        BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElAbortElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }
    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElDelayElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl {
    type O =
        BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub seconds: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
    backend_service: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
    type O =
        BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
    #[doc = "The full or partial URL to the BackendService resource being mirrored to."]
    pub backend_service: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl {
            backend_service: self.backend_service,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the BackendService resource being mirrored to."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are\nrepresented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElDynamic {
    per_try_timeout: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_try_timeout: Option<
        Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_conditions`.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with any 5xx response code,\n  or if the backend service does not respond at all, example: disconnects, reset, read timeout,\n* connection failure, and refused streams.\n* gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures connecting to backend services,\n  for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream:Loadbalancer will retry if the backend service resets the stream with a REFUSED_STREAM error code.\n  This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in the response header is set to unavailable"]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_try_timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0. If not specified, defaults to 1."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecfies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with any 5xx response code,\n  or if the backend service does not respond at all, example: disconnects, reset, read timeout,\n* connection failure, and refused streams.\n* gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures connecting to backend services,\n  for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream:Loadbalancer will retry if the backend service resets the stream with a REFUSED_STREAM error code.\n  This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElPerTryTimeoutElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.per_try_timeout", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
    #[doc = "Set the field `seconds`.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive.\nNote: these bounds are computed from: 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years"]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_template_rewrite: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
    #[doc = "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host header is replaced\nwith contents of hostRewrite.\n\nThe value must be between 1 and 255 characters."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching portion of the\nrequest's path is replaced by pathPrefixRewrite.\n\nThe value must be between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `path_template_rewrite`.\nIf specified, the pattern rewrites the URL path (based on the :path header) using the HTTP template syntax.\n\nA corresponding pathTemplateMatch must be specified. Any template variables must exist in the pathTemplateMatch field.\n\n* At least one variable must be specified in the pathTemplateMatch field\n* You can omit variables from the rewritten URL\n* The * and ** operators cannot be matched unless they have a corresponding variable name - e.g. {format=*} or {var=**}.\n\nFor example, a pathTemplateMatch of /static/{format=**} could be rewritten as /static/content/{format} to prefix\n/content to the URL. Variables can also be re-ordered in a rewrite, so that /{country}/{format}/{suffix=**} can be\nrewritten as /content/{format}/{country}/{suffix}.\n\nAt least one non-empty routeRules[].matchRules[].path_template_match is required.\n\nOnly one of pathPrefixRewrite or pathTemplateRewrite may be specified."]
    pub fn set_path_template_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_template_rewrite = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
            path_template_rewrite: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host header is replaced\nwith contents of hostRewrite.\n\nThe value must be between 1 and 255 characters."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching portion of the\nrequest's path is replaced by pathPrefixRewrite.\n\nThe value must be between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_prefix_rewrite", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_template_rewrite` after provisioning.\nIf specified, the pattern rewrites the URL path (based on the :path header) using the HTTP template syntax.\n\nA corresponding pathTemplateMatch must be specified. Any template variables must exist in the pathTemplateMatch field.\n\n* At least one variable must be specified in the pathTemplateMatch field\n* You can omit variables from the rewritten URL\n* The * and ** operators cannot be matched unless they have a corresponding variable name - e.g. {format=*} or {var=**}.\n\nFor example, a pathTemplateMatch of /static/{format=**} could be rewritten as /static/content/{format} to prefix\n/content to the URL. Variables can also be re-ordered in a rewrite, so that /{country}/{format}/{suffix=**} can be\nrewritten as /content/{format}/{country}/{suffix}.\n\nAt least one non-empty routeRules[].matchRules[].path_template_match is required.\n\nOnly one of pathPrefixRewrite or pathTemplateRewrite may be specified."]
    pub fn path_template_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_template_rewrite", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { # [doc = "Set the field `header_name`.\nThe name of the header to add."] pub fn set_header_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . header_name = Some (v . into ()) ; self } # [doc = "Set the field `header_value`.\nThe value of the header to add."] pub fn set_header_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . header_value = Some (v . into ()) ; self } # [doc = "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."] pub fn set_replace (mut self , v : impl Into < PrimField < bool > >) -> Self { self . replace = Some (v . into ()) ; self } }
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl
{}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { header_name : core :: default :: Default :: default () , header_value : core :: default :: Default :: default () , replace : core :: default :: Default :: default () , } } }
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { # [doc = "Set the field `header_name`.\nThe name of the header to add."] pub fn set_header_name (mut self , v : impl Into < PrimField < String > >) -> Self { self . header_name = Some (v . into ()) ; self } # [doc = "Set the field `header_value`.\nThe value of the header to add."] pub fn set_header_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . header_value = Some (v . into ()) ; self } # [doc = "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."] pub fn set_replace (mut self , v : impl Into < PrimField < bool > >) -> Self { self . replace = Some (v . into ()) ; self } }
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl
{}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { header_name : core :: default :: Default :: default () , header_value : core :: default :: Default :: default () , replace : core :: default :: Default :: default () , } } }
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header.\nIf true, headerValue is set for the header, discarding any values that were set for that header."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic { request_headers_to_add : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl >> , response_headers_to_add : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl { # [serde (skip_serializing_if = "Option::is_none")] request_headers_to_remove : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] response_headers_to_remove : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] request_headers_to_add : Option < Vec < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl > > , # [serde (skip_serializing_if = "Option::is_none")] response_headers_to_add : Option < Vec < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl > > , dynamic : ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElDynamic , }
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl {
    #[doc = "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request prior to\nforwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response prior to sending the\nresponse back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl
{}
impl
    BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl
{
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl
    {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl { request_headers_to_remove : core :: default :: Default :: default () , response_headers_to_remove : core :: default :: Default :: default () , request_headers_to_add : core :: default :: Default :: default () , response_headers_to_add : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef { ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef { shared : shared , base : base . to_string () , } } }
impl
    ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request prior to\nforwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response prior to sending the\nresponse back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]    pub fn request_headers_to_add (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_add", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]    pub fn response_headers_to_add (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_add", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElDynamic { header_action : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl { # [serde (skip_serializing_if = "Option::is_none")] backend_service : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] weight : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] header_action : Option < Vec < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl > > , dynamic : ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElDynamic , }
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
    #[doc = "Set the field `backend_service`.\nThe full or partial URL to the default BackendService resource. Before forwarding the\nrequest to backendService, the loadbalancer applies any relevant headerActions\nspecified as part of this backendServiceWeight."]
    pub fn set_backend_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.backend_service = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\nSpecifies the fraction of traffic sent to backendService, computed as\nweight / (sum of all weightedBackendService weights in routeAction) .\n\nThe selection of a backend service is determined only for new traffic. Once a user's request\nhas been directed to a backendService, subsequent requests will be sent to the same backendService\nas determined by the BackendService's session affinity policy.\n\nThe value must be between 0 and 1000"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl {
            backend_service: core::default::Default::default(),
            weight: core::default::Default::default(),
            header_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nThe full or partial URL to the default BackendService resource. Before forwarding the\nrequest to backendService, the loadbalancer applies any relevant headerActions\nspecified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as\nweight / (sum of all weightedBackendService weights in routeAction) .\n\nThe selection of a backend service is determined only for new traffic. Once a user's request\nhas been directed to a backendService, subsequent requests will be sent to the same backendService\nas determined by the BackendService's session affinity policy.\n\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]    pub fn header_action (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElHeaderActionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElDynamic {
    cors_policy:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl>>,
    fault_injection_policy: Option<
        DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl>,
    >,
    max_stream_duration: Option<
        DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl>,
    >,
    request_mirror_policy: Option<
        DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl>,
    >,
    retry_policy:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl>>,
    url_rewrite:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl>>,
    weighted_backend_services: Option<
        DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl>,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_policy: Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy:
        Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_stream_duration:
        Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mirror_policy:
        Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_backend_services:
        Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElDefaultRouteActionElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionEl {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyEl>>,
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
    #[doc = "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyEl,
            >,
        >,
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
    #[doc = "Set the field `max_stream_duration`.\n"]
    pub fn set_max_stream_duration(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.max_stream_duration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.max_stream_duration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyEl,
            >,
        >,
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
    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyEl>>,
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
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteEl>>,
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
    #[doc = "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_backend_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultRouteActionEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionEl {}
impl BuildComputeRegionUrlMapPathMatcherElDefaultRouteActionEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionEl {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            max_stream_duration: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRef {
        ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElCorsPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElFaultInjectionPolicyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fault_injection_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_stream_duration` after provisioning.\n"]
    pub fn max_stream_duration(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElMaxStreamDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_stream_duration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRequestMirrorPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_mirror_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElWeightedBackendServicesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weighted_backend_services", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_response_code: Option<PrimField<String>>,
    strip_query: PrimField<bool>,
}
impl ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    #[doc = "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_response_code = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    #[doc = "If set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: PrimField<bool>,
}
impl BuildComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
        ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
        ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one that was\nsupplied in the request. The value must be between 1 and 255 characters."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https. If set to\nfalse, the URL scheme of the redirected request will remain the same as that of the\nrequest. This must only be set for UrlMaps used in TargetHttpProxys. Setting this\ntrue for TargetHttpsProxy is not permitted. The default is set to false."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one that was\nsupplied in the request. pathRedirect cannot be supplied together with\nprefixRedirect. Supply one alone or neither. If neither is supplied, the path of the\noriginal request will be used for the redirect. The value must be between 1 and 1024\ncharacters."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch,\nretaining the remaining portion of the URL before redirecting the request.\nprefixRedirect cannot be supplied together with pathRedirect. Supply one alone or\nneither. If neither is supplied, the path of the original request will be used for\nthe redirect. The value must be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redirect_response_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed prior\nto redirecting the request. If set to false, the query portion of the original URL is\nretained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
    #[doc = "Set the field `header_name`.\nThe name of the header."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn set_replace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replace = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {}
impl BuildComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
    #[doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace: Option<PrimField<bool>>,
}
impl ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
    #[doc = "Set the field `header_name`.\nThe name of the header."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }
    #[doc = "Set the field `header_value`.\nThe value of the header to add."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }
    #[doc = "Set the field `replace`.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn set_replace(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.replace = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {}
impl BuildComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
            replace: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
    #[doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header.\nThe default value is false."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElHeaderActionElDynamic {
    request_headers_to_add:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl>>,
    response_headers_to_add:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add:
        Option<Vec<ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add:
        Option<Vec<ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElHeaderActionElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElHeaderActionEl {
    #[doc = "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request before forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response before sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElHeaderActionEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElHeaderActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElHeaderActionEl {}
impl BuildComputeRegionUrlMapPathMatcherElHeaderActionEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElHeaderActionEl {
        ComputeRegionUrlMapPathMatcherElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElHeaderActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapPathMatcherElHeaderActionElRef {
        ComputeRegionUrlMapPathMatcherElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request before forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response before sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElHeaderActionElRequestHeadersToAddElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_add", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElHeaderActionElResponseHeadersToAddElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_add", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
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
    disabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<PrimField<f64>>,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    #[doc = "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origin_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origin_regexes = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }
    #[doc = "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `max_age`.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    #[doc = "If true, specifies the CORS policy is disabled."]
    pub disabled: PrimField<bool>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl {
            allow_credentials: core::default::Default::default(),
            allow_headers: core::default::Default::default(),
            allow_methods: core::default::Default::default(),
            allow_origin_regexes: core::default::Default::default(),
            allow_origins: core::default::Default::default(),
            disabled: self.disabled,
            expose_headers: core::default::Default::default(),
            max_age: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origin_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origin_regexes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expose_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    http_status: PrimField<f64>,
    percentage: PrimField<f64>,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl
{
    #[doc = "The HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub http_status: PrimField<f64>,
    #[doc = "The percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub percentage: PrimField<f64>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: self.http_status,
            percentage: self.percentage,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef
    {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub fn http_status(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}
impl
    ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl
{
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl
{
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { nanos : core :: default :: Default :: default () , seconds : self . seconds , } } }
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."] pub fn nanos (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nanos" , self . base)) } # [doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."] pub fn seconds (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.seconds" , self . base)) } }
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElDynamic { fixed_delay : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl { percentage : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] fixed_delay : Option < Vec < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl > > , dynamic : ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElDynamic , }
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc = "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_delay = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl
{
    #[doc = "The percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub percentage: PrimField<f64>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: self.percentage,
            fixed_delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef
    {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
    #[doc = "Get a reference to the value of field `fixed_delay` after provisioning.\n"]    pub fn fixed_delay (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef >{
        ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDynamic {
    abort: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl,
        >,
    >,
    delay: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<
        Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<
        Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortEl >>,
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
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayEl >>,
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
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> ListRef<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElAbortElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }
    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> ListRef<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElDelayElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    backend_service: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    #[doc = "The RegionBackendService resource being mirrored to."]
    pub backend_service: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl {
            backend_service: self.backend_service,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nThe RegionBackendService resource being mirrored to."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl
{
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef
    {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElDynamic {
    per_try_timeout: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    num_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_try_timeout: Option<
        Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    #[doc = "Set the field `num_retries`.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn set_num_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.num_retries = Some(v.into());
        self
    }
    #[doc = "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n- 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nfor example: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n- gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n- connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n- retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n- refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n- cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n- deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n- resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n- unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_try_timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl {
            num_retries: core::default::Default::default(),
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n- 5xx: Loadbalancer will attempt a retry if the backend service responds with\nany 5xx response code, or if the backend service does not respond at all,\nfor example: disconnects, reset, read timeout, connection failure, and refused\nstreams.\n- gateway-error: Similar to 5xx, but only applies to response codes\n502, 503 or 504.\n- connect-failure: Loadbalancer will retry on failures\nconnecting to backend services, for example due to connection timeouts.\n- retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\nCurrently the only retriable error supported is 409.\n- refused-stream: Loadbalancer will retry if the backend service resets the stream with a\nREFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n- cancelled: Loadbalancer will retry if the gRPC status code in the response\nheader is set to cancelled\n- deadline-exceeded: Loadbalancer will retry if the\ngRPC status code in the response header is set to deadline-exceeded\n- resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\nheader is set to resource-exhausted\n- unavailable: Loadbalancer will retry if\nthe gRPC status code in the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> ListRef<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElPerTryTimeoutElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.per_try_timeout", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    #[doc = "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_prefix_rewrite", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl
{
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { }
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl
{
    #[doc = "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc = "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc = "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { header_name : self . header_name , header_value : self . header_value , replace : self . replace , } } }
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl
{
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { }
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl
{
    #[doc = "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc = "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc = "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { header_name : self . header_name , header_value : self . header_value , replace : self . replace , } } }
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElDynamic { request_headers_to_add : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl >> , response_headers_to_add : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl { # [serde (skip_serializing_if = "Option::is_none")] request_headers_to_remove : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] response_headers_to_remove : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] request_headers_to_add : Option < Vec < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl > > , # [serde (skip_serializing_if = "Option::is_none")] response_headers_to_add : Option < Vec < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl > > , dynamic : ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElDynamic , }
impl
    ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl
{
    #[doc = "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl
{}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl { ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl { request_headers_to_remove : core :: default :: Default :: default () , response_headers_to_remove : core :: default :: Default :: default () , request_headers_to_add : core :: default :: Default :: default () , response_headers_to_add : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef { ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."] pub fn request_headers_to_remove (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.request_headers_to_remove" , self . base)) } # [doc = "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."] pub fn response_headers_to_remove (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.response_headers_to_remove" , self . base)) } # [doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"] pub fn request_headers_to_add (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.request_headers_to_add" , self . base)) } # [doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"] pub fn response_headers_to_add (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.response_headers_to_add" , self . base)) } }
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElDynamic { header_action : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl { backend_service : PrimField < String > , weight : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] header_action : Option < Vec < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl > > , dynamic : ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElDynamic , }
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    #[doc = "The default RegionBackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub backend_service: PrimField<String>,
    #[doc = "Specifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub weight: PrimField<f64>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl {
            backend_service: self.backend_service,
            weight: self.weight,
            header_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nThe default RegionBackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]    pub fn header_action (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElHeaderActionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElDynamic {
    cors_policy:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>>,
    fault_injection_policy: Option<
        DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl>,
    >,
    request_mirror_policy: Option<
        DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl>,
    >,
    retry_policy:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>>,
    timeout: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>>,
    url_rewrite:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>>,
    weighted_backend_services: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_policy: Option<Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy:
        Option<Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mirror_policy:
        Option<Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_backend_services: Option<
        Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyEl>,
        >,
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
    #[doc = "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyEl,
            >,
        >,
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
    #[doc = "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyEl,
            >,
        >,
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
    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyEl>,
        >,
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
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteEl>,
        >,
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
    #[doc = "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_backend_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElCorsPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElFaultInjectionPolicyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fault_injection_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRequestMirrorPolicyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_mirror_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElWeightedBackendServicesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weighted_backend_services", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_response_code: Option<PrimField<String>>,
    strip_query: PrimField<bool>,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    #[doc = "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_response_code = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    #[doc = "If set to true, any accompanying query portion of the original URL is removed\nprior to redirecting the request. If set to false, the query portion of the\noriginal URL is retained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub strip_query: PrimField<bool>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
        ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: self.strip_query,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redirect_response_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is removed\nprior to redirecting the request. If set to false, the query portion of the\noriginal URL is retained.\n This field is required to ensure an empty block is not set. The normal default value is false."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElPathRuleElDynamic {
    route_action: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl>>,
    url_redirect: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElPathRuleEl {
    paths: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_action: Option<Vec<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_redirect: Option<Vec<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElPathRuleElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElPathRuleEl {
    #[doc = "Set the field `service`.\nThe region backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.route_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.route_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_redirect`.\n"]
    pub fn set_url_redirect(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_redirect = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElPathRuleEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElPathRuleEl {
    #[doc = "The list of path patterns to match. Each must start with / and the only place a\n\\* is allowed is at the end following a /. The string fed to the path matcher\ndoes not include any text after the first ? or #, and those chars are not\nallowed here."]
    pub paths: SetField<PrimField<String>>,
}
impl BuildComputeRegionUrlMapPathMatcherElPathRuleEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElPathRuleEl {
        ComputeRegionUrlMapPathMatcherElPathRuleEl {
            paths: self.paths,
            service: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElPathRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElPathRuleElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapPathMatcherElPathRuleElRef {
        ComputeRegionUrlMapPathMatcherElPathRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElPathRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `paths` after provisioning.\nThe list of path patterns to match. Each must start with / and the only place a\n\\* is allowed is at the end following a /. The string fed to the path matcher\ndoes not include any text after the first ? or #, and those chars are not\nallowed here."]
    pub fn paths(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe region backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRouteActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }
    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElUrlRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    #[doc = "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc = "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc = "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
    #[doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    #[doc = "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc = "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc = "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: self.replace,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
    #[doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub fn replace(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElDynamic {
    request_headers_to_add: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl,
        >,
    >,
    response_headers_to_add: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_remove: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_add: Option<
        Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_headers_to_add: Option<
        Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    #[doc = "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl {
            request_headers_to_remove: core::default::Default::default(),
            response_headers_to_remove: core::default::Default::default(),
            request_headers_to_add: core::default::Default::default(),
            response_headers_to_add: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn request_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn response_headers_to_remove(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_remove", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRequestHeadersToAddElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_headers_to_add", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"]
    pub fn response_headers_to_add(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElResponseHeadersToAddElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.response_headers_to_add", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    range_end: PrimField<f64>,
    range_start: PrimField<f64>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl
{
    #[doc = "The end of the range (exclusive)."]
    pub range_end: PrimField<f64>,
    #[doc = "The start of the range (inclusive)."]
    pub range_start: PrimField<f64>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl {
            range_end: self.range_end,
            range_start: self.range_start,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef
    {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `range_end` after provisioning.\nThe end of the range (exclusive)."]
    pub fn range_end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_end", self.base))
    }
    #[doc = "Get a reference to the value of field `range_start` after provisioning.\nThe start of the range (inclusive)."]
    pub fn range_start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.range_start", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElDynamic {
    range_match: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<PrimField<String>>,
    header_name: PrimField<String>,
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
    range_match: Option<
        Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    #[doc = "Set the field `exact_match`.\nThe value should exactly match contents of exactMatch. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn set_exact_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact_match = Some(v.into());
        self
    }
    #[doc = "Set the field `invert_match`.\nIf set to false, the headerMatch is considered a match if the match criteria\nabove are met. If set to true, the headerMatch is considered a match if the\nmatch criteria above are NOT met. Defaults to false."]
    pub fn set_invert_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.invert_match = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_match`.\nThe value of the header must start with the contents of prefixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn set_prefix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_match = Some(v.into());
        self
    }
    #[doc = "Set the field `present_match`.\nA header with the contents of headerName must exist. The match takes place\nwhether or not the request's header has a value or not. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn set_present_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.present_match = Some(v.into());
        self
    }
    #[doc = "Set the field `regex_match`.\nThe value of the header must match the regular expression specified in\nregexMatch. For regular expression grammar, please see:\nen.cppreference.com/w/cpp/regex/ecmascript  For matching against a port\nspecified in the HTTP request, use a headerMatch with headerName set to PORT and\na regular expression that satisfies the RFC2616 Host header's port specifier.\nOnly one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or\nrangeMatch must be set."]
    pub fn set_regex_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_match = Some(v.into());
        self
    }
    #[doc = "Set the field `suffix_match`.\nThe value of the header must end with the contents of suffixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn set_suffix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix_match = Some(v.into());
        self
    }
    #[doc = "Set the field `range_match`.\n"]
    pub fn set_range_match(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchEl,
            >,
        >,
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
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    type O =
        BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    #[doc = "The name of the HTTP header to match. For matching against the HTTP request's\nauthority, use a headerMatch with the header name \":authority\". For matching a\nrequest's method, use the headerName \":method\"."]
    pub header_name: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl {
            exact_match: core::default::Default::default(),
            header_name: self.header_name,
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
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact_match` after provisioning.\nThe value should exactly match contents of exactMatch. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn exact_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the HTTP header to match. For matching against the HTTP request's\nauthority, use a headerMatch with the header name \":authority\". For matching a\nrequest's method, use the headerName \":method\"."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
    #[doc = "Get a reference to the value of field `invert_match` after provisioning.\nIf set to false, the headerMatch is considered a match if the match criteria\nabove are met. If set to true, the headerMatch is considered a match if the\nmatch criteria above are NOT met. Defaults to false."]
    pub fn invert_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.invert_match", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_match` after provisioning.\nThe value of the header must start with the contents of prefixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn prefix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }
    #[doc = "Get a reference to the value of field `present_match` after provisioning.\nA header with the contents of headerName must exist. The match takes place\nwhether or not the request's header has a value or not. Only one of exactMatch,\nprefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set."]
    pub fn present_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.present_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regex_match` after provisioning.\nThe value of the header must match the regular expression specified in\nregexMatch. For regular expression grammar, please see:\nen.cppreference.com/w/cpp/regex/ecmascript  For matching against a port\nspecified in the HTTP request, use a headerMatch with headerName set to PORT and\na regular expression that satisfies the RFC2616 Host header's port specifier.\nOnly one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or\nrangeMatch must be set."]
    pub fn regex_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }
    #[doc = "Get a reference to the value of field `suffix_match` after provisioning.\nThe value of the header must end with the contents of suffixMatch. Only one of\nexactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch\nmust be set."]
    pub fn suffix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix_match", self.base))
    }
    #[doc = "Get a reference to the value of field `range_match` after provisioning.\n"]
    pub fn range_match(
        &self,
    ) -> ListRef<
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRangeMatchElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.range_match", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    name: PrimField<String>,
    value: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl
{
    #[doc = "Name of metadata label. The name can have a maximum length of 1024 characters\nand must be at least 1 character long."]
    pub name: PrimField<String>,
    #[doc = "The value of the label must match the specified value. value can have a maximum\nlength of 1024 characters."]
    pub value: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl
    {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl {
            name: self.name,
            value: self.value,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef
    {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of metadata label. The name can have a maximum length of 1024 characters\nand must be at least 1 character long."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the label must match the specified value. value can have a maximum\nlength of 1024 characters."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElDynamic {
    filter_labels: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    filter_match_criteria: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_labels: Option<
        Vec<
            ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl,
        >,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    #[doc = "Set the field `filter_labels`.\n"]
    pub fn set_filter_labels(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_labels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_labels = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    type O =
        BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    #[doc = "Specifies how individual filterLabel matches within the list of filterLabels\ncontribute towards the overall metadataFilter match. Supported values are:\n\n* MATCH_ANY: At least one of the filterLabels must have a matching label in the\nprovided metadata.\n* MATCH_ALL: All filterLabels must have matching labels in\nthe provided metadata. Possible values: [\"MATCH_ALL\", \"MATCH_ANY\"]"]
    pub filter_match_criteria: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl {
            filter_match_criteria: self.filter_match_criteria,
            filter_labels: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter_match_criteria` after provisioning.\nSpecifies how individual filterLabel matches within the list of filterLabels\ncontribute towards the overall metadataFilter match. Supported values are:\n\n* MATCH_ANY: At least one of the filterLabels must have a matching label in the\nprovided metadata.\n* MATCH_ALL: All filterLabels must have matching labels in\nthe provided metadata. Possible values: [\"MATCH_ALL\", \"MATCH_ANY\"]"]
    pub fn filter_match_criteria(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_match_criteria", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_labels` after provisioning.\n"]
    pub fn filter_labels(
        &self,
    ) -> ListRef<
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElFilterLabelsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_labels", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_match: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    present_match: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_match: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    #[doc = "Set the field `exact_match`.\nThe queryParameterMatch matches if the value of the parameter exactly matches\nthe contents of exactMatch. Only one of presentMatch, exactMatch and regexMatch\nmust be set."]
    pub fn set_exact_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact_match = Some(v.into());
        self
    }
    #[doc = "Set the field `present_match`.\nSpecifies that the queryParameterMatch matches if the request contains the query\nparameter, irrespective of whether the parameter has a value or not. Only one of\npresentMatch, exactMatch and regexMatch must be set."]
    pub fn set_present_match(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.present_match = Some(v.into());
        self
    }
    #[doc = "Set the field `regex_match`.\nThe queryParameterMatch matches if the value of the parameter matches the\nregular expression specified by regexMatch. For the regular expression grammar,\nplease see en.cppreference.com/w/cpp/regex/ecmascript  Only one of presentMatch,\nexactMatch and regexMatch must be set."]
    pub fn set_regex_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_match = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    #[doc = "The name of the query parameter to match. The query parameter must exist in the\nrequest, in the absence of which the request match fails."]
    pub name: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl {
            exact_match: core::default::Default::default(),
            name: self.name,
            present_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exact_match` after provisioning.\nThe queryParameterMatch matches if the value of the parameter exactly matches\nthe contents of exactMatch. Only one of presentMatch, exactMatch and regexMatch\nmust be set."]
    pub fn exact_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact_match", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the query parameter to match. The query parameter must exist in the\nrequest, in the absence of which the request match fails."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `present_match` after provisioning.\nSpecifies that the queryParameterMatch matches if the request contains the query\nparameter, irrespective of whether the parameter has a value or not. Only one of\npresentMatch, exactMatch and regexMatch must be set."]
    pub fn present_match(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.present_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regex_match` after provisioning.\nThe queryParameterMatch matches if the value of the parameter matches the\nregular expression specified by regexMatch. For the regular expression grammar,\nplease see en.cppreference.com/w/cpp/regex/ecmascript  Only one of presentMatch,\nexactMatch and regexMatch must be set."]
    pub fn regex_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElDynamic {
    header_matches: Option<
        DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>,
    >,
    metadata_filters: Option<
        DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>,
    >,
    query_parameter_matches: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    full_path_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_case: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_template_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_match: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_matches:
        Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_filters:
        Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_parameter_matches: Option<
        Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    #[doc = "Set the field `full_path_match`.\nFor satisfying the matchRule condition, the path of the request must exactly\nmatch the value specified in fullPathMatch after removing any query parameters\nand anchor that may be part of the original URL. FullPathMatch must be between 1\nand 1024 characters. Only one of prefixMatch, fullPathMatch or regexMatch must\nbe specified."]
    pub fn set_full_path_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.full_path_match = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_case`.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive.\nDefaults to false."]
    pub fn set_ignore_case(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_case = Some(v.into());
        self
    }
    #[doc = "Set the field `path_template_match`.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn set_path_template_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_template_match = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_match`.\nFor satisfying the matchRule condition, the request's path must begin with the\nspecified prefixMatch. prefixMatch must begin with a /. The value must be\nbetween 1 and 1024 characters. Only one of prefixMatch, fullPathMatch or\nregexMatch must be specified."]
    pub fn set_prefix_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_match = Some(v.into());
        self
    }
    #[doc = "Set the field `regex_match`.\nFor satisfying the matchRule condition, the path of the request must satisfy the\nregular expression specified in regexMatch after removing any query parameters\nand anchor supplied with the original URL. For regular expression grammar please\nsee en.cppreference.com/w/cpp/regex/ecmascript  Only one of prefixMatch,\nfullPathMatch or regexMatch must be specified."]
    pub fn set_regex_match(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.regex_match = Some(v.into());
        self
    }
    #[doc = "Set the field `header_matches`.\n"]
    pub fn set_header_matches(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_matches = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_matches = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `metadata_filters`.\n"]
    pub fn set_metadata_filters(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.metadata_filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_filters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_parameter_matches`.\n"]
    pub fn set_query_parameter_matches(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_parameter_matches = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_parameter_matches = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl {
            full_path_match: core::default::Default::default(),
            ignore_case: core::default::Default::default(),
            path_template_match: core::default::Default::default(),
            prefix_match: core::default::Default::default(),
            regex_match: core::default::Default::default(),
            header_matches: core::default::Default::default(),
            metadata_filters: core::default::Default::default(),
            query_parameter_matches: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `full_path_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request must exactly\nmatch the value specified in fullPathMatch after removing any query parameters\nand anchor that may be part of the original URL. FullPathMatch must be between 1\nand 1024 characters. Only one of prefixMatch, fullPathMatch or regexMatch must\nbe specified."]
    pub fn full_path_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_path_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_case` after provisioning.\nSpecifies that prefixMatch and fullPathMatch matches are case sensitive.\nDefaults to false."]
    pub fn ignore_case(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_case", self.base))
    }
    #[doc = "Get a reference to the value of field `path_template_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request\nmust match the wildcard pattern specified in pathTemplateMatch\nafter removing any query parameters and anchor that may be part\nof the original URL.\n\npathTemplateMatch must be between 1 and 255 characters\n(inclusive).  The pattern specified by pathTemplateMatch may\nhave at most 5 wildcard operators and at most 5 variable\ncaptures in total."]
    pub fn path_template_match(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_template_match", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_match` after provisioning.\nFor satisfying the matchRule condition, the request's path must begin with the\nspecified prefixMatch. prefixMatch must begin with a /. The value must be\nbetween 1 and 1024 characters. Only one of prefixMatch, fullPathMatch or\nregexMatch must be specified."]
    pub fn prefix_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_match", self.base))
    }
    #[doc = "Get a reference to the value of field `regex_match` after provisioning.\nFor satisfying the matchRule condition, the path of the request must satisfy the\nregular expression specified in regexMatch after removing any query parameters\nand anchor supplied with the original URL. For regular expression grammar please\nsee en.cppreference.com/w/cpp/regex/ecmascript  Only one of prefixMatch,\nfullPathMatch or regexMatch must be specified."]
    pub fn regex_match(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.regex_match", self.base))
    }
    #[doc = "Get a reference to the value of field `header_matches` after provisioning.\n"]
    pub fn header_matches(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElHeaderMatchesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_matches", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_filters` after provisioning.\n"]
    pub fn metadata_filters(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElMetadataFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata_filters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_parameter_matches` after provisioning.\n"]
    pub fn query_parameter_matches(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElQueryParameterMatchesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_parameter_matches", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
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
    max_age: Option<PrimField<f64>>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    #[doc = "Set the field `allow_credentials`.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn set_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_headers`.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn set_allow_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_methods`.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn set_allow_methods(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origin_regexes`.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origin_regexes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origin_regexes = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_origins`.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn set_allow_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allow_origins = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nIf true, specifies the CORS policy is disabled.\nwhich indicates that the CORS policy is in effect. Defaults to false."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `expose_headers`.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `max_age`.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn set_max_age(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl {
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
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_credentials` after provisioning.\nIn response to a preflight request, setting this to true indicates that the\nactual request can include user credentials. This translates to the Access-\nControl-Allow-Credentials header. Defaults to false."]
    pub fn allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_headers` after provisioning.\nSpecifies the content for the Access-Control-Allow-Headers header."]
    pub fn allow_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_methods` after provisioning.\nSpecifies the content for the Access-Control-Allow-Methods header."]
    pub fn allow_methods(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origin_regexes` after provisioning.\nSpecifies the regular expression patterns that match allowed origins. For\nregular expression grammar please see en.cppreference.com/w/cpp/regex/ecmascript\nAn origin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origin_regexes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origin_regexes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_origins` after provisioning.\nSpecifies the list of origins that will be allowed to do CORS requests. An\norigin is allowed if it matches either allow_origins or allow_origin_regex."]
    pub fn allow_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allow_origins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nIf true, specifies the CORS policy is disabled.\nwhich indicates that the CORS policy is in effect. Defaults to false."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\nSpecifies the content for the Access-Control-Expose-Headers header."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.expose_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_age` after provisioning.\nSpecifies how long the results of a preflight request can be cached. This\ntranslates to the content for the Access-Control-Max-Age header."]
    pub fn max_age(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    http_status: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage: Option<PrimField<f64>>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    #[doc = "Set the field `http_status`.\nThe HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub fn set_http_status(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.http_status = Some(v.into());
        self
    }
    #[doc = "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl
{}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl
    {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl {
            http_status: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef
    {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `http_status` after provisioning.\nThe HTTP status code used to abort the request. The value must be between 200\nand 599 inclusive."]
    pub fn http_status(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_status", self.base))
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) which will be\naborted as part of fault injection. The value must be between 0.0 and 100.0\ninclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { # [doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."] pub fn set_nanos (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . nanos = Some (v . into ()) ; self } }
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl
{
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl { nanos : core :: default :: Default :: default () , seconds : self . seconds , } } }
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."] pub fn nanos (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.nanos" , self . base)) } # [doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."] pub fn seconds (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.seconds" , self . base)) } }
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElDynamic { fixed_delay : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl { # [serde (skip_serializing_if = "Option::is_none")] percentage : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] fixed_delay : Option < Vec < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl > > , dynamic : ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElDynamic , }
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    #[doc = "Set the field `percentage`.\nThe percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub fn set_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percentage = Some(v.into());
        self
    }
    #[doc = "Set the field `fixed_delay`.\n"]
    pub fn set_fixed_delay(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_delay = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_delay = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl
{}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl
    {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl {
            percentage: core::default::Default::default(),
            fixed_delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef
    {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `percentage` after provisioning.\nThe percentage of traffic (connections/operations/requests) on which delay will\nbe introduced as part of fault injection. The value must be between 0.0 and\n100.0 inclusive."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
    #[doc = "Get a reference to the value of field `fixed_delay` after provisioning.\n"]    pub fn fixed_delay (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElFixedDelayElRef >{
        ListRef::new(self.shared().clone(), format!("{}.fixed_delay", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDynamic {
    abort: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl,
        >,
    >,
    delay: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    abort: Option<
        Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    delay: Option<
        Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    #[doc = "Set the field `abort`.\n"]
    pub fn set_abort(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortEl >>,
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
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayEl >>,
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
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl {
            abort: core::default::Default::default(),
            delay: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `abort` after provisioning.\n"]
    pub fn abort(
        &self,
    ) -> ListRef<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElAbortElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.abort", self.base))
    }
    #[doc = "Get a reference to the value of field `delay` after provisioning.\n"]
    pub fn delay(
        &self,
    ) -> ListRef<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElDelayElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.delay", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    backend_service: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    #[doc = "The RegionBackendService resource being mirrored to."]
    pub backend_service: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl {
            backend_service: self.backend_service,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nThe RegionBackendService resource being mirrored to."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl
{
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef
    {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElDynamic {
    per_try_timeout: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    num_retries: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_conditions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    per_try_timeout: Option<
        Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    #[doc = "Set the field `retry_conditions`.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  for example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
    pub fn set_retry_conditions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.retry_conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `per_try_timeout`.\n"]
    pub fn set_per_try_timeout(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.per_try_timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.per_try_timeout = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    type O =
        BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    #[doc = "Specifies the allowed number retries. This number must be > 0."]
    pub num_retries: PrimField<f64>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl {
            num_retries: self.num_retries,
            retry_conditions: core::default::Default::default(),
            per_try_timeout: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `num_retries` after provisioning.\nSpecifies the allowed number retries. This number must be > 0."]
    pub fn num_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.num_retries", self.base))
    }
    #[doc = "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more conditions when this retry rule applies. Valid values are:\n\n* 5xx: Loadbalancer will attempt a retry if the backend service responds with\n  any 5xx response code, or if the backend service does not respond at all,\n  for example: disconnects, reset, read timeout, connection failure, and refused\n  streams.\n* gateway-error: Similar to 5xx, but only applies to response codes\n  502, 503 or 504.\n* connect-failure: Loadbalancer will retry on failures\n  connecting to backend services, for example due to connection timeouts.\n* retriable-4xx: Loadbalancer will retry for retriable 4xx response codes.\n  Currently the only retriable error supported is 409.\n* refused-stream: Loadbalancer will retry if the backend service resets the stream with a\n  REFUSED_STREAM error code. This reset type indicates that it is safe to retry.\n* cancelled: Loadbalancer will retry if the gRPC status code in the response\n  header is set to cancelled\n* deadline-exceeded: Loadbalancer will retry if the\n  gRPC status code in the response header is set to deadline-exceeded\n* resource-exhausted: Loadbalancer will retry if the gRPC status code in the response\n  header is set to resource-exhausted\n* unavailable: Loadbalancer will retry if the gRPC status code in\n  the response header is set to unavailable"]
    pub fn retry_conditions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.retry_conditions", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `per_try_timeout` after provisioning.\n"]
    pub fn per_try_timeout(
        &self,
    ) -> ListRef<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElPerTryTimeoutElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.per_try_timeout", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub seconds: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations\nless than one second are represented with a 0 'seconds' field and a positive\n'nanos' field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000\ninclusive."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_prefix_rewrite: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_template_rewrite: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    #[doc = "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn set_host_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `path_prefix_rewrite`.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn set_path_prefix_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_prefix_rewrite = Some(v.into());
        self
    }
    #[doc = "Set the field `path_template_rewrite`.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite and pathTemplateRewrite may be\nspecified."]
    pub fn set_path_template_rewrite(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_template_rewrite = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl {
            host_rewrite: core::default::Default::default(),
            path_prefix_rewrite: core::default::Default::default(),
            path_template_rewrite: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected service, the request's host\nheader is replaced with contents of hostRewrite. The value must be between 1 and\n255 characters."]
    pub fn host_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `path_prefix_rewrite` after provisioning.\nPrior to forwarding the request to the selected backend service, the matching\nportion of the request's path is replaced by pathPrefixRewrite. The value must\nbe between 1 and 1024 characters."]
    pub fn path_prefix_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_prefix_rewrite", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_template_rewrite` after provisioning.\nPrior to forwarding the request to the selected origin, if the\nrequest matched a pathTemplateMatch, the matching portion of the\nrequest's path is replaced re-written using the pattern specified\nby pathTemplateRewrite.\n\npathTemplateRewrite must be between 1 and 255 characters\n(inclusive), must start with a '/', and must only use variables\ncaptured by the route's pathTemplate matchers.\n\npathTemplateRewrite may only be used when all of a route's\nMatchRules specify pathTemplate.\n\nOnly one of pathPrefixRewrite and pathTemplateRewrite may be\nspecified."]
    pub fn path_template_rewrite(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_template_rewrite", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl
{
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { }
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl
{
    #[doc = "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc = "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc = "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl { header_name : self . header_name , header_value : self . header_value , replace : self . replace , } } }
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl
{
    header_name: PrimField<String>,
    header_value: PrimField<String>,
    replace: PrimField<bool>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { }
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl
{
    #[doc = "The name of the header."]
    pub header_name: PrimField<String>,
    #[doc = "The value of the header to add."]
    pub header_value: PrimField<String>,
    #[doc = "If false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."]
    pub replace: PrimField<bool>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl { header_name : self . header_name , header_value : self . header_value , replace : self . replace , } } }
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header."] pub fn header_name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_name" , self . base)) } # [doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."] pub fn header_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.header_value" , self . base)) } # [doc = "Get a reference to the value of field `replace` after provisioning.\nIf false, headerValue is appended to any values that already exist for the\nheader. If true, headerValue is set for the header, discarding any values that\nwere set for that header."] pub fn replace (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.replace" , self . base)) } }
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElDynamic { request_headers_to_add : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl >> , response_headers_to_add : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl { # [serde (skip_serializing_if = "Option::is_none")] request_headers_to_remove : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] response_headers_to_remove : Option < ListField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] request_headers_to_add : Option < Vec < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl > > , # [serde (skip_serializing_if = "Option::is_none")] response_headers_to_add : Option < Vec < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl > > , dynamic : ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElDynamic , }
impl
    ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl
{
    #[doc = "Set the field `request_headers_to_remove`.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."]
    pub fn set_request_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.request_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `response_headers_to_remove`.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."]
    pub fn set_response_headers_to_remove(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.response_headers_to_remove = Some(v.into());
        self
    }
    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response_headers_to_add`.\n"]
    pub fn set_response_headers_to_add(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response_headers_to_add = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response_headers_to_add = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl { type O = BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl
{}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl { pub fn build (self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl { ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl { request_headers_to_remove : core :: default :: Default :: default () , response_headers_to_remove : core :: default :: Default :: default () , request_headers_to_add : core :: default :: Default :: default () , response_headers_to_add : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef { fn new (shared : StackShared , base : String) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef { ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef { shared : shared , base : base . to_string () , } } }
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `request_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the request\nprior to forwarding the request to the backendService."] pub fn request_headers_to_remove (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.request_headers_to_remove" , self . base)) } # [doc = "Get a reference to the value of field `response_headers_to_remove` after provisioning.\nA list of header names for headers that need to be removed from the response\nprior to sending the response back to the client."] pub fn response_headers_to_remove (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.response_headers_to_remove" , self . base)) } # [doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"] pub fn request_headers_to_add (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRequestHeadersToAddElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.request_headers_to_add" , self . base)) } # [doc = "Get a reference to the value of field `response_headers_to_add` after provisioning.\n"] pub fn response_headers_to_add (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElResponseHeadersToAddElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.response_headers_to_add" , self . base)) } }
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElDynamic { header_action : Option < DynamicBlock < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl >> , }
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl { backend_service : PrimField < String > , weight : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] header_action : Option < Vec < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl > > , dynamic : ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElDynamic , }
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v : impl Into < BlockAssignable < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl
{
    type O = BlockAssignable<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    #[doc = "The default RegionBackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub backend_service: PrimField<String>,
    #[doc = "Specifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub weight: PrimField<f64>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
    pub fn build(
        self,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl {
            backend_service: self.backend_service,
            weight: self.weight,
            header_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend_service` after provisioning.\nThe default RegionBackendService resource. Before\nforwarding the request to backendService, the loadbalancer applies any relevant\nheaderActions specified as part of this backendServiceWeight."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nSpecifies the fraction of traffic sent to backendService, computed as weight /\n(sum of all weightedBackendService weights in routeAction) . The selection of a\nbackend service is determined only for new traffic. Once a user's request has\nbeen directed to a backendService, subsequent requests will be sent to the same\nbackendService as determined by the BackendService's session affinity policy.\nThe value must be between 0 and 1000"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]    pub fn header_action (& self) -> ListRef < ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElHeaderActionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElDynamic {
    cors_policy:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>>,
    fault_injection_policy: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl,
        >,
    >,
    request_mirror_policy: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl,
        >,
    >,
    retry_policy: Option<
        DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>,
    >,
    timeout:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>>,
    url_rewrite:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>>,
    weighted_backend_services: Option<
        DynamicBlock<
            ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_policy: Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fault_injection_policy: Option<
        Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_mirror_policy:
        Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy:
        Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_rewrite: Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_backend_services: Option<
        Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl>,
    >,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
    #[doc = "Set the field `cors_policy`.\n"]
    pub fn set_cors_policy(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyEl>,
        >,
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
    #[doc = "Set the field `fault_injection_policy`.\n"]
    pub fn set_fault_injection_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyEl,
            >,
        >,
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
    #[doc = "Set the field `request_mirror_policy`.\n"]
    pub fn set_request_mirror_policy(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyEl,
            >,
        >,
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
    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyEl>,
        >,
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
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.timeout = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.timeout = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v: impl Into<
            BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteEl>,
        >,
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
    #[doc = "Set the field `weighted_backend_services`.\n"]
    pub fn set_weighted_backend_services(
        mut self,
        v: impl Into<
            BlockAssignable<
                ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.weighted_backend_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_backend_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl {
            cors_policy: core::default::Default::default(),
            fault_injection_policy: core::default::Default::default(),
            request_mirror_policy: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            weighted_backend_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cors_policy` after provisioning.\n"]
    pub fn cors_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElCorsPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `fault_injection_policy` after provisioning.\n"]
    pub fn fault_injection_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElFaultInjectionPolicyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fault_injection_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_mirror_policy` after provisioning.\n"]
    pub fn request_mirror_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRequestMirrorPolicyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.request_mirror_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElUrlRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
    #[doc = "Get a reference to the value of field `weighted_backend_services` after provisioning.\n"]
    pub fn weighted_backend_services(
        &self,
    ) -> ListRef<
        ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElWeightedBackendServicesElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.weighted_backend_services", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    host_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    https_redirect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_redirect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_response_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strip_query: Option<PrimField<bool>>,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    #[doc = "Set the field `host_redirect`.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn set_host_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `https_redirect`.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn set_https_redirect(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.https_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `path_redirect`.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn set_path_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_redirect`.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn set_prefix_redirect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_redirect = Some(v.into());
        self
    }
    #[doc = "Set the field `redirect_response_code`.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn set_redirect_response_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.redirect_response_code = Some(v.into());
        self
    }
    #[doc = "Set the field `strip_query`.\nIf set to true, any accompanying query portion of the original URL is\nremoved prior to redirecting the request. If set to false, the query\nportion of the original URL is retained. The default value is false."]
    pub fn set_strip_query(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.strip_query = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl {
            host_redirect: core::default::Default::default(),
            https_redirect: core::default::Default::default(),
            path_redirect: core::default::Default::default(),
            prefix_redirect: core::default::Default::default(),
            redirect_response_code: core::default::Default::default(),
            strip_query: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `host_redirect` after provisioning.\nThe host that will be used in the redirect response instead of the one\nthat was supplied in the request. The value must be between 1 and 255\ncharacters."]
    pub fn host_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `https_redirect` after provisioning.\nIf set to true, the URL scheme in the redirected request is set to https.\nIf set to false, the URL scheme of the redirected request will remain the\nsame as that of the request. This must only be set for UrlMaps used in\nTargetHttpProxys. Setting this true for TargetHttpsProxy is not\npermitted. The default is set to false."]
    pub fn https_redirect(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.https_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_redirect` after provisioning.\nThe path that will be used in the redirect response instead of the one\nthat was supplied in the request. pathRedirect cannot be supplied\ntogether with prefixRedirect. Supply one alone or neither. If neither is\nsupplied, the path of the original request will be used for the redirect.\nThe value must be between 1 and 1024 characters."]
    pub fn path_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.path_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `prefix_redirect` after provisioning.\nThe prefix that replaces the prefixMatch specified in the\nHttpRouteRuleMatch, retaining the remaining portion of the URL before\nredirecting the request. prefixRedirect cannot be supplied together with\npathRedirect. Supply one alone or neither. If neither is supplied, the\npath of the original request will be used for the redirect. The value\nmust be between 1 and 1024 characters."]
    pub fn prefix_redirect(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.prefix_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_response_code` after provisioning.\nThe HTTP Status code to use for this RedirectAction. Supported values are:\n\n* MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301.\n\n* FOUND, which corresponds to 302.\n\n* SEE_OTHER which corresponds to 303.\n\n* TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method\nwill be retained.\n\n* PERMANENT_REDIRECT, which corresponds to 308. In this case,\nthe request method will be retained. Possible values: [\"FOUND\", \"MOVED_PERMANENTLY_DEFAULT\", \"PERMANENT_REDIRECT\", \"SEE_OTHER\", \"TEMPORARY_REDIRECT\"]"]
    pub fn redirect_response_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.redirect_response_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strip_query` after provisioning.\nIf set to true, any accompanying query portion of the original URL is\nremoved prior to redirecting the request. If set to false, the query\nportion of the original URL is retained. The default value is false."]
    pub fn strip_query(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.strip_query", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElRouteRulesElDynamic {
    header_action: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl>>,
    match_rules: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl>>,
    route_action: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl>>,
    url_redirect: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesEl {
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_rules: Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_action: Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_redirect: Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElRouteRulesElDynamic,
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesEl {
    #[doc = "Set the field `service`.\nThe region backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `match_rules`.\n"]
    pub fn set_match_rules(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `route_action`.\n"]
    pub fn set_route_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.route_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.route_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `url_redirect`.\n"]
    pub fn set_url_redirect(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_redirect = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherElRouteRulesEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherElRouteRulesEl {
    #[doc = "For routeRules within a given pathMatcher, priority determines the order\nin which load balancer will interpret routeRules. RouteRules are evaluated\nin order of priority, from the lowest to highest number. The priority of\na rule decreases as its number increases (1, 2, 3, N+1). The first rule\nthat matches the request is applied.\n\nYou cannot configure two or more routeRules with the same priority.\nPriority for each rule must be set to a number between 0 and\n2147483647 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules\nin the future without affecting the rest of the rules. For example,\n1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which\nyou could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the\nfuture without any impact on existing rules."]
    pub priority: PrimField<f64>,
}
impl BuildComputeRegionUrlMapPathMatcherElRouteRulesEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherElRouteRulesEl {
        ComputeRegionUrlMapPathMatcherElRouteRulesEl {
            priority: self.priority,
            service: core::default::Default::default(),
            header_action: core::default::Default::default(),
            match_rules: core::default::Default::default(),
            route_action: core::default::Default::default(),
            url_redirect: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
        ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRouteRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nFor routeRules within a given pathMatcher, priority determines the order\nin which load balancer will interpret routeRules. RouteRules are evaluated\nin order of priority, from the lowest to highest number. The priority of\na rule decreases as its number increases (1, 2, 3, N+1). The first rule\nthat matches the request is applied.\n\nYou cannot configure two or more routeRules with the same priority.\nPriority for each rule must be set to a number between 0 and\n2147483647 inclusive.\n\nPriority numbers can have gaps, which enable you to add or remove rules\nin the future without affecting the rest of the rules. For example,\n1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers to which\nyou could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the\nfuture without any impact on existing rules."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe region backend service resource to which traffic is\ndirected if this rule is matched. If routeAction is additionally specified,\nadvanced routing actions like URL Rewrites, etc. take effect prior to sending\nthe request to the backend. However, if service is specified, routeAction cannot\ncontain any weightedBackendService s. Conversely, if routeAction specifies any\nweightedBackendServices, service must not be specified. Only one of urlRedirect,\nservice or routeAction.weightedBackendService must be set."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `match_rules` after provisioning.\n"]
    pub fn match_rules(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElMatchRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match_rules", self.base))
    }
    #[doc = "Get a reference to the value of field `route_action` after provisioning.\n"]
    pub fn route_action(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRouteActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_action", self.base))
    }
    #[doc = "Get a reference to the value of field `url_redirect` after provisioning.\n"]
    pub fn url_redirect(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElUrlRedirectElRef> {
        ListRef::new(self.shared().clone(), format!("{}.url_redirect", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRegionUrlMapPathMatcherElDynamic {
    default_route_action:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultRouteActionEl>>,
    default_url_redirect:
        Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl>>,
    header_action: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElHeaderActionEl>>,
    path_rule: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElPathRuleEl>>,
    route_rules: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherElRouteRulesEl>>,
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapPathMatcherEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_route_action: Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultRouteActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_url_redirect: Option<Vec<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeRegionUrlMapPathMatcherElHeaderActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path_rule: Option<Vec<ComputeRegionUrlMapPathMatcherElPathRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_rules: Option<Vec<ComputeRegionUrlMapPathMatcherElRouteRulesEl>>,
    dynamic: ComputeRegionUrlMapPathMatcherElDynamic,
}
impl ComputeRegionUrlMapPathMatcherEl {
    #[doc = "Set the field `default_service`.\nA reference to a RegionBackendService resource. This will be used if\nnone of the pathRules defined by this PathMatcher is matched by\nthe URL's path portion."]
    pub fn set_default_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_service = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional description of this resource."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `default_route_action`.\n"]
    pub fn set_default_route_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultRouteActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_route_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_route_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_url_redirect`.\n"]
    pub fn set_default_url_redirect(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_url_redirect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_url_redirect = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `path_rule`.\n"]
    pub fn set_path_rule(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElPathRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.path_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.path_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `route_rules`.\n"]
    pub fn set_route_rules(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionUrlMapPathMatcherElRouteRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.route_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.route_rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapPathMatcherEl {
    type O = BlockAssignable<ComputeRegionUrlMapPathMatcherEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapPathMatcherEl {
    #[doc = "The name to which this PathMatcher is referred by the HostRule."]
    pub name: PrimField<String>,
}
impl BuildComputeRegionUrlMapPathMatcherEl {
    pub fn build(self) -> ComputeRegionUrlMapPathMatcherEl {
        ComputeRegionUrlMapPathMatcherEl {
            default_service: core::default::Default::default(),
            description: core::default::Default::default(),
            name: self.name,
            default_route_action: core::default::Default::default(),
            default_url_redirect: core::default::Default::default(),
            header_action: core::default::Default::default(),
            path_rule: core::default::Default::default(),
            route_rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapPathMatcherElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapPathMatcherElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapPathMatcherElRef {
        ComputeRegionUrlMapPathMatcherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapPathMatcherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_service` after provisioning.\nA reference to a RegionBackendService resource. This will be used if\nnone of the pathRules defined by this PathMatcher is matched by\nthe URL's path portion."]
    pub fn default_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_service", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name to which this PathMatcher is referred by the HostRule."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `default_route_action` after provisioning.\n"]
    pub fn default_route_action(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultRouteActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_route_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_url_redirect` after provisioning.\n"]
    pub fn default_url_redirect(
        &self,
    ) -> ListRef<ComputeRegionUrlMapPathMatcherElDefaultUrlRedirectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_url_redirect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeRegionUrlMapPathMatcherElHeaderActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_action", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path_rule` after provisioning.\n"]
    pub fn path_rule(&self) -> ListRef<ComputeRegionUrlMapPathMatcherElPathRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.path_rule", self.base))
    }
    #[doc = "Get a reference to the value of field `route_rules` after provisioning.\n"]
    pub fn route_rules(&self) -> ListRef<ComputeRegionUrlMapPathMatcherElRouteRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.route_rules", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapTestEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    host: PrimField<String>,
    path: PrimField<String>,
    service: PrimField<String>,
}
impl ComputeRegionUrlMapTestEl {
    #[doc = "Set the field `description`.\nDescription of this test case."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionUrlMapTestEl {
    type O = BlockAssignable<ComputeRegionUrlMapTestEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapTestEl {
    #[doc = "Host portion of the URL."]
    pub host: PrimField<String>,
    #[doc = "Path portion of the URL."]
    pub path: PrimField<String>,
    #[doc = "A reference to expected RegionBackendService resource the given URL should be mapped to."]
    pub service: PrimField<String>,
}
impl BuildComputeRegionUrlMapTestEl {
    pub fn build(self) -> ComputeRegionUrlMapTestEl {
        ComputeRegionUrlMapTestEl {
            description: core::default::Default::default(),
            host: self.host,
            path: self.path,
            service: self.service,
        }
    }
}
pub struct ComputeRegionUrlMapTestElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapTestElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapTestElRef {
        ComputeRegionUrlMapTestElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapTestElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of this test case."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\nHost portion of the URL."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath portion of the URL."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nA reference to expected RegionBackendService resource the given URL should be mapped to."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionUrlMapTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRegionUrlMapTimeoutsEl {
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
impl ToListMappable for ComputeRegionUrlMapTimeoutsEl {
    type O = BlockAssignable<ComputeRegionUrlMapTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionUrlMapTimeoutsEl {}
impl BuildComputeRegionUrlMapTimeoutsEl {
    pub fn build(self) -> ComputeRegionUrlMapTimeoutsEl {
        ComputeRegionUrlMapTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionUrlMapTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionUrlMapTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionUrlMapTimeoutsElRef {
        ComputeRegionUrlMapTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionUrlMapTimeoutsElRef {
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
struct ComputeRegionUrlMapDynamic {
    default_route_action: Option<DynamicBlock<ComputeRegionUrlMapDefaultRouteActionEl>>,
    default_url_redirect: Option<DynamicBlock<ComputeRegionUrlMapDefaultUrlRedirectEl>>,
    header_action: Option<DynamicBlock<ComputeRegionUrlMapHeaderActionEl>>,
    host_rule: Option<DynamicBlock<ComputeRegionUrlMapHostRuleEl>>,
    path_matcher: Option<DynamicBlock<ComputeRegionUrlMapPathMatcherEl>>,
    test: Option<DynamicBlock<ComputeRegionUrlMapTestEl>>,
}
