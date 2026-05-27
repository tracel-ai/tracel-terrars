use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLossPreventionInspectTemplateData {
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
    template_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_config: Option<Vec<DataLossPreventionInspectTemplateInspectConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataLossPreventionInspectTemplateTimeoutsEl>,
    dynamic: DataLossPreventionInspectTemplateDynamic,
}
struct DataLossPreventionInspectTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLossPreventionInspectTemplateData>,
}
#[derive(Clone)]
pub struct DataLossPreventionInspectTemplate(Rc<DataLossPreventionInspectTemplate_>);
impl DataLossPreventionInspectTemplate {
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
    #[doc = "Set the field `description`.\nA description of the inspect template."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser set display name of the inspect template."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `template_id`.\nThe template id can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is\n100 characters. Can be empty to allow the system to generate one."]
    pub fn set_template_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().template_id = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_config`.\n"]
    pub fn set_inspect_config(
        self,
        v: impl Into<BlockAssignable<DataLossPreventionInspectTemplateInspectConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inspect_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inspect_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataLossPreventionInspectTemplateTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the inspect template."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the inspect template."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the inspect template. Set by the server."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the inspect template in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_id` after provisioning.\nThe template id can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is\n100 characters. Can be empty to allow the system to generate one."]
    pub fn template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_config` after provisioning.\n"]
    pub fn inspect_config(&self) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inspect_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionInspectTemplateTimeoutsElRef {
        DataLossPreventionInspectTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataLossPreventionInspectTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataLossPreventionInspectTemplate {}
impl ToListMappable for DataLossPreventionInspectTemplate {
    type O = ListRef<DataLossPreventionInspectTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataLossPreventionInspectTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_data_loss_prevention_inspect_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLossPreventionInspectTemplate {
    pub tf_id: String,
    #[doc = "The parent of the inspect template in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub parent: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplate {
    pub fn build(self, stack: &mut Stack) -> DataLossPreventionInspectTemplate {
        let out = DataLossPreventionInspectTemplate(Rc::new(DataLossPreventionInspectTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLossPreventionInspectTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                template_id: core::default::Default::default(),
                inspect_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataLossPreventionInspectTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLossPreventionInspectTemplateRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the inspect template."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the inspect template."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the inspect template. Set by the server."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the inspect template in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_id` after provisioning.\nThe template id can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is\n100 characters. Can be empty to allow the system to generate one."]
    pub fn template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_config` after provisioning.\n"]
    pub fn inspect_config(&self) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.inspect_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionInspectTemplateTimeoutsElRef {
        DataLossPreventionInspectTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl
{
    path: PrimField<String>,
}
impl
    DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl
{
}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl
{
    #[doc = "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl { DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl { path : self . path , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef { DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `path` after provisioning.\nA url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"] pub fn path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.path" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    words: ListField<PrimField<String>>,
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl {}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl
{
    #[doc = "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: ListField<PrimField<String>>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl
    {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
            words: self.words,
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef
    {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `words` after provisioning.\nWords or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub fn words(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.words", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElDynamic { cloud_storage_path : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl >> , word_list : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl { # [serde (skip_serializing_if = "Option::is_none")] cloud_storage_path : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl > > , # [serde (skip_serializing_if = "Option::is_none")] word_list : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
    #[doc = "Set the field `cloud_storage_path`.\n"]
    pub fn set_cloud_storage_path(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl >>,
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
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl >>,
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
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]    pub fn cloud_storage_path (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(
        &self,
    ) -> ListRef<
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl
{
    score: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl
{
    #[doc = "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl { DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl { score : self . score , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef { DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef { shared : shared , base : base . to_string () , } } }
impl
    DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElDynamic { sensitivity_score : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl { name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] version : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] sensitivity_score : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sensitivity_score = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
    #[doc = "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]    pub fn sensitivity_score (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitivity_score", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_indexes: Option<ListField<PrimField<f64>>>,
    pattern: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
    #[doc = "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.group_indexes = Some(v.into());
        self
    }
}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
    type O =
        BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
    #[doc = "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
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
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    score: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl
{
    #[doc = "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {
            score: self.score,
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef
    {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
    name: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
    #[doc = "Resource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub name: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
            name: self.name,
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {}
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDynamic {
    dictionary: Option<
        DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl>,
    >,
    info_type: Option<
        DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl>,
    >,
    regex: Option<
        DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl>,
    >,
    sensitivity_score: Option<
        DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl,
        >,
    >,
    stored_type: Option<
        DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl>,
    >,
    surrogate_type: Option<
        DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    likelihood: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dictionary:
        Option<Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info_type:
        Option<Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitivity_score: Option<
        Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    stored_type:
        Option<Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    surrogate_type: Option<
        Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl>,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDynamic,
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
    #[doc = "Set the field `exclusion_type`.\nIf set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching. Possible values: [\"EXCLUSION_TYPE_EXCLUDE\"]"]
    pub fn set_exclusion_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exclusion_type = Some(v.into());
        self
    }
    #[doc = "Set the field `likelihood`.\nLikelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria\nspecified by the rule. Default value: \"VERY_LIKELY\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_likelihood(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.likelihood = Some(v.into());
        self
    }
    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dictionary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dictionary = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `info_type`.\n"]
    pub fn set_info_type(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.info_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.regex = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.regex = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sensitivity_score = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `stored_type`.\n"]
    pub fn set_stored_type(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.stored_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.stored_type = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `surrogate_type`.\n"]
    pub fn set_surrogate_type(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.surrogate_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.surrogate_type = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
    type O = BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {}
impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
            exclusion_type: core::default::Default::default(),
            likelihood: core::default::Default::default(),
            dictionary: core::default::Default::default(),
            info_type: core::default::Default::default(),
            regex: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            stored_type: core::default::Default::default(),
            surrogate_type: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclusion_type` after provisioning.\nIf set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching. Possible values: [\"EXCLUSION_TYPE_EXCLUDE\"]"]
    pub fn exclusion_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exclusion_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `likelihood` after provisioning.\nLikelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria\nspecified by the rule. Default value: \"VERY_LIKELY\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn likelihood(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.likelihood", self.base))
    }
    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }
    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef> {
        ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }
    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> ListRef<
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitivity_score", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `stored_type` after provisioning.\n"]
    pub fn stored_type(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.stored_type", self.base))
    }
    #[doc = "Get a reference to the value of field `surrogate_type` after provisioning.\n"]
    pub fn surrogate_type(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.surrogate_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
    score: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
    #[doc = "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
        DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
            score: self.score,
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
        DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesElDynamic {
    sensitivity_score: Option<
        DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl>,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitivity_score:
        Option<Vec<DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElInfoTypesElDynamic,
}
impl DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    #[doc = "Set the field `version`.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sensitivity_score = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    type O = BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    #[doc = "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
        DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
        DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitivity_score", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl
{
    score: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl { }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl
{
    #[doc = "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl { DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl { score : self . score , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef { DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"] pub fn score (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.score" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElDynamic { sensitivity_score : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl { name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] version : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] sensitivity_score : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sensitivity_score = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl
{
    #[doc = "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: PrimField<String>,
}
impl
    BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl
{
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl
    {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef { DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]    pub fn sensitivity_score (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitivity_score", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElDynamic { info_type : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl { max_findings : PrimField < f64 > , # [serde (skip_serializing_if = "Option::is_none")] info_type : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    #[doc = "Set the field `info_type`.\n"]
    pub fn set_info_type(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.info_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    #[doc = "Max findings limit for the given infoType."]
    pub max_findings: PrimField<f64>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
            max_findings: self.max_findings,
            info_type: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_findings` after provisioning.\nMax findings limit for the given infoType."]
    pub fn max_findings(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_findings", self.base))
    }
    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]    pub fn info_type (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef >{
        ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElLimitsElDynamic {
    max_findings_per_info_type: Option<
        DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    max_findings_per_item: PrimField<f64>,
    max_findings_per_request: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_findings_per_info_type: Option<
        Vec<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl>,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElLimitsElDynamic,
}
impl DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    #[doc = "Set the field `max_findings_per_info_type`.\n"]
    pub fn set_max_findings_per_info_type(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.max_findings_per_info_type = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.max_findings_per_info_type = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    type O = BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElLimitsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    #[doc = "Max number of findings that will be returned for each item scanned. The maximum returned is 2000."]
    pub max_findings_per_item: PrimField<f64>,
    #[doc = "Max number of findings that will be returned per request/job. The maximum returned is 2000."]
    pub max_findings_per_request: PrimField<f64>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
        DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
            max_findings_per_item: self.max_findings_per_item,
            max_findings_per_request: self.max_findings_per_request,
            max_findings_per_info_type: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_findings_per_item` after provisioning.\nMax number of findings that will be returned for each item scanned. The maximum returned is 2000."]
    pub fn max_findings_per_item(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_findings_per_item", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_findings_per_request` after provisioning.\nMax number of findings that will be returned per request/job. The maximum returned is 2000."]
    pub fn max_findings_per_request(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_findings_per_request", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_findings_per_info_type` after provisioning.\n"]
    pub fn max_findings_per_info_type(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.max_findings_per_info_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    score: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl
{
    #[doc = "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl
    {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
            score: self.score,
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef
    {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElDynamic {
    sensitivity_score: Option<
        DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitivity_score: Option<
        Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl>,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElDynamic,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sensitivity_score = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    type O = BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    #[doc = "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> ListRef<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sensitivity_score", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl
{
    path: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl { }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl
{
    #[doc = "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl { path : self . path , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `path` after provisioning.\nA url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"] pub fn path (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.path" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl
{
    words: ListField<PrimField<String>>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl { }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl
{
    #[doc = "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: ListField<PrimField<String>>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl { words : self . words , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `words` after provisioning.\nWords or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."] pub fn words (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.words" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElDynamic { cloud_storage_path : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl >> , word_list : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl { # [serde (skip_serializing_if = "Option::is_none")] cloud_storage_path : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl > > , # [serde (skip_serializing_if = "Option::is_none")] word_list : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
    #[doc = "Set the field `cloud_storage_path`.\n"]
    pub fn set_cloud_storage_path(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl >>,
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
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl >>,
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
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl
{}
impl
    BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl
{
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl
    {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl { cloud_storage_path : core :: default :: Default :: default () , word_list : core :: default :: Default :: default () , dynamic : Default :: default () , }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef { shared : shared , base : base . to_string () , } } }
impl
    DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]    pub fn cloud_storage_path (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_storage_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]    pub fn word_list (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef >{
        ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    group_indexes: Option<ListField<PrimField<f64>>>,
    pattern: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl { # [doc = "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."] pub fn set_group_indexes (mut self , v : impl Into < ListField < PrimField < f64 > > >) -> Self { self . group_indexes = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl
{
    #[doc = "Pattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl { group_indexes : core :: default :: Default :: default () , pattern : self . pattern , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."] pub fn group_indexes (& self) -> ListRef < PrimExpr < f64 > > { ListRef :: new (self . shared () . clone () , format ! ("{}.group_indexes" , self . base)) } # [doc = "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."] pub fn pattern (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.pattern" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    window_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window_before: Option<PrimField<f64>>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl { # [doc = "Set the field `window_after`.\nNumber of characters after the finding to consider."] pub fn set_window_after (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . window_after = Some (v . into ()) ; self } # [doc = "Set the field `window_before`.\nNumber of characters before the finding to consider."] pub fn set_window_before (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . window_before = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl
{}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl { window_after : core :: default :: Default :: default () , window_before : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `window_after` after provisioning.\nNumber of characters after the finding to consider."] pub fn window_after (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.window_after" , self . base)) } # [doc = "Get a reference to the value of field `window_before` after provisioning.\nNumber of characters before the finding to consider."] pub fn window_before (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.window_before" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElDynamic { hotword_regex : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl >> , proximity : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl { # [serde (skip_serializing_if = "Option::is_none")] hotword_regex : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl > > , # [serde (skip_serializing_if = "Option::is_none")] proximity : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl { # [doc = "Set the field `hotword_regex`.\n"] pub fn set_hotword_regex (mut self , v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . hotword_regex = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . hotword_regex = Some (d) ; } } self } # [doc = "Set the field `proximity`.\n"] pub fn set_proximity (mut self , v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . proximity = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . proximity = Some (d) ; } } self } }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl
{}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl { hotword_regex : core :: default :: Default :: default () , proximity : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"] pub fn hotword_regex (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.hotword_regex" , self . base)) } # [doc = "Get a reference to the value of field `proximity` after provisioning.\n"] pub fn proximity (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.proximity" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl
{
    score: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl { }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl
{
    #[doc = "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl { score : self . score , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"] pub fn score (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.score" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElDynamic { sensitivity_score : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl { name : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] version : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] sensitivity_score : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl { # [doc = "Set the field `version`.\nVersion name for this InfoType."] pub fn set_version (mut self , v : impl Into < PrimField < String > >) -> Self { self . version = Some (v . into ()) ; self } # [doc = "Set the field `sensitivity_score`.\n"] pub fn set_sensitivity_score (mut self , v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . sensitivity_score = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . sensitivity_score = Some (d) ; } } self } }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl
{
    #[doc = "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl { name : self . name , version : core :: default :: Default :: default () , sensitivity_score : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."] pub fn name (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.name" , self . base)) } # [doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."] pub fn version (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.version" , self . base)) } # [doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"] pub fn sensitivity_score (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.sensitivity_score" , self . base)) } }
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElDynamic { info_types : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl { # [serde (skip_serializing_if = "Option::is_none")] info_types : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl { # [doc = "Set the field `info_types`.\n"] pub fn set_info_types (mut self , v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . info_types = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . info_types = Some (d) ; } } self } }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl
{}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl { info_types : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `info_types` after provisioning.\n"] pub fn info_types (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.info_types" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group_indexes: Option<ListField<PrimField<f64>>>,
    pattern: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    #[doc = "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.group_indexes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl
{
    #[doc = "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl
    {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef
    {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
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
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDynamic { dictionary : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl >> , exclude_by_hotword : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl >> , exclude_info_types : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl >> , regex : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl { matching_type : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] dictionary : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl > > , # [serde (skip_serializing_if = "Option::is_none")] exclude_by_hotword : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl > > , # [serde (skip_serializing_if = "Option::is_none")] exclude_info_types : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl > > , # [serde (skip_serializing_if = "Option::is_none")] regex : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dictionary = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dictionary = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exclude_by_hotword`.\n"]
    pub fn set_exclude_by_hotword(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_by_hotword = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_by_hotword = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `exclude_info_types`.\n"]
    pub fn set_exclude_info_types(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclude_info_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_info_types = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.regex = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.regex = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
    #[doc = "How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub matching_type: PrimField<String>,
}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
            matching_type: self.matching_type,
            dictionary: core::default::Default::default(),
            exclude_by_hotword: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            regex: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `matching_type` after provisioning.\nHow the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub fn matching_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.matching_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]    pub fn dictionary (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef >{
        ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }
    #[doc = "Get a reference to the value of field `exclude_by_hotword` after provisioning.\n"]    pub fn exclude_by_hotword (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_by_hotword", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_info_types` after provisioning.\n"]    pub fn exclude_info_types (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_info_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(
        &self,
    ) -> ListRef<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    group_indexes: Option<ListField<PrimField<f64>>>,
    pattern: PrimField<String>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
    #[doc = "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(mut self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.group_indexes = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl
{
    #[doc = "Pattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: PrimField<String>,
}
impl
    BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl
{
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl
    {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl { group_indexes : core :: default :: Default :: default () , pattern : self . pattern , }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef { shared : shared , base : base . to_string () , } } }
impl
    DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn group_indexes(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.group_indexes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_likelihood: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_likelihood: Option<PrimField<f64>>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl { # [doc = "Set the field `fixed_likelihood`.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"] pub fn set_fixed_likelihood (mut self , v : impl Into < PrimField < String > >) -> Self { self . fixed_likelihood = Some (v . into ()) ; self } # [doc = "Set the field `relative_likelihood`.\nIncrease or decrease the likelihood by the specified number of levels. For example,\nif a finding would be POSSIBLE without the detection rule and relativeLikelihood is 1,\nthen it is upgraded to LIKELY, while a value of -1 would downgrade it to UNLIKELY.\nLikelihood may never drop below VERY_UNLIKELY or exceed VERY_LIKELY, so applying an\nadjustment of 1 followed by an adjustment of -1 when base likelihood is VERY_LIKELY\nwill result in a final likelihood of LIKELY. Either this or fixed_likelihood can be set."] pub fn set_relative_likelihood (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . relative_likelihood = Some (v . into ()) ; self } }
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl { type O = BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl
{}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl { pub fn build (self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl { fixed_likelihood : core :: default :: Default :: default () , relative_likelihood : core :: default :: Default :: default () , } } }
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef { fn new (shared : StackShared , base : String) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef { DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef { shared : shared , base : base . to_string () , } } }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `fixed_likelihood` after provisioning.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"] pub fn fixed_likelihood (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.fixed_likelihood" , self . base)) } # [doc = "Get a reference to the value of field `relative_likelihood` after provisioning.\nIncrease or decrease the likelihood by the specified number of levels. For example,\nif a finding would be POSSIBLE without the detection rule and relativeLikelihood is 1,\nthen it is upgraded to LIKELY, while a value of -1 would downgrade it to UNLIKELY.\nLikelihood may never drop below VERY_UNLIKELY or exceed VERY_LIKELY, so applying an\nadjustment of 1 followed by an adjustment of -1 when base likelihood is VERY_LIKELY\nwill result in a final likelihood of LIKELY. Either this or fixed_likelihood can be set."] pub fn relative_likelihood (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.relative_likelihood" , self . base)) } }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    window_after: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    window_before: Option<PrimField<f64>>,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
    #[doc = "Set the field `window_after`.\nNumber of characters after the finding to consider. Either this or window_before must be specified"]
    pub fn set_window_after(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.window_after = Some(v.into());
        self
    }
    #[doc = "Set the field `window_before`.\nNumber of characters before the finding to consider. Either this or window_after must be specified"]
    pub fn set_window_before(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.window_before = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl
{}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl
    {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef
    {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef { shared : shared , base : base . to_string () , }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `window_after` after provisioning.\nNumber of characters after the finding to consider. Either this or window_before must be specified"]
    pub fn window_after(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.window_after", self.base))
    }
    #[doc = "Get a reference to the value of field `window_before` after provisioning.\nNumber of characters before the finding to consider. Either this or window_after must be specified"]
    pub fn window_before(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.window_before", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElDynamic { hotword_regex : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl >> , likelihood_adjustment : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl >> , proximity : Option < DynamicBlock < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl >> , }
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl { # [serde (skip_serializing_if = "Option::is_none")] hotword_regex : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl > > , # [serde (skip_serializing_if = "Option::is_none")] likelihood_adjustment : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl > > , # [serde (skip_serializing_if = "Option::is_none")] proximity : Option < Vec < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl > > , dynamic : DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElDynamic , }
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
    #[doc = "Set the field `hotword_regex`.\n"]
    pub fn set_hotword_regex(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hotword_regex = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_regex = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `likelihood_adjustment`.\n"]
    pub fn set_likelihood_adjustment(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.likelihood_adjustment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.likelihood_adjustment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proximity`.\n"]
    pub fn set_proximity(
        mut self,
        v : impl Into < BlockAssignable < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.proximity = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.proximity = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl
{
    type O = BlockAssignable<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
            hotword_regex: core::default::Default::default(),
            likelihood_adjustment: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]    pub fn hotword_regex (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.hotword_regex", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `likelihood_adjustment` after provisioning.\n"]    pub fn likelihood_adjustment (& self) -> ListRef < DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.likelihood_adjustment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> ListRef<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElDynamic {
    exclusion_rule: Option<
        DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl,
        >,
    >,
    hotword_rule: Option<
        DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl>,
    >,
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_rule: Option<
        Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    hotword_rule:
        Option<Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElDynamic,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
    #[doc = "Set the field `exclusion_rule`.\n"]
    pub fn set_exclusion_rule(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exclusion_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hotword_rule`.\n"]
    pub fn set_hotword_rule(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hotword_rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_rule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
    type O = BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
            exclusion_rule: core::default::Default::default(),
            hotword_rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `exclusion_rule` after provisioning.\n"]
    pub fn exclusion_rule(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclusion_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `hotword_rule` after provisioning.\n"]
    pub fn hotword_rule(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.hotword_rule", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElDynamic {
    info_types:
        Option<DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl>>,
    rules: Option<DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl>>,
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    info_types: Option<Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElDynamic,
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v: impl Into<
            BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.info_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rules = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
    type O = BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetEl {}
impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
            info_types: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElDynamic {
    custom_info_types:
        Option<DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl>>,
    info_types: Option<DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl>>,
    limits: Option<DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElLimitsEl>>,
    rule_set: Option<DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetEl>>,
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_info_types: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_quote: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_likelihood: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_info_types:
        Option<Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info_types: Option<Vec<DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<Vec<DataLossPreventionInspectTemplateInspectConfigElLimitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_set: Option<Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElDynamic,
}
impl DataLossPreventionInspectTemplateInspectConfigEl {
    #[doc = "Set the field `content_options`.\nList of options defining data content to scan. If empty, text, images, and other content will be included. Possible values: [\"CONTENT_TEXT\", \"CONTENT_IMAGE\"]"]
    pub fn set_content_options(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.content_options = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_info_types`.\nWhen true, excludes type information of the findings."]
    pub fn set_exclude_info_types(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.exclude_info_types = Some(v.into());
        self
    }
    #[doc = "Set the field `include_quote`.\nWhen true, a contextual quote from the data that triggered a finding is included in the response."]
    pub fn set_include_quote(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_quote = Some(v.into());
        self
    }
    #[doc = "Set the field `min_likelihood`.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info Default value: \"POSSIBLE\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_min_likelihood(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_likelihood = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_info_types`.\n"]
    pub fn set_custom_info_types(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_info_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_info_types = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.info_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElLimitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.limits = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rule_set`.\n"]
    pub fn set_rule_set(
        mut self,
        v: impl Into<BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule_set = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_set = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLossPreventionInspectTemplateInspectConfigEl {
    type O = BlockAssignable<DataLossPreventionInspectTemplateInspectConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateInspectConfigEl {}
impl BuildDataLossPreventionInspectTemplateInspectConfigEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigEl {
        DataLossPreventionInspectTemplateInspectConfigEl {
            content_options: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            include_quote: core::default::Default::default(),
            min_likelihood: core::default::Default::default(),
            custom_info_types: core::default::Default::default(),
            info_types: core::default::Default::default(),
            limits: core::default::Default::default(),
            rule_set: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateInspectConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateInspectConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRef {
        DataLossPreventionInspectTemplateInspectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateInspectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_options` after provisioning.\nList of options defining data content to scan. If empty, text, images, and other content will be included. Possible values: [\"CONTENT_TEXT\", \"CONTENT_IMAGE\"]"]
    pub fn content_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_info_types` after provisioning.\nWhen true, excludes type information of the findings."]
    pub fn exclude_info_types(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.exclude_info_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_quote` after provisioning.\nWhen true, a contextual quote from the data that triggered a finding is included in the response."]
    pub fn include_quote(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_quote", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_likelihood` after provisioning.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info Default value: \"POSSIBLE\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn min_likelihood(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_likelihood", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_info_types` after provisioning.\n"]
    pub fn custom_info_types(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_info_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElLimitsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }
    #[doc = "Get a reference to the value of field `rule_set` after provisioning.\n"]
    pub fn rule_set(
        &self,
    ) -> ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule_set", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataLossPreventionInspectTemplateTimeoutsEl {
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
impl ToListMappable for DataLossPreventionInspectTemplateTimeoutsEl {
    type O = BlockAssignable<DataLossPreventionInspectTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLossPreventionInspectTemplateTimeoutsEl {}
impl BuildDataLossPreventionInspectTemplateTimeoutsEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateTimeoutsEl {
        DataLossPreventionInspectTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataLossPreventionInspectTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLossPreventionInspectTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataLossPreventionInspectTemplateTimeoutsElRef {
        DataLossPreventionInspectTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLossPreventionInspectTemplateTimeoutsElRef {
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
struct DataLossPreventionInspectTemplateDynamic {
    inspect_config: Option<DynamicBlock<DataLossPreventionInspectTemplateInspectConfigEl>>,
}
