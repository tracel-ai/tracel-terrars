use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CesToolsetData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execution_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    toolset_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mcp_toolset: Option<Vec<CesToolsetMcpToolsetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_api_toolset: Option<Vec<CesToolsetOpenApiToolsetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CesToolsetTimeoutsEl>,
    dynamic: CesToolsetDynamic,
}
struct CesToolset_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CesToolsetData>,
}
#[derive(Clone)]
pub struct CesToolset(Rc<CesToolset_>);
impl CesToolset {
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
    #[doc = "Set the field `description`.\nThe description of the toolset."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the toolset. Must be unique within the same app."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `mcp_toolset`.\n"]
    pub fn set_mcp_toolset(self, v: impl Into<BlockAssignable<CesToolsetMcpToolsetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mcp_toolset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mcp_toolset = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `open_api_toolset`.\n"]
    pub fn set_open_api_toolset(
        self,
        v: impl Into<BlockAssignable<CesToolsetOpenApiToolsetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().open_api_toolset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.open_api_toolset = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CesToolsetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the toolset was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the toolset."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the toolset. Must be unique within the same app."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nETag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the toolset.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `toolset_id` after provisioning.\nThe ID to use for the toolset, which will become the final component of\nthe toolset's resource name. If not provided, a unique ID will be\nautomatically assigned for the toolset."]
    pub fn toolset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.toolset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the toolset was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mcp_toolset` after provisioning.\n"]
    pub fn mcp_toolset(&self) -> ListRef<CesToolsetMcpToolsetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mcp_toolset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `open_api_toolset` after provisioning.\n"]
    pub fn open_api_toolset(&self) -> ListRef<CesToolsetOpenApiToolsetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_toolset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesToolsetTimeoutsElRef {
        CesToolsetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CesToolset {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CesToolset {}
impl ToListMappable for CesToolset {
    type O = ListRef<CesToolsetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CesToolset_ {
    fn extract_resource_type(&self) -> String {
        "google_ces_toolset".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCesToolset {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "The ID to use for the toolset, which will become the final component of\nthe toolset's resource name. If not provided, a unique ID will be\nautomatically assigned for the toolset."]
    pub toolset_id: PrimField<String>,
}
impl BuildCesToolset {
    pub fn build(self, stack: &mut Stack) -> CesToolset {
        let out = CesToolset(Rc::new(CesToolset_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CesToolsetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app: self.app,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                execution_type: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                toolset_id: self.toolset_id,
                mcp_toolset: core::default::Default::default(),
                open_api_toolset: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CesToolsetRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CesToolsetRef {
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
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the toolset was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the toolset."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the toolset. Must be unique within the same app."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nETag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the toolset.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `toolset_id` after provisioning.\nThe ID to use for the toolset, which will become the final component of\nthe toolset's resource name. If not provided, a unique ID will be\nautomatically assigned for the toolset."]
    pub fn toolset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.toolset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the toolset was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mcp_toolset` after provisioning.\n"]
    pub fn mcp_toolset(&self) -> ListRef<CesToolsetMcpToolsetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mcp_toolset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `open_api_toolset` after provisioning.\n"]
    pub fn open_api_toolset(&self) -> ListRef<CesToolsetOpenApiToolsetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.open_api_toolset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesToolsetTimeoutsElRef {
        CesToolsetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl {
    api_key_secret_version: PrimField<String>,
    key_name: PrimField<String>,
    request_location: PrimField<String>,
}
impl CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl {}
impl ToListMappable for CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl {
    type O = BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl {
    #[doc = "The name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub api_key_secret_version: PrimField<String>,
    #[doc = "The parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub key_name: PrimField<String>,
    #[doc = "Key location in the request. For API key auth on MCP toolsets,\nthe API key can only be sent in the request header.\nPossible values:\nHEADER"]
    pub request_location: PrimField<String>,
}
impl BuildCesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl {
    pub fn build(self) -> CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl {
        CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl {
            api_key_secret_version: self.api_key_secret_version,
            key_name: self.key_name,
            request_location: self.request_location,
        }
    }
}
pub struct CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigElRef {
        CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_secret_version` after provisioning.\nThe name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn api_key_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\nThe parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `request_location` after provisioning.\nKey location in the request. For API key auth on MCP toolsets,\nthe API key can only be sent in the request header.\nPossible values:\nHEADER"]
    pub fn request_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}
impl CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl {
    type O = BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl {}
impl BuildCesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl {
    pub fn build(self) -> CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl {
        CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl {
            token: core::default::Default::default(),
        }
    }
}
pub struct CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigElRef {
        CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl {
    client_id: PrimField<String>,
    client_secret_version: PrimField<String>,
    oauth_grant_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    token_endpoint: PrimField<String>,
}
impl CesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl {
    #[doc = "Set the field `scopes`.\nThe OAuth scopes to grant."]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl {
    type O = BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl {
    #[doc = "The client ID from the OAuth provider."]
    pub client_id: PrimField<String>,
    #[doc = "The name of the SecretManager secret version resource storing the\nclient secret.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\n\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub client_secret_version: PrimField<String>,
    #[doc = "OAuth grant types.\nPossible values:\nCLIENT_CREDENTIAL"]
    pub oauth_grant_type: PrimField<String>,
    #[doc = "The token endpoint in the OAuth provider to exchange for an access token."]
    pub token_endpoint: PrimField<String>,
}
impl BuildCesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl {
    pub fn build(self) -> CesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl {
        CesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl {
            client_id: self.client_id,
            client_secret_version: self.client_secret_version,
            oauth_grant_type: self.oauth_grant_type,
            scopes: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}
pub struct CesToolsetMcpToolsetElApiAuthenticationElOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElApiAuthenticationElOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetMcpToolsetElApiAuthenticationElOauthConfigElRef {
        CesToolsetMcpToolsetElApiAuthenticationElOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElApiAuthenticationElOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client ID from the OAuth provider."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret_version` after provisioning.\nThe name of the SecretManager secret version resource storing the\nclient secret.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\n\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn client_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_grant_type` after provisioning.\nOAuth grant types.\nPossible values:\nCLIENT_CREDENTIAL"]
    pub fn oauth_grant_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_grant_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
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
pub struct CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    service_account: PrimField<String>,
}
impl CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    #[doc = "Set the field `scopes`.\nThe OAuth scopes to grant. If not specified, the default scope\n'https://www.googleapis.com/auth/cloud-platform' is used."]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    type O = BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    #[doc = "The email address of the service account used for authenticatation. CES\nuses this service account to exchange an access token and the access token\nis then sent in the 'Authorization' header of the request.\n\nThe service account must have the\n'roles/iam.serviceAccountTokenCreator' role granted to the\nCES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub service_account: PrimField<String>,
}
impl BuildCesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    pub fn build(self) -> CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
        CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
            scopes: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}
pub struct CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
        CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant. If not specified, the default scope\n'https://www.googleapis.com/auth/cloud-platform' is used."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe email address of the service account used for authenticatation. CES\nuses this service account to exchange an access token and the access token\nis then sent in the 'Authorization' header of the request.\n\nThe service account must have the\n'roles/iam.serviceAccountTokenCreator' role granted to the\nCES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl ToListMappable for CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {
    type O =
        BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl BuildCesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {
    pub fn build(self) -> CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {
        CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
    }
}
pub struct CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
        CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct CesToolsetMcpToolsetElApiAuthenticationElDynamic {
    api_key_config: Option<DynamicBlock<CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl>>,
    bearer_token_config:
        Option<DynamicBlock<CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl>>,
    oauth_config: Option<DynamicBlock<CesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl>>,
    service_account_auth_config:
        Option<DynamicBlock<CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl>>,
    service_agent_id_token_auth_config: Option<
        DynamicBlock<CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct CesToolsetMcpToolsetElApiAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_config: Option<Vec<CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bearer_token_config: Option<Vec<CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_config: Option<Vec<CesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_auth_config:
        Option<Vec<CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_agent_id_token_auth_config:
        Option<Vec<CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl>>,
    dynamic: CesToolsetMcpToolsetElApiAuthenticationElDynamic,
}
impl CesToolsetMcpToolsetElApiAuthenticationEl {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl Into<BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigEl>>,
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
        v: impl Into<BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigEl>>,
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
        v: impl Into<BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationElOauthConfigEl>>,
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
    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v: impl Into<
            BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_account_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_auth_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_agent_id_token_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_agent_id_token_auth_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolsetMcpToolsetElApiAuthenticationEl {
    type O = BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetElApiAuthenticationEl {}
impl BuildCesToolsetMcpToolsetElApiAuthenticationEl {
    pub fn build(self) -> CesToolsetMcpToolsetElApiAuthenticationEl {
        CesToolsetMcpToolsetElApiAuthenticationEl {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolsetMcpToolsetElApiAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElApiAuthenticationElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetMcpToolsetElApiAuthenticationElRef {
        CesToolsetMcpToolsetElApiAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElApiAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> ListRef<CesToolsetMcpToolsetElApiAuthenticationElApiKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_key_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> ListRef<CesToolsetMcpToolsetElApiAuthenticationElBearerTokenConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bearer_token_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(
        &self,
    ) -> ListRef<CesToolsetMcpToolsetElApiAuthenticationElOauthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> ListRef<CesToolsetMcpToolsetElApiAuthenticationElServiceAccountAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_auth_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]
    pub fn service_agent_id_token_auth_config(
        &self,
    ) -> ListRef<CesToolsetMcpToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_agent_id_token_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolsetMcpToolsetElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl CesToolsetMcpToolsetElServiceDirectoryConfigEl {}
impl ToListMappable for CesToolsetMcpToolsetElServiceDirectoryConfigEl {
    type O = BlockAssignable<CesToolsetMcpToolsetElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetElServiceDirectoryConfigEl {
    #[doc = "The name of [Service\nDirectory](https://cloud.google.com/service-directory) service.\nFormat:\n'projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}'.\nLocation of the service directory must be the same as the location of the\napp."]
    pub service: PrimField<String>,
}
impl BuildCesToolsetMcpToolsetElServiceDirectoryConfigEl {
    pub fn build(self) -> CesToolsetMcpToolsetElServiceDirectoryConfigEl {
        CesToolsetMcpToolsetElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct CesToolsetMcpToolsetElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElServiceDirectoryConfigElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetMcpToolsetElServiceDirectoryConfigElRef {
        CesToolsetMcpToolsetElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of [Service\nDirectory](https://cloud.google.com/service-directory) service.\nFormat:\n'projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}'.\nLocation of the service directory must be the same as the location of the\napp."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolsetMcpToolsetElTlsConfigElCaCertsEl {
    cert: PrimField<String>,
    display_name: PrimField<String>,
}
impl CesToolsetMcpToolsetElTlsConfigElCaCertsEl {}
impl ToListMappable for CesToolsetMcpToolsetElTlsConfigElCaCertsEl {
    type O = BlockAssignable<CesToolsetMcpToolsetElTlsConfigElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetElTlsConfigElCaCertsEl {
    #[doc = "The allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, CES will use Google's default trust\nstore to verify certificates. N.B. Make sure the HTTPS server\ncertificates are signed with \"subject alt name\". For instance a\ncertificate can be self-signed using the following command,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub cert: PrimField<String>,
    #[doc = "The name of the allowed custom CA certificates. This\ncan be used to disambiguate the custom CA certificates."]
    pub display_name: PrimField<String>,
}
impl BuildCesToolsetMcpToolsetElTlsConfigElCaCertsEl {
    pub fn build(self) -> CesToolsetMcpToolsetElTlsConfigElCaCertsEl {
        CesToolsetMcpToolsetElTlsConfigElCaCertsEl {
            cert: self.cert,
            display_name: self.display_name,
        }
    }
}
pub struct CesToolsetMcpToolsetElTlsConfigElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElTlsConfigElCaCertsElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetMcpToolsetElTlsConfigElCaCertsElRef {
        CesToolsetMcpToolsetElTlsConfigElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElTlsConfigElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\nThe allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, CES will use Google's default trust\nstore to verify certificates. N.B. Make sure the HTTPS server\ncertificates are signed with \"subject alt name\". For instance a\ncertificate can be self-signed using the following command,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of the allowed custom CA certificates. This\ncan be used to disambiguate the custom CA certificates."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesToolsetMcpToolsetElTlsConfigElDynamic {
    ca_certs: Option<DynamicBlock<CesToolsetMcpToolsetElTlsConfigElCaCertsEl>>,
}
#[derive(Serialize)]
pub struct CesToolsetMcpToolsetElTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<Vec<CesToolsetMcpToolsetElTlsConfigElCaCertsEl>>,
    dynamic: CesToolsetMcpToolsetElTlsConfigElDynamic,
}
impl CesToolsetMcpToolsetElTlsConfigEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<BlockAssignable<CesToolsetMcpToolsetElTlsConfigElCaCertsEl>>,
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
impl ToListMappable for CesToolsetMcpToolsetElTlsConfigEl {
    type O = BlockAssignable<CesToolsetMcpToolsetElTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetElTlsConfigEl {}
impl BuildCesToolsetMcpToolsetElTlsConfigEl {
    pub fn build(self) -> CesToolsetMcpToolsetElTlsConfigEl {
        CesToolsetMcpToolsetElTlsConfigEl {
            ca_certs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolsetMcpToolsetElTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElTlsConfigElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetMcpToolsetElTlsConfigElRef {
        CesToolsetMcpToolsetElTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> ListRef<CesToolsetMcpToolsetElTlsConfigElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesToolsetMcpToolsetElDynamic {
    api_authentication: Option<DynamicBlock<CesToolsetMcpToolsetElApiAuthenticationEl>>,
    service_directory_config: Option<DynamicBlock<CesToolsetMcpToolsetElServiceDirectoryConfigEl>>,
    tls_config: Option<DynamicBlock<CesToolsetMcpToolsetElTlsConfigEl>>,
}
#[derive(Serialize)]
pub struct CesToolsetMcpToolsetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_headers: Option<RecField<PrimField<String>>>,
    server_address: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_authentication: Option<Vec<CesToolsetMcpToolsetElApiAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config: Option<Vec<CesToolsetMcpToolsetElServiceDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<Vec<CesToolsetMcpToolsetElTlsConfigEl>>,
    dynamic: CesToolsetMcpToolsetElDynamic,
}
impl CesToolsetMcpToolsetEl {
    #[doc = "Set the field `custom_headers`.\nThe custom headers to send in the request to the MCP server. The values\nmust be in the format '$context.variables.<name_of_variable>' and can be\nset in the session variables. See\nhttps://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/tool/open-api#openapi-injection\nfor more details."]
    pub fn set_custom_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.custom_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl Into<BlockAssignable<CesToolsetMcpToolsetElApiAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<BlockAssignable<CesToolsetMcpToolsetElServiceDirectoryConfigEl>>,
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
        v: impl Into<BlockAssignable<CesToolsetMcpToolsetElTlsConfigEl>>,
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
impl ToListMappable for CesToolsetMcpToolsetEl {
    type O = BlockAssignable<CesToolsetMcpToolsetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetMcpToolsetEl {
    #[doc = "The address of the MCP server, for example, \"https://example.com/mcp/\". If\nthe server is built with the MCP SDK, the url should be suffixed with\n\"/mcp/\". Only Streamable HTTP transport based servers are supported. See\nhttps://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http\nfor more details."]
    pub server_address: PrimField<String>,
}
impl BuildCesToolsetMcpToolsetEl {
    pub fn build(self) -> CesToolsetMcpToolsetEl {
        CesToolsetMcpToolsetEl {
            custom_headers: core::default::Default::default(),
            server_address: self.server_address,
            api_authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolsetMcpToolsetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetMcpToolsetElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetMcpToolsetElRef {
        CesToolsetMcpToolsetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetMcpToolsetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_headers` after provisioning.\nThe custom headers to send in the request to the MCP server. The values\nmust be in the format '$context.variables.<name_of_variable>' and can be\nset in the session variables. See\nhttps://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/tool/open-api#openapi-injection\nfor more details."]
    pub fn custom_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.custom_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `server_address` after provisioning.\nThe address of the MCP server, for example, \"https://example.com/mcp/\". If\nthe server is built with the MCP SDK, the url should be suffixed with\n\"/mcp/\". Only Streamable HTTP transport based servers are supported. See\nhttps://modelcontextprotocol.io/specification/2025-03-26/basic/transports#streamable-http\nfor more details."]
    pub fn server_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> ListRef<CesToolsetMcpToolsetElApiAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<CesToolsetMcpToolsetElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<CesToolsetMcpToolsetElTlsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
    api_key_secret_version: PrimField<String>,
    key_name: PrimField<String>,
    request_location: PrimField<String>,
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {}
impl ToListMappable for CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
    type O = BlockAssignable<CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
    #[doc = "The name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub api_key_secret_version: PrimField<String>,
    #[doc = "The parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub key_name: PrimField<String>,
    #[doc = "Key location in the request.\nPossible values:\nHEADER\nQUERY_STRING"]
    pub request_location: PrimField<String>,
}
impl BuildCesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
    pub fn build(self) -> CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
        CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl {
            api_key_secret_version: self.api_key_secret_version,
            key_name: self.key_name,
            request_location: self.request_location,
        }
    }
}
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
        CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_secret_version` after provisioning.\nThe name of the SecretManager secret version resource storing the API key.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn api_key_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_key_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key_name` after provisioning.\nThe parameter name or the header name of the API key.\nE.g., If the API request is \"https://example.com/act?X-Api-Key=\", \"X-Api-Key\" would be the parameter name."]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }
    #[doc = "Get a reference to the value of field `request_location` after provisioning.\nKey location in the request.\nPossible values:\nHEADER\nQUERY_STRING"]
    pub fn request_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<PrimField<String>>,
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.token = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
    type O = BlockAssignable<CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {}
