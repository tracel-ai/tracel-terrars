use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComposerImageVersionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataComposerImageVersions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComposerImageVersionsData>,
}
#[derive(Clone)]
pub struct DataComposerImageVersions(Rc<DataComposerImageVersions_>);
impl DataComposerImageVersions {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `image_versions` after provisioning.\n"]
    pub fn image_versions(&self) -> ListRef<DataComposerImageVersionsImageVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
impl Referable for DataComposerImageVersions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComposerImageVersions {}
impl ToListMappable for DataComposerImageVersions {
    type O = ListRef<DataComposerImageVersionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComposerImageVersions_ {
    fn extract_datasource_type(&self) -> String {
        "google_composer_image_versions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComposerImageVersions {
    pub tf_id: String,
}
impl BuildDataComposerImageVersions {
    pub fn build(self, stack: &mut Stack) -> DataComposerImageVersions {
        let out = DataComposerImageVersions(Rc::new(DataComposerImageVersions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComposerImageVersionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComposerImageVersionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerImageVersionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComposerImageVersionsRef {
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
    #[doc = "Get a reference to the value of field `image_versions` after provisioning.\n"]
    pub fn image_versions(&self) -> ListRef<DataComposerImageVersionsImageVersionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_versions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComposerImageVersionsImageVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_version_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_python_versions: Option<ListField<PrimField<String>>>,
}
impl DataComposerImageVersionsImageVersionsEl {
    #[doc = "Set the field `image_version_id`.\n"]
    pub fn set_image_version_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image_version_id = Some(v.into());
        self
    }
    #[doc = "Set the field `supported_python_versions`.\n"]
    pub fn set_supported_python_versions(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.supported_python_versions = Some(v.into());
        self
    }
}
impl ToListMappable for DataComposerImageVersionsImageVersionsEl {
    type O = BlockAssignable<DataComposerImageVersionsImageVersionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComposerImageVersionsImageVersionsEl {}
impl BuildDataComposerImageVersionsImageVersionsEl {
    pub fn build(self) -> DataComposerImageVersionsImageVersionsEl {
        DataComposerImageVersionsImageVersionsEl {
            image_version_id: core::default::Default::default(),
            supported_python_versions: core::default::Default::default(),
        }
    }
}
pub struct DataComposerImageVersionsImageVersionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComposerImageVersionsImageVersionsElRef {
    fn new(shared: StackShared, base: String) -> DataComposerImageVersionsImageVersionsElRef {
        DataComposerImageVersionsImageVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComposerImageVersionsImageVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_version_id` after provisioning.\n"]
    pub fn image_version_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_version_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `supported_python_versions` after provisioning.\n"]
    pub fn supported_python_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_python_versions", self.base),
        )
    }
}
