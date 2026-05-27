use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppEngineServiceSplitTrafficData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    migrate_traffic: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split: Option<Vec<AppEngineServiceSplitTrafficSplitEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppEngineServiceSplitTrafficTimeoutsEl>,
    dynamic: AppEngineServiceSplitTrafficDynamic,
}
struct AppEngineServiceSplitTraffic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppEngineServiceSplitTrafficData>,
}
#[derive(Clone)]
pub struct AppEngineServiceSplitTraffic(Rc<AppEngineServiceSplitTraffic_>);
impl AppEngineServiceSplitTraffic {
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
    #[doc = "Set the field `migrate_traffic`.\nIf set to true traffic will be migrated to this version."]
    pub fn set_migrate_traffic(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().migrate_traffic = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `split`.\n"]
    pub fn set_split(
        self,
        v: impl Into<BlockAssignable<AppEngineServiceSplitTrafficSplitEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().split = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.split = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppEngineServiceSplitTrafficTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `migrate_traffic` after provisioning.\nIf set to true traffic will be migrated to this version."]
    pub fn migrate_traffic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.migrate_traffic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service these settings apply to."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `split` after provisioning.\n"]
    pub fn split(&self) -> ListRef<AppEngineServiceSplitTrafficSplitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.split", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineServiceSplitTrafficTimeoutsElRef {
        AppEngineServiceSplitTrafficTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AppEngineServiceSplitTraffic {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppEngineServiceSplitTraffic {}
impl ToListMappable for AppEngineServiceSplitTraffic {
    type O = ListRef<AppEngineServiceSplitTrafficRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppEngineServiceSplitTraffic_ {
    fn extract_resource_type(&self) -> String {
        "google_app_engine_service_split_traffic".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppEngineServiceSplitTraffic {
    pub tf_id: String,
    #[doc = "The name of the service these settings apply to."]
    pub service: PrimField<String>,
}
impl BuildAppEngineServiceSplitTraffic {
    pub fn build(self, stack: &mut Stack) -> AppEngineServiceSplitTraffic {
        let out = AppEngineServiceSplitTraffic(Rc::new(AppEngineServiceSplitTraffic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppEngineServiceSplitTrafficData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                migrate_traffic: core::default::Default::default(),
                project: core::default::Default::default(),
                service: self.service,
                split: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppEngineServiceSplitTrafficRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineServiceSplitTrafficRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppEngineServiceSplitTrafficRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `migrate_traffic` after provisioning.\nIf set to true traffic will be migrated to this version."]
    pub fn migrate_traffic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.migrate_traffic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of the service these settings apply to."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `split` after provisioning.\n"]
    pub fn split(&self) -> ListRef<AppEngineServiceSplitTrafficSplitElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.split", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineServiceSplitTrafficTimeoutsElRef {
        AppEngineServiceSplitTrafficTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineServiceSplitTrafficSplitEl {
    allocations: RecField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shard_by: Option<PrimField<String>>,
}
impl AppEngineServiceSplitTrafficSplitEl {
    #[doc = "Set the field `shard_by`.\nMechanism used to determine which version a request is sent to. The traffic selection algorithm will be stable for either type until allocations are changed. Possible values: [\"UNSPECIFIED\", \"COOKIE\", \"IP\", \"RANDOM\"]"]
    pub fn set_shard_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.shard_by = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineServiceSplitTrafficSplitEl {
    type O = BlockAssignable<AppEngineServiceSplitTrafficSplitEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineServiceSplitTrafficSplitEl {
    #[doc = "Mapping from version IDs within the service to fractional (0.000, 1] allocations of traffic for that version. Each version can be specified only once, but some versions in the service may not have any traffic allocation. Services that have traffic allocated cannot be deleted until either the service is deleted or their traffic allocation is removed. Allocations must sum to 1. Up to two decimal place precision is supported for IP-based splits and up to three decimal places is supported for cookie-based splits."]
    pub allocations: RecField<PrimField<String>>,
}
impl BuildAppEngineServiceSplitTrafficSplitEl {
    pub fn build(self) -> AppEngineServiceSplitTrafficSplitEl {
        AppEngineServiceSplitTrafficSplitEl {
            allocations: self.allocations,
            shard_by: core::default::Default::default(),
        }
    }
}
pub struct AppEngineServiceSplitTrafficSplitElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineServiceSplitTrafficSplitElRef {
    fn new(shared: StackShared, base: String) -> AppEngineServiceSplitTrafficSplitElRef {
        AppEngineServiceSplitTrafficSplitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineServiceSplitTrafficSplitElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allocations` after provisioning.\nMapping from version IDs within the service to fractional (0.000, 1] allocations of traffic for that version. Each version can be specified only once, but some versions in the service may not have any traffic allocation. Services that have traffic allocated cannot be deleted until either the service is deleted or their traffic allocation is removed. Allocations must sum to 1. Up to two decimal place precision is supported for IP-based splits and up to three decimal places is supported for cookie-based splits."]
    pub fn allocations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.allocations", self.base))
    }
    #[doc = "Get a reference to the value of field `shard_by` after provisioning.\nMechanism used to determine which version a request is sent to. The traffic selection algorithm will be stable for either type until allocations are changed. Possible values: [\"UNSPECIFIED\", \"COOKIE\", \"IP\", \"RANDOM\"]"]
    pub fn shard_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.shard_by", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineServiceSplitTrafficTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AppEngineServiceSplitTrafficTimeoutsEl {
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
impl ToListMappable for AppEngineServiceSplitTrafficTimeoutsEl {
    type O = BlockAssignable<AppEngineServiceSplitTrafficTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineServiceSplitTrafficTimeoutsEl {}
impl BuildAppEngineServiceSplitTrafficTimeoutsEl {
    pub fn build(self) -> AppEngineServiceSplitTrafficTimeoutsEl {
        AppEngineServiceSplitTrafficTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AppEngineServiceSplitTrafficTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineServiceSplitTrafficTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppEngineServiceSplitTrafficTimeoutsElRef {
        AppEngineServiceSplitTrafficTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineServiceSplitTrafficTimeoutsElRef {
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
struct AppEngineServiceSplitTrafficDynamic {
    split: Option<DynamicBlock<AppEngineServiceSplitTrafficSplitEl>>,
}
