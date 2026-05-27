use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRouterRoutePolicyData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    router: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    terms: Option<Vec<ComputeRouterRoutePolicyTermsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRouterRoutePolicyTimeoutsEl>,
    dynamic: ComputeRouterRoutePolicyDynamic,
}
struct ComputeRouterRoutePolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRouterRoutePolicyData>,
}
#[derive(Clone)]
pub struct ComputeRouterRoutePolicy(Rc<ComputeRouterRoutePolicy_>);
impl ComputeRouterRoutePolicy {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where the router and NAT reside."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nThis is policy's type, which is one of IMPORT or EXPORT Possible values: [\"ROUTE_POLICY_TYPE_IMPORT\", \"ROUTE_POLICY_TYPE_EXPORT\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `terms`.\n"]
    pub fn set_terms(self, v: impl Into<BlockAssignable<ComputeRouterRoutePolicyTermsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().terms = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.terms = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRouterRoutePolicyTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the route policy. This policy's name, which must be a resource ID segment and unique within all policies owned by the Router"]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router and NAT reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this route policy will be configured."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThis is policy's type, which is one of IMPORT or EXPORT Possible values: [\"ROUTE_POLICY_TYPE_IMPORT\", \"ROUTE_POLICY_TYPE_EXPORT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terms` after provisioning.\n"]
    pub fn terms(&self) -> ListRef<ComputeRouterRoutePolicyTermsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.terms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterRoutePolicyTimeoutsElRef {
        ComputeRouterRoutePolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRouterRoutePolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRouterRoutePolicy {}
impl ToListMappable for ComputeRouterRoutePolicy {
    type O = ListRef<ComputeRouterRoutePolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRouterRoutePolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_router_route_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRouterRoutePolicy {
    pub tf_id: String,
    #[doc = "Name of the route policy. This policy's name, which must be a resource ID segment and unique within all policies owned by the Router"]
    pub name: PrimField<String>,
    #[doc = "The name of the Cloud Router in which this route policy will be configured."]
    pub router: PrimField<String>,
}
impl BuildComputeRouterRoutePolicy {
    pub fn build(self, stack: &mut Stack) -> ComputeRouterRoutePolicy {
        let out = ComputeRouterRoutePolicy(Rc::new(ComputeRouterRoutePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRouterRoutePolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                router: self.router,
                type_: core::default::Default::default(),
                terms: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRouterRoutePolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterRoutePolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRouterRoutePolicyRef {
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
    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nThe fingerprint used for optimistic locking of this resource.  Used\ninternally during updates."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the route policy. This policy's name, which must be a resource ID segment and unique within all policies owned by the Router"]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where the router and NAT reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `router` after provisioning.\nThe name of the Cloud Router in which this route policy will be configured."]
    pub fn router(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.router", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThis is policy's type, which is one of IMPORT or EXPORT Possible values: [\"ROUTE_POLICY_TYPE_IMPORT\", \"ROUTE_POLICY_TYPE_EXPORT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terms` after provisioning.\n"]
    pub fn terms(&self) -> ListRef<ComputeRouterRoutePolicyTermsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.terms", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRouterRoutePolicyTimeoutsElRef {
        ComputeRouterRoutePolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRouterRoutePolicyTermsElActionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl ComputeRouterRoutePolicyTermsElActionsEl {
    #[doc = "Set the field `description`.\nDescription of the expression"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nString indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its\npurpose."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouterRoutePolicyTermsElActionsEl {
    type O = BlockAssignable<ComputeRouterRoutePolicyTermsElActionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterRoutePolicyTermsElActionsEl {
    #[doc = "Textual representation of an expression in Common Expression\nLanguage syntax."]
    pub expression: PrimField<String>,
}
impl BuildComputeRouterRoutePolicyTermsElActionsEl {
    pub fn build(self) -> ComputeRouterRoutePolicyTermsElActionsEl {
        ComputeRouterRoutePolicyTermsElActionsEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouterRoutePolicyTermsElActionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterRoutePolicyTermsElActionsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterRoutePolicyTermsElActionsElRef {
        ComputeRouterRoutePolicyTermsElActionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterRoutePolicyTermsElActionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression\nLanguage syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error\nreporting, e.g. a file name and a position in the file"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its\npurpose."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouterRoutePolicyTermsElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl ComputeRouterRoutePolicyTermsElMatchEl {
    #[doc = "Set the field `description`.\nDescription of the expression"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRouterRoutePolicyTermsElMatchEl {
    type O = BlockAssignable<ComputeRouterRoutePolicyTermsElMatchEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterRoutePolicyTermsElMatchEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildComputeRouterRoutePolicyTermsElMatchEl {
    pub fn build(self) -> ComputeRouterRoutePolicyTermsElMatchEl {
        ComputeRouterRoutePolicyTermsElMatchEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouterRoutePolicyTermsElMatchElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterRoutePolicyTermsElMatchElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterRoutePolicyTermsElMatchElRef {
        ComputeRouterRoutePolicyTermsElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterRoutePolicyTermsElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeRouterRoutePolicyTermsElDynamic {
    actions: Option<DynamicBlock<ComputeRouterRoutePolicyTermsElActionsEl>>,
    match_: Option<DynamicBlock<ComputeRouterRoutePolicyTermsElMatchEl>>,
}
#[derive(Serialize)]
pub struct ComputeRouterRoutePolicyTermsEl {
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<Vec<ComputeRouterRoutePolicyTermsElActionsEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<ComputeRouterRoutePolicyTermsElMatchEl>>,
    dynamic: ComputeRouterRoutePolicyTermsElDynamic,
}
impl ComputeRouterRoutePolicyTermsEl {
    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        mut self,
        v: impl Into<BlockAssignable<ComputeRouterRoutePolicyTermsElActionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.actions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.actions = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl Into<BlockAssignable<ComputeRouterRoutePolicyTermsElMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeRouterRoutePolicyTermsEl {
    type O = BlockAssignable<ComputeRouterRoutePolicyTermsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterRoutePolicyTermsEl {
    #[doc = "The evaluation priority for this term, which must be between 0 (inclusive) and 2147483648 (exclusive), and unique within the list."]
    pub priority: PrimField<f64>,
}
impl BuildComputeRouterRoutePolicyTermsEl {
    pub fn build(self) -> ComputeRouterRoutePolicyTermsEl {
        ComputeRouterRoutePolicyTermsEl {
            priority: self.priority,
            actions: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeRouterRoutePolicyTermsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterRoutePolicyTermsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterRoutePolicyTermsElRef {
        ComputeRouterRoutePolicyTermsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterRoutePolicyTermsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `priority` after provisioning.\nThe evaluation priority for this term, which must be between 0 (inclusive) and 2147483648 (exclusive), and unique within the list."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> ListRef<ComputeRouterRoutePolicyTermsElActionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }
    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeRouterRoutePolicyTermsElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRouterRoutePolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeRouterRoutePolicyTimeoutsEl {
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
impl ToListMappable for ComputeRouterRoutePolicyTimeoutsEl {
    type O = BlockAssignable<ComputeRouterRoutePolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRouterRoutePolicyTimeoutsEl {}
impl BuildComputeRouterRoutePolicyTimeoutsEl {
    pub fn build(self) -> ComputeRouterRoutePolicyTimeoutsEl {
        ComputeRouterRoutePolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeRouterRoutePolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRouterRoutePolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRouterRoutePolicyTimeoutsElRef {
        ComputeRouterRoutePolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRouterRoutePolicyTimeoutsElRef {
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
struct ComputeRouterRoutePolicyDynamic {
    terms: Option<DynamicBlock<ComputeRouterRoutePolicyTermsEl>>,
}
