use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataKmsKeyHandlesData {
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
    project: Option<PrimField<String>>,
    resource_type_selector: PrimField<String>,
}
struct DataKmsKeyHandles_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataKmsKeyHandlesData>,
}
#[derive(Clone)]
pub struct DataKmsKeyHandles(Rc<DataKmsKeyHandles_>);
impl DataKmsKeyHandles {
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
    #[doc = "Set the field `project`.\nProject ID of the project."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `key_handles` after provisioning.\nA list of all the retrieved key handles"]
    pub fn key_handles(&self) -> ListRef<DataKmsKeyHandlesKeyHandlesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_handles", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `resource_type_selector` after provisioning.\n\n\t\t\t\t\tThe resource_type_selector argument is used to add a filter query parameter that limits which key handles are retrieved by the data source: ?filter=resource_type_selector=\"{{resource_type_selector}}\".\n\t\t\t\t\tExample values:\n\t\t\t\t\t* resource_type_selector=\"{SERVICE}.googleapis.com/{TYPE}\".\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyHandles/list)\n\t\t\t\t"]
    pub fn resource_type_selector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type_selector", self.extract_ref()),
        )
    }
}
impl Referable for DataKmsKeyHandles {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataKmsKeyHandles {}
impl ToListMappable for DataKmsKeyHandles {
    type O = ListRef<DataKmsKeyHandlesRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataKmsKeyHandles_ {
    fn extract_datasource_type(&self) -> String {
        "google_kms_key_handles".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataKmsKeyHandles {
    pub tf_id: String,
    #[doc = "The canonical id for the location. For example: \"us-east1\"."]
    pub location: PrimField<String>,
    #[doc = "\n\t\t\t\t\tThe resource_type_selector argument is used to add a filter query parameter that limits which key handles are retrieved by the data source: ?filter=resource_type_selector=\"{{resource_type_selector}}\".\n\t\t\t\t\tExample values:\n\t\t\t\t\t* resource_type_selector=\"{SERVICE}.googleapis.com/{TYPE}\".\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyHandles/list)\n\t\t\t\t"]
    pub resource_type_selector: PrimField<String>,
}
impl BuildDataKmsKeyHandles {
    pub fn build(self, stack: &mut Stack) -> DataKmsKeyHandles {
        let out = DataKmsKeyHandles(Rc::new(DataKmsKeyHandles_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataKmsKeyHandlesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                resource_type_selector: self.resource_type_selector,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataKmsKeyHandlesRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsKeyHandlesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataKmsKeyHandlesRef {
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
    #[doc = "Get a reference to the value of field `key_handles` after provisioning.\nA list of all the retrieved key handles"]
    pub fn key_handles(&self) -> ListRef<DataKmsKeyHandlesKeyHandlesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.key_handles", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `resource_type_selector` after provisioning.\n\n\t\t\t\t\tThe resource_type_selector argument is used to add a filter query parameter that limits which key handles are retrieved by the data source: ?filter=resource_type_selector=\"{{resource_type_selector}}\".\n\t\t\t\t\tExample values:\n\t\t\t\t\t* resource_type_selector=\"{SERVICE}.googleapis.com/{TYPE}\".\n\t\t\t\t\t[See the documentation about using filters](https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyHandles/list)\n\t\t\t\t"]
    pub fn resource_type_selector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type_selector", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataKmsKeyHandlesKeyHandlesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type_selector: Option<PrimField<String>>,
}
impl DataKmsKeyHandlesKeyHandlesEl {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_type_selector`.\n"]
    pub fn set_resource_type_selector(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type_selector = Some(v.into());
        self
    }
}
impl ToListMappable for DataKmsKeyHandlesKeyHandlesEl {
    type O = BlockAssignable<DataKmsKeyHandlesKeyHandlesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataKmsKeyHandlesKeyHandlesEl {}
impl BuildDataKmsKeyHandlesKeyHandlesEl {
    pub fn build(self) -> DataKmsKeyHandlesKeyHandlesEl {
        DataKmsKeyHandlesKeyHandlesEl {
            kms_key: core::default::Default::default(),
            name: core::default::Default::default(),
            resource_type_selector: core::default::Default::default(),
        }
    }
}
pub struct DataKmsKeyHandlesKeyHandlesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataKmsKeyHandlesKeyHandlesElRef {
    fn new(shared: StackShared, base: String) -> DataKmsKeyHandlesKeyHandlesElRef {
        DataKmsKeyHandlesKeyHandlesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataKmsKeyHandlesKeyHandlesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_type_selector` after provisioning.\n"]
    pub fn resource_type_selector(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type_selector", self.base),
        )
    }
}
