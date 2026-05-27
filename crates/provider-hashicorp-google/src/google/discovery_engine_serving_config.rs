use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineServingConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost_control_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_id: Option<PrimField<String>>,
    engine_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_control_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promote_control_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_control_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serving_config_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms_control_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineServingConfigTimeoutsEl>,
}
struct DiscoveryEngineServingConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineServingConfigData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineServingConfig(Rc<DiscoveryEngineServingConfig_>);
impl DiscoveryEngineServingConfig {
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
    #[doc = "Set the field `boost_control_ids`.\nThe resource IDs of the boost controls to be applied."]
    pub fn set_boost_control_ids(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().boost_control_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `collection_id`.\nThe collection ID. Currently only accepts \"default_collection\"."]
    pub fn set_collection_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().collection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_control_ids`.\nThe resource IDs of the filter controls to be applied."]
    pub fn set_filter_control_ids(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().filter_control_ids = Some(v.into());
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
    #[doc = "Set the field `promote_control_ids`.\nThe resource IDs of the promote controls to be applied."]
    pub fn set_promote_control_ids(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().promote_control_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `redirect_control_ids`.\nThe resource IDs of the redirect controls to be applied."]
    pub fn set_redirect_control_ids(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().redirect_control_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `serving_config_id`.\n'The unique ID of the serving config. Currently only accepts \"default_search\".'"]
    pub fn set_serving_config_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().serving_config_id = Some(v.into());
        self
    }
    #[doc = "Set the field `synonyms_control_ids`.\nThe resource IDs of the synonyms controls to be applied."]
    pub fn set_synonyms_control_ids(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().synonyms_control_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineServingConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `boost_control_ids` after provisioning.\nThe resource IDs of the boost controls to be applied."]
    pub fn boost_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boost_control_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID. Currently only accepts \"default_collection\"."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe ID of the engine associated with the serving config."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter_control_ids` after provisioning.\nThe resource IDs of the filter controls to be applied."]
    pub fn filter_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_control_ids", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the serving config. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/servingConfigs/{serving_config_id}'."]
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
    #[doc = "Get a reference to the value of field `promote_control_ids` after provisioning.\nThe resource IDs of the promote controls to be applied."]
    pub fn promote_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.promote_control_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_control_ids` after provisioning.\nThe resource IDs of the redirect controls to be applied."]
    pub fn redirect_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_control_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serving_config_id` after provisioning.\n'The unique ID of the serving config. Currently only accepts \"default_search\".'"]
    pub fn serving_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serving_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `synonyms_control_ids` after provisioning.\nThe resource IDs of the synonyms controls to be applied."]
    pub fn synonyms_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.synonyms_control_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineServingConfigTimeoutsElRef {
        DiscoveryEngineServingConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineServingConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineServingConfig {}
impl ToListMappable for DiscoveryEngineServingConfig {
    type O = ListRef<DiscoveryEngineServingConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineServingConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_serving_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineServingConfig {
    pub tf_id: String,
    #[doc = "The ID of the engine associated with the serving config."]
    pub engine_id: PrimField<String>,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineServingConfig {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineServingConfig {
        let out = DiscoveryEngineServingConfig(Rc::new(DiscoveryEngineServingConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineServingConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                boost_control_ids: core::default::Default::default(),
                collection_id: core::default::Default::default(),
                engine_id: self.engine_id,
                filter_control_ids: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                promote_control_ids: core::default::Default::default(),
                redirect_control_ids: core::default::Default::default(),
                serving_config_id: core::default::Default::default(),
                synonyms_control_ids: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineServingConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineServingConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineServingConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boost_control_ids` after provisioning.\nThe resource IDs of the boost controls to be applied."]
    pub fn boost_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boost_control_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID. Currently only accepts \"default_collection\"."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe ID of the engine associated with the serving config."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter_control_ids` after provisioning.\nThe resource IDs of the filter controls to be applied."]
    pub fn filter_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_control_ids", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the serving config. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/servingConfigs/{serving_config_id}'."]
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
    #[doc = "Get a reference to the value of field `promote_control_ids` after provisioning.\nThe resource IDs of the promote controls to be applied."]
    pub fn promote_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.promote_control_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `redirect_control_ids` after provisioning.\nThe resource IDs of the redirect controls to be applied."]
    pub fn redirect_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.redirect_control_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `serving_config_id` after provisioning.\n'The unique ID of the serving config. Currently only accepts \"default_search\".'"]
    pub fn serving_config_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serving_config_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `synonyms_control_ids` after provisioning.\nThe resource IDs of the synonyms controls to be applied."]
    pub fn synonyms_control_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.synonyms_control_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineServingConfigTimeoutsElRef {
        DiscoveryEngineServingConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineServingConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineServingConfigTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineServingConfigTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineServingConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineServingConfigTimeoutsEl {}
impl BuildDiscoveryEngineServingConfigTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineServingConfigTimeoutsEl {
        DiscoveryEngineServingConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineServingConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineServingConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineServingConfigTimeoutsElRef {
        DiscoveryEngineServingConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineServingConfigTimeoutsElRef {
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
