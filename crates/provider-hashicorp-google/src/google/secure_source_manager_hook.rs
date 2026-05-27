use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecureSourceManagerHookData {
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
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    events: Option<ListField<PrimField<String>>>,
    hook_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    repository_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitive_query_string: Option<PrimField<String>>,
    target_uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_option: Option<Vec<SecureSourceManagerHookPushOptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SecureSourceManagerHookTimeoutsEl>,
    dynamic: SecureSourceManagerHookDynamic,
}
struct SecureSourceManagerHook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecureSourceManagerHookData>,
}
#[derive(Clone)]
pub struct SecureSourceManagerHook(Rc<SecureSourceManagerHook_>);
impl SecureSourceManagerHook {
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
    #[doc = "Set the field `disabled`.\nDetermines if the hook disabled or not.\nSet to true to stop sending traffic."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `events`.\nThe events that trigger hook on. Possible values: [\"PUSH\", \"PULL_REQUEST\"]"]
    pub fn set_events(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().events = Some(v.into());
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
    #[doc = "Set the field `sensitive_query_string`.\nThe sensitive query string to be appended to the target URI."]
    pub fn set_sensitive_query_string(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().sensitive_query_string = Some(v.into());
        self
    }
    #[doc = "Set the field `push_option`.\n"]
    pub fn set_push_option(
        self,
        v: impl Into<BlockAssignable<SecureSourceManagerHookPushOptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().push_option = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.push_option = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SecureSourceManagerHookTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate timestamp."]
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
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nDetermines if the hook disabled or not.\nSet to true to stop sending traffic."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `events` after provisioning.\nThe events that trigger hook on. Possible values: [\"PUSH\", \"PULL_REQUEST\"]"]
    pub fn events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.events", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hook_id` after provisioning.\nThe ID for the Hook."]
    pub fn hook_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hook_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Repository."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for a Hook. The name should be of the format:\n'projects/{project}/locations/{location_id}/repositories/{repository_id}/hooks/{hook_id}'"]
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
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe ID for the Repository."]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sensitive_query_string` after provisioning.\nThe sensitive query string to be appended to the target URI."]
    pub fn sensitive_query_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sensitive_query_string", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_uri` after provisioning.\nThe target URI to which the payloads will be delivered."]
    pub fn target_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the hook."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate timestamp."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `push_option` after provisioning.\n"]
    pub fn push_option(&self) -> ListRef<SecureSourceManagerHookPushOptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.push_option", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerHookTimeoutsElRef {
        SecureSourceManagerHookTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SecureSourceManagerHook {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecureSourceManagerHook {}
impl ToListMappable for SecureSourceManagerHook {
    type O = ListRef<SecureSourceManagerHookRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecureSourceManagerHook_ {
    fn extract_resource_type(&self) -> String {
        "google_secure_source_manager_hook".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecureSourceManagerHook {
    pub tf_id: String,
    #[doc = "The ID for the Hook."]
    pub hook_id: PrimField<String>,
    #[doc = "The location for the Repository."]
    pub location: PrimField<String>,
    #[doc = "The ID for the Repository."]
    pub repository_id: PrimField<String>,
    #[doc = "The target URI to which the payloads will be delivered."]
    pub target_uri: PrimField<String>,
}
impl BuildSecureSourceManagerHook {
    pub fn build(self, stack: &mut Stack) -> SecureSourceManagerHook {
        let out = SecureSourceManagerHook(Rc::new(SecureSourceManagerHook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SecureSourceManagerHookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                disabled: core::default::Default::default(),
                events: core::default::Default::default(),
                hook_id: self.hook_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                repository_id: self.repository_id,
                sensitive_query_string: core::default::Default::default(),
                target_uri: self.target_uri,
                push_option: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecureSourceManagerHookRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerHookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecureSourceManagerHookRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate timestamp."]
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
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nDetermines if the hook disabled or not.\nSet to true to stop sending traffic."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `events` after provisioning.\nThe events that trigger hook on. Possible values: [\"PUSH\", \"PULL_REQUEST\"]"]
    pub fn events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.events", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hook_id` after provisioning.\nThe ID for the Hook."]
    pub fn hook_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.hook_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Repository."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nA unique identifier for a Hook. The name should be of the format:\n'projects/{project}/locations/{location_id}/repositories/{repository_id}/hooks/{hook_id}'"]
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
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\nThe ID for the Repository."]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sensitive_query_string` after provisioning.\nThe sensitive query string to be appended to the target URI."]
    pub fn sensitive_query_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sensitive_query_string", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target_uri` after provisioning.\nThe target URI to which the payloads will be delivered."]
    pub fn target_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the hook."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate timestamp."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `push_option` after provisioning.\n"]
    pub fn push_option(&self) -> ListRef<SecureSourceManagerHookPushOptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.push_option", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerHookTimeoutsElRef {
        SecureSourceManagerHookTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecureSourceManagerHookPushOptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_filter: Option<PrimField<String>>,
}
impl SecureSourceManagerHookPushOptionEl {
    #[doc = "Set the field `branch_filter`.\nTrigger hook for matching branches only.\nSpecified as glob pattern. If empty or *, events for all branches are\nreported. Examples: main, {main,release*}.\nSee https://pkg.go.dev/github.com/gobwas/glob documentation."]
    pub fn set_branch_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.branch_filter = Some(v.into());
        self
    }
}
impl ToListMappable for SecureSourceManagerHookPushOptionEl {
    type O = BlockAssignable<SecureSourceManagerHookPushOptionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerHookPushOptionEl {}
impl BuildSecureSourceManagerHookPushOptionEl {
    pub fn build(self) -> SecureSourceManagerHookPushOptionEl {
        SecureSourceManagerHookPushOptionEl {
            branch_filter: core::default::Default::default(),
        }
    }
}
pub struct SecureSourceManagerHookPushOptionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerHookPushOptionElRef {
    fn new(shared: StackShared, base: String) -> SecureSourceManagerHookPushOptionElRef {
        SecureSourceManagerHookPushOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerHookPushOptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `branch_filter` after provisioning.\nTrigger hook for matching branches only.\nSpecified as glob pattern. If empty or *, events for all branches are\nreported. Examples: main, {main,release*}.\nSee https://pkg.go.dev/github.com/gobwas/glob documentation."]
    pub fn branch_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.branch_filter", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SecureSourceManagerHookTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SecureSourceManagerHookTimeoutsEl {
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
impl ToListMappable for SecureSourceManagerHookTimeoutsEl {
    type O = BlockAssignable<SecureSourceManagerHookTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecureSourceManagerHookTimeoutsEl {}
impl BuildSecureSourceManagerHookTimeoutsEl {
    pub fn build(self) -> SecureSourceManagerHookTimeoutsEl {
        SecureSourceManagerHookTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SecureSourceManagerHookTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecureSourceManagerHookTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SecureSourceManagerHookTimeoutsElRef {
        SecureSourceManagerHookTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecureSourceManagerHookTimeoutsElRef {
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
struct SecureSourceManagerHookDynamic {
    push_option: Option<DynamicBlock<SecureSourceManagerHookPushOptionEl>>,
}
