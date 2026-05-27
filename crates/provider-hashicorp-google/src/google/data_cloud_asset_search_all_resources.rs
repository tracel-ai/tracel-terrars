use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudAssetSearchAllResourcesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<PrimField<String>>,
    scope: PrimField<String>,
}
struct DataCloudAssetSearchAllResources_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudAssetSearchAllResourcesData>,
}
#[derive(Clone)]
pub struct DataCloudAssetSearchAllResources(Rc<DataCloudAssetSearchAllResources_>);
impl DataCloudAssetSearchAllResources {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `asset_types`.\n"]
    pub fn set_asset_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().asset_types = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `query`.\n"]
    pub fn set_query(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().query = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `asset_types` after provisioning.\n"]
    pub fn asset_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asset_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `results` after provisioning.\n"]
    pub fn results(&self) -> ListRef<DataCloudAssetSearchAllResourcesResultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.results", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudAssetSearchAllResources {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudAssetSearchAllResources {}
impl ToListMappable for DataCloudAssetSearchAllResources {
    type O = ListRef<DataCloudAssetSearchAllResourcesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudAssetSearchAllResources_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_asset_search_all_resources".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudAssetSearchAllResources {
    pub tf_id: String,
    #[doc = ""]
    pub scope: PrimField<String>,
}
impl BuildDataCloudAssetSearchAllResources {
    pub fn build(self, stack: &mut Stack) -> DataCloudAssetSearchAllResources {
        let out = DataCloudAssetSearchAllResources(Rc::new(DataCloudAssetSearchAllResources_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudAssetSearchAllResourcesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                asset_types: core::default::Default::default(),
                id: core::default::Default::default(),
                query: core::default::Default::default(),
                scope: self.scope,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCloudAssetSearchAllResourcesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudAssetSearchAllResourcesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudAssetSearchAllResourcesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `asset_types` after provisioning.\n"]
    pub fn asset_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.asset_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `results` after provisioning.\n"]
    pub fn results(&self) -> ListRef<DataCloudAssetSearchAllResourcesResultsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.results", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scope", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudAssetSearchAllResourcesResultsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    folders: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    organization: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_asset_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_full_resource_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataCloudAssetSearchAllResourcesResultsEl {
    #[doc = "Set the field `asset_type`.\n"]
    pub fn set_asset_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.asset_type = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `folders`.\n"]
    pub fn set_folders(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.folders = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_keys`.\n"]
    pub fn set_kms_keys(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.kms_keys = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tags`.\n"]
    pub fn set_network_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.network_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `organization`.\n"]
    pub fn set_organization(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.organization = Some(v.into());
        self
    }
    #[doc = "Set the field `parent_asset_type`.\n"]
    pub fn set_parent_asset_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parent_asset_type = Some(v.into());
        self
    }
    #[doc = "Set the field `parent_full_resource_name`.\n"]
    pub fn set_parent_full_resource_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parent_full_resource_name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudAssetSearchAllResourcesResultsEl {
    type O = BlockAssignable<DataCloudAssetSearchAllResourcesResultsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudAssetSearchAllResourcesResultsEl {}
impl BuildDataCloudAssetSearchAllResourcesResultsEl {
    pub fn build(self) -> DataCloudAssetSearchAllResourcesResultsEl {
        DataCloudAssetSearchAllResourcesResultsEl {
            asset_type: core::default::Default::default(),
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            folders: core::default::Default::default(),
            kms_keys: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: core::default::Default::default(),
            name: core::default::Default::default(),
            network_tags: core::default::Default::default(),
            organization: core::default::Default::default(),
            parent_asset_type: core::default::Default::default(),
            parent_full_resource_name: core::default::Default::default(),
            project: core::default::Default::default(),
            state: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataCloudAssetSearchAllResourcesResultsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudAssetSearchAllResourcesResultsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudAssetSearchAllResourcesResultsElRef {
        DataCloudAssetSearchAllResourcesResultsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudAssetSearchAllResourcesResultsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asset_type` after provisioning.\n"]
    pub fn asset_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.asset_type", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `folders` after provisioning.\n"]
    pub fn folders(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.folders", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_keys` after provisioning.\n"]
    pub fn kms_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.kms_keys", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\n"]
    pub fn network_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.network_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `organization` after provisioning.\n"]
    pub fn organization(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }
    #[doc = "Get a reference to the value of field `parent_asset_type` after provisioning.\n"]
    pub fn parent_asset_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_asset_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parent_full_resource_name` after provisioning.\n"]
    pub fn parent_full_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent_full_resource_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
