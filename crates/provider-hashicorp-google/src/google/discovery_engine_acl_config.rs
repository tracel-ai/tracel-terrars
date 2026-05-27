use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineAclConfigData {
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
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_config: Option<Vec<DiscoveryEngineAclConfigIdpConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineAclConfigTimeoutsEl>,
    dynamic: DiscoveryEngineAclConfigDynamic,
}
struct DiscoveryEngineAclConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineAclConfigData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineAclConfig(Rc<DiscoveryEngineAclConfig_>);
impl DiscoveryEngineAclConfig {
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
    #[doc = "Set the field `idp_config`.\n"]
    pub fn set_idp_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineAclConfigIdpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().idp_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.idp_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineAclConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the aclConfig. Values are of the format\n'projects/{project}/locations/{location}/aclConfig'."]
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
    #[doc = "Get a reference to the value of field `idp_config` after provisioning.\n"]
    pub fn idp_config(&self) -> ListRef<DiscoveryEngineAclConfigIdpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.idp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineAclConfigTimeoutsElRef {
        DiscoveryEngineAclConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineAclConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineAclConfig {}
impl ToListMappable for DiscoveryEngineAclConfig {
    type O = ListRef<DiscoveryEngineAclConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineAclConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_acl_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineAclConfig {
    pub tf_id: String,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineAclConfig {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineAclConfig {
        let out = DiscoveryEngineAclConfig(Rc::new(DiscoveryEngineAclConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineAclConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                idp_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineAclConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAclConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineAclConfigRef {
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the aclConfig. Values are of the format\n'projects/{project}/locations/{location}/aclConfig'."]
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
    #[doc = "Get a reference to the value of field `idp_config` after provisioning.\n"]
    pub fn idp_config(&self) -> ListRef<DiscoveryEngineAclConfigIdpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.idp_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineAclConfigTimeoutsElRef {
        DiscoveryEngineAclConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    workforce_pool_name: Option<PrimField<String>>,
}
impl DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl {
    #[doc = "Set the field `workforce_pool_name`.\nWorkforce pool name: \"locations/global/workforcePools/pool_id\""]
    pub fn set_workforce_pool_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.workforce_pool_name = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl {
    type O = BlockAssignable<DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl {}
impl BuildDiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl {
    pub fn build(self) -> DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl {
        DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl {
            workforce_pool_name: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigElRef {
        DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `workforce_pool_name` after provisioning.\nWorkforce pool name: \"locations/global/workforcePools/pool_id\""]
    pub fn workforce_pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workforce_pool_name", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineAclConfigIdpConfigElDynamic {
    external_idp_config:
        Option<DynamicBlock<DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineAclConfigIdpConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idp_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_idp_config: Option<Vec<DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl>>,
    dynamic: DiscoveryEngineAclConfigIdpConfigElDynamic,
}
impl DiscoveryEngineAclConfigIdpConfigEl {
    #[doc = "Set the field `idp_type`.\nIdentity provider type. Possible values: [\"GSUITE\", \"THIRD_PARTY\"]"]
    pub fn set_idp_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.idp_type = Some(v.into());
        self
    }
    #[doc = "Set the field `external_idp_config`.\n"]
    pub fn set_external_idp_config(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external_idp_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external_idp_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineAclConfigIdpConfigEl {
    type O = BlockAssignable<DiscoveryEngineAclConfigIdpConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineAclConfigIdpConfigEl {}
impl BuildDiscoveryEngineAclConfigIdpConfigEl {
    pub fn build(self) -> DiscoveryEngineAclConfigIdpConfigEl {
        DiscoveryEngineAclConfigIdpConfigEl {
            idp_type: core::default::Default::default(),
            external_idp_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineAclConfigIdpConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAclConfigIdpConfigElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineAclConfigIdpConfigElRef {
        DiscoveryEngineAclConfigIdpConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineAclConfigIdpConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idp_type` after provisioning.\nIdentity provider type. Possible values: [\"GSUITE\", \"THIRD_PARTY\"]"]
    pub fn idp_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.idp_type", self.base))
    }
    #[doc = "Get a reference to the value of field `external_idp_config` after provisioning.\n"]
    pub fn external_idp_config(
        &self,
    ) -> ListRef<DiscoveryEngineAclConfigIdpConfigElExternalIdpConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_idp_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineAclConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineAclConfigTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineAclConfigTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineAclConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineAclConfigTimeoutsEl {}
impl BuildDiscoveryEngineAclConfigTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineAclConfigTimeoutsEl {
        DiscoveryEngineAclConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineAclConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineAclConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineAclConfigTimeoutsElRef {
        DiscoveryEngineAclConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineAclConfigTimeoutsElRef {
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
struct DiscoveryEngineAclConfigDynamic {
    idp_config: Option<DynamicBlock<DiscoveryEngineAclConfigIdpConfigEl>>,
}
