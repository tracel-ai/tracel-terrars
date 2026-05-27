use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLossPreventionStoredInfoTypeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stored_info_type_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dictionary: Option<Vec<DataLossPreventionStoredInfoTypeDictionaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large_custom_dictionary: Option<Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<Vec<DataLossPreventionStoredInfoTypeRegexEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataLossPreventionStoredInfoTypeTimeoutsEl>,
    dynamic: DataLossPreventionStoredInfoTypeDynamic,
}
struct DataLossPreventionStoredInfoType_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLossPreventionStoredInfoTypeData>,
}
#[derive(Clone)]
pub struct DataLossPreventionStoredInfoType(Rc<DataLossPreventionStoredInfoType_>);
impl DataLossPreventionStoredInfoType {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nA description of the info type."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser set display name of the info type."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `stored_info_type_id`.\nThe storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
    pub fn set_stored_info_type_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stored_info_type_id = Some(v.into());
        self
    }
    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        self,
        v: impl Into<BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dictionary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dictionary = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `large_custom_dictionary`.\n"]
    pub fn set_large_custom_dictionary(
        self,
        v: impl Into<BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().large_custom_dictionary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.large_custom_dictionary = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        self,
        v: impl Into<BlockAssignable<DataLossPreventionStoredInfoTypeRegexEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().regex = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.regex = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataLossPreventionStoredInfoTypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the info type."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the info type."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the info type. Set by the server."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the info type in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stored_info_type_id` after provisioning.\nThe storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
    pub fn stored_info_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stored_info_type_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(&self) -> ListRef<DataLossPreventionStoredInfoTypeDictionaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dictionary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `large_custom_dictionary` after provisioning.\n"]
    pub fn large_custom_dictionary(
        &self,
    ) -> ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.large_custom_dictionary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> ListRef<DataLossPreventionStoredInfoTypeRegexElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.regex", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionStoredInfoTypeTimeoutsElRef {
        DataLossPreventionStoredInfoTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataLossPreventionStoredInfoType {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataLossPreventionStoredInfoType {}
impl ToListMappable for DataLossPreventionStoredInfoType {
    type O = ListRef<DataLossPreventionStoredInfoTypeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataLossPreventionStoredInfoType_ {
    fn extract_resource_type(&self) -> String {
        "google_data_loss_prevention_stored_info_type".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLossPreventionStoredInfoType {
    pub tf_id: String,
    #[doc = "The parent of the info type in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub parent: PrimField<String>,
}
impl BuildDataLossPreventionStoredInfoType {
    pub fn build(self, stack: &mut Stack) -> DataLossPreventionStoredInfoType {
        let out = DataLossPreventionStoredInfoType(Rc::new(DataLossPreventionStoredInfoType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLossPreventionStoredInfoTypeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                stored_info_type_id: core::default::Default::default(),
                dictionary: core::default::Default::default(),
                large_custom_dictionary: core::default::Default::default(),
                regex: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataLossPreventionStoredInfoTypeRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLossPreventionStoredInfoTypeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the info type."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the info type."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the info type. Set by the server."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the info type in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `stored_info_type_id` after provisioning.\nThe storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
    pub fn stored_info_type_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.stored_info_type_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(&self) -> ListRef<DataLossPreventionStoredInfoTypeDictionaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dictionary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `large_custom_dictionary` after provisioning.\n"]
    pub fn large_custom_dictionary(
        &self,
    ) -> ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.large_custom_dictionary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> ListRef<DataLossPreventionStoredInfoTypeRegexElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.regex", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionStoredInfoTypeTimeoutsElRef {
        DataLossPreventionStoredInfoTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
    path: PrimField<String>,
}
impl DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {}
impl ToListMappable for DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
    type O = BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
    #[doc = "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: PrimField<String>,
}
impl BuildDataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
        DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl { path: self.path }
    }
}
pub struct DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
        DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nA url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeDictionaryElWordListEl {
    words: ListField<PrimField<String>>,
}
impl DataLossPreventionStoredInfoTypeDictionaryElWordListEl {}
impl ToListMappable for DataLossPreventionStoredInfoTypeDictionaryElWordListEl {
    type O = BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryElWordListEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeDictionaryElWordListEl {
    #[doc = "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: ListField<PrimField<String>>,
}
impl BuildDataLossPreventionStoredInfoTypeDictionaryElWordListEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeDictionaryElWordListEl {
        DataLossPreventionStoredInfoTypeDictionaryElWordListEl { words: self.words }
    }
}
pub struct DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
        DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `words` after provisioning.\nWords or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub fn words(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.words", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionStoredInfoTypeDictionaryElDynamic {
    cloud_storage_path:
        Option<DynamicBlock<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl>>,
    word_list: Option<DynamicBlock<DataLossPreventionStoredInfoTypeDictionaryElWordListEl>>,
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeDictionaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage_path: Option<Vec<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    word_list: Option<Vec<DataLossPreventionStoredInfoTypeDictionaryElWordListEl>>,
    dynamic: DataLossPreventionStoredInfoTypeDictionaryElDynamic,
}
impl DataLossPreventionStoredInfoTypeDictionaryEl {
    #[doc = "Set the field `cloud_storage_path`.\n"]
    pub fn set_cloud_storage_path(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_storage_path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_path = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `word_list`.\n"]
    pub fn set_word_list(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryElWordListEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.word_list = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.word_list = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionStoredInfoTypeDictionaryEl {
    type O = BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeDictionaryEl {}
impl BuildDataLossPreventionStoredInfoTypeDictionaryEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeDictionaryEl {
        DataLossPreventionStoredInfoTypeDictionaryEl {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionStoredInfoTypeDictionaryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeDictionaryElRef {
    fn new(shared: StackShared, base: String) -> DataLossPreventionStoredInfoTypeDictionaryElRef {
        DataLossPreventionStoredInfoTypeDictionaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeDictionaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> ListRef<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> ListRef<DataLossPreventionStoredInfoTypeDictionaryElWordListElRef> {
        ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
    name: PrimField<String>,
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {}
impl ToListMappable
    for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl
{
    type O = BlockAssignable<
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
    #[doc = "Name describing the field."]
    pub name: PrimField<String>,
}
impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
    pub fn build(
        self,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
            name: self.name,
        }
    }
}
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName describing the field."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
    dataset_id: PrimField<String>,
    project_id: PrimField<String>,
    table_id: PrimField<String>,
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {}
impl ToListMappable
    for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl
{
    type O = BlockAssignable<
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
    #[doc = "The dataset ID of the table."]
    pub dataset_id: PrimField<String>,
    #[doc = "The Google Cloud Platform project ID of the project containing the table."]
    pub project_id: PrimField<String>,
    #[doc = "The name of the table."]
    pub table_id: PrimField<String>,
}
impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
    pub fn build(
        self,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID of the table."]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud Platform project ID of the project containing the table."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe name of the table."]
    pub fn table_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElDynamic {
    field: Option<
        DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl>,
    >,
    table: Option<
        DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl>,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field:
        Option<Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table:
        Option<Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl>>,
    dynamic: DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElDynamic,
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.table = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.table = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
    type O =
        BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {}
impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
            field: core::default::Default::default(),
            table: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(
        &self,
    ) -> ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.field", self.base))
    }
    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(
        &self,
    ) -> ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
    url: PrimField<String>,
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {}
impl ToListMappable
    for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl
{
    type O = BlockAssignable<
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
    #[doc = "The url, in the format 'gs://<bucket>/<path>'. Trailing wildcard in the path is allowed."]
    pub url: PrimField<String>,
}
impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
    pub fn build(
        self,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
            url: self.url,
        }
    }
}
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe url, in the format 'gs://<bucket>/<path>'. Trailing wildcard in the path is allowed."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
    path: PrimField<String>,
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {}
impl ToListMappable for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
    type O = BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
    #[doc = "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: PrimField<String>,
}
impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl { path: self.path }
    }
}
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nA url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElDynamic {
    big_query_field: Option<
        DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl>,
    >,
    cloud_storage_file_set: Option<
        DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl>,
    >,
    output_path:
        Option<DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl>>,
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    big_query_field:
        Option<Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage_file_set:
        Option<Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_path: Option<Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl>>,
    dynamic: DataLossPreventionStoredInfoTypeLargeCustomDictionaryElDynamic,
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
    #[doc = "Set the field `big_query_field`.\n"]
    pub fn set_big_query_field(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.big_query_field = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.big_query_field = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_storage_file_set`.\n"]
    pub fn set_cloud_storage_file_set(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cloud_storage_file_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_file_set = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `output_path`.\n"]
    pub fn set_output_path(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.output_path = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.output_path = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
    type O = BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {}
impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
            big_query_field: core::default::Default::default(),
            cloud_storage_file_set: core::default::Default::default(),
            output_path: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `big_query_field` after provisioning.\n"]
    pub fn big_query_field(
        &self,
    ) -> ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.big_query_field", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_storage_file_set` after provisioning.\n"]
    pub fn cloud_storage_file_set(
        &self,
    ) -> ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_file_set", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `output_path` after provisioning.\n"]
    pub fn output_path(
        &self,
    ) -> ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef> {
        ListRef::new(self.shared().clone(), format!("{}.output_path", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeRegexEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_indexes: Option<ListField<PrimField<f64>>>,
    pattern: PrimField<String>,
}
impl DataLossPreventionStoredInfoTypeRegexEl {
    #[doc = "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.group_indexes = Some(v.into());
        self
    }
}
impl ToListMappable for DataLossPreventionStoredInfoTypeRegexEl {
    type O = BlockAssignable<DataLossPreventionStoredInfoTypeRegexEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeRegexEl {
    #[doc = "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: PrimField<String>,
}
impl BuildDataLossPreventionStoredInfoTypeRegexEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeRegexEl {
        DataLossPreventionStoredInfoTypeRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}
pub struct DataLossPreventionStoredInfoTypeRegexElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeRegexElRef {
    fn new(shared: StackShared, base: String) -> DataLossPreventionStoredInfoTypeRegexElRef {
        DataLossPreventionStoredInfoTypeRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeRegexElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn group_indexes(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_indexes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataLossPreventionStoredInfoTypeTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for DataLossPreventionStoredInfoTypeTimeoutsEl {
    type O = BlockAssignable<DataLossPreventionStoredInfoTypeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionStoredInfoTypeTimeoutsEl {}
impl BuildDataLossPreventionStoredInfoTypeTimeoutsEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeTimeoutsEl {
        DataLossPreventionStoredInfoTypeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionStoredInfoTypeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionStoredInfoTypeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataLossPreventionStoredInfoTypeTimeoutsElRef {
        DataLossPreventionStoredInfoTypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionStoredInfoTypeTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionStoredInfoTypeDynamic {
    dictionary: Option<DynamicBlock<DataLossPreventionStoredInfoTypeDictionaryEl>>,
    large_custom_dictionary:
        Option<DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl>>,
    regex: Option<DynamicBlock<DataLossPreventionStoredInfoTypeRegexEl>>,
}
