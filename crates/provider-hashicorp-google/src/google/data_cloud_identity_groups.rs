use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudIdentityGroupsData {
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
struct DataCloudIdentityGroups_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudIdentityGroupsData>,
}
#[derive(Clone)]
pub struct DataCloudIdentityGroups(Rc<DataCloudIdentityGroups_>);
impl DataCloudIdentityGroups {
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
    #[doc = "Get a reference to the value of field `groups` after provisioning.\nList of Cloud Identity groups."]
    pub fn groups(&self) -> ListRef<DataCloudIdentityGroupsGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the entity under which this Group resides in the\nCloud Identity resource hierarchy.\n\nMust be of the form identitysources/{identity_source_id} for external-identity-mapped\ngroups or customers/{customer_id} for Google Groups."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudIdentityGroups {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudIdentityGroups {}
impl ToListMappable for DataCloudIdentityGroups {
    type O = ListRef<DataCloudIdentityGroupsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudIdentityGroups_ {
    fn extract_datasource_type(&self) -> String {
        "google_cloud_identity_groups".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudIdentityGroups {
    pub tf_id: String,
    #[doc = "The resource name of the entity under which this Group resides in the\nCloud Identity resource hierarchy.\n\nMust be of the form identitysources/{identity_source_id} for external-identity-mapped\ngroups or customers/{customer_id} for Google Groups."]
    pub parent: PrimField<String>,
}
impl BuildDataCloudIdentityGroups {
    pub fn build(self, stack: &mut Stack) -> DataCloudIdentityGroups {
        let out = DataCloudIdentityGroups(Rc::new(DataCloudIdentityGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudIdentityGroupsData {
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
pub struct DataCloudIdentityGroupsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudIdentityGroupsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `groups` after provisioning.\nList of Cloud Identity groups."]
    pub fn groups(&self) -> ListRef<DataCloudIdentityGroupsGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the entity under which this Group resides in the\nCloud Identity resource hierarchy.\n\nMust be of the form identitysources/{identity_source_id} for external-identity-mapped\ngroups or customers/{customer_id} for Google Groups."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
    type O = BlockAssignable<DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {}
impl BuildDataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
    pub fn build(self) -> DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
        DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
        DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupsGroupsElGroupKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupsGroupsElGroupKeyEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupsGroupsElGroupKeyEl {
    type O = BlockAssignable<DataCloudIdentityGroupsGroupsElGroupKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupsGroupsElGroupKeyEl {}
impl BuildDataCloudIdentityGroupsGroupsElGroupKeyEl {
    pub fn build(self) -> DataCloudIdentityGroupsGroupsElGroupKeyEl {
        DataCloudIdentityGroupsGroupsElGroupKeyEl {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupsGroupsElGroupKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupsGroupsElGroupKeyElRef {
    fn new(shared: StackShared, base: String) -> DataCloudIdentityGroupsGroupsElGroupKeyElRef {
        DataCloudIdentityGroupsGroupsElGroupKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupsGroupsElGroupKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudIdentityGroupsGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_group_keys: Option<ListField<DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_time: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_key: Option<ListField<DataCloudIdentityGroupsGroupsElGroupKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_group_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_time: Option<PrimField<String>>,
}
impl DataCloudIdentityGroupsGroupsEl {
    #[doc = "Set the field `additional_group_keys`.\n"]
    pub fn set_additional_group_keys(
        mut self,
        v: impl Into<ListField<DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl>>,
    ) -> Self {
        self.additional_group_keys = Some(v.into());
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
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `group_key`.\n"]
    pub fn set_group_key(
        mut self,
        v: impl Into<ListField<DataCloudIdentityGroupsGroupsElGroupKeyEl>>,
    ) -> Self {
        self.group_key = Some(v.into());
        self
    }
    #[doc = "Set the field `initial_group_config`.\n"]
    pub fn set_initial_group_config(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.initial_group_config = Some(v.into());
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
    #[doc = "Set the field `parent`.\n"]
    pub fn set_parent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.parent = Some(v.into());
        self
    }
    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_time = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudIdentityGroupsGroupsEl {
    type O = BlockAssignable<DataCloudIdentityGroupsGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudIdentityGroupsGroupsEl {}
impl BuildDataCloudIdentityGroupsGroupsEl {
    pub fn build(self) -> DataCloudIdentityGroupsGroupsEl {
        DataCloudIdentityGroupsGroupsEl {
            additional_group_keys: core::default::Default::default(),
            create_time: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            group_key: core::default::Default::default(),
            initial_group_config: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            parent: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}
pub struct DataCloudIdentityGroupsGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudIdentityGroupsGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataCloudIdentityGroupsGroupsElRef {
        DataCloudIdentityGroupsGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudIdentityGroupsGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_group_keys` after provisioning.\n"]
    pub fn additional_group_keys(
        &self,
    ) -> ListRef<DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.additional_group_keys", self.base),
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `group_key` after provisioning.\n"]
    pub fn group_key(&self) -> ListRef<DataCloudIdentityGroupsGroupsElGroupKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.group_key", self.base))
    }
    #[doc = "Get a reference to the value of field `initial_group_config` after provisioning.\n"]
    pub fn initial_group_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.initial_group_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parent", self.base))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
