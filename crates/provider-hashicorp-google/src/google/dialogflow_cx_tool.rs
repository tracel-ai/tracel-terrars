use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DialogflowCxToolData {
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
    description: PrimField<String>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_spec: Option<Vec<DialogflowCxToolDataStoreSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_spec: Option<Vec<DialogflowCxToolFunctionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_api_spec: Option<Vec<DialogflowCxToolOpenApiSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowCxToolTimeoutsEl>,
    dynamic: DialogflowCxToolDynamic,
}
struct DialogflowCxTool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowCxToolData>,
}
#[derive(Clone)]
pub struct DialogflowCxTool(Rc<DialogflowCxTool_>);
impl DialogflowCxTool {
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
    #[doc = "Set the field `parent`.\nThe agent to create a Tool for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn set_parent(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().parent = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store_spec`.\n"]
    pub fn set_data_store_spec(
        self,
        v: impl Into<BlockAssignable<DialogflowCxToolDataStoreSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_store_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_store_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `function_spec`.\n"]
    pub fn set_function_spec(
        self,
        v: impl Into<BlockAssignable<DialogflowCxToolFunctionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().function_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.function_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `open_api_spec`.\n"]
    pub fn set_open_api_spec(
        self,
        v: impl Into<BlockAssignable<DialogflowCxToolOpenApiSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().open_api_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.open_api_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowCxToolTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHigh level description of the Tool and its usage."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the tool, unique within the agent."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Tool.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/tools/<Tool ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Tool for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tool_type` after provisioning.\nThe tool type."]
    pub fn tool_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tool_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_spec` after provisioning.\n"]
    pub fn data_store_spec(&self) -> ListRef<DialogflowCxToolDataStoreSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `function_spec` after provisioning.\n"]
    pub fn function_spec(&self) -> ListRef<DialogflowCxToolFunctionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.function_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `open_api_spec` after provisioning.\n"]
    pub fn open_api_spec(&self) -> ListRef<DialogflowCxToolOpenApiSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxToolTimeoutsElRef {
        DialogflowCxToolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DialogflowCxTool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DialogflowCxTool {}
impl ToListMappable for DialogflowCxTool {
    type O = ListRef<DialogflowCxToolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DialogflowCxTool_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_cx_tool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDialogflowCxTool {
    pub tf_id: String,
    #[doc = "High level description of the Tool and its usage."]
    pub description: PrimField<String>,
    #[doc = "The human-readable name of the tool, unique within the agent."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxTool {
    pub fn build(self, stack: &mut Stack) -> DialogflowCxTool {
        let out = DialogflowCxTool(Rc::new(DialogflowCxTool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowCxToolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: self.description,
                display_name: self.display_name,
                id: core::default::Default::default(),
                parent: core::default::Default::default(),
                data_store_spec: core::default::Default::default(),
                function_spec: core::default::Default::default(),
                open_api_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DialogflowCxToolRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DialogflowCxToolRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nHigh level description of the Tool and its usage."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the tool, unique within the agent."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the Tool.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/tools/<Tool ID>."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe agent to create a Tool for.\nFormat: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tool_type` after provisioning.\nThe tool type."]
    pub fn tool_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tool_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_spec` after provisioning.\n"]
    pub fn data_store_spec(&self) -> ListRef<DialogflowCxToolDataStoreSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `function_spec` after provisioning.\n"]
    pub fn function_spec(&self) -> ListRef<DialogflowCxToolFunctionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.function_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `open_api_spec` after provisioning.\n"]
    pub fn open_api_spec(&self) -> ListRef<DialogflowCxToolOpenApiSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxToolTimeoutsElRef {
        DialogflowCxToolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolDataStoreSpecElDataStoreConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_processing_mode: Option<PrimField<String>>,
}
impl DialogflowCxToolDataStoreSpecElDataStoreConnectionsEl {
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
impl ToListMappable for DialogflowCxToolDataStoreSpecElDataStoreConnectionsEl {
    type O = BlockAssignable<DialogflowCxToolDataStoreSpecElDataStoreConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolDataStoreSpecElDataStoreConnectionsEl {}
impl BuildDialogflowCxToolDataStoreSpecElDataStoreConnectionsEl {
    pub fn build(self) -> DialogflowCxToolDataStoreSpecElDataStoreConnectionsEl {
        DialogflowCxToolDataStoreSpecElDataStoreConnectionsEl {
            data_store: core::default::Default::default(),
            data_store_type: core::default::Default::default(),
            document_processing_mode: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolDataStoreSpecElDataStoreConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolDataStoreSpecElDataStoreConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolDataStoreSpecElDataStoreConnectionsElRef {
        DialogflowCxToolDataStoreSpecElDataStoreConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolDataStoreSpecElDataStoreConnectionsElRef {
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
pub struct DialogflowCxToolDataStoreSpecElFallbackPromptEl {}
impl DialogflowCxToolDataStoreSpecElFallbackPromptEl {}
impl ToListMappable for DialogflowCxToolDataStoreSpecElFallbackPromptEl {
    type O = BlockAssignable<DialogflowCxToolDataStoreSpecElFallbackPromptEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolDataStoreSpecElFallbackPromptEl {}
impl BuildDialogflowCxToolDataStoreSpecElFallbackPromptEl {
    pub fn build(self) -> DialogflowCxToolDataStoreSpecElFallbackPromptEl {
        DialogflowCxToolDataStoreSpecElFallbackPromptEl {}
    }
}
pub struct DialogflowCxToolDataStoreSpecElFallbackPromptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolDataStoreSpecElFallbackPromptElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolDataStoreSpecElFallbackPromptElRef {
        DialogflowCxToolDataStoreSpecElFallbackPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolDataStoreSpecElFallbackPromptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxToolDataStoreSpecElDynamic {
    data_store_connections:
        Option<DynamicBlock<DialogflowCxToolDataStoreSpecElDataStoreConnectionsEl>>,
    fallback_prompt: Option<DynamicBlock<DialogflowCxToolDataStoreSpecElFallbackPromptEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxToolDataStoreSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_connections: Option<Vec<DialogflowCxToolDataStoreSpecElDataStoreConnectionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fallback_prompt: Option<Vec<DialogflowCxToolDataStoreSpecElFallbackPromptEl>>,
    dynamic: DialogflowCxToolDataStoreSpecElDynamic,
}
impl DialogflowCxToolDataStoreSpecEl {
    #[doc = "Set the field `data_store_connections`.\n"]
    pub fn set_data_store_connections(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolDataStoreSpecElDataStoreConnectionsEl>>,
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
        v: impl Into<BlockAssignable<DialogflowCxToolDataStoreSpecElFallbackPromptEl>>,
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
impl ToListMappable for DialogflowCxToolDataStoreSpecEl {
    type O = BlockAssignable<DialogflowCxToolDataStoreSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolDataStoreSpecEl {}
impl BuildDialogflowCxToolDataStoreSpecEl {
    pub fn build(self) -> DialogflowCxToolDataStoreSpecEl {
        DialogflowCxToolDataStoreSpecEl {
            data_store_connections: core::default::Default::default(),
            fallback_prompt: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxToolDataStoreSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolDataStoreSpecElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolDataStoreSpecElRef {
        DialogflowCxToolDataStoreSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolDataStoreSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_store_connections` after provisioning.\n"]
    pub fn data_store_connections(
        &self,
    ) -> ListRef<DialogflowCxToolDataStoreSpecElDataStoreConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_connections", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fallback_prompt` after provisioning.\n"]
    pub fn fallback_prompt(&self) -> ListRef<DialogflowCxToolDataStoreSpecElFallbackPromptElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fallback_prompt", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolFunctionSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    input_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_schema: Option<PrimField<String>>,
}
impl DialogflowCxToolFunctionSpecEl {
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
impl ToListMappable for DialogflowCxToolFunctionSpecEl {
    type O = BlockAssignable<DialogflowCxToolFunctionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolFunctionSpecEl {}
impl BuildDialogflowCxToolFunctionSpecEl {
    pub fn build(self) -> DialogflowCxToolFunctionSpecEl {
        DialogflowCxToolFunctionSpecEl {
            input_schema: core::default::Default::default(),
            output_schema: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolFunctionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolFunctionSpecElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolFunctionSpecElRef {
        DialogflowCxToolFunctionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolFunctionSpecElRef {
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
pub struct DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<PrimField<String>>,
    key_name: PrimField<String>,
    request_location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_version_for_api_key: Option<PrimField<String>>,
}
impl DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl {
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
impl ToListMappable for DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl {
    type O = BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl {
    #[doc = "The parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub key_name: PrimField<String>,
    #[doc = "Key location in the request.\nSee [RequestLocation](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#requestlocation) for valid values."]
    pub request_location: PrimField<String>,
}
impl BuildDialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl {
    pub fn build(self) -> DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl {
        DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl {
            api_key: core::default::Default::default(),
            key_name: self.key_name,
            request_location: self.request_location,
            secret_version_for_api_key: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigElRef {
        DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigElRef {
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
pub struct DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_version_for_token: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}
impl DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl {
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
impl ToListMappable for DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl {
    type O = BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl {}
impl BuildDialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl {
    pub fn build(self) -> DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl {
        DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl {
            secret_version_for_token: core::default::Default::default(),
            token: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
        DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigElRef {
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
pub struct DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl {
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
impl DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl {
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
impl ToListMappable for DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl {
    type O = BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl {
    #[doc = "The client ID from the OAuth provider."]
    pub client_id: PrimField<String>,
    #[doc = "OAuth grant types.\nSee [OauthGrantType](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#oauthgranttype) for valid values"]
    pub oauth_grant_type: PrimField<String>,
    #[doc = "The token endpoint in the OAuth provider to exchange for an access token."]
    pub token_endpoint: PrimField<String>,
}
impl BuildDialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl {
    pub fn build(self) -> DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl {
        DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            oauth_grant_type: self.oauth_grant_type,
            scopes: core::default::Default::default(),
            secret_version_for_client_secret: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}
pub struct DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigElRef {
        DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigElRef {
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
pub struct DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_agent_auth: Option<PrimField<String>>,
}
impl DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
    #[doc = "Set the field `service_agent_auth`.\nOptional. Indicate the auth token type generated from the Diglogflow service agent.\nThe generated token is sent in the Authorization header.\nSee [ServiceAgentAuth](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.tools#serviceagentauth) for valid values."]
    pub fn set_service_agent_auth(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_agent_auth = Some(v.into());
        self
    }
}
impl ToListMappable for DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
    type O = BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {}
impl BuildDialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
    pub fn build(self) -> DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
        DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl {
            service_agent_auth: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
        DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef {
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
struct DialogflowCxToolOpenApiSpecElAuthenticationElDynamic {
    api_key_config:
        Option<DynamicBlock<DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl>>,
    bearer_token_config:
        Option<DynamicBlock<DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl>>,
    oauth_config: Option<DynamicBlock<DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl>>,
    service_agent_auth_config:
        Option<DynamicBlock<DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxToolOpenApiSpecElAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_config: Option<Vec<DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bearer_token_config:
        Option<Vec<DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_config: Option<Vec<DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_agent_auth_config:
        Option<Vec<DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl>>,
    dynamic: DialogflowCxToolOpenApiSpecElAuthenticationElDynamic,
}
impl DialogflowCxToolOpenApiSpecElAuthenticationEl {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigEl>>,
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
        v: impl Into<BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigEl>>,
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
        v: impl Into<BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigEl>>,
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
            BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigEl>,
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
impl ToListMappable for DialogflowCxToolOpenApiSpecElAuthenticationEl {
    type O = BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolOpenApiSpecElAuthenticationEl {}
impl BuildDialogflowCxToolOpenApiSpecElAuthenticationEl {
    pub fn build(self) -> DialogflowCxToolOpenApiSpecElAuthenticationEl {
        DialogflowCxToolOpenApiSpecElAuthenticationEl {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_agent_auth_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxToolOpenApiSpecElAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolOpenApiSpecElAuthenticationElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolOpenApiSpecElAuthenticationElRef {
        DialogflowCxToolOpenApiSpecElAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolOpenApiSpecElAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> ListRef<DialogflowCxToolOpenApiSpecElAuthenticationElApiKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_key_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> ListRef<DialogflowCxToolOpenApiSpecElAuthenticationElBearerTokenConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bearer_token_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(
        &self,
    ) -> ListRef<DialogflowCxToolOpenApiSpecElAuthenticationElOauthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_agent_auth_config` after provisioning.\n"]
    pub fn service_agent_auth_config(
        &self,
    ) -> ListRef<DialogflowCxToolOpenApiSpecElAuthenticationElServiceAgentAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_agent_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl DialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl {}
impl ToListMappable for DialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl {
    type O = BlockAssignable<DialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl {
    #[doc = "The name of [Service Directory](https://cloud.google.com/service-directory/docs) service.\nFormat: projects/<ProjectID>/locations/<LocationID>/namespaces/<NamespaceID>/services/<ServiceID>. LocationID of the service directory must be the same as the location of the agent."]
    pub service: PrimField<String>,
}
impl BuildDialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl {
    pub fn build(self) -> DialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl {
        DialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct DialogflowCxToolOpenApiSpecElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolOpenApiSpecElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolOpenApiSpecElServiceDirectoryConfigElRef {
        DialogflowCxToolOpenApiSpecElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolOpenApiSpecElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of [Service Directory](https://cloud.google.com/service-directory/docs) service.\nFormat: projects/<ProjectID>/locations/<LocationID>/namespaces/<NamespaceID>/services/<ServiceID>. LocationID of the service directory must be the same as the location of the agent."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl {
    cert: PrimField<String>,
    display_name: PrimField<String>,
}
impl DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl {}
impl ToListMappable for DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl {
    type O = BlockAssignable<DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl {
    #[doc = "The allowed custom CA certificates (in DER format) for HTTPS verification. This overrides the default SSL trust store.\nIf this is empty or unspecified, Dialogflow will use Google's default trust store to verify certificates.\nN.B. Make sure the HTTPS server certificates are signed with \"subject alt name\".\nFor instance a certificate can be self-signed using the following command:\n'''\n  openssl x509 -req -days 200 -in example.com.csr \\\n    -signkey example.com.key \\\n    -out example.com.crt \\\n    -extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")\n'''\nA base64-encoded string."]
    pub cert: PrimField<String>,
    #[doc = "The name of the allowed custom CA certificates. This can be used to disambiguate the custom CA certificates."]
    pub display_name: PrimField<String>,
}
impl BuildDialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl {
    pub fn build(self) -> DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl {
        DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl {
            cert: self.cert,
            display_name: self.display_name,
        }
    }
}
pub struct DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsElRef {
        DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsElRef {
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
struct DialogflowCxToolOpenApiSpecElTlsConfigElDynamic {
    ca_certs: Option<DynamicBlock<DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxToolOpenApiSpecElTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<Vec<DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl>>,
    dynamic: DialogflowCxToolOpenApiSpecElTlsConfigElDynamic,
}
impl DialogflowCxToolOpenApiSpecElTlsConfigEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsEl>>,
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
impl ToListMappable for DialogflowCxToolOpenApiSpecElTlsConfigEl {
    type O = BlockAssignable<DialogflowCxToolOpenApiSpecElTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolOpenApiSpecElTlsConfigEl {}
impl BuildDialogflowCxToolOpenApiSpecElTlsConfigEl {
    pub fn build(self) -> DialogflowCxToolOpenApiSpecElTlsConfigEl {
        DialogflowCxToolOpenApiSpecElTlsConfigEl {
            ca_certs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxToolOpenApiSpecElTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolOpenApiSpecElTlsConfigElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolOpenApiSpecElTlsConfigElRef {
        DialogflowCxToolOpenApiSpecElTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolOpenApiSpecElTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> ListRef<DialogflowCxToolOpenApiSpecElTlsConfigElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize, Default)]
struct DialogflowCxToolOpenApiSpecElDynamic {
    authentication: Option<DynamicBlock<DialogflowCxToolOpenApiSpecElAuthenticationEl>>,
    service_directory_config:
        Option<DynamicBlock<DialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl>>,
    tls_config: Option<DynamicBlock<DialogflowCxToolOpenApiSpecElTlsConfigEl>>,
}
#[derive(Serialize)]
pub struct DialogflowCxToolOpenApiSpecEl {
    text_schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication: Option<Vec<DialogflowCxToolOpenApiSpecElAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config: Option<Vec<DialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<Vec<DialogflowCxToolOpenApiSpecElTlsConfigEl>>,
    dynamic: DialogflowCxToolOpenApiSpecElDynamic,
}
impl DialogflowCxToolOpenApiSpecEl {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl Into<BlockAssignable<DialogflowCxToolOpenApiSpecElAuthenticationEl>>,
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
        v: impl Into<BlockAssignable<DialogflowCxToolOpenApiSpecElServiceDirectoryConfigEl>>,
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
        v: impl Into<BlockAssignable<DialogflowCxToolOpenApiSpecElTlsConfigEl>>,
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
impl ToListMappable for DialogflowCxToolOpenApiSpecEl {
    type O = BlockAssignable<DialogflowCxToolOpenApiSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolOpenApiSpecEl {
    #[doc = "The OpenAPI schema specified as a text.\nThis field is part of a union field 'schema': only one of 'textSchema' may be set."]
    pub text_schema: PrimField<String>,
}
impl BuildDialogflowCxToolOpenApiSpecEl {
    pub fn build(self) -> DialogflowCxToolOpenApiSpecEl {
        DialogflowCxToolOpenApiSpecEl {
            text_schema: self.text_schema,
            authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DialogflowCxToolOpenApiSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolOpenApiSpecElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolOpenApiSpecElRef {
        DialogflowCxToolOpenApiSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolOpenApiSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `text_schema` after provisioning.\nThe OpenAPI schema specified as a text.\nThis field is part of a union field 'schema': only one of 'textSchema' may be set."]
    pub fn text_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.text_schema", self.base))
    }
    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> ListRef<DialogflowCxToolOpenApiSpecElAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<DialogflowCxToolOpenApiSpecElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<DialogflowCxToolOpenApiSpecElTlsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
}
#[derive(Serialize)]
pub struct DialogflowCxToolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DialogflowCxToolTimeoutsEl {
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
impl ToListMappable for DialogflowCxToolTimeoutsEl {
    type O = BlockAssignable<DialogflowCxToolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDialogflowCxToolTimeoutsEl {}
impl BuildDialogflowCxToolTimeoutsEl {
    pub fn build(self) -> DialogflowCxToolTimeoutsEl {
        DialogflowCxToolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DialogflowCxToolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DialogflowCxToolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowCxToolTimeoutsElRef {
        DialogflowCxToolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DialogflowCxToolTimeoutsElRef {
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
struct DialogflowCxToolDynamic {
    data_store_spec: Option<DynamicBlock<DialogflowCxToolDataStoreSpecEl>>,
    function_spec: Option<DynamicBlock<DialogflowCxToolFunctionSpecEl>>,
    open_api_spec: Option<DynamicBlock<DialogflowCxToolOpenApiSpecEl>>,
}
