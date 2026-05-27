use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeImagesData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataComputeImages_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeImagesData>,
}
#[derive(Clone)]
pub struct DataComputeImages(Rc<DataComputeImages_>);
impl DataComputeImages {
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
    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> ListRef<DataComputeImagesImagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.images", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeImages {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeImages {}
impl ToListMappable for DataComputeImages {
    type O = ListRef<DataComputeImagesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeImages_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_images".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeImages {
    pub tf_id: String,
}
impl BuildDataComputeImages {
    pub fn build(self, stack: &mut Stack) -> DataComputeImages {
        let out = DataComputeImages(Rc::new(DataComputeImages_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeImagesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeImagesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeImagesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeImagesRef {
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
    #[doc = "Get a reference to the value of field `images` after provisioning.\n"]
    pub fn images(&self) -> ListRef<DataComputeImagesImagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.images", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeImagesImagesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_size_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_timestamp: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_disk: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_disk_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image_id: Option<PrimField<String>>,
}
impl DataComputeImagesImagesEl {
    #[doc = "Set the field `archive_size_bytes`.\n"]
    pub fn set_archive_size_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.archive_size_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `creation_timestamp`.\n"]
    pub fn set_creation_timestamp(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.creation_timestamp = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `family`.\n"]
    pub fn set_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.family = Some(v.into());
        self
    }
    #[doc = "Set the field `image_id`.\n"]
    pub fn set_image_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.image_id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_link = Some(v.into());
        self
    }
    #[doc = "Set the field `source_disk`.\n"]
    pub fn set_source_disk(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_disk = Some(v.into());
        self
    }
    #[doc = "Set the field `source_disk_id`.\n"]
    pub fn set_source_disk_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_disk_id = Some(v.into());
        self
    }
    #[doc = "Set the field `source_image_id`.\n"]
    pub fn set_source_image_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_image_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeImagesImagesEl {
    type O = BlockAssignable<DataComputeImagesImagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeImagesImagesEl {}
impl BuildDataComputeImagesImagesEl {
    pub fn build(self) -> DataComputeImagesImagesEl {
        DataComputeImagesImagesEl {
            archive_size_bytes: core::default::Default::default(),
            creation_timestamp: core::default::Default::default(),
            description: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            family: core::default::Default::default(),
            image_id: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            self_link: core::default::Default::default(),
            source_disk: core::default::Default::default(),
            source_disk_id: core::default::Default::default(),
            source_image_id: core::default::Default::default(),
        }
    }
}
pub struct DataComputeImagesImagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeImagesImagesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeImagesImagesElRef {
        DataComputeImagesImagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeImagesImagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_size_bytes` after provisioning.\n"]
    pub fn archive_size_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.archive_size_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\n"]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `family` after provisioning.\n"]
    pub fn family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.family", self.base))
    }
    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `source_disk` after provisioning.\n"]
    pub fn source_disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_disk", self.base))
    }
    #[doc = "Get a reference to the value of field `source_disk_id` after provisioning.\n"]
    pub fn source_disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_disk_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_image_id` after provisioning.\n"]
    pub fn source_image_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_image_id", self.base),
        )
    }
}
