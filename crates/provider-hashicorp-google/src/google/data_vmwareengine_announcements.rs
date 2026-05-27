use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVmwareengineAnnouncementsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    parent: PrimField<String>,
}
struct DataVmwareengineAnnouncements_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVmwareengineAnnouncementsData>,
}
#[derive(Clone)]
pub struct DataVmwareengineAnnouncements(Rc<DataVmwareengineAnnouncements_>);
impl DataVmwareengineAnnouncements {
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
    #[doc = "Set the field `name`.\nThe resource name of the specific Announcements to retrieve. If provided, the 'announcements' list will contain only this announcement."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `announcements` after provisioning.\n A list of announcements"]
    pub fn announcements(&self) -> ListRef<DataVmwareengineAnnouncementsAnnouncementsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.announcements", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the specific Announcements to retrieve. If provided, the 'announcements' list will contain only this announcement."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nRequired. The resource name of the location to be queried for announcements. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: projects/my-project/locations/us-west1-a"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
}
impl Referable for DataVmwareengineAnnouncements {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVmwareengineAnnouncements {}
impl ToListMappable for DataVmwareengineAnnouncements {
    type O = ListRef<DataVmwareengineAnnouncementsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVmwareengineAnnouncements_ {
    fn extract_datasource_type(&self) -> String {
        "google_vmwareengine_announcements".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVmwareengineAnnouncements {
    pub tf_id: String,
    #[doc = "Required. The resource name of the location to be queried for announcements. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: projects/my-project/locations/us-west1-a"]
    pub parent: PrimField<String>,
}
impl BuildDataVmwareengineAnnouncements {
    pub fn build(self, stack: &mut Stack) -> DataVmwareengineAnnouncements {
        let out = DataVmwareengineAnnouncements(Rc::new(DataVmwareengineAnnouncements_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVmwareengineAnnouncementsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVmwareengineAnnouncementsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineAnnouncementsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVmwareengineAnnouncementsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `announcements` after provisioning.\n A list of announcements"]
    pub fn announcements(&self) -> ListRef<DataVmwareengineAnnouncementsAnnouncementsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.announcements", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the specific Announcements to retrieve. If provided, the 'announcements' list will contain only this announcement."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nRequired. The resource name of the location to be queried for announcements. Resource names are schemeless URIs that follow the conventions in https://cloud.google.com/apis/design/resource_names. For example: projects/my-project/locations/us-west1-a"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataVmwareengineAnnouncementsAnnouncementsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_resource_type: Option<PrimField<String>>,
}
impl DataVmwareengineAnnouncementsAnnouncementsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `target_resource_type`.\n"]
    pub fn set_target_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_resource_type = Some(v.into());
        self
    }
}
impl ToListMappable for DataVmwareengineAnnouncementsAnnouncementsEl {
    type O = BlockAssignable<DataVmwareengineAnnouncementsAnnouncementsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVmwareengineAnnouncementsAnnouncementsEl {}
impl BuildDataVmwareengineAnnouncementsAnnouncementsEl {
    pub fn build(self) -> DataVmwareengineAnnouncementsAnnouncementsEl {
        DataVmwareengineAnnouncementsAnnouncementsEl {
            code: core::default::Default::default(),
            metadata: core::default::Default::default(),
            name: core::default::Default::default(),
            target_resource_type: core::default::Default::default(),
        }
    }
}
pub struct DataVmwareengineAnnouncementsAnnouncementsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVmwareengineAnnouncementsAnnouncementsElRef {
    fn new(shared: StackShared, base: String) -> DataVmwareengineAnnouncementsAnnouncementsElRef {
        DataVmwareengineAnnouncementsAnnouncementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVmwareengineAnnouncementsAnnouncementsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `target_resource_type` after provisioning.\n"]
    pub fn target_resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_resource_type", self.base),
        )
    }
}
