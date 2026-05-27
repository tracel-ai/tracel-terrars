use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataArtifactRegistryTagsData {
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
}
struct DataArtifactRegistryTags_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataArtifactRegistryTagsData>,
}
#[derive(Clone)]
pub struct DataArtifactRegistryTags(Rc<DataArtifactRegistryTags_>);
impl DataArtifactRegistryTags {
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataArtifactRegistryTagsTagsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
impl Referable for DataArtifactRegistryTags {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataArtifactRegistryTags {}
impl ToListMappable for DataArtifactRegistryTags {
    type O = ListRef<DataArtifactRegistryTagsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataArtifactRegistryTags_ {
    fn extract_datasource_type(&self) -> String {
        "google_artifact_registry_tags".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataArtifactRegistryTags {
    pub tf_id: String,
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = ""]
    pub package_name: PrimField<String>,
    #[doc = ""]
    pub repository_id: PrimField<String>,
}
impl BuildDataArtifactRegistryTags {
    pub fn build(self, stack: &mut Stack) -> DataArtifactRegistryTags {
        let out = DataArtifactRegistryTags(Rc::new(DataArtifactRegistryTags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataArtifactRegistryTagsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                package_name: self.package_name,
                project: core::default::Default::default(),
                repository_id: self.repository_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataArtifactRegistryTagsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryTagsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataArtifactRegistryTagsRef {
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
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataArtifactRegistryTagsTagsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataArtifactRegistryTagsTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataArtifactRegistryTagsTagsEl {
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
impl ToListMappable for DataArtifactRegistryTagsTagsEl {
    type O = BlockAssignable<DataArtifactRegistryTagsTagsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataArtifactRegistryTagsTagsEl {}
impl BuildDataArtifactRegistryTagsTagsEl {
    pub fn build(self) -> DataArtifactRegistryTagsTagsEl {
        DataArtifactRegistryTagsTagsEl {
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataArtifactRegistryTagsTagsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryTagsTagsElRef {
    fn new(shared: StackShared, base: String) -> DataArtifactRegistryTagsTagsElRef {
        DataArtifactRegistryTagsTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataArtifactRegistryTagsTagsElRef {
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
