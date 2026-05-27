use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct FirebaseAppHostingTrafficData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backend: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout_policy: Option<Vec<FirebaseAppHostingTrafficRolloutPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<FirebaseAppHostingTrafficTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<FirebaseAppHostingTrafficTimeoutsEl>,
    dynamic: FirebaseAppHostingTrafficDynamic,
}
struct FirebaseAppHostingTraffic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirebaseAppHostingTrafficData>,
}
#[derive(Clone)]
pub struct FirebaseAppHostingTraffic(Rc<FirebaseAppHostingTraffic_>);
impl FirebaseAppHostingTraffic {
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
    #[doc = "Set the field `rollout_policy`.\n"]
    pub fn set_rollout_policy(
        self,
        v: impl Into<BlockAssignable<FirebaseAppHostingTrafficRolloutPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rollout_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rollout_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        self,
        v: impl Into<BlockAssignable<FirebaseAppHostingTrafficTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<FirebaseAppHostingTrafficTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nId of the backend that this Traffic config applies to"]
    pub fn backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the backend was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `current` after provisioning.\nCurrent state of traffic allocation for the backend.\nWhen setting 'target', this field may differ for some time until the desired state is reached."]
    pub fn current(&self) -> ListRef<FirebaseAppHostingTrafficCurrentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.current", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the backend was deleted."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location the Backend that this Traffic config applies to"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the backend traffic config\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}/traffic'."]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the backend was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rollout_policy` after provisioning.\n"]
    pub fn rollout_policy(&self) -> ListRef<FirebaseAppHostingTrafficRolloutPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<FirebaseAppHostingTrafficTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingTrafficTimeoutsElRef {
        FirebaseAppHostingTrafficTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for FirebaseAppHostingTraffic {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for FirebaseAppHostingTraffic {}
impl ToListMappable for FirebaseAppHostingTraffic {
    type O = ListRef<FirebaseAppHostingTrafficRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for FirebaseAppHostingTraffic_ {
    fn extract_resource_type(&self) -> String {
        "google_firebase_app_hosting_traffic".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildFirebaseAppHostingTraffic {
    pub tf_id: String,
    #[doc = "Id of the backend that this Traffic config applies to"]
    pub backend: PrimField<String>,
    #[doc = "The location the Backend that this Traffic config applies to"]
    pub location: PrimField<String>,
}
impl BuildFirebaseAppHostingTraffic {
    pub fn build(self, stack: &mut Stack) -> FirebaseAppHostingTraffic {
        let out = FirebaseAppHostingTraffic(Rc::new(FirebaseAppHostingTraffic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirebaseAppHostingTrafficData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backend: self.backend,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                rollout_policy: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct FirebaseAppHostingTrafficRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingTrafficRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl FirebaseAppHostingTrafficRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `backend` after provisioning.\nId of the backend that this Traffic config applies to"]
    pub fn backend(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.backend", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime at which the backend was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `current` after provisioning.\nCurrent state of traffic allocation for the backend.\nWhen setting 'target', this field may differ for some time until the desired state is reached."]
    pub fn current(&self) -> ListRef<FirebaseAppHostingTrafficCurrentElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.current", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nTime at which the backend was deleted."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum based on other values; may be sent\non update or delete to ensure operation is done on expected resource."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location the Backend that this Traffic config applies to"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the backend traffic config\n\nFormat:\n\n'projects/{project}/locations/{locationId}/backends/{backendId}/traffic'."]
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
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nSystem-assigned, unique identifier."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime at which the backend was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rollout_policy` after provisioning.\n"]
    pub fn rollout_policy(&self) -> ListRef<FirebaseAppHostingTrafficRolloutPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rollout_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<FirebaseAppHostingTrafficTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseAppHostingTrafficTimeoutsElRef {
        FirebaseAppHostingTrafficTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingTrafficCurrentElSplitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    build: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent: Option<PrimField<f64>>,
}
impl FirebaseAppHostingTrafficCurrentElSplitsEl {
    #[doc = "Set the field `build`.\n"]
    pub fn set_build(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.build = Some(v.into());
        self
    }
    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.percent = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingTrafficCurrentElSplitsEl {
    type O = BlockAssignable<FirebaseAppHostingTrafficCurrentElSplitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingTrafficCurrentElSplitsEl {}
impl BuildFirebaseAppHostingTrafficCurrentElSplitsEl {
    pub fn build(self) -> FirebaseAppHostingTrafficCurrentElSplitsEl {
        FirebaseAppHostingTrafficCurrentElSplitsEl {
            build: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingTrafficCurrentElSplitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingTrafficCurrentElSplitsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingTrafficCurrentElSplitsElRef {
        FirebaseAppHostingTrafficCurrentElSplitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingTrafficCurrentElSplitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `build` after provisioning.\n"]
    pub fn build(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build", self.base))
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingTrafficCurrentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    splits: Option<ListField<FirebaseAppHostingTrafficCurrentElSplitsEl>>,
}
impl FirebaseAppHostingTrafficCurrentEl {
    #[doc = "Set the field `splits`.\n"]
    pub fn set_splits(
        mut self,
        v: impl Into<ListField<FirebaseAppHostingTrafficCurrentElSplitsEl>>,
    ) -> Self {
        self.splits = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingTrafficCurrentEl {
    type O = BlockAssignable<FirebaseAppHostingTrafficCurrentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingTrafficCurrentEl {}
impl BuildFirebaseAppHostingTrafficCurrentEl {
    pub fn build(self) -> FirebaseAppHostingTrafficCurrentEl {
        FirebaseAppHostingTrafficCurrentEl {
            splits: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingTrafficCurrentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingTrafficCurrentElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingTrafficCurrentElRef {
        FirebaseAppHostingTrafficCurrentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingTrafficCurrentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `splits` after provisioning.\n"]
    pub fn splits(&self) -> ListRef<FirebaseAppHostingTrafficCurrentElSplitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.splits", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingTrafficRolloutPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    codebase_branch: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
}
impl FirebaseAppHostingTrafficRolloutPolicyEl {
    #[doc = "Set the field `codebase_branch`.\nSpecifies a branch that triggers a new build to be started with this\npolicy. If not set, no automatic rollouts will happen."]
    pub fn set_codebase_branch(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.codebase_branch = Some(v.into());
        self
    }
    #[doc = "Set the field `disabled`.\nA flag that, if true, prevents rollouts from being created via this RolloutPolicy."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
}
impl ToListMappable for FirebaseAppHostingTrafficRolloutPolicyEl {
    type O = BlockAssignable<FirebaseAppHostingTrafficRolloutPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingTrafficRolloutPolicyEl {}
impl BuildFirebaseAppHostingTrafficRolloutPolicyEl {
    pub fn build(self) -> FirebaseAppHostingTrafficRolloutPolicyEl {
        FirebaseAppHostingTrafficRolloutPolicyEl {
            codebase_branch: core::default::Default::default(),
            disabled: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingTrafficRolloutPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingTrafficRolloutPolicyElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingTrafficRolloutPolicyElRef {
        FirebaseAppHostingTrafficRolloutPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingTrafficRolloutPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `codebase_branch` after provisioning.\nSpecifies a branch that triggers a new build to be started with this\npolicy. If not set, no automatic rollouts will happen."]
    pub fn codebase_branch(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.codebase_branch", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nA flag that, if true, prevents rollouts from being created via this RolloutPolicy."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `disabled_time` after provisioning.\nIf disabled is set, the time at which the rollouts were disabled."]
    pub fn disabled_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disabled_time", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingTrafficTargetElSplitsEl {
    build: PrimField<String>,
    percent: PrimField<f64>,
}
impl FirebaseAppHostingTrafficTargetElSplitsEl {}
impl ToListMappable for FirebaseAppHostingTrafficTargetElSplitsEl {
    type O = BlockAssignable<FirebaseAppHostingTrafficTargetElSplitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingTrafficTargetElSplitsEl {
    #[doc = "The build that traffic is being routed to."]
    pub build: PrimField<String>,
    #[doc = "The percentage of traffic to send to the build. Currently must be 100 or 0."]
    pub percent: PrimField<f64>,
}
impl BuildFirebaseAppHostingTrafficTargetElSplitsEl {
    pub fn build(self) -> FirebaseAppHostingTrafficTargetElSplitsEl {
        FirebaseAppHostingTrafficTargetElSplitsEl {
            build: self.build,
            percent: self.percent,
        }
    }
}
pub struct FirebaseAppHostingTrafficTargetElSplitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingTrafficTargetElSplitsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingTrafficTargetElSplitsElRef {
        FirebaseAppHostingTrafficTargetElSplitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingTrafficTargetElSplitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `build` after provisioning.\nThe build that traffic is being routed to."]
    pub fn build(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build", self.base))
    }
    #[doc = "Get a reference to the value of field `percent` after provisioning.\nThe percentage of traffic to send to the build. Currently must be 100 or 0."]
    pub fn percent(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}
#[derive(Serialize, Default)]
struct FirebaseAppHostingTrafficTargetElDynamic {
    splits: Option<DynamicBlock<FirebaseAppHostingTrafficTargetElSplitsEl>>,
}
#[derive(Serialize)]
pub struct FirebaseAppHostingTrafficTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    splits: Option<Vec<FirebaseAppHostingTrafficTargetElSplitsEl>>,
    dynamic: FirebaseAppHostingTrafficTargetElDynamic,
}
impl FirebaseAppHostingTrafficTargetEl {
    #[doc = "Set the field `splits`.\n"]
    pub fn set_splits(
        mut self,
        v: impl Into<BlockAssignable<FirebaseAppHostingTrafficTargetElSplitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.splits = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.splits = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for FirebaseAppHostingTrafficTargetEl {
    type O = BlockAssignable<FirebaseAppHostingTrafficTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingTrafficTargetEl {}
impl BuildFirebaseAppHostingTrafficTargetEl {
    pub fn build(self) -> FirebaseAppHostingTrafficTargetEl {
        FirebaseAppHostingTrafficTargetEl {
            splits: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct FirebaseAppHostingTrafficTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingTrafficTargetElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingTrafficTargetElRef {
        FirebaseAppHostingTrafficTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingTrafficTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `splits` after provisioning.\n"]
    pub fn splits(&self) -> ListRef<FirebaseAppHostingTrafficTargetElSplitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.splits", self.base))
    }
}
#[derive(Serialize)]
pub struct FirebaseAppHostingTrafficTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl FirebaseAppHostingTrafficTimeoutsEl {
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
impl ToListMappable for FirebaseAppHostingTrafficTimeoutsEl {
    type O = BlockAssignable<FirebaseAppHostingTrafficTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildFirebaseAppHostingTrafficTimeoutsEl {}
impl BuildFirebaseAppHostingTrafficTimeoutsEl {
    pub fn build(self) -> FirebaseAppHostingTrafficTimeoutsEl {
        FirebaseAppHostingTrafficTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct FirebaseAppHostingTrafficTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for FirebaseAppHostingTrafficTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> FirebaseAppHostingTrafficTimeoutsElRef {
        FirebaseAppHostingTrafficTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl FirebaseAppHostingTrafficTimeoutsElRef {
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
struct FirebaseAppHostingTrafficDynamic {
    rollout_policy: Option<DynamicBlock<FirebaseAppHostingTrafficRolloutPolicyEl>>,
    target: Option<DynamicBlock<FirebaseAppHostingTrafficTargetEl>>,
}
