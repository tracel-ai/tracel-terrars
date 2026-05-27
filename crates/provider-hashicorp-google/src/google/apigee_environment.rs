use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_proxy_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_proxy_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    org_id: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ip_resolution_config: Option<Vec<ApigeeEnvironmentClientIpResolutionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<Vec<ApigeeEnvironmentNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<ApigeeEnvironmentPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeEnvironmentTimeoutsEl>,
    dynamic: ApigeeEnvironmentDynamic,
}
struct ApigeeEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeEnvironmentData>,
}
#[derive(Clone)]
pub struct ApigeeEnvironment(Rc<ApigeeEnvironment_>);
impl ApigeeEnvironment {
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
    #[doc = "Set the field `api_proxy_type`.\nOptional. API Proxy type supported by the environment. The type can be set when creating\nthe Environment and cannot be changed. Possible values: [\"API_PROXY_TYPE_UNSPECIFIED\", \"PROGRAMMABLE\", \"CONFIGURABLE\"]"]
    pub fn set_api_proxy_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_proxy_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `deployment_type`.\nOptional. Deployment type supported by the environment. The deployment type can be\nset when creating the environment and cannot be changed. When you enable archive\ndeployment, you will be prevented from performing a subset of actions within the\nenvironment, including:\nManaging the deployment of API proxy or shared flow revisions;\nCreating, updating, or deleting resource files;\nCreating, updating, or deleting target servers. Possible values: [\"DEPLOYMENT_TYPE_UNSPECIFIED\", \"PROXY\", \"ARCHIVE\"]"]
    pub fn set_deployment_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the environment."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nDisplay name of the environment."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `forward_proxy_uri`.\nOptional. URI of the forward proxy to be applied to the runtime instances in this environment. Must be in the format of {scheme}://{hostname}:{port}. Note that the scheme must be one of \"http\" or \"https\", and the port must be supplied."]
    pub fn set_forward_proxy_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().forward_proxy_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nTypes that can be selected for an Environment. Each of the types are\nlimited by capability and capacity. Refer to Apigee's public documentation\nto understand about each of these types in details.\nAn Apigee org can support heterogeneous Environments. Possible values: [\"ENVIRONMENT_TYPE_UNSPECIFIED\", \"BASE\", \"INTERMEDIATE\", \"COMPREHENSIVE\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `client_ip_resolution_config`.\n"]
    pub fn set_client_ip_resolution_config(
        self,
        v: impl Into<BlockAssignable<ApigeeEnvironmentClientIpResolutionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_ip_resolution_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_ip_resolution_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl Into<BlockAssignable<ApigeeEnvironmentNodeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        self,
        v: impl Into<BlockAssignable<ApigeeEnvironmentPropertiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().properties = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.properties = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeEnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_proxy_type` after provisioning.\nOptional. API Proxy type supported by the environment. The type can be set when creating\nthe Environment and cannot be changed. Possible values: [\"API_PROXY_TYPE_UNSPECIFIED\", \"PROGRAMMABLE\", \"CONFIGURABLE\"]"]
    pub fn api_proxy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_proxy_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\nOptional. Deployment type supported by the environment. The deployment type can be\nset when creating the environment and cannot be changed. When you enable archive\ndeployment, you will be prevented from performing a subset of actions within the\nenvironment, including:\nManaging the deployment of API proxy or shared flow revisions;\nCreating, updating, or deleting resource files;\nCreating, updating, or deleting target servers. Possible values: [\"DEPLOYMENT_TYPE_UNSPECIFIED\", \"PROXY\", \"ARCHIVE\"]"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the environment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the environment."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forward_proxy_uri` after provisioning.\nOptional. URI of the forward proxy to be applied to the runtime instances in this environment. Must be in the format of {scheme}://{hostname}:{port}. Note that the scheme must be one of \"http\" or \"https\", and the port must be supplied."]
    pub fn forward_proxy_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forward_proxy_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource ID of the environment."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee environment,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nTypes that can be selected for an Environment. Each of the types are\nlimited by capability and capacity. Refer to Apigee's public documentation\nto understand about each of these types in details.\nAn Apigee org can support heterogeneous Environments. Possible values: [\"ENVIRONMENT_TYPE_UNSPECIFIED\", \"BASE\", \"INTERMEDIATE\", \"COMPREHENSIVE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_ip_resolution_config` after provisioning.\n"]
    pub fn client_ip_resolution_config(
        &self,
    ) -> ListRef<ApigeeEnvironmentClientIpResolutionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_ip_resolution_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<ApigeeEnvironmentNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<ApigeeEnvironmentPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEnvironmentTimeoutsElRef {
        ApigeeEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeEnvironment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeEnvironment {}
impl ToListMappable for ApigeeEnvironment {
    type O = ListRef<ApigeeEnvironmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_environment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeEnvironment {
    pub tf_id: String,
    #[doc = "The resource ID of the environment."]
    pub name: PrimField<String>,
    #[doc = "The Apigee Organization associated with the Apigee environment,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: PrimField<String>,
}
impl BuildApigeeEnvironment {
    pub fn build(self, stack: &mut Stack) -> ApigeeEnvironment {
        let out = ApigeeEnvironment(Rc::new(ApigeeEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_proxy_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                forward_proxy_uri: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                org_id: self.org_id,
                type_: core::default::Default::default(),
                client_ip_resolution_config: core::default::Default::default(),
                node_config: core::default::Default::default(),
                properties: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeEnvironmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_proxy_type` after provisioning.\nOptional. API Proxy type supported by the environment. The type can be set when creating\nthe Environment and cannot be changed. Possible values: [\"API_PROXY_TYPE_UNSPECIFIED\", \"PROGRAMMABLE\", \"CONFIGURABLE\"]"]
    pub fn api_proxy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_proxy_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\nOptional. Deployment type supported by the environment. The deployment type can be\nset when creating the environment and cannot be changed. When you enable archive\ndeployment, you will be prevented from performing a subset of actions within the\nenvironment, including:\nManaging the deployment of API proxy or shared flow revisions;\nCreating, updating, or deleting resource files;\nCreating, updating, or deleting target servers. Possible values: [\"DEPLOYMENT_TYPE_UNSPECIFIED\", \"PROXY\", \"ARCHIVE\"]"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the environment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the environment."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `forward_proxy_uri` after provisioning.\nOptional. URI of the forward proxy to be applied to the runtime instances in this environment. Must be in the format of {scheme}://{hostname}:{port}. Note that the scheme must be one of \"http\" or \"https\", and the port must be supplied."]
    pub fn forward_proxy_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.forward_proxy_uri", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource ID of the environment."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee environment,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nTypes that can be selected for an Environment. Each of the types are\nlimited by capability and capacity. Refer to Apigee's public documentation\nto understand about each of these types in details.\nAn Apigee org can support heterogeneous Environments. Possible values: [\"ENVIRONMENT_TYPE_UNSPECIFIED\", \"BASE\", \"INTERMEDIATE\", \"COMPREHENSIVE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_ip_resolution_config` after provisioning.\n"]
    pub fn client_ip_resolution_config(
        &self,
    ) -> ListRef<ApigeeEnvironmentClientIpResolutionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_ip_resolution_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<ApigeeEnvironmentNodeConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<ApigeeEnvironmentPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEnvironmentTimeoutsElRef {
        ApigeeEnvironmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl {
    ip_header_index: PrimField<f64>,
    ip_header_name: PrimField<String>,
}
impl ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl {}
impl ToListMappable for ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl {
    type O = BlockAssignable<ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl {
    #[doc = "The index of the ip in the header. Positive indices 0, 1, 2, 3 chooses indices from the left (first ips). Negative indices -1, -2, -3 chooses indices from the right (last ips)."]
    pub ip_header_index: PrimField<f64>,
    #[doc = "The name of the header to extract the client ip from. We are currently only supporting the X-Forwarded-For header."]
    pub ip_header_name: PrimField<String>,
}
impl BuildApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl {
    pub fn build(self) -> ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl {
        ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl {
            ip_header_index: self.ip_header_index,
            ip_header_name: self.ip_header_name,
        }
    }
}
pub struct ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmElRef {
        ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_header_index` after provisioning.\nThe index of the ip in the header. Positive indices 0, 1, 2, 3 chooses indices from the left (first ips). Negative indices -1, -2, -3 chooses indices from the right (last ips)."]
    pub fn ip_header_index(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_header_index", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ip_header_name` after provisioning.\nThe name of the header to extract the client ip from. We are currently only supporting the X-Forwarded-For header."]
    pub fn ip_header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_header_name", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApigeeEnvironmentClientIpResolutionConfigElDynamic {
    header_index_algorithm:
        Option<DynamicBlock<ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl>>,
}
#[derive(Serialize)]
pub struct ApigeeEnvironmentClientIpResolutionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_index_algorithm:
        Option<Vec<ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl>>,
    dynamic: ApigeeEnvironmentClientIpResolutionConfigElDynamic,
}
impl ApigeeEnvironmentClientIpResolutionConfigEl {
    #[doc = "Set the field `header_index_algorithm`.\n"]
    pub fn set_header_index_algorithm(
        mut self,
        v: impl Into<BlockAssignable<ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_index_algorithm = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_index_algorithm = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeEnvironmentClientIpResolutionConfigEl {
    type O = BlockAssignable<ApigeeEnvironmentClientIpResolutionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeEnvironmentClientIpResolutionConfigEl {}
impl BuildApigeeEnvironmentClientIpResolutionConfigEl {
    pub fn build(self) -> ApigeeEnvironmentClientIpResolutionConfigEl {
        ApigeeEnvironmentClientIpResolutionConfigEl {
            header_index_algorithm: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeEnvironmentClientIpResolutionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeEnvironmentClientIpResolutionConfigElRef {
    fn new(shared: StackShared, base: String) -> ApigeeEnvironmentClientIpResolutionConfigElRef {
        ApigeeEnvironmentClientIpResolutionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeEnvironmentClientIpResolutionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_index_algorithm` after provisioning.\n"]
    pub fn header_index_algorithm(
        &self,
    ) -> ListRef<ApigeeEnvironmentClientIpResolutionConfigElHeaderIndexAlgorithmElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.header_index_algorithm", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeEnvironmentNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_node_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_count: Option<PrimField<String>>,
}
impl ApigeeEnvironmentNodeConfigEl {
    #[doc = "Set the field `max_node_count`.\nThe maximum total number of gateway nodes that the is reserved for all instances that\nhas the specified environment. If not specified, the default is determined by the\nrecommended maximum number of nodes for that gateway."]
    pub fn set_max_node_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_node_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_node_count`.\nThe minimum total number of gateway nodes that the is reserved for all instances that\nhas the specified environment. If not specified, the default is determined by the\nrecommended minimum number of nodes for that gateway."]
    pub fn set_min_node_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_node_count = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeEnvironmentNodeConfigEl {
    type O = BlockAssignable<ApigeeEnvironmentNodeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeEnvironmentNodeConfigEl {}
impl BuildApigeeEnvironmentNodeConfigEl {
    pub fn build(self) -> ApigeeEnvironmentNodeConfigEl {
        ApigeeEnvironmentNodeConfigEl {
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
        }
    }
}
pub struct ApigeeEnvironmentNodeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeEnvironmentNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> ApigeeEnvironmentNodeConfigElRef {
        ApigeeEnvironmentNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeEnvironmentNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `current_aggregate_node_count` after provisioning.\nThe current total number of gateway nodes that each environment currently has across\nall instances."]
    pub fn current_aggregate_node_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.current_aggregate_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_node_count` after provisioning.\nThe maximum total number of gateway nodes that the is reserved for all instances that\nhas the specified environment. If not specified, the default is determined by the\nrecommended maximum number of nodes for that gateway."]
    pub fn max_node_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_node_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_node_count` after provisioning.\nThe minimum total number of gateway nodes that the is reserved for all instances that\nhas the specified environment. If not specified, the default is determined by the\nrecommended minimum number of nodes for that gateway."]
    pub fn min_node_count(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_node_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeEnvironmentPropertiesElPropertyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ApigeeEnvironmentPropertiesElPropertyEl {
    #[doc = "Set the field `name`.\nThe property key."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nThe property value."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeEnvironmentPropertiesElPropertyEl {
    type O = BlockAssignable<ApigeeEnvironmentPropertiesElPropertyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeEnvironmentPropertiesElPropertyEl {}
impl BuildApigeeEnvironmentPropertiesElPropertyEl {
    pub fn build(self) -> ApigeeEnvironmentPropertiesElPropertyEl {
        ApigeeEnvironmentPropertiesElPropertyEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApigeeEnvironmentPropertiesElPropertyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeEnvironmentPropertiesElPropertyElRef {
    fn new(shared: StackShared, base: String) -> ApigeeEnvironmentPropertiesElPropertyElRef {
        ApigeeEnvironmentPropertiesElPropertyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeEnvironmentPropertiesElPropertyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe property key."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe property value."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeEnvironmentPropertiesElDynamic {
    property: Option<DynamicBlock<ApigeeEnvironmentPropertiesElPropertyEl>>,
}
#[derive(Serialize)]
pub struct ApigeeEnvironmentPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    property: Option<Vec<ApigeeEnvironmentPropertiesElPropertyEl>>,
    dynamic: ApigeeEnvironmentPropertiesElDynamic,
}
impl ApigeeEnvironmentPropertiesEl {
    #[doc = "Set the field `property`.\n"]
    pub fn set_property(
        mut self,
        v: impl Into<BlockAssignable<ApigeeEnvironmentPropertiesElPropertyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.property = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.property = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeEnvironmentPropertiesEl {
    type O = BlockAssignable<ApigeeEnvironmentPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeEnvironmentPropertiesEl {}
impl BuildApigeeEnvironmentPropertiesEl {
    pub fn build(self) -> ApigeeEnvironmentPropertiesEl {
        ApigeeEnvironmentPropertiesEl {
            property: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeEnvironmentPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeEnvironmentPropertiesElRef {
    fn new(shared: StackShared, base: String) -> ApigeeEnvironmentPropertiesElRef {
        ApigeeEnvironmentPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeEnvironmentPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `property` after provisioning.\n"]
    pub fn property(&self) -> ListRef<ApigeeEnvironmentPropertiesElPropertyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.property", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeEnvironmentTimeoutsEl {
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
impl ToListMappable for ApigeeEnvironmentTimeoutsEl {
    type O = BlockAssignable<ApigeeEnvironmentTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeEnvironmentTimeoutsEl {}
impl BuildApigeeEnvironmentTimeoutsEl {
    pub fn build(self) -> ApigeeEnvironmentTimeoutsEl {
        ApigeeEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeEnvironmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeEnvironmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeEnvironmentTimeoutsElRef {
        ApigeeEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeEnvironmentTimeoutsElRef {
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
struct ApigeeEnvironmentDynamic {
    client_ip_resolution_config: Option<DynamicBlock<ApigeeEnvironmentClientIpResolutionConfigEl>>,
    node_config: Option<DynamicBlock<ApigeeEnvironmentNodeConfigEl>>,
    properties: Option<DynamicBlock<ApigeeEnvironmentPropertiesEl>>,
}
