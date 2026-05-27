use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CesToolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    app: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    tool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_function: Option<Vec<CesToolClientFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_tool: Option<Vec<CesToolDataStoreToolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_search_tool: Option<Vec<CesToolGoogleSearchToolEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_function: Option<Vec<CesToolPythonFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CesToolTimeoutsEl>,
    dynamic: CesToolDynamic,
}
struct CesTool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CesToolData>,
}
#[derive(Clone)]
pub struct CesTool(Rc<CesTool_>);
impl CesTool {
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
    #[doc = "Set the field `execution_type`.\nPossible values:\nSYNCHRONOUS\nASYNCHRONOUS"]
    pub fn set_execution_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().execution_type = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `client_function`.\n"]
    pub fn set_client_function(
        self,
        v: impl Into<BlockAssignable<CesToolClientFunctionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_function = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_function = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_store_tool`.\n"]
    pub fn set_data_store_tool(
        self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_store_tool = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_store_tool = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `google_search_tool`.\n"]
    pub fn set_google_search_tool(
        self,
        v: impl Into<BlockAssignable<CesToolGoogleSearchToolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().google_search_tool = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.google_search_tool = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `python_function`.\n"]
    pub fn set_python_function(
        self,
        v: impl Into<BlockAssignable<CesToolPythonFunctionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().python_function = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.python_function = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CesToolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the tool was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the tool, derived based on the tool's type. For\nexample, display name of a ClientFunction is derived\nfrom its 'name' property."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_type` after provisioning.\nPossible values:\nSYNCHRONOUS\nASYNCHRONOUS"]
    pub fn execution_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_summary` after provisioning.\nIf the tool is generated by the LLM assistant, this field contains a\ndescriptive summary of the generation."]
    pub fn generated_summary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_summary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the tool.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `open_api_tool` after provisioning.\nA remote API tool defined by an OpenAPI schema."]
    pub fn open_api_tool(&self) -> ListRef<CesToolOpenApiToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_tool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `system_tool` after provisioning.\nThe system tool."]
    pub fn system_tool(&self) -> ListRef<CesToolSystemToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.system_tool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tool_id` after provisioning.\nThe ID to use for the tool, which will become the final component of\nthe tool's resource name. If not provided, a unique ID will be\nautomatically assigned for the tool."]
    pub fn tool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the tool was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_function` after provisioning.\n"]
    pub fn client_function(&self) -> ListRef<CesToolClientFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_function", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_tool` after provisioning.\n"]
    pub fn data_store_tool(&self) -> ListRef<CesToolDataStoreToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_tool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `google_search_tool` after provisioning.\n"]
    pub fn google_search_tool(&self) -> ListRef<CesToolGoogleSearchToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_search_tool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `python_function` after provisioning.\n"]
    pub fn python_function(&self) -> ListRef<CesToolPythonFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_function", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesToolTimeoutsElRef {
        CesToolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CesTool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CesTool {}
impl ToListMappable for CesTool {
    type O = ListRef<CesToolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CesTool_ {
    fn extract_resource_type(&self) -> String {
        "google_ces_tool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCesTool {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The ID to use for the tool, which will become the final component of\nthe tool's resource name. If not provided, a unique ID will be\nautomatically assigned for the tool."]
    pub tool_id: PrimField<String>,
}
impl BuildCesTool {
    pub fn build(self, stack: &mut Stack) -> CesTool {
        let out = CesTool(Rc::new(CesTool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CesToolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app: self.app,
                deletion_policy: core::default::Default::default(),
                execution_type: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                tool_id: self.tool_id,
                client_function: core::default::Default::default(),
                data_store_tool: core::default::Default::default(),
                google_search_tool: core::default::Default::default(),
                python_function: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CesToolRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CesToolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the tool was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the tool, derived based on the tool's type. For\nexample, display name of a ClientFunction is derived\nfrom its 'name' property."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `execution_type` after provisioning.\nPossible values:\nSYNCHRONOUS\nASYNCHRONOUS"]
    pub fn execution_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.execution_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `generated_summary` after provisioning.\nIf the tool is generated by the LLM assistant, this field contains a\ndescriptive summary of the generation."]
    pub fn generated_summary(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.generated_summary", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the tool.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `open_api_tool` after provisioning.\nA remote API tool defined by an OpenAPI schema."]
    pub fn open_api_tool(&self) -> ListRef<CesToolOpenApiToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_tool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `system_tool` after provisioning.\nThe system tool."]
    pub fn system_tool(&self) -> ListRef<CesToolSystemToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.system_tool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tool_id` after provisioning.\nThe ID to use for the tool, which will become the final component of\nthe tool's resource name. If not provided, a unique ID will be\nautomatically assigned for the tool."]
    pub fn tool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.tool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the tool was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_function` after provisioning.\n"]
    pub fn client_function(&self) -> ListRef<CesToolClientFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_function", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_tool` after provisioning.\n"]
    pub fn data_store_tool(&self) -> ListRef<CesToolDataStoreToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_tool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `google_search_tool` after provisioning.\n"]
    pub fn google_search_tool(&self) -> ListRef<CesToolGoogleSearchToolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.google_search_tool", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `python_function` after provisioning.\n"]
    pub fn python_function(&self) -> ListRef<CesToolPythonFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.python_function", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesToolTimeoutsElRef {
        CesToolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_location: Option<PrimField<String>>,
}
impl CesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl {
    #[doc = "Set the field `api_key_secret_version`.\n"]
    pub fn set_api_key_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_key_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `request_location`.\n"]
    pub fn set_request_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.request_location = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl {
    type O = BlockAssignable<CesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl {}
impl BuildCesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl {
    pub fn build(self) -> CesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl {
        CesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl {
            api_key_secret_version: core::default::Default::default(),
            key_name: core::default::Default::default(),
            request_location: core::default::Default::default(),
        }
    }
}
pub struct CesToolOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
        CesToolOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElApiAuthenticationElApiKeyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_secret_version` after provisioning.\n"]
    pub fn api_key_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `request_location` after provisioning.\n"]
    pub fn request_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}
impl CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl {
    type O = BlockAssignable<CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl {}
impl BuildCesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl {
    pub fn build(self) -> CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl {
        CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl {
            token: core::default::Default::default(),
        }
    }
}
pub struct CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigElRef {
        CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolElApiAuthenticationElOauthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_secret_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_grant_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    token_endpoint: Option<PrimField<String>>,
}
impl CesToolOpenApiToolElApiAuthenticationElOauthConfigEl {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_id = Some(v.into());
        self
    }
    #[doc = "Set the field `client_secret_version`.\n"]
    pub fn set_client_secret_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.client_secret_version = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_grant_type`.\n"]
    pub fn set_oauth_grant_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oauth_grant_type = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token_endpoint = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolOpenApiToolElApiAuthenticationElOauthConfigEl {
    type O = BlockAssignable<CesToolOpenApiToolElApiAuthenticationElOauthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolElApiAuthenticationElOauthConfigEl {}
impl BuildCesToolOpenApiToolElApiAuthenticationElOauthConfigEl {
    pub fn build(self) -> CesToolOpenApiToolElApiAuthenticationElOauthConfigEl {
        CesToolOpenApiToolElApiAuthenticationElOauthConfigEl {
            client_id: core::default::Default::default(),
            client_secret_version: core::default::Default::default(),
            oauth_grant_type: core::default::Default::default(),
            scopes: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}
pub struct CesToolOpenApiToolElApiAuthenticationElOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElApiAuthenticationElOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolOpenApiToolElApiAuthenticationElOauthConfigElRef {
        CesToolOpenApiToolElApiAuthenticationElOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElApiAuthenticationElOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret_version` after provisioning.\n"]
    pub fn client_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_grant_type` after provisioning.\n"]
    pub fn oauth_grant_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_grant_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.token_endpoint", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<PrimField<String>>,
}
impl CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_account = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {
    type O = BlockAssignable<CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {}
impl BuildCesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {
    pub fn build(self) -> CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {
        CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl {
            service_account: core::default::Default::default(),
        }
    }
}
pub struct CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef {
        CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl ToListMappable for CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {
    type O =
        BlockAssignable<CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl BuildCesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {
    pub fn build(self) -> CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {
        CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
    }
}
pub struct CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
        CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolElApiAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_config: Option<ListField<CesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bearer_token_config:
        Option<ListField<CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_config: Option<ListField<CesToolOpenApiToolElApiAuthenticationElOauthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_auth_config:
        Option<ListField<CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_agent_id_token_auth_config:
        Option<ListField<CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl>>,
}
impl CesToolOpenApiToolElApiAuthenticationEl {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl Into<ListField<CesToolOpenApiToolElApiAuthenticationElApiKeyConfigEl>>,
    ) -> Self {
        self.api_key_config = Some(v.into());
        self
    }
    #[doc = "Set the field `bearer_token_config`.\n"]
    pub fn set_bearer_token_config(
        mut self,
        v: impl Into<ListField<CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigEl>>,
    ) -> Self {
        self.bearer_token_config = Some(v.into());
        self
    }
    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v: impl Into<ListField<CesToolOpenApiToolElApiAuthenticationElOauthConfigEl>>,
    ) -> Self {
        self.oauth_config = Some(v.into());
        self
    }
    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v: impl Into<ListField<CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigEl>>,
    ) -> Self {
        self.service_account_auth_config = Some(v.into());
        self
    }
    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v: impl Into<ListField<CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigEl>>,
    ) -> Self {
        self.service_agent_id_token_auth_config = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolOpenApiToolElApiAuthenticationEl {
    type O = BlockAssignable<CesToolOpenApiToolElApiAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolElApiAuthenticationEl {}
impl BuildCesToolOpenApiToolElApiAuthenticationEl {
    pub fn build(self) -> CesToolOpenApiToolElApiAuthenticationEl {
        CesToolOpenApiToolElApiAuthenticationEl {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
        }
    }
}
pub struct CesToolOpenApiToolElApiAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElApiAuthenticationElRef {
    fn new(shared: StackShared, base: String) -> CesToolOpenApiToolElApiAuthenticationElRef {
        CesToolOpenApiToolElApiAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElApiAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> ListRef<CesToolOpenApiToolElApiAuthenticationElApiKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_key_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> ListRef<CesToolOpenApiToolElApiAuthenticationElBearerTokenConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bearer_token_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(&self) -> ListRef<CesToolOpenApiToolElApiAuthenticationElOauthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> ListRef<CesToolOpenApiToolElApiAuthenticationElServiceAccountAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_auth_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]
    pub fn service_agent_id_token_auth_config(
        &self,
    ) -> ListRef<CesToolOpenApiToolElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_agent_id_token_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolElServiceDirectoryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}
impl CesToolOpenApiToolElServiceDirectoryConfigEl {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolOpenApiToolElServiceDirectoryConfigEl {
    type O = BlockAssignable<CesToolOpenApiToolElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolElServiceDirectoryConfigEl {}
impl BuildCesToolOpenApiToolElServiceDirectoryConfigEl {
    pub fn build(self) -> CesToolOpenApiToolElServiceDirectoryConfigEl {
        CesToolOpenApiToolElServiceDirectoryConfigEl {
            service: core::default::Default::default(),
        }
    }
}
pub struct CesToolOpenApiToolElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElServiceDirectoryConfigElRef {
    fn new(shared: StackShared, base: String) -> CesToolOpenApiToolElServiceDirectoryConfigElRef {
        CesToolOpenApiToolElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolElTlsConfigElCaCertsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
}
impl CesToolOpenApiToolElTlsConfigElCaCertsEl {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cert = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolOpenApiToolElTlsConfigElCaCertsEl {
    type O = BlockAssignable<CesToolOpenApiToolElTlsConfigElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolElTlsConfigElCaCertsEl {}
impl BuildCesToolOpenApiToolElTlsConfigElCaCertsEl {
    pub fn build(self) -> CesToolOpenApiToolElTlsConfigElCaCertsEl {
        CesToolOpenApiToolElTlsConfigElCaCertsEl {
            cert: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}
pub struct CesToolOpenApiToolElTlsConfigElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElTlsConfigElCaCertsElRef {
    fn new(shared: StackShared, base: String) -> CesToolOpenApiToolElTlsConfigElCaCertsElRef {
        CesToolOpenApiToolElTlsConfigElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElTlsConfigElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolElTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<ListField<CesToolOpenApiToolElTlsConfigElCaCertsEl>>,
}
impl CesToolOpenApiToolElTlsConfigEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<ListField<CesToolOpenApiToolElTlsConfigElCaCertsEl>>,
    ) -> Self {
        self.ca_certs = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolOpenApiToolElTlsConfigEl {
    type O = BlockAssignable<CesToolOpenApiToolElTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolElTlsConfigEl {}
impl BuildCesToolOpenApiToolElTlsConfigEl {
    pub fn build(self) -> CesToolOpenApiToolElTlsConfigEl {
        CesToolOpenApiToolElTlsConfigEl {
            ca_certs: core::default::Default::default(),
        }
    }
}
pub struct CesToolOpenApiToolElTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElTlsConfigElRef {
    fn new(shared: StackShared, base: String) -> CesToolOpenApiToolElTlsConfigElRef {
        CesToolOpenApiToolElTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> ListRef<CesToolOpenApiToolElTlsConfigElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolOpenApiToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_authentication: Option<ListField<CesToolOpenApiToolElApiAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_unknown_fields: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_api_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config: Option<ListField<CesToolOpenApiToolElServiceDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<ListField<CesToolOpenApiToolElTlsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl CesToolOpenApiToolEl {
    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl Into<ListField<CesToolOpenApiToolElApiAuthenticationEl>>,
    ) -> Self {
        self.api_authentication = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_unknown_fields`.\n"]
    pub fn set_ignore_unknown_fields(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_unknown_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `open_api_schema`.\n"]
    pub fn set_open_api_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.open_api_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<ListField<CesToolOpenApiToolElServiceDirectoryConfigEl>>,
    ) -> Self {
        self.service_directory_config = Some(v.into());
        self
    }
    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        mut self,
        v: impl Into<ListField<CesToolOpenApiToolElTlsConfigEl>>,
    ) -> Self {
        self.tls_config = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolOpenApiToolEl {
    type O = BlockAssignable<CesToolOpenApiToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolOpenApiToolEl {}
impl BuildCesToolOpenApiToolEl {
    pub fn build(self) -> CesToolOpenApiToolEl {
        CesToolOpenApiToolEl {
            api_authentication: core::default::Default::default(),
            description: core::default::Default::default(),
            ignore_unknown_fields: core::default::Default::default(),
            name: core::default::Default::default(),
            open_api_schema: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}
pub struct CesToolOpenApiToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolOpenApiToolElRef {
    fn new(shared: StackShared, base: String) -> CesToolOpenApiToolElRef {
        CesToolOpenApiToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolOpenApiToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> ListRef<CesToolOpenApiToolElApiAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `ignore_unknown_fields` after provisioning.\n"]
    pub fn ignore_unknown_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_unknown_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `open_api_schema` after provisioning.\n"]
    pub fn open_api_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.open_api_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<CesToolOpenApiToolElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<CesToolOpenApiToolElTlsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolSystemToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl CesToolSystemToolEl {
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
}
impl ToListMappable for CesToolSystemToolEl {
    type O = BlockAssignable<CesToolSystemToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolSystemToolEl {}
impl BuildCesToolSystemToolEl {
    pub fn build(self) -> CesToolSystemToolEl {
        CesToolSystemToolEl {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct CesToolSystemToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolSystemToolElRef {
    fn new(shared: StackShared, base: String) -> CesToolSystemToolElRef {
        CesToolSystemToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolSystemToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolClientFunctionElParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    any_of: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    defs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_items: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_items: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_items: Option<PrimField<bool>>,
}
impl CesToolClientFunctionElParametersEl {
    #[doc = "Set the field `additional_properties`.\nDefines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn set_additional_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `any_of`.\nThe instance value should be valid against at least one of the schemas in this list."]
    pub fn set_any_of(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.any_of = Some(v.into());
        self
    }
    #[doc = "Set the field `default`.\nDefault value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be compatible\nwith the defined 'type' and other schema constraints."]
    pub fn set_default(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default = Some(v.into());
        self
    }
    #[doc = "Set the field `defs`.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn set_defs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.defs = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the data."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_`.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn set_enum(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enum_ = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\nSchema of the elements of Type.ARRAY."]
    pub fn set_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.items = Some(v.into());
        self
    }
    #[doc = "Set the field `max_items`.\nMaximum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn set_max_items(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_items = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum`.\nMaximum value for Type.INTEGER and Type.NUMBER."]
    pub fn set_maximum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum = Some(v.into());
        self
    }
    #[doc = "Set the field `min_items`.\nMinimum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn set_min_items(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_items = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum`.\nMinimum value for Type.INTEGER and Type.NUMBER."]
    pub fn set_minimum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum = Some(v.into());
        self
    }
    #[doc = "Set the field `nullable`.\nIndicates if the value may be null."]
    pub fn set_nullable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nullable = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_items`.\nSchemas of initial elements of Type.ARRAY."]
    pub fn set_prefix_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_items = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nProperties of Type.OBJECT."]
    pub fn set_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_`.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }
    #[doc = "Set the field `required`.\nRequired properties of Type.OBJECT."]
    pub fn set_required(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.required = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe title of the schema."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `unique_items`.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn set_unique_items(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_items = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolClientFunctionElParametersEl {
    type O = BlockAssignable<CesToolClientFunctionElParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolClientFunctionElParametersEl {
    #[doc = "The type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub type_: PrimField<String>,
}
impl BuildCesToolClientFunctionElParametersEl {
    pub fn build(self) -> CesToolClientFunctionElParametersEl {
        CesToolClientFunctionElParametersEl {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            max_items: core::default::Default::default(),
            maximum: core::default::Default::default(),
            min_items: core::default::Default::default(),
            minimum: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: self.type_,
            unique_items: core::default::Default::default(),
        }
    }
}
pub struct CesToolClientFunctionElParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolClientFunctionElParametersElRef {
    fn new(shared: StackShared, base: String) -> CesToolClientFunctionElParametersElRef {
        CesToolClientFunctionElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolClientFunctionElParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_properties` after provisioning.\nDefines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn additional_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `any_of` after provisioning.\nThe instance value should be valid against at least one of the schemas in this list."]
    pub fn any_of(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
    #[doc = "Get a reference to the value of field `default` after provisioning.\nDefault value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be compatible\nwith the defined 'type' and other schema constraints."]
    pub fn default(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
    #[doc = "Get a reference to the value of field `defs` after provisioning.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn defs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the data."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `enum_` after provisioning.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn enum_(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\nSchema of the elements of Type.ARRAY."]
    pub fn items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMaximum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn max_items(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum` after provisioning.\nMaximum value for Type.INTEGER and Type.NUMBER."]
    pub fn maximum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }
    #[doc = "Get a reference to the value of field `min_items` after provisioning.\nMinimum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn min_items(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_items", self.base))
    }
    #[doc = "Get a reference to the value of field `minimum` after provisioning.\nMinimum value for Type.INTEGER and Type.NUMBER."]
    pub fn minimum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }
    #[doc = "Get a reference to the value of field `nullable` after provisioning.\nIndicates if the value may be null."]
    pub fn nullable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_items` after provisioning.\nSchemas of initial elements of Type.ARRAY."]
    pub fn prefix_items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nProperties of Type.OBJECT."]
    pub fn properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\nRequired properties of Type.OBJECT."]
    pub fn required(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the schema."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `unique_items` after provisioning.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn unique_items(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolClientFunctionElResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_properties: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    any_of: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    defs: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    enum_: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_items: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_items: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nullable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix_items: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<PrimField<String>>,
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    ref_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unique_items: Option<PrimField<bool>>,
}
impl CesToolClientFunctionElResponseEl {
    #[doc = "Set the field `additional_properties`.\nDefines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn set_additional_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_properties = Some(v.into());
        self
    }
    #[doc = "Set the field `any_of`.\nThe instance value should be valid against at least one of the schemas in this list."]
    pub fn set_any_of(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.any_of = Some(v.into());
        self
    }
    #[doc = "Set the field `default`.\nDefault value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be compatible\nwith the defined 'type' and other schema constraints."]
    pub fn set_default(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default = Some(v.into());
        self
    }
    #[doc = "Set the field `defs`.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn set_defs(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.defs = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the data."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `enum_`.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn set_enum(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.enum_ = Some(v.into());
        self
    }
    #[doc = "Set the field `items`.\nSchema of the elements of Type.ARRAY."]
    pub fn set_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.items = Some(v.into());
        self
    }
    #[doc = "Set the field `max_items`.\nMaximum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn set_max_items(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_items = Some(v.into());
        self
    }
    #[doc = "Set the field `maximum`.\nMaximum value for Type.INTEGER and Type.NUMBER."]
    pub fn set_maximum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum = Some(v.into());
        self
    }
    #[doc = "Set the field `min_items`.\nMinimum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn set_min_items(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_items = Some(v.into());
        self
    }
    #[doc = "Set the field `minimum`.\nMinimum value for Type.INTEGER and Type.NUMBER."]
    pub fn set_minimum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum = Some(v.into());
        self
    }
    #[doc = "Set the field `nullable`.\nIndicates if the value may be null."]
    pub fn set_nullable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.nullable = Some(v.into());
        self
    }
    #[doc = "Set the field `prefix_items`.\nSchemas of initial elements of Type.ARRAY."]
    pub fn set_prefix_items(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix_items = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nProperties of Type.OBJECT."]
    pub fn set_properties(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `ref_`.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn set_ref(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ref_ = Some(v.into());
        self
    }
    #[doc = "Set the field `required`.\nRequired properties of Type.OBJECT."]
    pub fn set_required(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.required = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nThe title of the schema."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `unique_items`.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn set_unique_items(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.unique_items = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolClientFunctionElResponseEl {
    type O = BlockAssignable<CesToolClientFunctionElResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolClientFunctionElResponseEl {
    #[doc = "The type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub type_: PrimField<String>,
}
impl BuildCesToolClientFunctionElResponseEl {
    pub fn build(self) -> CesToolClientFunctionElResponseEl {
        CesToolClientFunctionElResponseEl {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            max_items: core::default::Default::default(),
            maximum: core::default::Default::default(),
            min_items: core::default::Default::default(),
            minimum: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: self.type_,
            unique_items: core::default::Default::default(),
        }
    }
}
pub struct CesToolClientFunctionElResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolClientFunctionElResponseElRef {
    fn new(shared: StackShared, base: String) -> CesToolClientFunctionElResponseElRef {
        CesToolClientFunctionElResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolClientFunctionElResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `additional_properties` after provisioning.\nDefines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn additional_properties(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.additional_properties", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `any_of` after provisioning.\nThe instance value should be valid against at least one of the schemas in this list."]
    pub fn any_of(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
    #[doc = "Get a reference to the value of field `default` after provisioning.\nDefault value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be compatible\nwith the defined 'type' and other schema constraints."]
    pub fn default(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
    #[doc = "Get a reference to the value of field `defs` after provisioning.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn defs(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the data."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `enum_` after provisioning.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn enum_(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\nSchema of the elements of Type.ARRAY."]
    pub fn items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }
    #[doc = "Get a reference to the value of field `max_items` after provisioning.\nMaximum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn max_items(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_items", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum` after provisioning.\nMaximum value for Type.INTEGER and Type.NUMBER."]
    pub fn maximum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum", self.base))
    }
    #[doc = "Get a reference to the value of field `min_items` after provisioning.\nMinimum number of the elements for Type.ARRAY. (int64 format)"]
    pub fn min_items(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_items", self.base))
    }
    #[doc = "Get a reference to the value of field `minimum` after provisioning.\nMinimum value for Type.INTEGER and Type.NUMBER."]
    pub fn minimum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum", self.base))
    }
    #[doc = "Get a reference to the value of field `nullable` after provisioning.\nIndicates if the value may be null."]
    pub fn nullable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }
    #[doc = "Get a reference to the value of field `prefix_items` after provisioning.\nSchemas of initial elements of Type.ARRAY."]
    pub fn prefix_items(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nProperties of Type.OBJECT."]
    pub fn properties(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `ref_` after provisioning.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn ref_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }
    #[doc = "Get a reference to the value of field `required` after provisioning.\nRequired properties of Type.OBJECT."]
    pub fn required(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the schema."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `unique_items` after provisioning.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn unique_items(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesToolClientFunctionElDynamic {
    parameters: Option<DynamicBlock<CesToolClientFunctionElParametersEl>>,
    response: Option<DynamicBlock<CesToolClientFunctionElResponseEl>>,
}
#[derive(Serialize)]
pub struct CesToolClientFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<CesToolClientFunctionElParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response: Option<Vec<CesToolClientFunctionElResponseEl>>,
    dynamic: CesToolClientFunctionElDynamic,
}
impl CesToolClientFunctionEl {
    #[doc = "Set the field `description`.\nThe function description."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl Into<BlockAssignable<CesToolClientFunctionElParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v: impl Into<BlockAssignable<CesToolClientFunctionElResponseEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.response = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.response = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolClientFunctionEl {
    type O = BlockAssignable<CesToolClientFunctionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolClientFunctionEl {
    #[doc = "The function name."]
    pub name: PrimField<String>,
}
impl BuildCesToolClientFunctionEl {
    pub fn build(self) -> CesToolClientFunctionEl {
        CesToolClientFunctionEl {
            description: core::default::Default::default(),
            name: self.name,
            parameters: core::default::Default::default(),
            response: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolClientFunctionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolClientFunctionElRef {
    fn new(shared: StackShared, base: String) -> CesToolClientFunctionElRef {
        CesToolClientFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolClientFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe function description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe function name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<CesToolClientFunctionElParametersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> ListRef<CesToolClientFunctionElResponseElRef> {
        ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    attribute_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost_amount: Option<PrimField<f64>>,
}
impl
    CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl
{
    #[doc = "Set the field `attribute_value`.\nCan be one of:\n1. The numerical field value.\n2. The duration spec for freshness:\nThe value must be formatted as an XSD 'dayTimeDuration' value (a\nrestricted subset of an ISO 8601 duration value). The pattern for\nthis is: 'nDnM]'."]
    pub fn set_attribute_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_value = Some(v.into());
        self
    }
    #[doc = "Set the field `boost_amount`.\nThe value between -1 to 1 by which to boost the score if the\nattribute_value evaluates to the value specified above."]
    pub fn set_boost_amount(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boost_amount = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl { type O = BlockAssignable < CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildCesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl
{}
impl BuildCesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl { pub fn build (self) -> CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl { CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl { attribute_value : core :: default :: Default :: default () , boost_amount : core :: default :: Default :: default () , } } }
pub struct CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef { fn new (shared : StackShared , base : String) -> CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef { CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef { shared : shared , base : base . to_string () , } } }
impl CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `attribute_value` after provisioning.\nCan be one of:\n1. The numerical field value.\n2. The duration spec for freshness:\nThe value must be formatted as an XSD 'dayTimeDuration' value (a\nrestricted subset of an ISO 8601 duration value). The pattern for\nthis is: 'nDnM]'."] pub fn attribute_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.attribute_value" , self . base)) } # [doc = "Get a reference to the value of field `boost_amount` after provisioning.\nThe value between -1 to 1 by which to boost the score if the\nattribute_value evaluates to the value specified above."] pub fn boost_amount (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.boost_amount" , self . base)) } }
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElDynamic { control_points : Option < DynamicBlock < CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl >> , }
#[derive(Serialize)]
pub struct CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl { # [serde (skip_serializing_if = "Option::is_none")] attribute_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] field_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] interpolation_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] control_points : Option < Vec < CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl > > , dynamic : CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElDynamic , }
impl CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl {
    #[doc = "Set the field `attribute_type`.\nThe attribute type to be used to determine the boost amount. The\nattribute value can be derived from the field value of the specified\nfield_name. In the case of numerical it is straightforward i.e.\nattribute_value = numerical_field_value. In the case of freshness\nhowever, attribute_value = (time.now() - datetime_field_value).\nPossible values:\nNUMERICAL\nFRESHNESS"]
    pub fn set_attribute_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.attribute_type = Some(v.into());
        self
    }
    #[doc = "Set the field `field_name`.\nThe name of the field whose value will be used to determine the\nboost amount."]
    pub fn set_field_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.field_name = Some(v.into());
        self
    }
    #[doc = "Set the field `interpolation_type`.\nThe interpolation type to be applied to connect the control points\nlisted below.\nPossible values:\nLINEAR"]
    pub fn set_interpolation_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interpolation_type = Some(v.into());
        self
    }
    #[doc = "Set the field `control_points`.\n"]
    pub fn set_control_points(
        mut self,
        v : impl Into < BlockAssignable < CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.control_points = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.control_points = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl
{
    type O = BlockAssignable<
        CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl {}
impl BuildCesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl {
    pub fn build(
        self,
    ) -> CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl {
        CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl {
            attribute_type: core::default::Default::default(),
            field_name: core::default::Default::default(),
            interpolation_type: core::default::Default::default(),
            control_points: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef {
        CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `attribute_type` after provisioning.\nThe attribute type to be used to determine the boost amount. The\nattribute value can be derived from the field value of the specified\nfield_name. In the case of numerical it is straightforward i.e.\nattribute_value = numerical_field_value. In the case of freshness\nhowever, attribute_value = (time.now() - datetime_field_value).\nPossible values:\nNUMERICAL\nFRESHNESS"]
    pub fn attribute_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.attribute_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `field_name` after provisioning.\nThe name of the field whose value will be used to determine the\nboost amount."]
    pub fn field_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }
    #[doc = "Get a reference to the value of field `interpolation_type` after provisioning.\nThe interpolation type to be applied to connect the control points\nlisted below.\nPossible values:\nLINEAR"]
    pub fn interpolation_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interpolation_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `control_points` after provisioning.\n"]    pub fn control_points (& self) -> ListRef < CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElControlPointsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.control_points", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElDynamic {
    boost_control_spec: Option<
        DynamicBlock<
            CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    boost: Option<PrimField<f64>>,
    condition: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost_control_spec: Option<
        Vec<CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl>,
    >,
    dynamic: CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElDynamic,
}
impl CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
    #[doc = "Set the field `boost`.\nStrength of the boost, which should be in [-1, 1]. Negative boost means\ndemotion. Default is 0.0.\nSetting to 1.0 gives the suggestions a big promotion. However, it does\nnot necessarily mean that the top result will be a boosted suggestion.\nSetting to -1.0 gives the suggestions a big demotion. However, other\nsuggestions that are relevant might still be shown.\nSetting to 0.0 means no boost applied. The boosting condition is\nignored."]
    pub fn set_boost(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.boost = Some(v.into());
        self
    }
    #[doc = "Set the field `boost_control_spec`.\n"]
    pub fn set_boost_control_spec(
        mut self,
        v: impl Into<
            BlockAssignable<
                CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.boost_control_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.boost_control_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
    type O = BlockAssignable<CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
    #[doc = "An expression which specifies a boost condition. The syntax is the same\nas filter expression syntax. Currently, the only supported condition is\na list of BCP-47 lang codes.\nExample: To boost suggestions in languages en or fr:\n(lang_code: ANY(\"en\", \"fr\"))"]
    pub condition: PrimField<String>,
}
impl BuildCesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
    pub fn build(self) -> CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
        CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl {
            boost: core::default::Default::default(),
            condition: self.condition,
            boost_control_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef {
        CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `boost` after provisioning.\nStrength of the boost, which should be in [-1, 1]. Negative boost means\ndemotion. Default is 0.0.\nSetting to 1.0 gives the suggestions a big promotion. However, it does\nnot necessarily mean that the top result will be a boosted suggestion.\nSetting to -1.0 gives the suggestions a big demotion. However, other\nsuggestions that are relevant might still be shown.\nSetting to 0.0 means no boost applied. The boosting condition is\nignored."]
    pub fn boost(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.boost", self.base))
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\nAn expression which specifies a boost condition. The syntax is the same\nas filter expression syntax. Currently, the only supported condition is\na list of BCP-47 lang codes.\nExample: To boost suggestions in languages en or fr:\n(lang_code: ANY(\"en\", \"fr\"))"]
    pub fn condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `boost_control_spec` after provisioning.\n"]
    pub fn boost_control_spec(
        &self,
    ) -> ListRef<CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElBoostControlSpecElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.boost_control_spec", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElBoostSpecsElSpecElDynamic {
    condition_boost_specs:
        Option<DynamicBlock<CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl>>,
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElBoostSpecsElSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_boost_specs:
        Option<Vec<CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl>>,
    dynamic: CesToolDataStoreToolElBoostSpecsElSpecElDynamic,
}
impl CesToolDataStoreToolElBoostSpecsElSpecEl {
    #[doc = "Set the field `condition_boost_specs`.\n"]
    pub fn set_condition_boost_specs(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition_boost_specs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_boost_specs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElBoostSpecsElSpecEl {
    type O = BlockAssignable<CesToolDataStoreToolElBoostSpecsElSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElBoostSpecsElSpecEl {}
impl BuildCesToolDataStoreToolElBoostSpecsElSpecEl {
    pub fn build(self) -> CesToolDataStoreToolElBoostSpecsElSpecEl {
        CesToolDataStoreToolElBoostSpecsElSpecEl {
            condition_boost_specs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElBoostSpecsElSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElBoostSpecsElSpecElRef {
    fn new(shared: StackShared, base: String) -> CesToolDataStoreToolElBoostSpecsElSpecElRef {
        CesToolDataStoreToolElBoostSpecsElSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElBoostSpecsElSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition_boost_specs` after provisioning.\n"]
    pub fn condition_boost_specs(
        &self,
    ) -> ListRef<CesToolDataStoreToolElBoostSpecsElSpecElConditionBoostSpecsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_boost_specs", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElBoostSpecsElDynamic {
    spec: Option<DynamicBlock<CesToolDataStoreToolElBoostSpecsElSpecEl>>,
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElBoostSpecsEl {
    data_stores: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<CesToolDataStoreToolElBoostSpecsElSpecEl>>,
    dynamic: CesToolDataStoreToolElBoostSpecsElDynamic,
}
impl CesToolDataStoreToolElBoostSpecsEl {
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElBoostSpecsElSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElBoostSpecsEl {
    type O = BlockAssignable<CesToolDataStoreToolElBoostSpecsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElBoostSpecsEl {
    #[doc = "The Data Store where the boosting configuration is applied. Full resource\nname of DataStore, such as\nprojects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}."]
    pub data_stores: ListField<PrimField<String>>,
}
impl BuildCesToolDataStoreToolElBoostSpecsEl {
    pub fn build(self) -> CesToolDataStoreToolElBoostSpecsEl {
        CesToolDataStoreToolElBoostSpecsEl {
            data_stores: self.data_stores,
            spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElBoostSpecsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElBoostSpecsElRef {
    fn new(shared: StackShared, base: String) -> CesToolDataStoreToolElBoostSpecsElRef {
        CesToolDataStoreToolElBoostSpecsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElBoostSpecsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_stores` after provisioning.\nThe Data Store where the boosting configuration is applied. Full resource\nname of DataStore, such as\nprojects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}."]
    pub fn data_stores(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.data_stores", self.base))
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<CesToolDataStoreToolElBoostSpecsElSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    collection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source: Option<PrimField<String>>,
}
impl CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl {
    #[doc = "Set the field `collection`.\n"]
    pub fn set_collection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collection = Some(v.into());
        self
    }
    #[doc = "Set the field `collection_display_name`.\n"]
    pub fn set_collection_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.collection_display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_source = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl
{
    type O = BlockAssignable<
        CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl
{}
impl BuildCesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl {
    pub fn build(
        self,
    ) -> CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl {
        CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigEl {
            collection: core::default::Default::default(),
            collection_display_name: core::default::Default::default(),
            data_source: core::default::Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef {
        CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `collection` after provisioning.\n"]
    pub fn collection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }
    #[doc = "Get a reference to the value of field `collection_display_name` after provisioning.\n"]
    pub fn collection_display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_display_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl {
    name: PrimField<String>,
}
impl CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl {}
impl ToListMappable for CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl {
    type O = BlockAssignable<CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl {
    #[doc = "Full resource name of the DataStore.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}'"]
    pub name: PrimField<String>,
}
impl BuildCesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl {
    pub fn build(self) -> CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl {
        CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl { name: self.name }
    }
}
pub struct CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef {
        CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connector_config` after provisioning.\nThe connector config for the data store connection."]
    pub fn connector_config(
        &self,
    ) -> ListRef<
        CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElConnectorConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connector_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the data store was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the data store."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `document_processing_mode` after provisioning.\nThe document processing mode for the data store connection.\nOnly set for PUBLIC_WEB and UNSTRUCTURED data stores.\nPossible values:\nDOCUMENTS\nCHUNKS"]
    pub fn document_processing_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_processing_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull resource name of the DataStore.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/dataStores/{dataStore}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe type of the data store. This field is readonly and populated by the\nserver.\nPossible values:\nPUBLIC_WEB\nUNSTRUCTURED\nFAQ\nCONNECTOR"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDynamic {
    data_store:
        Option<DynamicBlock<CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl>>,
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElEngineSourceElDataStoreSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store: Option<Vec<CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl>>,
    dynamic: CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDynamic,
}
impl CesToolDataStoreToolElEngineSourceElDataStoreSourcesEl {
    #[doc = "Set the field `filter`.\nFilter specification for the DataStore.\nSee:\nhttps://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store`.\n"]
    pub fn set_data_store(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_store = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElEngineSourceElDataStoreSourcesEl {
    type O = BlockAssignable<CesToolDataStoreToolElEngineSourceElDataStoreSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElEngineSourceElDataStoreSourcesEl {}
impl BuildCesToolDataStoreToolElEngineSourceElDataStoreSourcesEl {
    pub fn build(self) -> CesToolDataStoreToolElEngineSourceElDataStoreSourcesEl {
        CesToolDataStoreToolElEngineSourceElDataStoreSourcesEl {
            filter: core::default::Default::default(),
            data_store: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElEngineSourceElDataStoreSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElEngineSourceElDataStoreSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElEngineSourceElDataStoreSourcesElRef {
        CesToolDataStoreToolElEngineSourceElDataStoreSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElEngineSourceElDataStoreSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nFilter specification for the DataStore.\nSee:\nhttps://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `data_store` after provisioning.\n"]
    pub fn data_store(
        &self,
    ) -> ListRef<CesToolDataStoreToolElEngineSourceElDataStoreSourcesElDataStoreElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_store", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElEngineSourceElDynamic {
    data_store_sources:
        Option<DynamicBlock<CesToolDataStoreToolElEngineSourceElDataStoreSourcesEl>>,
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElEngineSourceEl {
    engine: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_sources: Option<Vec<CesToolDataStoreToolElEngineSourceElDataStoreSourcesEl>>,
    dynamic: CesToolDataStoreToolElEngineSourceElDynamic,
}
impl CesToolDataStoreToolElEngineSourceEl {
    #[doc = "Set the field `filter`.\nA filter applied to the search across the Engine. Not relevant and not\nused if 'data_store_sources' is provided.\nSee:\nhttps://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn set_filter(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter = Some(v.into());
        self
    }
    #[doc = "Set the field `data_store_sources`.\n"]
    pub fn set_data_store_sources(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElEngineSourceElDataStoreSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_store_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElEngineSourceEl {
    type O = BlockAssignable<CesToolDataStoreToolElEngineSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElEngineSourceEl {
    #[doc = "Full resource name of the Engine.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine}'"]
    pub engine: PrimField<String>,
}
impl BuildCesToolDataStoreToolElEngineSourceEl {
    pub fn build(self) -> CesToolDataStoreToolElEngineSourceEl {
        CesToolDataStoreToolElEngineSourceEl {
            engine: self.engine,
            filter: core::default::Default::default(),
            data_store_sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElEngineSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElEngineSourceElRef {
    fn new(shared: StackShared, base: String) -> CesToolDataStoreToolElEngineSourceElRef {
        CesToolDataStoreToolElEngineSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElEngineSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `engine` after provisioning.\nFull resource name of the Engine.\nFormat:\n'projects/{project}/locations/{location}/collections/{collection}/engines/{engine}'"]
    pub fn engine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nA filter applied to the search across the Engine. Not relevant and not\nused if 'data_store_sources' is provided.\nSee:\nhttps://cloud.google.com/generative-ai-app-builder/docs/filter-search-metadata"]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
    #[doc = "Get a reference to the value of field `data_store_sources` after provisioning.\n"]
    pub fn data_store_sources(
        &self,
    ) -> ListRef<CesToolDataStoreToolElEngineSourceElDataStoreSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_sources", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElModalityConfigsElGroundingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grounding_level: Option<PrimField<f64>>,
}
impl CesToolDataStoreToolElModalityConfigsElGroundingConfigEl {
    #[doc = "Set the field `disabled`.\nWhether grounding is disabled."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `grounding_level`.\nThe groundedness threshold of the answer based on the retrieved sources.\nThe value has a configurable range of [1, 5]. The level is used to\nthreshold the groundedness of the answer, meaning that all responses with\na groundedness score below the threshold will fall back to returning\nrelevant snippets only.\nFor example, a level of 3 means that the groundedness score must be\n3 or higher for the response to be returned."]
    pub fn set_grounding_level(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.grounding_level = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElModalityConfigsElGroundingConfigEl {
    type O = BlockAssignable<CesToolDataStoreToolElModalityConfigsElGroundingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElModalityConfigsElGroundingConfigEl {}
impl BuildCesToolDataStoreToolElModalityConfigsElGroundingConfigEl {
    pub fn build(self) -> CesToolDataStoreToolElModalityConfigsElGroundingConfigEl {
        CesToolDataStoreToolElModalityConfigsElGroundingConfigEl {
            disabled: core::default::Default::default(),
            grounding_level: core::default::Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElModalityConfigsElGroundingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElModalityConfigsElGroundingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElModalityConfigsElGroundingConfigElRef {
        CesToolDataStoreToolElModalityConfigsElGroundingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElModalityConfigsElGroundingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether grounding is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `grounding_level` after provisioning.\nThe groundedness threshold of the answer based on the retrieved sources.\nThe value has a configurable range of [1, 5]. The level is used to\nthreshold the groundedness of the answer, meaning that all responses with\na groundedness score below the threshold will fall back to returning\nrelevant snippets only.\nFor example, a level of 3 means that the groundedness score must be\n3 or higher for the response to be returned."]
    pub fn grounding_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.grounding_level", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl {
    #[doc = "Set the field `model`.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl {
    type O =
        BlockAssignable<CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl {}
impl BuildCesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl {
    pub fn build(self) -> CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl {
        CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef {
        CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElModalityConfigsElRewriterConfigElDynamic {
    model_settings: Option<
        DynamicBlock<CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl>,
    >,
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElModalityConfigsElRewriterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings:
        Option<Vec<CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl>>,
    dynamic: CesToolDataStoreToolElModalityConfigsElRewriterConfigElDynamic,
}
impl CesToolDataStoreToolElModalityConfigsElRewriterConfigEl {
    #[doc = "Set the field `disabled`.\nWhether the rewriter is disabled."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt`.\nThe prompt definition. If not set, default prompt will be used."]
    pub fn set_prompt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl Into<
            BlockAssignable<CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElModalityConfigsElRewriterConfigEl {
    type O = BlockAssignable<CesToolDataStoreToolElModalityConfigsElRewriterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElModalityConfigsElRewriterConfigEl {}
impl BuildCesToolDataStoreToolElModalityConfigsElRewriterConfigEl {
    pub fn build(self) -> CesToolDataStoreToolElModalityConfigsElRewriterConfigEl {
        CesToolDataStoreToolElModalityConfigsElRewriterConfigEl {
            disabled: core::default::Default::default(),
            prompt: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElModalityConfigsElRewriterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElModalityConfigsElRewriterConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElModalityConfigsElRewriterConfigElRef {
        CesToolDataStoreToolElModalityConfigsElRewriterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElModalityConfigsElRewriterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether the rewriter is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\nThe prompt definition. If not set, default prompt will be used."]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(
        &self,
    ) -> ListRef<CesToolDataStoreToolElModalityConfigsElRewriterConfigElModelSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<PrimField<f64>>,
}
impl CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl {
    #[doc = "Set the field `model`.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn set_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.model = Some(v.into());
        self
    }
    #[doc = "Set the field `temperature`.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn set_temperature(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.temperature = Some(v.into());
        self
    }
}
impl ToListMappable
    for CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl
{
    type O = BlockAssignable<
        CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl {}
impl BuildCesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl {
    pub fn build(
        self,
    ) -> CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl {
        CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef {
        CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `model` after provisioning.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }
    #[doc = "Get a reference to the value of field `temperature` after provisioning.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn temperature(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElModalityConfigsElSummarizationConfigElDynamic {
    model_settings: Option<
        DynamicBlock<CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl>,
    >,
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElModalityConfigsElSummarizationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model_settings:
        Option<Vec<CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl>>,
    dynamic: CesToolDataStoreToolElModalityConfigsElSummarizationConfigElDynamic,
}
impl CesToolDataStoreToolElModalityConfigsElSummarizationConfigEl {
    #[doc = "Set the field `disabled`.\nWhether summarization is disabled."]
    pub fn set_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disabled = Some(v.into());
        self
    }
    #[doc = "Set the field `prompt`.\nThe prompt definition. If not set, default prompt will be used."]
    pub fn set_prompt(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prompt = Some(v.into());
        self
    }
    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.model_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.model_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElModalityConfigsElSummarizationConfigEl {
    type O = BlockAssignable<CesToolDataStoreToolElModalityConfigsElSummarizationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElModalityConfigsElSummarizationConfigEl {}
impl BuildCesToolDataStoreToolElModalityConfigsElSummarizationConfigEl {
    pub fn build(self) -> CesToolDataStoreToolElModalityConfigsElSummarizationConfigEl {
        CesToolDataStoreToolElModalityConfigsElSummarizationConfigEl {
            disabled: core::default::Default::default(),
            prompt: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElModalityConfigsElSummarizationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElModalityConfigsElSummarizationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolDataStoreToolElModalityConfigsElSummarizationConfigElRef {
        CesToolDataStoreToolElModalityConfigsElSummarizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElModalityConfigsElSummarizationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disabled` after provisioning.\nWhether summarization is disabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
    #[doc = "Get a reference to the value of field `prompt` after provisioning.\nThe prompt definition. If not set, default prompt will be used."]
    pub fn prompt(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(
        &self,
    ) -> ListRef<CesToolDataStoreToolElModalityConfigsElSummarizationConfigElModelSettingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.model_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElModalityConfigsElDynamic {
    grounding_config:
        Option<DynamicBlock<CesToolDataStoreToolElModalityConfigsElGroundingConfigEl>>,
    rewriter_config: Option<DynamicBlock<CesToolDataStoreToolElModalityConfigsElRewriterConfigEl>>,
    summarization_config:
        Option<DynamicBlock<CesToolDataStoreToolElModalityConfigsElSummarizationConfigEl>>,
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolElModalityConfigsEl {
    modality_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grounding_config: Option<Vec<CesToolDataStoreToolElModalityConfigsElGroundingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rewriter_config: Option<Vec<CesToolDataStoreToolElModalityConfigsElRewriterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summarization_config: Option<Vec<CesToolDataStoreToolElModalityConfigsElSummarizationConfigEl>>,
    dynamic: CesToolDataStoreToolElModalityConfigsElDynamic,
}
impl CesToolDataStoreToolElModalityConfigsEl {
    #[doc = "Set the field `grounding_config`.\n"]
    pub fn set_grounding_config(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElModalityConfigsElGroundingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grounding_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grounding_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rewriter_config`.\n"]
    pub fn set_rewriter_config(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElModalityConfigsElRewriterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rewriter_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rewriter_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `summarization_config`.\n"]
    pub fn set_summarization_config(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElModalityConfigsElSummarizationConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.summarization_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.summarization_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolDataStoreToolElModalityConfigsEl {
    type O = BlockAssignable<CesToolDataStoreToolElModalityConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolElModalityConfigsEl {
    #[doc = "The modality type.\nPossible values:\nTEXT\nAUDIO"]
    pub modality_type: PrimField<String>,
}
impl BuildCesToolDataStoreToolElModalityConfigsEl {
    pub fn build(self) -> CesToolDataStoreToolElModalityConfigsEl {
        CesToolDataStoreToolElModalityConfigsEl {
            modality_type: self.modality_type,
            grounding_config: core::default::Default::default(),
            rewriter_config: core::default::Default::default(),
            summarization_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElModalityConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElModalityConfigsElRef {
    fn new(shared: StackShared, base: String) -> CesToolDataStoreToolElModalityConfigsElRef {
        CesToolDataStoreToolElModalityConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElModalityConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `modality_type` after provisioning.\nThe modality type.\nPossible values:\nTEXT\nAUDIO"]
    pub fn modality_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.modality_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `grounding_config` after provisioning.\n"]
    pub fn grounding_config(
        &self,
    ) -> ListRef<CesToolDataStoreToolElModalityConfigsElGroundingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grounding_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rewriter_config` after provisioning.\n"]
    pub fn rewriter_config(
        &self,
    ) -> ListRef<CesToolDataStoreToolElModalityConfigsElRewriterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rewriter_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `summarization_config` after provisioning.\n"]
    pub fn summarization_config(
        &self,
    ) -> ListRef<CesToolDataStoreToolElModalityConfigsElSummarizationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.summarization_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CesToolDataStoreToolElDynamic {
    boost_specs: Option<DynamicBlock<CesToolDataStoreToolElBoostSpecsEl>>,
    engine_source: Option<DynamicBlock<CesToolDataStoreToolElEngineSourceEl>>,
    modality_configs: Option<DynamicBlock<CesToolDataStoreToolElModalityConfigsEl>>,
}
#[derive(Serialize)]
pub struct CesToolDataStoreToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boost_specs: Option<Vec<CesToolDataStoreToolElBoostSpecsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_source: Option<Vec<CesToolDataStoreToolElEngineSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modality_configs: Option<Vec<CesToolDataStoreToolElModalityConfigsEl>>,
    dynamic: CesToolDataStoreToolElDynamic,
}
impl CesToolDataStoreToolEl {
    #[doc = "Set the field `description`.\nThe tool description."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `max_results`.\nNumber of search results to return per query.\nThe default value is 10. The maximum allowed value is 10."]
    pub fn set_max_results(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_results = Some(v.into());
        self
    }
    #[doc = "Set the field `boost_specs`.\n"]
    pub fn set_boost_specs(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElBoostSpecsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.boost_specs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.boost_specs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `engine_source`.\n"]
    pub fn set_engine_source(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElEngineSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.engine_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.engine_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `modality_configs`.\n"]
    pub fn set_modality_configs(
        mut self,
        v: impl Into<BlockAssignable<CesToolDataStoreToolElModalityConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.modality_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.modality_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolDataStoreToolEl {
    type O = BlockAssignable<CesToolDataStoreToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolDataStoreToolEl {
    #[doc = "The data store tool name."]
    pub name: PrimField<String>,
}
impl BuildCesToolDataStoreToolEl {
    pub fn build(self) -> CesToolDataStoreToolEl {
        CesToolDataStoreToolEl {
            description: core::default::Default::default(),
            max_results: core::default::Default::default(),
            name: self.name,
            boost_specs: core::default::Default::default(),
            engine_source: core::default::Default::default(),
            modality_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolDataStoreToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolDataStoreToolElRef {
    fn new(shared: StackShared, base: String) -> CesToolDataStoreToolElRef {
        CesToolDataStoreToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolDataStoreToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe tool description."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `max_results` after provisioning.\nNumber of search results to return per query.\nThe default value is 10. The maximum allowed value is 10."]
    pub fn max_results(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_results", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe data store tool name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `boost_specs` after provisioning.\n"]
    pub fn boost_specs(&self) -> ListRef<CesToolDataStoreToolElBoostSpecsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boost_specs", self.base))
    }
    #[doc = "Get a reference to the value of field `engine_source` after provisioning.\n"]
    pub fn engine_source(&self) -> ListRef<CesToolDataStoreToolElEngineSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.engine_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `modality_configs` after provisioning.\n"]
    pub fn modality_configs(&self) -> ListRef<CesToolDataStoreToolElModalityConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.modality_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolGoogleSearchToolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    context_urls: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_domains: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_domains: Option<ListField<PrimField<String>>>,
}
impl CesToolGoogleSearchToolEl {
    #[doc = "Set the field `context_urls`.\nContent will be fetched directly from these URLs for context and grounding.\nMore details: https://cloud.google.com/vertex-ai/generative-ai/docs/url-context.\nExample: \"https://example.com/path.html\". A maximum of 20 URLs are allowed."]
    pub fn set_context_urls(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.context_urls = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the tool's purpose."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_domains`.\nList of domains to be excluded from the search results.\nExample: \"example.com\".\nA maximum of 2000 domains can be excluded."]
    pub fn set_exclude_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_domains = Some(v.into());
        self
    }
    #[doc = "Set the field `preferred_domains`.\nSpecifies domain names to guide the search.\nThe model will be instructed to prioritize these domains\nwhen formulating queries for google search.\nThis is a best-effort hint and these domains may or may\nnot be exclusively reflected in the final search results.\nExample: \"example.com\", \"another.site\".\nA maximum of 20 domains can be specified."]
    pub fn set_preferred_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.preferred_domains = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolGoogleSearchToolEl {
    type O = BlockAssignable<CesToolGoogleSearchToolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolGoogleSearchToolEl {
    #[doc = "The name of the tool."]
    pub name: PrimField<String>,
}
impl BuildCesToolGoogleSearchToolEl {
    pub fn build(self) -> CesToolGoogleSearchToolEl {
        CesToolGoogleSearchToolEl {
            context_urls: core::default::Default::default(),
            description: core::default::Default::default(),
            exclude_domains: core::default::Default::default(),
            name: self.name,
            preferred_domains: core::default::Default::default(),
        }
    }
}
pub struct CesToolGoogleSearchToolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolGoogleSearchToolElRef {
    fn new(shared: StackShared, base: String) -> CesToolGoogleSearchToolElRef {
        CesToolGoogleSearchToolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolGoogleSearchToolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `context_urls` after provisioning.\nContent will be fetched directly from these URLs for context and grounding.\nMore details: https://cloud.google.com/vertex-ai/generative-ai/docs/url-context.\nExample: \"https://example.com/path.html\". A maximum of 20 URLs are allowed."]
    pub fn context_urls(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.context_urls", self.base))
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the tool's purpose."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `exclude_domains` after provisioning.\nList of domains to be excluded from the search results.\nExample: \"example.com\".\nA maximum of 2000 domains can be excluded."]
    pub fn exclude_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_domains", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the tool."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `preferred_domains` after provisioning.\nSpecifies domain names to guide the search.\nThe model will be instructed to prioritize these domains\nwhen formulating queries for google search.\nThis is a best-effort hint and these domains may or may\nnot be exclusively reflected in the final search results.\nExample: \"example.com\", \"another.site\".\nA maximum of 20 domains can be specified."]
    pub fn preferred_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_domains", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolPythonFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    python_code: Option<PrimField<String>>,
}
impl CesToolPythonFunctionEl {
    #[doc = "Set the field `name`.\nThe name of the Python function to execute. Must match a Python function\nname defined in the python code. Case sensitive. If the name is not\nprovided, the first function defined in the python code will be used."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `python_code`.\nThe Python code to execute for the tool."]
    pub fn set_python_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.python_code = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolPythonFunctionEl {
    type O = BlockAssignable<CesToolPythonFunctionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolPythonFunctionEl {}
impl BuildCesToolPythonFunctionEl {
    pub fn build(self) -> CesToolPythonFunctionEl {
        CesToolPythonFunctionEl {
            name: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}
pub struct CesToolPythonFunctionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolPythonFunctionElRef {
    fn new(shared: StackShared, base: String) -> CesToolPythonFunctionElRef {
        CesToolPythonFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolPythonFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Python function, parsed from the python code's\ndocstring."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Python function to execute. Must match a Python function\nname defined in the python code. Case sensitive. If the name is not\nprovided, the first function defined in the python code will be used."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `python_code` after provisioning.\nThe Python code to execute for the tool."]
    pub fn python_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CesToolTimeoutsEl {
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
impl ToListMappable for CesToolTimeoutsEl {
    type O = BlockAssignable<CesToolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolTimeoutsEl {}
impl BuildCesToolTimeoutsEl {
    pub fn build(self) -> CesToolTimeoutsEl {
        CesToolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CesToolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CesToolTimeoutsElRef {
        CesToolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolTimeoutsElRef {
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
struct CesToolDynamic {
    client_function: Option<DynamicBlock<CesToolClientFunctionEl>>,
    data_store_tool: Option<DynamicBlock<CesToolDataStoreToolEl>>,
    google_search_tool: Option<DynamicBlock<CesToolGoogleSearchToolEl>>,
    python_function: Option<DynamicBlock<CesToolPythonFunctionEl>>,
}
