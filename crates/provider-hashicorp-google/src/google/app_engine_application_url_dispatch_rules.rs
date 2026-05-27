use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppEngineApplicationUrlDispatchRulesData {
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
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_rules: Option<Vec<AppEngineApplicationUrlDispatchRulesDispatchRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppEngineApplicationUrlDispatchRulesTimeoutsEl>,
    dynamic: AppEngineApplicationUrlDispatchRulesDynamic,
}
struct AppEngineApplicationUrlDispatchRules_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppEngineApplicationUrlDispatchRulesData>,
}
#[derive(Clone)]
pub struct AppEngineApplicationUrlDispatchRules(Rc<AppEngineApplicationUrlDispatchRules_>);
impl AppEngineApplicationUrlDispatchRules {
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
    #[doc = "Set the field `dispatch_rules`.\n"]
    pub fn set_dispatch_rules(
        self,
        v: impl Into<BlockAssignable<AppEngineApplicationUrlDispatchRulesDispatchRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dispatch_rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dispatch_rules = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<AppEngineApplicationUrlDispatchRulesTimeoutsEl>,
    ) -> Self {
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
    #[doc = "Get a reference to the value of field `dispatch_rules` after provisioning.\n"]
    pub fn dispatch_rules(
        &self,
    ) -> ListRef<AppEngineApplicationUrlDispatchRulesDispatchRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dispatch_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineApplicationUrlDispatchRulesTimeoutsElRef {
        AppEngineApplicationUrlDispatchRulesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AppEngineApplicationUrlDispatchRules {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppEngineApplicationUrlDispatchRules {}
impl ToListMappable for AppEngineApplicationUrlDispatchRules {
    type O = ListRef<AppEngineApplicationUrlDispatchRulesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppEngineApplicationUrlDispatchRules_ {
    fn extract_resource_type(&self) -> String {
        "google_app_engine_application_url_dispatch_rules".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppEngineApplicationUrlDispatchRules {
    pub tf_id: String,
}
impl BuildAppEngineApplicationUrlDispatchRules {
    pub fn build(self, stack: &mut Stack) -> AppEngineApplicationUrlDispatchRules {
        let out =
            AppEngineApplicationUrlDispatchRules(Rc::new(AppEngineApplicationUrlDispatchRules_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(AppEngineApplicationUrlDispatchRulesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    project: core::default::Default::default(),
                    dispatch_rules: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppEngineApplicationUrlDispatchRulesRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineApplicationUrlDispatchRulesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppEngineApplicationUrlDispatchRulesRef {
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
    #[doc = "Get a reference to the value of field `dispatch_rules` after provisioning.\n"]
    pub fn dispatch_rules(
        &self,
    ) -> ListRef<AppEngineApplicationUrlDispatchRulesDispatchRulesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dispatch_rules", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineApplicationUrlDispatchRulesTimeoutsElRef {
        AppEngineApplicationUrlDispatchRulesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineApplicationUrlDispatchRulesDispatchRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    path: PrimField<String>,
    service: PrimField<String>,
}
impl AppEngineApplicationUrlDispatchRulesDispatchRulesEl {
    #[doc = "Set the field `domain`.\nDomain name to match against. The wildcard \"*\" is supported if specified before a period: \"*.\".\nDefaults to matching all domains: \"*\"."]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineApplicationUrlDispatchRulesDispatchRulesEl {
    type O = BlockAssignable<AppEngineApplicationUrlDispatchRulesDispatchRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineApplicationUrlDispatchRulesDispatchRulesEl {
    #[doc = "Pathname within the host. Must start with a \"/\". A single \"*\" can be included at the end of the path.\nThe sum of the lengths of the domain and path may not exceed 100 characters."]
    pub path: PrimField<String>,
    #[doc = "Pathname within the host. Must start with a \"/\". A single \"*\" can be included at the end of the path.\nThe sum of the lengths of the domain and path may not exceed 100 characters."]
    pub service: PrimField<String>,
}
impl BuildAppEngineApplicationUrlDispatchRulesDispatchRulesEl {
    pub fn build(self) -> AppEngineApplicationUrlDispatchRulesDispatchRulesEl {
        AppEngineApplicationUrlDispatchRulesDispatchRulesEl {
            domain: core::default::Default::default(),
            path: self.path,
            service: self.service,
        }
    }
}
pub struct AppEngineApplicationUrlDispatchRulesDispatchRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineApplicationUrlDispatchRulesDispatchRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppEngineApplicationUrlDispatchRulesDispatchRulesElRef {
        AppEngineApplicationUrlDispatchRulesDispatchRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineApplicationUrlDispatchRulesDispatchRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\nDomain name to match against. The wildcard \"*\" is supported if specified before a period: \"*.\".\nDefaults to matching all domains: \"*\"."]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nPathname within the host. Must start with a \"/\". A single \"*\" can be included at the end of the path.\nThe sum of the lengths of the domain and path may not exceed 100 characters."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nPathname within the host. Must start with a \"/\". A single \"*\" can be included at the end of the path.\nThe sum of the lengths of the domain and path may not exceed 100 characters."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineApplicationUrlDispatchRulesTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AppEngineApplicationUrlDispatchRulesTimeoutsEl {
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
impl ToListMappable for AppEngineApplicationUrlDispatchRulesTimeoutsEl {
    type O = BlockAssignable<AppEngineApplicationUrlDispatchRulesTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineApplicationUrlDispatchRulesTimeoutsEl {}
impl BuildAppEngineApplicationUrlDispatchRulesTimeoutsEl {
    pub fn build(self) -> AppEngineApplicationUrlDispatchRulesTimeoutsEl {
        AppEngineApplicationUrlDispatchRulesTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AppEngineApplicationUrlDispatchRulesTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineApplicationUrlDispatchRulesTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppEngineApplicationUrlDispatchRulesTimeoutsElRef {
        AppEngineApplicationUrlDispatchRulesTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineApplicationUrlDispatchRulesTimeoutsElRef {
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
struct AppEngineApplicationUrlDispatchRulesDynamic {
    dispatch_rules: Option<DynamicBlock<AppEngineApplicationUrlDispatchRulesDispatchRulesEl>>,
}
