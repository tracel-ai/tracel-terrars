use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataTagsTagKeysData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
}
struct DataTagsTagKeys_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataTagsTagKeysData>,
}
#[derive(Clone)]
pub struct DataTagsTagKeys(Rc<DataTagsTagKeys_>);
impl DataTagsTagKeys {
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
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> ListRef<DataTagsTagKeysKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
}
impl Referable for DataTagsTagKeys {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataTagsTagKeys {}
impl ToListMappable for DataTagsTagKeys {
    type O = ListRef<DataTagsTagKeysRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataTagsTagKeys_ {
    fn extract_datasource_type(&self) -> String {
        "google_tags_tag_keys".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataTagsTagKeys {
    pub tf_id: String,
    #[doc = ""]
    pub parent: PrimField<String>,
}
impl BuildDataTagsTagKeys {
    pub fn build(self, stack: &mut Stack) -> DataTagsTagKeys {
        let out = DataTagsTagKeys(Rc::new(DataTagsTagKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataTagsTagKeysData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataTagsTagKeysRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataTagsTagKeysRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataTagsTagKeysRef {
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
    #[doc = "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> ListRef<DataTagsTagKeysKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.keys", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataTagsTagKeysKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_values_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaced_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose_data: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataTagsTagKeysKeysEl {
    #[doc = "Set the field `allowed_values_regex`.\n"]
    pub fn set_allowed_values_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.allowed_values_regex = Some(v.into());
        self
    }
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create_time = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deletion_policy = Some(v.into());
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
    #[doc = "Set the field `namespaced_name`.\n"]
    pub fn set_namespaced_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespaced_name = Some(v.into());
        self
    }
    #[doc = "Set the field `parent`.\n"]
    pub fn set_parent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parent = Some(v.into());
        self
    }
    #[doc = "Set the field `purpose`.\n"]
    pub fn set_purpose(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.purpose = Some(v.into());
        self
    }
    #[doc = "Set the field `purpose_data`.\n"]
    pub fn set_purpose_data(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.purpose_data = Some(v.into());
        self
    }
    #[doc = "Set the field `short_name`.\n"]
    pub fn set_short_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.short_name = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataTagsTagKeysKeysEl {
    type O = BlockAssignable<DataTagsTagKeysKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataTagsTagKeysKeysEl {}
impl BuildDataTagsTagKeysKeysEl {
    pub fn build(self) -> DataTagsTagKeysKeysEl {
        DataTagsTagKeysKeysEl {
            allowed_values_regex: core::default::Default::default(),
            create_time: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            namespaced_name: core::default::Default::default(),
            parent: core::default::Default::default(),
            purpose: core::default::Default::default(),
            purpose_data: core::default::Default::default(),
            short_name: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataTagsTagKeysKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataTagsTagKeysKeysElRef {
    fn new(shared: StackShared, base: String) -> DataTagsTagKeysKeysElRef {
        DataTagsTagKeysKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataTagsTagKeysKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_values_regex` after provisioning.\n"]
    pub fn allowed_values_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allowed_values_regex", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `namespaced_name` after provisioning.\n"]
    pub fn namespaced_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespaced_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.base))
    }
    #[doc = "Get a reference to the value of field `purpose` after provisioning.\n"]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.base))
    }
    #[doc = "Get a reference to the value of field `purpose_data` after provisioning.\n"]
    pub fn purpose_data(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.purpose_data", self.base))
    }
    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