impl BuildCesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
    pub fn build(self) -> CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
        CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl {
            token: core::default::Default::default(),
        }
    }
}
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef {
        CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl {
    client_id: PrimField<String>,
    client_secret_version: PrimField<String>,
    oauth_grant_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    token_endpoint: PrimField<String>,
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl {
    #[doc = "Set the field `scopes`.\nThe OAuth scopes to grant."]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl {
    type O = BlockAssignable<CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl {
    #[doc = "The client ID from the OAuth provider."]
    pub client_id: PrimField<String>,
    #[doc = "The name of the SecretManager secret version resource storing the\nclient secret.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\n\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub client_secret_version: PrimField<String>,
    #[doc = "OAuth grant types.\nPossible values:\nCLIENT_CREDENTIAL"]
    pub oauth_grant_type: PrimField<String>,
    #[doc = "The token endpoint in the OAuth provider to exchange for an access token."]
    pub token_endpoint: PrimField<String>,
}
impl BuildCesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl {
    pub fn build(self) -> CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl {
        CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl {
            client_id: self.client_id,
            client_secret_version: self.client_secret_version,
            oauth_grant_type: self.oauth_grant_type,
            scopes: core::default::Default::default(),
            token_endpoint: self.token_endpoint,
        }
    }
}
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
        CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client ID from the OAuth provider."]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret_version` after provisioning.\nThe name of the SecretManager secret version resource storing the\nclient secret.\nFormat: 'projects/{project}/secrets/{secret}/versions/{version}'\n\nNote: You should grant 'roles/secretmanager.secretAccessor' role to the CES\nservice agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn client_secret_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_grant_type` after provisioning.\nOAuth grant types.\nPossible values:\nCLIENT_CREDENTIAL"]
    pub fn oauth_grant_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oauth_grant_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
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
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    service_account: PrimField<String>,
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    #[doc = "Set the field `scopes`.\nThe OAuth scopes to grant. If not specified, the default scope\n'https://www.googleapis.com/auth/cloud-platform' is used."]
    pub fn set_scopes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
}
impl ToListMappable for CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    type O =
        BlockAssignable<CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    #[doc = "The email address of the service account used for authenticatation. CES\nuses this service account to exchange an access token and the access token\nis then sent in the 'Authorization' header of the request.\n\nThe service account must have the\n'roles/iam.serviceAccountTokenCreator' role granted to the\nCES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub service_account: PrimField<String>,
}
impl BuildCesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
    pub fn build(self) -> CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
        CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl {
            scopes: core::default::Default::default(),
            service_account: self.service_account,
        }
    }
}
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
        CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nThe OAuth scopes to grant. If not specified, the default scope\n'https://www.googleapis.com/auth/cloud-platform' is used."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account` after provisioning.\nThe email address of the service account used for authenticatation. CES\nuses this service account to exchange an access token and the access token\nis then sent in the 'Authorization' header of the request.\n\nThe service account must have the\n'roles/iam.serviceAccountTokenCreator' role granted to the\nCES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_account", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl ToListMappable
    for CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl
{
    type O = BlockAssignable<
        CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
impl BuildCesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {
    pub fn build(
        self,
    ) -> CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {
        CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl {}
    }
}
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
        CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct CesToolsetOpenApiToolsetElApiAuthenticationElDynamic {
    api_key_config:
        Option<DynamicBlock<CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl>>,
    bearer_token_config:
        Option<DynamicBlock<CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl>>,
    oauth_config: Option<DynamicBlock<CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl>>,
    service_account_auth_config: Option<
        DynamicBlock<CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl>,
    >,
    service_agent_id_token_auth_config: Option<
        DynamicBlock<CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl>,
    >,
}
#[derive(Serialize)]
pub struct CesToolsetOpenApiToolsetElApiAuthenticationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key_config: Option<Vec<CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bearer_token_config:
        Option<Vec<CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_config: Option<Vec<CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account_auth_config:
        Option<Vec<CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_agent_id_token_auth_config:
        Option<Vec<CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl>>,
    dynamic: CesToolsetOpenApiToolsetElApiAuthenticationElDynamic,
}
impl CesToolsetOpenApiToolsetElApiAuthenticationEl {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl Into<BlockAssignable<CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigEl>>,
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
        v: impl Into<BlockAssignable<CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigEl>>,
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
        v: impl Into<BlockAssignable<CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigEl>>,
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
    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_account_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account_auth_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.service_agent_id_token_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.service_agent_id_token_auth_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CesToolsetOpenApiToolsetElApiAuthenticationEl {
    type O = BlockAssignable<CesToolsetOpenApiToolsetElApiAuthenticationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetElApiAuthenticationEl {}
impl BuildCesToolsetOpenApiToolsetElApiAuthenticationEl {
    pub fn build(self) -> CesToolsetOpenApiToolsetElApiAuthenticationEl {
        CesToolsetOpenApiToolsetElApiAuthenticationEl {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolsetOpenApiToolsetElApiAuthenticationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElApiAuthenticationElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetOpenApiToolsetElApiAuthenticationElRef {
        CesToolsetOpenApiToolsetElApiAuthenticationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElApiAuthenticationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> ListRef<CesToolsetOpenApiToolsetElApiAuthenticationElApiKeyConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_key_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> ListRef<CesToolsetOpenApiToolsetElApiAuthenticationElBearerTokenConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bearer_token_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(
        &self,
    ) -> ListRef<CesToolsetOpenApiToolsetElApiAuthenticationElOauthConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }
    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> ListRef<CesToolsetOpenApiToolsetElApiAuthenticationElServiceAccountAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_account_auth_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]
    pub fn service_agent_id_token_auth_config(
        &self,
    ) -> ListRef<CesToolsetOpenApiToolsetElApiAuthenticationElServiceAgentIdTokenAuthConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_agent_id_token_auth_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CesToolsetOpenApiToolsetElServiceDirectoryConfigEl {
    service: PrimField<String>,
}
impl CesToolsetOpenApiToolsetElServiceDirectoryConfigEl {}
impl ToListMappable for CesToolsetOpenApiToolsetElServiceDirectoryConfigEl {
    type O = BlockAssignable<CesToolsetOpenApiToolsetElServiceDirectoryConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetElServiceDirectoryConfigEl {
    #[doc = "The name of [Service\nDirectory](https://cloud.google.com/service-directory) service.\nFormat:\n'projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}'.\nLocation of the service directory must be the same as the location of the\napp."]
    pub service: PrimField<String>,
}
impl BuildCesToolsetOpenApiToolsetElServiceDirectoryConfigEl {
    pub fn build(self) -> CesToolsetOpenApiToolsetElServiceDirectoryConfigEl {
        CesToolsetOpenApiToolsetElServiceDirectoryConfigEl {
            service: self.service,
        }
    }
}
pub struct CesToolsetOpenApiToolsetElServiceDirectoryConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElServiceDirectoryConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CesToolsetOpenApiToolsetElServiceDirectoryConfigElRef {
        CesToolsetOpenApiToolsetElServiceDirectoryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElServiceDirectoryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nThe name of [Service\nDirectory](https://cloud.google.com/service-directory) service.\nFormat:\n'projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}'.\nLocation of the service directory must be the same as the location of the\napp."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolsetOpenApiToolsetElTlsConfigElCaCertsEl {
    cert: PrimField<String>,
    display_name: PrimField<String>,
}
impl CesToolsetOpenApiToolsetElTlsConfigElCaCertsEl {}
impl ToListMappable for CesToolsetOpenApiToolsetElTlsConfigElCaCertsEl {
    type O = BlockAssignable<CesToolsetOpenApiToolsetElTlsConfigElCaCertsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetElTlsConfigElCaCertsEl {
    #[doc = "The allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, CES will use Google's default trust\nstore to verify certificates. N.B. Make sure the HTTPS server\ncertificates are signed with \"subject alt name\". For instance a\ncertificate can be self-signed using the following command,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub cert: PrimField<String>,
    #[doc = "The name of the allowed custom CA certificates. This\ncan be used to disambiguate the custom CA certificates."]
    pub display_name: PrimField<String>,
}
impl BuildCesToolsetOpenApiToolsetElTlsConfigElCaCertsEl {
    pub fn build(self) -> CesToolsetOpenApiToolsetElTlsConfigElCaCertsEl {
        CesToolsetOpenApiToolsetElTlsConfigElCaCertsEl {
            cert: self.cert,
            display_name: self.display_name,
        }
    }
}
pub struct CesToolsetOpenApiToolsetElTlsConfigElCaCertsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElTlsConfigElCaCertsElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetOpenApiToolsetElTlsConfigElCaCertsElRef {
        CesToolsetOpenApiToolsetElTlsConfigElCaCertsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElTlsConfigElCaCertsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cert` after provisioning.\nThe allowed custom CA certificates (in DER format) for\nHTTPS verification. This overrides the default SSL trust store. If this\nis empty or unspecified, CES will use Google's default trust\nstore to verify certificates. N.B. Make sure the HTTPS server\ncertificates are signed with \"subject alt name\". For instance a\ncertificate can be self-signed using the following command,\nopenssl x509 -req -days 200 -in example.com.csr \\\n-signkey example.com.key \\\n-out example.com.crt \\\n-extfile <(printf \"\\nsubjectAltName='DNS:www.example.com'\")"]
    pub fn cert(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe name of the allowed custom CA certificates. This\ncan be used to disambiguate the custom CA certificates."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesToolsetOpenApiToolsetElTlsConfigElDynamic {
    ca_certs: Option<DynamicBlock<CesToolsetOpenApiToolsetElTlsConfigElCaCertsEl>>,
}
#[derive(Serialize)]
pub struct CesToolsetOpenApiToolsetElTlsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ca_certs: Option<Vec<CesToolsetOpenApiToolsetElTlsConfigElCaCertsEl>>,
    dynamic: CesToolsetOpenApiToolsetElTlsConfigElDynamic,
}
impl CesToolsetOpenApiToolsetElTlsConfigEl {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl Into<BlockAssignable<CesToolsetOpenApiToolsetElTlsConfigElCaCertsEl>>,
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
impl ToListMappable for CesToolsetOpenApiToolsetElTlsConfigEl {
    type O = BlockAssignable<CesToolsetOpenApiToolsetElTlsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetElTlsConfigEl {}
impl BuildCesToolsetOpenApiToolsetElTlsConfigEl {
    pub fn build(self) -> CesToolsetOpenApiToolsetElTlsConfigEl {
        CesToolsetOpenApiToolsetElTlsConfigEl {
            ca_certs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolsetOpenApiToolsetElTlsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElTlsConfigElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetOpenApiToolsetElTlsConfigElRef {
        CesToolsetOpenApiToolsetElTlsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElTlsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> ListRef<CesToolsetOpenApiToolsetElTlsConfigElCaCertsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}
#[derive(Serialize, Default)]
struct CesToolsetOpenApiToolsetElDynamic {
    api_authentication: Option<DynamicBlock<CesToolsetOpenApiToolsetElApiAuthenticationEl>>,
    service_directory_config:
        Option<DynamicBlock<CesToolsetOpenApiToolsetElServiceDirectoryConfigEl>>,
    tls_config: Option<DynamicBlock<CesToolsetOpenApiToolsetElTlsConfigEl>>,
}
#[derive(Serialize)]
pub struct CesToolsetOpenApiToolsetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_unknown_fields: Option<PrimField<bool>>,
    open_api_schema: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_authentication: Option<Vec<CesToolsetOpenApiToolsetElApiAuthenticationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_directory_config: Option<Vec<CesToolsetOpenApiToolsetElServiceDirectoryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_config: Option<Vec<CesToolsetOpenApiToolsetElTlsConfigEl>>,
    dynamic: CesToolsetOpenApiToolsetElDynamic,
}
impl CesToolsetOpenApiToolsetEl {
    #[doc = "Set the field `ignore_unknown_fields`.\nIf true, the agent will ignore unknown fields in the API response for all\noperations defined in the OpenAPI schema."]
    pub fn set_ignore_unknown_fields(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_unknown_fields = Some(v.into());
        self
    }
    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl Into<BlockAssignable<CesToolsetOpenApiToolsetElApiAuthenticationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_authentication = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v: impl Into<BlockAssignable<CesToolsetOpenApiToolsetElServiceDirectoryConfigEl>>,
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
        v: impl Into<BlockAssignable<CesToolsetOpenApiToolsetElTlsConfigEl>>,
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
impl ToListMappable for CesToolsetOpenApiToolsetEl {
    type O = BlockAssignable<CesToolsetOpenApiToolsetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetOpenApiToolsetEl {
    #[doc = "The OpenAPI schema of the toolset."]
    pub open_api_schema: PrimField<String>,
}
impl BuildCesToolsetOpenApiToolsetEl {
    pub fn build(self) -> CesToolsetOpenApiToolsetEl {
        CesToolsetOpenApiToolsetEl {
            ignore_unknown_fields: core::default::Default::default(),
            open_api_schema: self.open_api_schema,
            api_authentication: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CesToolsetOpenApiToolsetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetOpenApiToolsetElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetOpenApiToolsetElRef {
        CesToolsetOpenApiToolsetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetOpenApiToolsetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ignore_unknown_fields` after provisioning.\nIf true, the agent will ignore unknown fields in the API response for all\noperations defined in the OpenAPI schema."]
    pub fn ignore_unknown_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_unknown_fields", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `open_api_schema` after provisioning.\nThe OpenAPI schema of the toolset."]
    pub fn open_api_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.open_api_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe server URL of the Open API schema.\nThis field is only set in toolsets in the environment dependencies\nduring the export process if the schema contains a server url.\nDuring the import process, if this url is present in the environment dependencies\nand the schema has the $env_var placeholder,\nit will replace the placeholder in the schema."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> ListRef<CesToolsetOpenApiToolsetElApiAuthenticationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.api_authentication", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> ListRef<CesToolsetOpenApiToolsetElServiceDirectoryConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_directory_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> ListRef<CesToolsetOpenApiToolsetElTlsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }
}
#[derive(Serialize)]
pub struct CesToolsetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CesToolsetTimeoutsEl {
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
impl ToListMappable for CesToolsetTimeoutsEl {
    type O = BlockAssignable<CesToolsetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCesToolsetTimeoutsEl {}
impl BuildCesToolsetTimeoutsEl {
    pub fn build(self) -> CesToolsetTimeoutsEl {
        CesToolsetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CesToolsetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CesToolsetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CesToolsetTimeoutsElRef {
        CesToolsetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CesToolsetTimeoutsElRef {
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
struct CesToolsetDynamic {
    mcp_toolset: Option<DynamicBlock<CesToolsetMcpToolsetEl>>,
    open_api_toolset: Option<DynamicBlock<CesToolsetOpenApiToolsetEl>>,
}
