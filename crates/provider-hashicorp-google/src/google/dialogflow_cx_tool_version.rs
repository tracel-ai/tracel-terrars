use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxToolVersionData {
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
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxToolVersionTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool: Option<Vec<DialogflowCxToolVersionToolEl>>,
    dynamic: DialogflowCxToolVersionDynamic,
}
struct DialogflowCxToolVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxToolVersionData>,
}
#[derive(Clone)]
pub struct DialogflowCxToolVersion(Rc<DialogflowCxToolVersion_>);
impl DialogflowCxToolVersion {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxToolVersionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(self, v: impl Into<BlockAssignable<DialogflowCxToolVersionToolEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tool = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tool = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nLast time the tool version was created or modified.\nUses RFC 3339, where generated output will always be Z-normalized and use 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the tool version."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the tool version.\nFormat: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/tools/<ToolID>/versions/<VersionID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe tool to create a Version for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/tools/<Tool ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast time the tool version was created or modified.\nUses RFC 3339, where generated output will always be Z-normalized and use 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxToolVersionTimeoutsElRef {
        DialogflowCxToolVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(&self) -> ListRef<DialogflowCxToolVersionToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tool", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxToolVersion {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxToolVersion {}
impl ToListMappable for DialogflowCxToolVersion {
    type O = ListRef<DialogflowCxToolVersionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxToolVersion_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_tool_version".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxToolVersion {
    pub tf_id: String,
    #[doc = "The display name of the tool version."]
    pub display_name: PrimField<String>,
    #[doc = "The tool to create a Version for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/tools/<Tool ID>."]
    pub parent: PrimField<String>,
}
impl BuildDialogflowCxToolVersion {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxToolVersion {
        let out = DialogflowCxToolVersion(Rc::new(DialogflowCxToolVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxToolVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                parent: self.parent,
                timeouts: core::default::Default::default(),
                tool: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxToolVersionRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxToolVersionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nLast time the tool version was created or modified.\nUses RFC 3339, where generated output will always be Z-normalized and use 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the tool version."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the tool version.\nFormat: projects/<ProjectID>/locations/<LocationID>/agents/<AgentID>/tools/<ToolID>/versions/<VersionID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe tool to create a Version for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/tools/<Tool ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nLast time the tool version was created or modified.\nUses RFC 3339, where generated output will always be Z-normalized and use 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxToolVersionTimeoutsElRef {
        DialogflowCxToolVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(&self) -> ListRef<DialogflowCxToolVersionToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.tool", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl DialogflowCxToolVersionTimeoutsEl {
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
}
impl ToListMappable for DialogflowCxToolVersionTimeoutsEl {
    type O = BlockAssignable<DialogflowCxToolVersionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionTimeoutsEl {}
impl BuildDialogflowCxToolVersionTimeoutsEl {
    pub fn build(self) -> DialogflowCxToolVersionTimeoutsEl {
        DialogflowCxToolVersionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolVersionTimeoutsElRef {
        DialogflowCxToolVersionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionTimeoutsElRef {
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
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_processing_mode: Option<PrimField<String>>,
}
impl DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl {
    #[doc = "Set the field `data_store`.\nThe full name of the referenced data store. Formats: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore} projects/{project}/locations/{location}/dataStores/{dataStore}"]
    pub fn set_data_store(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_store = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store_type`.\nThe type of the connected data store.\nSee [DataStoreType](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/DataStoreConnection#datastoretype) for valid values."]
    pub fn set_data_store_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_store_type = Some(v.into());
        self
    }
    #[doc = "Set the field `document_processing_mode`.\nThe document processing mode for the data store connection. Should only be set for PUBLIC_WEB and UNSTRUCTURED data stores. If not set it is considered as DOCUMENTS, as this is the legacy mode.\nSee [DocumentProcessingMode](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/DataStoreConnection#documentprocessingmode) for valid values."]
    pub fn set_document_processing_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.document_processing_mode = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl {}
impl BuildDialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl {
        DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl {
            data_store: core::default::Default::default(),
            data_store_type: core::default::Default::default(),
            document_processing_mode: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsElRef {
        DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store` after provisioning.\nThe full name of the referenced data store. Formats: projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore} projects/{project}/locations/{location}/dataStores/{dataStore}"]
    pub fn data_store(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
    #[doc = "Get a reference to the value of field `data_store_type` after provisioning.\nThe type of the connected data store.\nSee [DataStoreType](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/DataStoreConnection#datastoretype) for valid values."]
    pub fn data_store_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_store_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `document_processing_mode` after provisioning.\nThe document processing mode for the data store connection. Should only be set for PUBLIC_WEB and UNSTRUCTURED data stores. If not set it is considered as DOCUMENTS, as this is the legacy mode.\nSee [DocumentProcessingMode](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/DataStoreConnection#documentprocessingmode) for valid values."]
    pub fn document_processing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_processing_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl {}
impl DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl {}
impl ToListMappable for DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl {}
impl BuildDialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl {
        DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl {}
    }
}
pub struct DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptElRef {
        DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxToolVersionToolElDataStoreSpecElDynamic {
    data_store_connections:
        Option<DynamicBlock<DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl>>,
    fallback_prompt:
        Option<DynamicBlock<DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElDataStoreSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_connections:
        Option<Vec<DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fallback_prompt: Option<Vec<DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl>>,
    dynamic: DialogflowCxToolVersionToolElDataStoreSpecElDynamic,
}
impl DialogflowCxToolVersionToolElDataStoreSpecEl {
    #[doc = "Set the field `data_store_connections`.\n"]
    pub fn set_data_store_connections(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_store_connections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_connections = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `fallback_prompt`.\n"]
    pub fn set_fallback_prompt(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fallback_prompt = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fallback_prompt = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxToolVersionToolElDataStoreSpecEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolElDataStoreSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElDataStoreSpecEl {}
impl BuildDialogflowCxToolVersionToolElDataStoreSpecEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElDataStoreSpecEl {
        DialogflowCxToolVersionToolElDataStoreSpecEl {
            data_store_connections: core::default::Default::default(),
            fallback_prompt: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElDataStoreSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElDataStoreSpecElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolVersionToolElDataStoreSpecElRef {
        DialogflowCxToolVersionToolElDataStoreSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElDataStoreSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store_connections` after provisioning.\n"]
    pub fn data_store_connections(
        &self,
    ) -> ListRef<DialogflowCxToolVersionToolElDataStoreSpecElDataStoreConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fallback_prompt` after provisioning.\n"]
    pub fn fallback_prompt(
        &self,
    ) -> ListRef<DialogflowCxToolVersionToolElDataStoreSpecElFallbackPromptElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fallback_prompt", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElFunctionSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_schema: Option<PrimField<String>>,
}
impl DialogflowCxToolVersionToolElFunctionSpecEl {
    #[doc = "Set the field `input_schema`.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the input of the function.\nThis input is a JSON object that contains the function's parameters as properties of the object"]
    pub fn set_input_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.input_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `output_schema`.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the output of the function.\nThis output is a JSON object that contains the function's parameters as properties of the object"]
    pub fn set_output_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_schema = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxToolVersionToolElFunctionSpecEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolElFunctionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElFunctionSpecEl {}
impl BuildDialogflowCxToolVersionToolElFunctionSpecEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElFunctionSpecEl {
        DialogflowCxToolVersionToolElFunctionSpecEl {
            input_schema: core::default::Default::default(),
            output_schema: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElFunctionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElFunctionSpecElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolVersionToolElFunctionSpecElRef {
        DialogflowCxToolVersionToolElFunctionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElFunctionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `input_schema` after provisioning.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the input of the function.\nThis input is a JSON object that contains the function's parameters as properties of the object"]
    pub fn input_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.input_schema", self.base))
    }
    #[doc = "Get a reference to the value of field `output_schema` after provisioning.\nOptional. The JSON schema is encapsulated in a [google.protobuf.Struct](https://protobuf.dev/reference/protobuf/google.protobuf/#struct) to describe the output of the function.\nThis output is a JSON object that contains the function's parameters as properties of the object"]
    pub fn output_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.output_schema", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<PrimField<String>>,
    key_name: PrimField<String>,
    request_location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_version_for_api_key: Option<PrimField<String>>,
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl {
    #[doc = "Set the field `api_key`.\nOptional. The API key. If the 'secretVersionForApiKey'' field is set, this field will be ignored."]
    pub fn set_api_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_key = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_version_for_api_key`.\nOptional. The name of the SecretManager secret version resource storing the API key.\nIf this field is set, the apiKey field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn set_secret_version_for_api_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_version_for_api_key = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl {
    type O =
        BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl {
    #[doc = "The parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub key_name: PrimField<String>,
    #[doc = "Key location in the request.\nSee [RequestLocation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#requestlocation) for valid values."]
    pub request_location: PrimField<String>,
}
impl BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl {
            api_key: core::default::Default::default(),
            key_name: self.key_name,
            request_location: self.request_location,
            secret_version_for_api_key: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigElRef {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\nOptional. The API key. If the 'secretVersionForApiKey'' field is set, this field will be ignored."]
    pub fn api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\nThe parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `request_location` after provisioning.\nKey location in the request.\nSee [RequestLocation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#requestlocation) for valid values."]
    pub fn request_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `secret_version_for_api_key` after provisioning.\nOptional. The name of the SecretManager secret version resource storing the API key.\nIf this field is set, the apiKey field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn secret_version_for_api_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version_for_api_key", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_version_for_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl {
    #[doc = "Set the field `secret_version_for_token`.\nOptional. The name of the SecretManager secret version resource storing the Bearer token. If this field is set, the 'token' field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn set_secret_version_for_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_version_for_token = Some(v.into());
        self
    }
    #[doc = "Set the field `token`.\nOptional. The text token appended to the text Bearer to the request Authorization header.\n[Session parameters reference](https://cloud.google.com/dialogflow/cx/docs/concept/parameter#session-ref) can be used to pass the token dynamically, e.g. '$session.params.parameter-id'."]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl
{
    type O = BlockAssignable<
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl {}
impl BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl {
    pub fn build(
        self,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl {
            secret_version_for_token: core::default::Default::default(),
            token: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `secret_version_for_token` after provisioning.\nOptional. The name of the SecretManager secret version resource storing the Bearer token. If this field is set, the 'token' field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn secret_version_for_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version_for_token", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\nOptional. The text token appended to the text Bearer to the request Authorization header.\n[Session parameters reference](https://cloud.google.com/dialogflow/cx/docs/concept/parameter#session-ref) can be used to pass the token dynamically, e.g. '$session.params.parameter-id'."]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl {
    client_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret: Option<PrimField<String>>,
    oauth_grant_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_version_for_client_secret: Option<PrimField<String>>,
    token_endpoint: PrimField<String>,
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl {
    #[doc = "Set the field `client_secret`.\nOptional. The client secret from the OAuth provider. If the 'secretVersionForClientSecret' field is set, this field will be ignored."]
    pub fn set_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\nOptional. The OAuth scopes to grant."]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `secret_version_for_client_secret`.\nOptional. The name of the SecretManager secret version resource storing the client secret.\nIf this field is set, the clientSecret field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn set_secret_version_for_client_secret(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_version_for_client_secret = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl {
    type O =
        BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl {
    #[doc = "The client ID from the OAuth provider."]
    pub client_id: PrimField<String>,
    #[doc = "OAuth grant types.\nSee [OauthGrantType](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#oauthgranttype) for valid values"]
    pub oauth_grant_type: PrimField<String>,
    #[doc = "The token endpoint in the OAuth provider to exchange for an access token."]
    pub token_endpoint: PrimField<String>,
}
impl BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            oauth_grant_type: self.oauth_grant_type,
            scopes: core::default::Default::default(),
            secret_version_for_client_secret: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigElRef {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client ID from the OAuth provider."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nOptional. The client secret from the OAuth provider. If the 'secretVersionForClientSecret' field is set, this field will be ignored."]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_grant_type` after provisioning.\nOAuth grant types.\nSee [OauthGrantType](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#oauthgranttype) for valid values"]
    pub fn oauth_grant_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_grant_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nOptional. The OAuth scopes to grant."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `secret_version_for_client_secret` after provisioning.\nOptional. The name of the SecretManager secret version resource storing the client secret.\nIf this field is set, the clientSecret field will be ignored.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn secret_version_for_client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_version_for_client_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\nThe token endpoint in the OAuth provider to exchange for an access token."]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_agent_auth: Option<PrimField<String>>,
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
    #[doc = "Set the field `service_agent_auth`.\nOptional. Indicate the auth token type generated from the Diglogflow service agent.\nThe generated token is sent in the Authorization header.\nSee [ServiceAgentAuth](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#serviceagentauth) for valid values."]
    pub fn set_service_agent_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_agent_auth = Some(v.into());
        self
    }
}
impl ToListMappable
    for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl
{
    type O = BlockAssignable<
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl
{}
impl BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
    pub fn build(
        self,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
            service_agent_auth: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_agent_auth` after provisioning.\nOptional. Indicate the auth token type generated from the Diglogflow service agent.\nThe generated token is sent in the Authorization header.\nSee [ServiceAgentAuth](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#serviceagentauth) for valid values."]
    pub fn service_agent_auth(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_agent_auth", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElDynamic {
    api_key_config: Option<
        DynamicBlock<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl>,
    >,
    bearer_token_config: Option<
        DynamicBlock<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl>,
    >,
    oauth_config: Option<
        DynamicBlock<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl>,
    >,
    service_agent_auth_config: Option<
        DynamicBlock<
            DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_config:
        Option<Vec<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bearer_token_config:
        Option<Vec<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_config:
        Option<Vec<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_agent_auth_config: Option<
        Vec<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl>,
    >,
    dynamic: DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElDynamic,
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_key_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `bearer_token_config`.\n"]
    pub fn set_bearer_token_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bearer_token_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bearer_token_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_agent_auth_config`.\n"]
    pub fn set_service_agent_auth_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_agent_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_agent_auth_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl {}
impl BuildDialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_agent_auth_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElRef {
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> ListRef<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElApiKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_key_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> ListRef<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElBearerTokenConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bearer_token_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(
        &self,
    ) -> ListRef<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElOauthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_agent_auth_config` after provisioning.\n"]
    pub fn service_agent_auth_config(
        &self,
    ) -> ListRef<
        DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_agent_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl {}
impl ToListMappable for DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl {
    #[doc = "The name of [Service Directory](https://cloud.google.com/service-directory/docs) service.\nFormat: projects/<ProjectID>/locations/<LocationID>/namespaces/<NamespaceID>/services/<ServiceID>. LocationID of the service directory must be the same as the location of the agent."]
    pub service: PrimField<String>,
}
impl BuildDialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl {
        DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigElRef {
        DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of [Service Directory](https://cloud.google.com/service-directory/docs) service.\nFormat: projects/<ProjectID>/locations/<LocationID>/namespaces/<NamespaceID>/services/<ServiceID>. LocationID of the service directory must be the same as the location of the agent."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl {
    cert: PrimField<String>,
    display_name: PrimField<String>,
}
impl DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl {}
impl ToListMappable for DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl {
    #[doc = "The allowed custom CA certificates (in DER format) for HTTPS verification. This overrides the default SSL trust store.\nIf this is empty or unspecified, Dialogflow will use Google's default trust store to verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt name\".\nFor instance a certificate can be self-signed using the following command:\n'''\n  openssl x509 -req -days 200 -in example.com.csr \\\n    -signkey example.com.key \\\n    -out example.com.crt \\\n    -extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")\n'''\nA base64-encoded string."]
    pub cert: PrimField<String>,
    #[doc = "The name of the allowed custom CA certificates. This can be used to disambiguate the custom CA certificates."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl {
        DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl {
            cert: self.cert,
            display_name: self.display_name,
        }
    }
}
pub struct DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsElRef {
        DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\nThe allowed custom CA certificates (in DER format) for HTTPS verification. This overrides the default SSL trust store.\nIf this is empty or unspecified, Dialogflow will use Google's default trust store to verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt name\".\nFor instance a certificate can be self-signed using the following command:\n'''\n  openssl x509 -req -days 200 -in example.com.csr \\\n    -signkey example.com.key \\\n    -out example.com.crt \\\n    -extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")\n'''\nA base64-encoded string."]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of the allowed custom CA certificates. This can be used to disambiguate the custom CA certificates."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElDynamic {
    ca_certs: Option<DynamicBlock<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<Vec<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl>>,
    dynamic: DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElDynamic,
}
impl DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ca_certs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ca_certs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl {}
impl BuildDialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl {
        DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl {
            ca_certs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElRef {
        DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(
        &self,
    ) -> ListRef<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxToolVersionToolElOpenApiSpecElDynamic {
    authentication:
        Option<DynamicBlock<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl>>,
    service_directory_config:
        Option<DynamicBlock<DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl>>,
    tls_config: Option<DynamicBlock<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolElOpenApiSpecEl {
    text_schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication: Option<Vec<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config:
        Option<Vec<DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<Vec<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl>>,
    dynamic: DialogflowCxToolVersionToolElOpenApiSpecElDynamic,
}
impl DialogflowCxToolVersionToolElOpenApiSpecEl {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<
            BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_directory_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_directory_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tls_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tls_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxToolVersionToolElOpenApiSpecEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolElOpenApiSpecEl {
    #[doc = "The OpenAPI schema specified as a text.\nThis field is part of a union field 'schema': only one of 'textSchema' may be set."]
    pub text_schema: PrimField<String>,
}
impl BuildDialogflowCxToolVersionToolElOpenApiSpecEl {
    pub fn build(self) -> DialogflowCxToolVersionToolElOpenApiSpecEl {
        DialogflowCxToolVersionToolElOpenApiSpecEl {
            text_schema: self.text_schema,
            authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElOpenApiSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElOpenApiSpecElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolVersionToolElOpenApiSpecElRef {
        DialogflowCxToolVersionToolElOpenApiSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElOpenApiSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text_schema` after provisioning.\nThe OpenAPI schema specified as a text.\nThis field is part of a union field 'schema': only one of 'textSchema' may be set."]
    pub fn text_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_schema", self.base))
    }
    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(
        &self,
    ) -> ListRef<DialogflowCxToolVersionToolElOpenApiSpecElAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<DialogflowCxToolVersionToolElOpenApiSpecElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<DialogflowCxToolVersionToolElOpenApiSpecElTlsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxToolVersionToolElDynamic {
    data_store_spec: Option<DynamicBlock<DialogflowCxToolVersionToolElDataStoreSpecEl>>,
    function_spec: Option<DynamicBlock<DialogflowCxToolVersionToolElFunctionSpecEl>>,
    open_api_spec: Option<DynamicBlock<DialogflowCxToolVersionToolElOpenApiSpecEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxToolVersionToolEl {
    description: PrimField<String>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_spec: Option<Vec<DialogflowCxToolVersionToolElDataStoreSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_spec: Option<Vec<DialogflowCxToolVersionToolElFunctionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_api_spec: Option<Vec<DialogflowCxToolVersionToolElOpenApiSpecEl>>,
    dynamic: DialogflowCxToolVersionToolElDynamic,
}
impl DialogflowCxToolVersionToolEl {
    #[doc = "Set the field `data_store_spec`.\n"]
    pub fn set_data_store_spec(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolVersionToolElDataStoreSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_store_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `function_spec`.\n"]
    pub fn set_function_spec(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolVersionToolElFunctionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.function_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.function_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `open_api_spec`.\n"]
    pub fn set_open_api_spec(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolVersionToolElOpenApiSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.open_api_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.open_api_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DialogflowCxToolVersionToolEl {
    type O = BlockAssignable<DialogflowCxToolVersionToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolVersionToolEl {
    #[doc = "High level description of the Tool and its usage."]
    pub description: PrimField<String>,
    #[doc = "The human-readable name of the tool, unique within the agent."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxToolVersionToolEl {
    pub fn build(self) -> DialogflowCxToolVersionToolEl {
        DialogflowCxToolVersionToolEl {
            description: self.description,
            display_name: self.display_name,
            data_store_spec: core::default::Default::default(),
            function_spec: core::default::Default::default(),
            open_api_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxToolVersionToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolVersionToolElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolVersionToolElRef {
        DialogflowCxToolVersionToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolVersionToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHigh level description of the Tool and its usage."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the tool, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Tool.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/tools/<Tool ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `tool_type` after provisioning.\nThe tool type."]
    pub fn tool_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tool_type", self.base))
    }
    #[doc = "Get a reference to the value of field `data_store_spec` after provisioning.\n"]
    pub fn data_store_spec(&self) -> ListRef<DialogflowCxToolVersionToolElDataStoreSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_spec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `function_spec` after provisioning.\n"]
    pub fn function_spec(&self) -> ListRef<DialogflowCxToolVersionToolElFunctionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.function_spec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `open_api_spec` after provisioning.\n"]
    pub fn open_api_spec(&self) -> ListRef<DialogflowCxToolVersionToolElOpenApiSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_spec", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxToolVersionDynamic {
    tool: Option<DynamicBlock<DialogflowCxToolVersionToolEl>>,
}
