use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataContainerEngineVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_prefix: Option<PrimField<String>>,
}
struct DataContainerEngineVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataContainerEngineVersionsData>,
}
#[derive(Clone)]
pub struct DataContainerEngineVersions(Rc<DataContainerEngineVersions_>);
impl DataContainerEngineVersions {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `version_prefix`.\n"]
    pub fn set_version_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version_prefix = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `default_cluster_version` after provisioning.\n"]
    pub fn default_cluster_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_cluster_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `latest_master_version` after provisioning.\n"]
    pub fn latest_master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_master_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_node_version` after provisioning.\n"]
    pub fn latest_node_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_node_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel_default_version` after provisioning.\n"]
    pub fn release_channel_default_version(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.release_channel_default_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel_latest_version` after provisioning.\n"]
    pub fn release_channel_latest_version(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.release_channel_latest_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel_upgrade_target_version` after provisioning.\n"]
    pub fn release_channel_upgrade_target_version(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!(
                "{}.release_channel_upgrade_target_version",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `valid_master_versions` after provisioning.\n"]
    pub fn valid_master_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.valid_master_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `valid_node_versions` after provisioning.\n"]
    pub fn valid_node_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.valid_node_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_prefix` after provisioning.\n"]
    pub fn version_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_prefix", self.extract_ref()),
        )
    }
}
impl Referable for DataContainerEngineVersions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataContainerEngineVersions {}
impl ToListMappable for DataContainerEngineVersions {
    type O = ListRef<DataContainerEngineVersionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataContainerEngineVersions_ {
    fn extract_datasource_type(&self) -> String {
        "google_container_engine_versions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataContainerEngineVersions {
    pub tf_id: String,
}
impl BuildDataContainerEngineVersions {
    pub fn build(self, stack: &mut Stack) -> DataContainerEngineVersions {
        let out = DataContainerEngineVersions(Rc::new(DataContainerEngineVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataContainerEngineVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                version_prefix: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataContainerEngineVersionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataContainerEngineVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataContainerEngineVersionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `default_cluster_version` after provisioning.\n"]
    pub fn default_cluster_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_cluster_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `latest_master_version` after provisioning.\n"]
    pub fn latest_master_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_master_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `latest_node_version` after provisioning.\n"]
    pub fn latest_node_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_node_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel_default_version` after provisioning.\n"]
    pub fn release_channel_default_version(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.release_channel_default_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel_latest_version` after provisioning.\n"]
    pub fn release_channel_latest_version(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.release_channel_latest_version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `release_channel_upgrade_target_version` after provisioning.\n"]
    pub fn release_channel_upgrade_target_version(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!(
                "{}.release_channel_upgrade_target_version",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `valid_master_versions` after provisioning.\n"]
    pub fn valid_master_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.valid_master_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `valid_node_versions` after provisioning.\n"]
    pub fn valid_node_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.valid_node_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version_prefix` after provisioning.\n"]
    pub fn version_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version_prefix", self.extract_ref()),
        )
    }
}
