use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataArtifactRegistryFileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    file_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    output_path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    repository_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataArtifactRegistryFileTimeoutsEl>,
}
struct DataArtifactRegistryFile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataArtifactRegistryFileData>,
}
#[derive(Clone)]
pub struct DataArtifactRegistryFile(Rc<DataArtifactRegistryFile_>);
impl DataArtifactRegistryFile {
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
    #[doc = "Set the field `overwrite`.\n"]
    pub fn set_overwrite(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().overwrite = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataArtifactRegistryFileTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_id` after provisioning.\n"]
    pub fn file_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hashes` after provisioning.\n"]
    pub fn hashes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.hashes", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_base64sha256` after provisioning.\n"]
    pub fn output_base64sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_base64sha256", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_path` after provisioning.\n"]
    pub fn output_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_sha256` after provisioning.\n"]
    pub fn output_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_sha256", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `overwrite` after provisioning.\n"]
    pub fn overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `size_bytes` after provisioning.\n"]
    pub fn size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataArtifactRegistryFileTimeoutsElRef {
        DataArtifactRegistryFileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataArtifactRegistryFile {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataArtifactRegistryFile {}
impl ToListMappable for DataArtifactRegistryFile {
    type O = ListRef<DataArtifactRegistryFileRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataArtifactRegistryFile_ {
    fn extract_datasource_type(&self) -> String {
        "google_artifact_registry_file".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataArtifactRegistryFile {
    pub tf_id: String,
    #[doc = ""]
    pub file_id: PrimField<String>,
    #[doc = ""]
    pub location: PrimField<String>,
    #[doc = ""]
    pub output_path: PrimField<String>,
    #[doc = ""]
    pub repository_id: PrimField<String>,
}
impl BuildDataArtifactRegistryFile {
    pub fn build(self, stack: &mut Stack) -> DataArtifactRegistryFile {
        let out = DataArtifactRegistryFile(Rc::new(DataArtifactRegistryFile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataArtifactRegistryFileData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                file_id: self.file_id,
                id: core::default::Default::default(),
                location: self.location,
                output_path: self.output_path,
                overwrite: core::default::Default::default(),
                project: core::default::Default::default(),
                repository_id: self.repository_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataArtifactRegistryFileRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryFileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataArtifactRegistryFileRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `file_id` after provisioning.\n"]
    pub fn file_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.file_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hashes` after provisioning.\n"]
    pub fn hashes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.hashes", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_base64sha256` after provisioning.\n"]
    pub fn output_base64sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_base64sha256", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_path` after provisioning.\n"]
    pub fn output_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_path", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `output_sha256` after provisioning.\n"]
    pub fn output_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_sha256", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `overwrite` after provisioning.\n"]
    pub fn overwrite(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.overwrite", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `size_bytes` after provisioning.\n"]
    pub fn size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.size_bytes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataArtifactRegistryFileTimeoutsElRef {
        DataArtifactRegistryFileTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataArtifactRegistryFileTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataArtifactRegistryFileTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataArtifactRegistryFileTimeoutsEl {
    type O = BlockAssignable<DataArtifactRegistryFileTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataArtifactRegistryFileTimeoutsEl {}
impl BuildDataArtifactRegistryFileTimeoutsEl {
    pub fn build(self) -> DataArtifactRegistryFileTimeoutsEl {
        DataArtifactRegistryFileTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataArtifactRegistryFileTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataArtifactRegistryFileTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataArtifactRegistryFileTimeoutsElRef {
        DataArtifactRegistryFileTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataArtifactRegistryFileTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
