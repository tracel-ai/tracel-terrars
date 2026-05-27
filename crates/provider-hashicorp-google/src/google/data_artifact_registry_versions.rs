use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataArtifactRegistryVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    package_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    repository_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view: Option<PrimField<String>>,
}
struct DataArtifactRegistryVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataArtifactRegistryVersionsData>,
}
#[derive(Clone)]
pub struct DataArtifactRegistryVersions(Rc<DataArtifactRegistryVersions_>);
impl DataArtifactRegistryVersions {
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
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
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
    #[doc = "Set the field `view`.\n"]
    pub fn set_view(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().view = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `package_name` after provisioning.\n"]
    pub fn package_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\n"]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> ListRef<DataArtifactRegistryVersionsVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view", self.extract_ref()),
        )
    }
}
impl Referable for DataArtifactRegistryVersions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataArtifactRegistryVersions {}
impl ToListMappable for DataArtifactRegistryVersions {
    type O = ListRef<DataArtifactRegistryVersionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataArtifactRegistryVersions_ {
    fn extract_datasource_type(&self) -> String {
        "google_artifact_registry_versions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataArtifactRegistryVersions {
    pub tf_id: String,
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = ""]
    pub package_name: PrimField<String>,
    #[doc = ""]
    pub repository_id: PrimField<String>,
}
impl BuildDataArtifactRegistryVersions {
    pub fn build(self, stack: &mut Stack) -> DataArtifactRegistryVersions {
        let out = DataArtifactRegistryVersions(Rc::new(DataArtifactRegistryVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataArtifactRegistryVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                package_name: self.package_name,
                project: core::default::Default::default(),
                repository_id: self.repository_id,
                view: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataArtifactRegistryVersionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataArtifactRegistryVersionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `package_name` after provisioning.\n"]
    pub fn package_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.package_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `repository_id` after provisioning.\n"]
    pub fn repository_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> ListRef<DataArtifactRegistryVersionsVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `view` after provisioning.\n"]
    pub fn view(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataArtifactRegistryVersionsVersionsElRelatedTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataArtifactRegistryVersionsVersionsElRelatedTagsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataArtifactRegistryVersionsVersionsElRelatedTagsEl {
    type O = BlockAssignable<DataArtifactRegistryVersionsVersionsElRelatedTagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataArtifactRegistryVersionsVersionsElRelatedTagsEl {}
impl BuildDataArtifactRegistryVersionsVersionsElRelatedTagsEl {
    pub fn build(self) -> DataArtifactRegistryVersionsVersionsElRelatedTagsEl {
        DataArtifactRegistryVersionsVersionsElRelatedTagsEl {
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataArtifactRegistryVersionsVersionsElRelatedTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryVersionsVersionsElRelatedTagsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataArtifactRegistryVersionsVersionsElRelatedTagsElRef {
        DataArtifactRegistryVersionsVersionsElRelatedTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataArtifactRegistryVersionsVersionsElRelatedTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataArtifactRegistryVersionsVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_tags: Option<ListField<DataArtifactRegistryVersionsVersionsElRelatedTagsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataArtifactRegistryVersionsVersionsEl {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
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
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `related_tags`.\n"]
    pub fn set_related_tags(
        mut self,
        v: impl Into<ListField<DataArtifactRegistryVersionsVersionsElRelatedTagsEl>>,
    ) -> Self {
        self.related_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataArtifactRegistryVersionsVersionsEl {
    type O = BlockAssignable<DataArtifactRegistryVersionsVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataArtifactRegistryVersionsVersionsEl {}
impl BuildDataArtifactRegistryVersionsVersionsEl {
    pub fn build(self) -> DataArtifactRegistryVersionsVersionsEl {
        DataArtifactRegistryVersionsVersionsEl {
            annotations: core::default::Default::default(),
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            related_tags: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataArtifactRegistryVersionsVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryVersionsVersionsElRef {
    fn new(shared: StackShared, base: String) -> DataArtifactRegistryVersionsVersionsElRef {
        DataArtifactRegistryVersionsVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataArtifactRegistryVersionsVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `related_tags` after provisioning.\n"]
    pub fn related_tags(&self) -> ListRef<DataArtifactRegistryVersionsVersionsElRelatedTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.related_tags", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
