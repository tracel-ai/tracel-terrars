use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsKeyRingsData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataKmsKeyRings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsKeyRingsData>,
}
#[derive(Clone)]
pub struct DataKmsKeyRings(Rc<DataKmsKeyRings_>);
impl DataKmsKeyRings {
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
    #[doc = "Set the field `filter`.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which keys are retrieved by the data source: ?filter={{filter}}.\n\t\t\t\t\tExample values:\n\t\t\t\t\t\n\t\t\t\t\t* \"name:my-key-\" will retrieve key rings that contain \"my-key-\" anywhere in their name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}.\n\t\t\t\t\t* \"name=projects/my-project/locations/global/keyRings/my-key-ring\" will only retrieve a key ring with that exact name.\n\t\t\t\t\t\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nProject ID of the project."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which keys are retrieved by the data source: ?filter={{filter}}.\n\t\t\t\t\tExample values:\n\t\t\t\t\t\n\t\t\t\t\t* \"name:my-key-\" will retrieve key rings that contain \"my-key-\" anywhere in their name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}.\n\t\t\t\t\t* \"name=projects/my-project/locations/global/keyRings/my-key-ring\" will only retrieve a key ring with that exact name.\n\t\t\t\t\t\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
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
    #[doc = "Get a reference to the value of field `key_rings` after provisioning.\nA list of all the retrieved key rings"]
    pub fn key_rings(&self) -> ListRef<DataKmsKeyRingsKeyRingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_rings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe canonical id for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
impl Referable for DataKmsKeyRings {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsKeyRings {}
impl ToListMappable for DataKmsKeyRings {
    type O = ListRef<DataKmsKeyRingsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsKeyRings_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_key_rings".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsKeyRings {
    pub tf_id: String,
    #[doc = "The canonical id for the location. For example: \"us-east1\"."]
    pub location: PrimField<String>,
}
impl BuildDataKmsKeyRings {
    pub fn build(self, stack: &mut Stack) -> DataKmsKeyRings {
        let out = DataKmsKeyRings(Rc::new(DataKmsKeyRings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsKeyRingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataKmsKeyRingsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsKeyRingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsKeyRingsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\n\n\t\t\t\t\tThe filter argument is used to add a filter query parameter that limits which keys are retrieved by the data source: ?filter={{filter}}.\n\t\t\t\t\tExample values:\n\t\t\t\t\t\n\t\t\t\t\t* \"name:my-key-\" will retrieve key rings that contain \"my-key-\" anywhere in their name. Note: names take the form projects/{{project}}/locations/{{location}}/keyRings/{{keyRing}}.\n\t\t\t\t\t* \"name=projects/my-project/locations/global/keyRings/my-key-ring\" will only retrieve a key ring with that exact name.\n\t\t\t\t\t\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/sorting-and-filtering)\n\t\t\t\t"]
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
    #[doc = "Get a reference to the value of field `key_rings` after provisioning.\nA list of all the retrieved key rings"]
    pub fn key_rings(&self) -> ListRef<DataKmsKeyRingsKeyRingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_rings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe canonical id for the location. For example: \"us-east1\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nProject ID of the project."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataKmsKeyRingsKeyRingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl DataKmsKeyRingsKeyRingsEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for DataKmsKeyRingsKeyRingsEl {
    type O = BlockAssignable<DataKmsKeyRingsKeyRingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsKeyRingsKeyRingsEl {}
impl BuildDataKmsKeyRingsKeyRingsEl {
    pub fn build(self) -> DataKmsKeyRingsKeyRingsEl {
        DataKmsKeyRingsKeyRingsEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct DataKmsKeyRingsKeyRingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsKeyRingsKeyRingsElRef {
    fn new(shared: StackShared, base: String) -> DataKmsKeyRingsKeyRingsElRef {
        DataKmsKeyRingsKeyRingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsKeyRingsKeyRingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
