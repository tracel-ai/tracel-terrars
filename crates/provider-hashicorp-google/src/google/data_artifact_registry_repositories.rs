use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataArtifactRegistryRepositoriesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataArtifactRegistryRepositories_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataArtifactRegistryRepositoriesData>,
}
#[derive(Clone)]
pub struct DataArtifactRegistryRepositories(Rc<DataArtifactRegistryRepositories_>);
impl DataArtifactRegistryRepositories {
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
    #[doc = "Set the field `name_filter`.\n"]
    pub fn set_name_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_filter = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_filter` after provisioning.\n"]
    pub fn name_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> ListRef<DataArtifactRegistryRepositoriesRepositoriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repositories", self.extract_ref()),
        )
    }
}
impl Referable for DataArtifactRegistryRepositories {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataArtifactRegistryRepositories {}
impl ToListMappable for DataArtifactRegistryRepositories {
    type O = ListRef<DataArtifactRegistryRepositoriesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataArtifactRegistryRepositories_ {
    fn extract_datasource_type(&self) -> String {
        "google_artifact_registry_repositories".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataArtifactRegistryRepositories {
    pub tf_id: String,
    #[doc = ""]
    pub location: PrimField<String>,
}
impl BuildDataArtifactRegistryRepositories {
    pub fn build(self, stack: &mut Stack) -> DataArtifactRegistryRepositories {
        let out = DataArtifactRegistryRepositories(Rc::new(DataArtifactRegistryRepositories_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataArtifactRegistryRepositoriesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                name_filter: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataArtifactRegistryRepositoriesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryRepositoriesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataArtifactRegistryRepositoriesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name_filter` after provisioning.\n"]
    pub fn name_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name_filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repositories` after provisioning.\n"]
    pub fn repositories(&self) -> ListRef<DataArtifactRegistryRepositoriesRepositoriesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.repositories", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataArtifactRegistryRepositoriesRepositoriesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataArtifactRegistryRepositoriesRepositoriesEl {
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
    #[doc = "Set the field `format`.\n"]
    pub fn set_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.format = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `repository_id`.\n"]
    pub fn set_repository_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.repository_id = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataArtifactRegistryRepositoriesRepositoriesEl {
    type O = BlockAssignable<DataArtifactRegistryRepositoriesRepositoriesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataArtifactRegistryRepositoriesRepositoriesEl {}
impl BuildDataArtifactRegistryRepositoriesRepositoriesEl {
    pub fn build(self) -> DataArtifactRegistryRepositoriesRepositoriesEl {
        DataArtifactRegistryRepositoriesRepositoriesEl {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            format: core::default::Default::default(),
            id: core::default::Default::default(),
            repository_id: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataArtifactRegistryRepositoriesRepositoriesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryRepositoriesRepositoriesElRef {
    fn new(shared: StackShared, base: String) -> DataArtifactRegistryRepositoriesRepositoriesElRef {
        DataArtifactRegistryRepositoriesRepositoriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataArtifactRegistryRepositoriesRepositoriesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\n"]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
