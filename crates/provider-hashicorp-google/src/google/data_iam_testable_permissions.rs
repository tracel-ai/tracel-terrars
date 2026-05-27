use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataIamTestablePermissionsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_support_level: Option<PrimField<String>>,
    full_resource_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stages: Option<ListField<PrimField<String>>>,
}
struct DataIamTestablePermissions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataIamTestablePermissionsData>,
}
#[derive(Clone)]
pub struct DataIamTestablePermissions(Rc<DataIamTestablePermissions_>);
impl DataIamTestablePermissions {
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
    #[doc = "Set the field `custom_support_level`.\n"]
    pub fn set_custom_support_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_support_level = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `stages`.\n"]
    pub fn set_stages(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().stages = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `custom_support_level` after provisioning.\n"]
    pub fn custom_support_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_support_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `full_resource_name` after provisioning.\n"]
    pub fn full_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_resource_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<DataIamTestablePermissionsPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stages` after provisioning.\n"]
    pub fn stages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stages", self.extract_ref()),
        )
    }
}
impl Referable for DataIamTestablePermissions {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataIamTestablePermissions {}
impl ToListMappable for DataIamTestablePermissions {
    type O = ListRef<DataIamTestablePermissionsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataIamTestablePermissions_ {
    fn extract_datasource_type(&self) -> String {
        "google_iam_testable_permissions".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataIamTestablePermissions {
    pub tf_id: String,
    #[doc = ""]
    pub full_resource_name: PrimField<String>,
}
impl BuildDataIamTestablePermissions {
    pub fn build(self, stack: &mut Stack) -> DataIamTestablePermissions {
        let out = DataIamTestablePermissions(Rc::new(DataIamTestablePermissions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataIamTestablePermissionsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                custom_support_level: core::default::Default::default(),
                full_resource_name: self.full_resource_name,
                id: core::default::Default::default(),
                stages: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataIamTestablePermissionsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamTestablePermissionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataIamTestablePermissionsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `custom_support_level` after provisioning.\n"]
    pub fn custom_support_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_support_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `full_resource_name` after provisioning.\n"]
    pub fn full_resource_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.full_resource_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> ListRef<DataIamTestablePermissionsPermissionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.permissions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stages` after provisioning.\n"]
    pub fn stages(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.stages", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataIamTestablePermissionsPermissionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_support_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl DataIamTestablePermissionsPermissionsEl {
    #[doc = "Set the field `api_disabled`.\n"]
    pub fn set_api_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.api_disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_support_level`.\n"]
    pub fn set_custom_support_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_support_level = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `stage`.\n"]
    pub fn set_stage(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stage = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for DataIamTestablePermissionsPermissionsEl {
    type O = BlockAssignable<DataIamTestablePermissionsPermissionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataIamTestablePermissionsPermissionsEl {}
impl BuildDataIamTestablePermissionsPermissionsEl {
    pub fn build(self) -> DataIamTestablePermissionsPermissionsEl {
        DataIamTestablePermissionsPermissionsEl {
            api_disabled: core::default::Default::default(),
            custom_support_level: core::default::Default::default(),
            name: core::default::Default::default(),
            stage: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct DataIamTestablePermissionsPermissionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataIamTestablePermissionsPermissionsElRef {
    fn new(shared: StackShared, base: String) -> DataIamTestablePermissionsPermissionsElRef {
        DataIamTestablePermissionsPermissionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataIamTestablePermissionsPermissionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_disabled` after provisioning.\n"]
    pub fn api_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `custom_support_level` after provisioning.\n"]
    pub fn custom_support_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_support_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
