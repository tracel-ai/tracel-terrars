use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApigeeApiProductData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    approval_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environments: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    org_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxies: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_counter_scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_time_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    space: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<ApigeeApiProductAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    graphql_operation_group: Option<Vec<ApigeeApiProductGraphqlOperationGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_operation_group: Option<Vec<ApigeeApiProductGrpcOperationGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_group: Option<Vec<ApigeeApiProductOperationGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeApiProductTimeoutsEl>,
    dynamic: ApigeeApiProductDynamic,
}
struct ApigeeApiProduct_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeApiProductData>,
}
#[derive(Clone)]
pub struct ApigeeApiProduct(Rc<ApigeeApiProduct_>);
impl ApigeeApiProduct {
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
    #[doc = "Set the field `api_resources`.\nComma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the proxy.pathsuffix variable.\nThe proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the apiResources element is defined to be /forecastrss and the base path defined for the API proxy is /weather, then only requests to /weather/forecastrss are permitted by the API product."]
    pub fn set_api_resources(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().api_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `approval_type`.\nFlag that specifies how API keys are approved to access the APIs defined by the API product.\nValid values are 'auto' or 'manual'. Possible values: [\"auto\", \"manual\"]"]
    pub fn set_approval_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().approval_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nDescription of the API product. Include key information about the API product that is not captured by other fields."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `environments`.\nComma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected.\nBy specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment."]
    pub fn set_environments(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().environments = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `proxies`.\nComma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies.\nApigee rejects requests to API proxies that are not listed."]
    pub fn set_proxies(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().proxies = Some(v.into());
        self
    }
    #[doc = "Set the field `quota`.\nNumber of request messages permitted per app by this API product for the specified quotaInterval and quotaTimeUnit.\nFor example, a quota of 50, for a quotaInterval of 12 and a quotaTimeUnit of hours means 50 requests are allowed every 12 hours."]
    pub fn set_quota(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().quota = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_counter_scope`.\nScope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself. Possible values: [\"QUOTA_COUNTER_SCOPE_UNSPECIFIED\", \"PROXY\", \"OPERATION\"]"]
    pub fn set_quota_counter_scope(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().quota_counter_scope = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_interval`.\nTime interval over which the number of request messages is calculated."]
    pub fn set_quota_interval(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().quota_interval = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_time_unit`.\nTime unit defined for the quotaInterval. Valid values include second, minute, hour, day, month or year."]
    pub fn set_quota_time_unit(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().quota_time_unit = Some(v.into());
        self
    }
    #[doc = "Set the field `scopes`.\nComma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product."]
    pub fn set_scopes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().scopes = Some(v.into());
        self
    }
    #[doc = "Set the field `space`.\nOptional. The resource ID of the parent Space. If not set, the parent resource will be the Organization."]
    pub fn set_space(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().space = Some(v.into());
        self
    }
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        self,
        v: impl Into<BlockAssignable<ApigeeApiProductAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attributes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `graphql_operation_group`.\n"]
    pub fn set_graphql_operation_group(
        self,
        v: impl Into<BlockAssignable<ApigeeApiProductGraphqlOperationGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().graphql_operation_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.graphql_operation_group = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `grpc_operation_group`.\n"]
    pub fn set_grpc_operation_group(
        self,
        v: impl Into<BlockAssignable<ApigeeApiProductGrpcOperationGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().grpc_operation_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.grpc_operation_group = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `operation_group`.\n"]
    pub fn set_operation_group(
        self,
        v: impl Into<BlockAssignable<ApigeeApiProductOperationGroupEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().operation_group = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.operation_group = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeApiProductTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_resources` after provisioning.\nComma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the proxy.pathsuffix variable.\nThe proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the apiResources element is defined to be /forecastrss and the base path defined for the API proxy is /weather, then only requests to /weather/forecastrss are permitted by the API product."]
    pub fn api_resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.api_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approval_type` after provisioning.\nFlag that specifies how API keys are approved to access the APIs defined by the API product.\nValid values are 'auto' or 'manual'. Possible values: [\"auto\", \"manual\"]"]
    pub fn approval_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approval_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nResponse only. Creation time of this environment as milliseconds since epoch."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the API product. Include key information about the API product that is not captured by other fields."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nName displayed in the UI or developer portal to developers registering for API access."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environments` after provisioning.\nComma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected.\nBy specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment."]
    pub fn environments(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.environments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_modified_at` after provisioning.\nResponse only. Modified time of this environment as milliseconds since epoch."]
    pub fn last_modified_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nInternal name of the API product."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee API product,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxies` after provisioning.\nComma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies.\nApigee rejects requests to API proxies that are not listed."]
    pub fn proxies(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.proxies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota` after provisioning.\nNumber of request messages permitted per app by this API product for the specified quotaInterval and quotaTimeUnit.\nFor example, a quota of 50, for a quotaInterval of 12 and a quotaTimeUnit of hours means 50 requests are allowed every 12 hours."]
    pub fn quota(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_counter_scope` after provisioning.\nScope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself. Possible values: [\"QUOTA_COUNTER_SCOPE_UNSPECIFIED\", \"PROXY\", \"OPERATION\"]"]
    pub fn quota_counter_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_counter_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_interval` after provisioning.\nTime interval over which the number of request messages is calculated."]
    pub fn quota_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_time_unit` after provisioning.\nTime unit defined for the quotaInterval. Valid values include second, minute, hour, day, month or year."]
    pub fn quota_time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_time_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nComma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `space` after provisioning.\nOptional. The resource ID of the parent Space. If not set, the parent resource will be the Organization."]
    pub fn space(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.space", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphql_operation_group` after provisioning.\n"]
    pub fn graphql_operation_group(&self) -> ListRef<ApigeeApiProductGraphqlOperationGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.graphql_operation_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_operation_group` after provisioning.\n"]
    pub fn grpc_operation_group(&self) -> ListRef<ApigeeApiProductGrpcOperationGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_operation_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation_group` after provisioning.\n"]
    pub fn operation_group(&self) -> ListRef<ApigeeApiProductOperationGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operation_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeApiProductTimeoutsElRef {
        ApigeeApiProductTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApigeeApiProduct {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApigeeApiProduct {}
impl ToListMappable for ApigeeApiProduct {
    type O = ListRef<ApigeeApiProductRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApigeeApiProduct_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_api_product".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApigeeApiProduct {
    pub tf_id: String,
    #[doc = "Name displayed in the UI or developer portal to developers registering for API access."]
    pub display_name: PrimField<String>,
    #[doc = "Internal name of the API product."]
    pub name: PrimField<String>,
    #[doc = "The Apigee Organization associated with the Apigee API product,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: PrimField<String>,
}
impl BuildApigeeApiProduct {
    pub fn build(self, stack: &mut Stack) -> ApigeeApiProduct {
        let out = ApigeeApiProduct(Rc::new(ApigeeApiProduct_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeApiProductData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_resources: core::default::Default::default(),
                approval_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                environments: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                org_id: self.org_id,
                proxies: core::default::Default::default(),
                quota: core::default::Default::default(),
                quota_counter_scope: core::default::Default::default(),
                quota_interval: core::default::Default::default(),
                quota_time_unit: core::default::Default::default(),
                scopes: core::default::Default::default(),
                space: core::default::Default::default(),
                attributes: core::default::Default::default(),
                graphql_operation_group: core::default::Default::default(),
                grpc_operation_group: core::default::Default::default(),
                operation_group: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApigeeApiProductRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApigeeApiProductRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_resources` after provisioning.\nComma-separated list of API resources to be bundled in the API product. By default, the resource paths are mapped from the proxy.pathsuffix variable.\nThe proxy path suffix is defined as the URI fragment following the ProxyEndpoint base path. For example, if the apiResources element is defined to be /forecastrss and the base path defined for the API proxy is /weather, then only requests to /weather/forecastrss are permitted by the API product."]
    pub fn api_resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.api_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `approval_type` after provisioning.\nFlag that specifies how API keys are approved to access the APIs defined by the API product.\nValid values are 'auto' or 'manual'. Possible values: [\"auto\", \"manual\"]"]
    pub fn approval_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.approval_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nResponse only. Creation time of this environment as milliseconds since epoch."]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the API product. Include key information about the API product that is not captured by other fields."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nName displayed in the UI or developer portal to developers registering for API access."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `environments` after provisioning.\nComma-separated list of environment names to which the API product is bound. Requests to environments that are not listed are rejected.\nBy specifying one or more environments, you can bind the resources listed in the API product to a specific environment, preventing developers from accessing those resources through API proxies deployed in another environment."]
    pub fn environments(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.environments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_modified_at` after provisioning.\nResponse only. Modified time of this environment as milliseconds since epoch."]
    pub fn last_modified_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nInternal name of the API product."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee API product,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxies` after provisioning.\nComma-separated list of API proxy names to which this API product is bound. By specifying API proxies, you can associate resources in the API product with specific API proxies, preventing developers from accessing those resources through other API proxies.\nApigee rejects requests to API proxies that are not listed."]
    pub fn proxies(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.proxies", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota` after provisioning.\nNumber of request messages permitted per app by this API product for the specified quotaInterval and quotaTimeUnit.\nFor example, a quota of 50, for a quotaInterval of 12 and a quotaTimeUnit of hours means 50 requests are allowed every 12 hours."]
    pub fn quota(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_counter_scope` after provisioning.\nScope of the quota decides how the quota counter gets applied and evaluate for quota violation. If the Scope is set as PROXY, then all the operations defined for the APIproduct that are associated with the same proxy will share the same quota counter set at the APIproduct level, making it a global counter at a proxy level. If the Scope is set as OPERATION, then each operations get the counter set at the API product dedicated, making it a local counter. Note that, the QuotaCounterScope applies only when an operation does not have dedicated quota set for itself. Possible values: [\"QUOTA_COUNTER_SCOPE_UNSPECIFIED\", \"PROXY\", \"OPERATION\"]"]
    pub fn quota_counter_scope(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_counter_scope", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_interval` after provisioning.\nTime interval over which the number of request messages is calculated."]
    pub fn quota_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_interval", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `quota_time_unit` after provisioning.\nTime unit defined for the quotaInterval. Valid values include second, minute, hour, day, month or year."]
    pub fn quota_time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quota_time_unit", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nComma-separated list of OAuth scopes that are validated at runtime. Apigee validates that the scopes in any access token presented match the scopes defined in the OAuth policy associated with the API product."]
    pub fn scopes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scopes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `space` after provisioning.\nOptional. The resource ID of the parent Space. If not set, the parent resource will be the Organization."]
    pub fn space(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.space", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `graphql_operation_group` after provisioning.\n"]
    pub fn graphql_operation_group(&self) -> ListRef<ApigeeApiProductGraphqlOperationGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.graphql_operation_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grpc_operation_group` after provisioning.\n"]
    pub fn grpc_operation_group(&self) -> ListRef<ApigeeApiProductGrpcOperationGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grpc_operation_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `operation_group` after provisioning.\n"]
    pub fn operation_group(&self) -> ListRef<ApigeeApiProductOperationGroupElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operation_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeApiProductTimeoutsElRef {
        ApigeeApiProductTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ApigeeApiProductAttributesEl {
    #[doc = "Set the field `name`.\nKey of the attribute."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nValue of the attribute."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeApiProductAttributesEl {
    type O = BlockAssignable<ApigeeApiProductAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductAttributesEl {}
impl BuildApigeeApiProductAttributesEl {
    pub fn build(self) -> ApigeeApiProductAttributesEl {
        ApigeeApiProductAttributesEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductAttributesElRef {
    fn new(shared: StackShared, base: String) -> ApigeeApiProductAttributesElRef {
        ApigeeApiProductAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nKey of the attribute."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the attribute."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl {
    #[doc = "Set the field `name`.\nKey of the attribute."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nValue of the attribute."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl {
    type O = BlockAssignable<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl {}
impl BuildApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl {
    pub fn build(self) -> ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl {
        ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesElRef {
        ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nKey of the attribute."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the attribute."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_types: Option<SetField<PrimField<String>>>,
}
impl ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl {
    #[doc = "Set the field `operation`.\nGraphQL operation name. The name and operation type will be used to apply quotas. If no name is specified, the quota will be applied to all GraphQL operations irrespective of their operation names in the payload."]
    pub fn set_operation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operation = Some(v.into());
        self
    }
    #[doc = "Set the field `operation_types`.\nRequired. GraphQL operation types. Valid values include query or mutation.\nNote: Apigee does not currently support subscription types."]
    pub fn set_operation_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.operation_types = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl {
    type O = BlockAssignable<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl {}
impl BuildApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl {
    pub fn build(self) -> ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl {
        ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl {
            operation: core::default::Default::default(),
            operation_types: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsElRef {
        ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operation` after provisioning.\nGraphQL operation name. The name and operation type will be used to apply quotas. If no name is specified, the quota will be applied to all GraphQL operations irrespective of their operation names in the payload."]
    pub fn operation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }
    #[doc = "Get a reference to the value of field `operation_types` after provisioning.\nRequired. GraphQL operation types. Valid values include query or mutation.\nNote: Apigee does not currently support subscription types."]
    pub fn operation_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.operation_types", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_unit: Option<PrimField<String>>,
}
impl ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl {
    #[doc = "Set the field `interval`.\nRequired. Time interval over which the number of request messages is calculated."]
    pub fn set_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval = Some(v.into());
        self
    }
    #[doc = "Set the field `limit`.\nRequired. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
    pub fn set_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.limit = Some(v.into());
        self
    }
    #[doc = "Set the field `time_unit`.\nTime unit defined for the interval. Valid values include second, minute, hour, day, month or year. If limit and interval are valid, the default value is hour; otherwise, the default is null."]
    pub fn set_time_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_unit = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl {
    type O = BlockAssignable<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl {}
impl BuildApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl {
    pub fn build(self) -> ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl {
        ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl {
            interval: core::default::Default::default(),
            limit: core::default::Default::default(),
            time_unit: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaElRef {
        ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\nRequired. Time interval over which the number of request messages is calculated."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `limit` after provisioning.\nRequired. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
    pub fn limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }
    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\nTime unit defined for the interval. Valid values include second, minute, hour, day, month or year. If limit and interval are valid, the default value is hour; otherwise, the default is null."]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeApiProductGraphqlOperationGroupElOperationConfigsElDynamic {
    attributes:
        Option<DynamicBlock<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl>>,
    operations:
        Option<DynamicBlock<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl>>,
    quota: Option<DynamicBlock<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl>>,
}
#[derive(Serialize)]
pub struct ApigeeApiProductGraphqlOperationGroupElOperationConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operations: Option<Vec<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<Vec<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl>>,
    dynamic: ApigeeApiProductGraphqlOperationGroupElOperationConfigsElDynamic,
}
impl ApigeeApiProductGraphqlOperationGroupElOperationConfigsEl {
    #[doc = "Set the field `api_source`.\nRequired. Name of the API proxy endpoint or remote service with which the GraphQL operation and quota are associated."]
    pub fn set_api_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_source = Some(v.into());
        self
    }
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl Into<
            BlockAssignable<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElAttributesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attributes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl Into<
            BlockAssignable<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElOperationsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `quota`.\n"]
    pub fn set_quota(
        mut self,
        v: impl Into<BlockAssignable<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.quota = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.quota = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeApiProductGraphqlOperationGroupElOperationConfigsEl {
    type O = BlockAssignable<ApigeeApiProductGraphqlOperationGroupElOperationConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductGraphqlOperationGroupElOperationConfigsEl {}
impl BuildApigeeApiProductGraphqlOperationGroupElOperationConfigsEl {
    pub fn build(self) -> ApigeeApiProductGraphqlOperationGroupElOperationConfigsEl {
        ApigeeApiProductGraphqlOperationGroupElOperationConfigsEl {
            api_source: core::default::Default::default(),
            attributes: core::default::Default::default(),
            operations: core::default::Default::default(),
            quota: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeApiProductGraphqlOperationGroupElOperationConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductGraphqlOperationGroupElOperationConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductGraphqlOperationGroupElOperationConfigsElRef {
        ApigeeApiProductGraphqlOperationGroupElOperationConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductGraphqlOperationGroupElOperationConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_source` after provisioning.\nRequired. Name of the API proxy endpoint or remote service with which the GraphQL operation and quota are associated."]
    pub fn api_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_source", self.base))
    }
    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(
        &self,
    ) -> ListRef<ApigeeApiProductGraphqlOperationGroupElOperationConfigsElQuotaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.quota", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeApiProductGraphqlOperationGroupElDynamic {
    operation_configs:
        Option<DynamicBlock<ApigeeApiProductGraphqlOperationGroupElOperationConfigsEl>>,
}
#[derive(Serialize)]
pub struct ApigeeApiProductGraphqlOperationGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_config_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_configs: Option<Vec<ApigeeApiProductGraphqlOperationGroupElOperationConfigsEl>>,
    dynamic: ApigeeApiProductGraphqlOperationGroupElDynamic,
}
impl ApigeeApiProductGraphqlOperationGroupEl {
    #[doc = "Set the field `operation_config_type`.\nFlag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values include proxy or remoteservice. Defaults to proxy. Set to proxy when Apigee API proxies are associated with the API product. Set to remoteservice when non-Apigee proxies like Istio-Envoy are associated with the API product. Possible values: [\"proxy\", \"remoteservice\"]"]
    pub fn set_operation_config_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operation_config_type = Some(v.into());
        self
    }
    #[doc = "Set the field `operation_configs`.\n"]
    pub fn set_operation_configs(
        mut self,
        v: impl Into<BlockAssignable<ApigeeApiProductGraphqlOperationGroupElOperationConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operation_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operation_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeApiProductGraphqlOperationGroupEl {
    type O = BlockAssignable<ApigeeApiProductGraphqlOperationGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductGraphqlOperationGroupEl {}
impl BuildApigeeApiProductGraphqlOperationGroupEl {
    pub fn build(self) -> ApigeeApiProductGraphqlOperationGroupEl {
        ApigeeApiProductGraphqlOperationGroupEl {
            operation_config_type: core::default::Default::default(),
            operation_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeApiProductGraphqlOperationGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductGraphqlOperationGroupElRef {
    fn new(shared: StackShared, base: String) -> ApigeeApiProductGraphqlOperationGroupElRef {
        ApigeeApiProductGraphqlOperationGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductGraphqlOperationGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operation_config_type` after provisioning.\nFlag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values include proxy or remoteservice. Defaults to proxy. Set to proxy when Apigee API proxies are associated with the API product. Set to remoteservice when non-Apigee proxies like Istio-Envoy are associated with the API product. Possible values: [\"proxy\", \"remoteservice\"]"]
    pub fn operation_config_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation_config_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl {
    #[doc = "Set the field `name`.\nKey of the attribute."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nValue of the attribute."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl {
    type O = BlockAssignable<ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl {}
impl BuildApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl {
    pub fn build(self) -> ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl {
        ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesElRef {
        ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nKey of the attribute."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the attribute."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_unit: Option<PrimField<String>>,
}
impl ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl {
    #[doc = "Set the field `interval`.\nRequired. Time interval over which the number of request messages is calculated."]
    pub fn set_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval = Some(v.into());
        self
    }
    #[doc = "Set the field `limit`.\nRequired. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
    pub fn set_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.limit = Some(v.into());
        self
    }
    #[doc = "Set the field `time_unit`.\nTime unit defined for the interval. Valid values include second, minute, hour, day, month or year. If limit and interval are valid, the default value is hour; otherwise, the default is null."]
    pub fn set_time_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_unit = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl {
    type O = BlockAssignable<ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl {}
impl BuildApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl {
    pub fn build(self) -> ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl {
        ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl {
            interval: core::default::Default::default(),
            limit: core::default::Default::default(),
            time_unit: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaElRef {
        ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\nRequired. Time interval over which the number of request messages is calculated."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `limit` after provisioning.\nRequired. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
    pub fn limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }
    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\nTime unit defined for the interval. Valid values include second, minute, hour, day, month or year. If limit and interval are valid, the default value is hour; otherwise, the default is null."]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeApiProductGrpcOperationGroupElOperationConfigsElDynamic {
    attributes:
        Option<DynamicBlock<ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl>>,
    quota: Option<DynamicBlock<ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl>>,
}
#[derive(Serialize)]
pub struct ApigeeApiProductGrpcOperationGroupElOperationConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<Vec<ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl>>,
    dynamic: ApigeeApiProductGrpcOperationGroupElOperationConfigsElDynamic,
}
impl ApigeeApiProductGrpcOperationGroupElOperationConfigsEl {
    #[doc = "Set the field `api_source`.\nRequired. Name of the API proxy with which the gRPC operation and quota are associated."]
    pub fn set_api_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_source = Some(v.into());
        self
    }
    #[doc = "Set the field `methods`.\nList of unqualified gRPC method names for the proxy to which quota will be applied. If this field is empty, the Quota will apply to all operations on the gRPC service defined on the proxy.\n\nExample: Given a proxy that is configured to serve com.petstore.PetService, the methods com.petstore.PetService.ListPets and com.petstore.PetService.GetPet would be specified here as simply [\"ListPets\", \"GetPet\"].\n\nNote: Currently, you can specify only a single GraphQLOperation. Specifying more than one will cause the operation to fail."]
    pub fn set_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.methods = Some(v.into());
        self
    }
    #[doc = "Set the field `service`.\nRequired. gRPC Service name associated to be associated with the API proxy, on which quota rules can be applied upon."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl Into<
            BlockAssignable<ApigeeApiProductGrpcOperationGroupElOperationConfigsElAttributesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attributes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `quota`.\n"]
    pub fn set_quota(
        mut self,
        v: impl Into<BlockAssignable<ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.quota = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.quota = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeApiProductGrpcOperationGroupElOperationConfigsEl {
    type O = BlockAssignable<ApigeeApiProductGrpcOperationGroupElOperationConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductGrpcOperationGroupElOperationConfigsEl {}
impl BuildApigeeApiProductGrpcOperationGroupElOperationConfigsEl {
    pub fn build(self) -> ApigeeApiProductGrpcOperationGroupElOperationConfigsEl {
        ApigeeApiProductGrpcOperationGroupElOperationConfigsEl {
            api_source: core::default::Default::default(),
            methods: core::default::Default::default(),
            service: core::default::Default::default(),
            attributes: core::default::Default::default(),
            quota: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeApiProductGrpcOperationGroupElOperationConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductGrpcOperationGroupElOperationConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductGrpcOperationGroupElOperationConfigsElRef {
        ApigeeApiProductGrpcOperationGroupElOperationConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductGrpcOperationGroupElOperationConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_source` after provisioning.\nRequired. Name of the API proxy with which the gRPC operation and quota are associated."]
    pub fn api_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_source", self.base))
    }
    #[doc = "Get a reference to the value of field `methods` after provisioning.\nList of unqualified gRPC method names for the proxy to which quota will be applied. If this field is empty, the Quota will apply to all operations on the gRPC service defined on the proxy.\n\nExample: Given a proxy that is configured to serve com.petstore.PetService, the methods com.petstore.PetService.ListPets and com.petstore.PetService.GetPet would be specified here as simply [\"ListPets\", \"GetPet\"].\n\nNote: Currently, you can specify only a single GraphQLOperation. Specifying more than one will cause the operation to fail."]
    pub fn methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nRequired. gRPC Service name associated to be associated with the API proxy, on which quota rules can be applied upon."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(
        &self,
    ) -> ListRef<ApigeeApiProductGrpcOperationGroupElOperationConfigsElQuotaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.quota", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeApiProductGrpcOperationGroupElDynamic {
    operation_configs: Option<DynamicBlock<ApigeeApiProductGrpcOperationGroupElOperationConfigsEl>>,
}
#[derive(Serialize)]
pub struct ApigeeApiProductGrpcOperationGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_configs: Option<Vec<ApigeeApiProductGrpcOperationGroupElOperationConfigsEl>>,
    dynamic: ApigeeApiProductGrpcOperationGroupElDynamic,
}
impl ApigeeApiProductGrpcOperationGroupEl {
    #[doc = "Set the field `operation_configs`.\n"]
    pub fn set_operation_configs(
        mut self,
        v: impl Into<BlockAssignable<ApigeeApiProductGrpcOperationGroupElOperationConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operation_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operation_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeApiProductGrpcOperationGroupEl {
    type O = BlockAssignable<ApigeeApiProductGrpcOperationGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductGrpcOperationGroupEl {}
impl BuildApigeeApiProductGrpcOperationGroupEl {
    pub fn build(self) -> ApigeeApiProductGrpcOperationGroupEl {
        ApigeeApiProductGrpcOperationGroupEl {
            operation_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeApiProductGrpcOperationGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductGrpcOperationGroupElRef {
    fn new(shared: StackShared, base: String) -> ApigeeApiProductGrpcOperationGroupElRef {
        ApigeeApiProductGrpcOperationGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductGrpcOperationGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductOperationGroupElOperationConfigsElAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ApigeeApiProductOperationGroupElOperationConfigsElAttributesEl {
    #[doc = "Set the field `name`.\nKey of the attribute."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nValue of the attribute."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeApiProductOperationGroupElOperationConfigsElAttributesEl {
    type O = BlockAssignable<ApigeeApiProductOperationGroupElOperationConfigsElAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductOperationGroupElOperationConfigsElAttributesEl {}
impl BuildApigeeApiProductOperationGroupElOperationConfigsElAttributesEl {
    pub fn build(self) -> ApigeeApiProductOperationGroupElOperationConfigsElAttributesEl {
        ApigeeApiProductOperationGroupElOperationConfigsElAttributesEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductOperationGroupElOperationConfigsElAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductOperationGroupElOperationConfigsElAttributesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductOperationGroupElOperationConfigsElAttributesElRef {
        ApigeeApiProductOperationGroupElOperationConfigsElAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductOperationGroupElOperationConfigsElAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nKey of the attribute."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the attribute."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductOperationGroupElOperationConfigsElOperationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    methods: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl ApigeeApiProductOperationGroupElOperationConfigsElOperationsEl {
    #[doc = "Set the field `methods`.\nMethods refers to the REST verbs, when none specified, all verb types are allowed."]
    pub fn set_methods(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.methods = Some(v.into());
        self
    }
    #[doc = "Set the field `resource`.\nRequired. REST resource path associated with the API proxy or remote service."]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeApiProductOperationGroupElOperationConfigsElOperationsEl {
    type O = BlockAssignable<ApigeeApiProductOperationGroupElOperationConfigsElOperationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductOperationGroupElOperationConfigsElOperationsEl {}
impl BuildApigeeApiProductOperationGroupElOperationConfigsElOperationsEl {
    pub fn build(self) -> ApigeeApiProductOperationGroupElOperationConfigsElOperationsEl {
        ApigeeApiProductOperationGroupElOperationConfigsElOperationsEl {
            methods: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductOperationGroupElOperationConfigsElOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductOperationGroupElOperationConfigsElOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductOperationGroupElOperationConfigsElOperationsElRef {
        ApigeeApiProductOperationGroupElOperationConfigsElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductOperationGroupElOperationConfigsElOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `methods` after provisioning.\nMethods refers to the REST verbs, when none specified, all verb types are allowed."]
    pub fn methods(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.methods", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nRequired. REST resource path associated with the API proxy or remote service."]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductOperationGroupElOperationConfigsElQuotaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_unit: Option<PrimField<String>>,
}
impl ApigeeApiProductOperationGroupElOperationConfigsElQuotaEl {
    #[doc = "Set the field `interval`.\nRequired. Time interval over which the number of request messages is calculated."]
    pub fn set_interval(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interval = Some(v.into());
        self
    }
    #[doc = "Set the field `limit`.\nRequired. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
    pub fn set_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.limit = Some(v.into());
        self
    }
    #[doc = "Set the field `time_unit`.\nTime unit defined for the interval. Valid values include second, minute, hour, day, month or year. If limit and interval are valid, the default value is hour; otherwise, the default is null."]
    pub fn set_time_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_unit = Some(v.into());
        self
    }
}
impl ToListMappable for ApigeeApiProductOperationGroupElOperationConfigsElQuotaEl {
    type O = BlockAssignable<ApigeeApiProductOperationGroupElOperationConfigsElQuotaEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductOperationGroupElOperationConfigsElQuotaEl {}
impl BuildApigeeApiProductOperationGroupElOperationConfigsElQuotaEl {
    pub fn build(self) -> ApigeeApiProductOperationGroupElOperationConfigsElQuotaEl {
        ApigeeApiProductOperationGroupElOperationConfigsElQuotaEl {
            interval: core::default::Default::default(),
            limit: core::default::Default::default(),
            time_unit: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductOperationGroupElOperationConfigsElQuotaElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductOperationGroupElOperationConfigsElQuotaElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductOperationGroupElOperationConfigsElQuotaElRef {
        ApigeeApiProductOperationGroupElOperationConfigsElQuotaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductOperationGroupElOperationConfigsElQuotaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `interval` after provisioning.\nRequired. Time interval over which the number of request messages is calculated."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `limit` after provisioning.\nRequired. Upper limit allowed for the time interval and time unit specified. Requests exceeding this limit will be rejected."]
    pub fn limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }
    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\nTime unit defined for the interval. Valid values include second, minute, hour, day, month or year. If limit and interval are valid, the default value is hour; otherwise, the default is null."]
    pub fn time_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeApiProductOperationGroupElOperationConfigsElDynamic {
    attributes:
        Option<DynamicBlock<ApigeeApiProductOperationGroupElOperationConfigsElAttributesEl>>,
    operations:
        Option<DynamicBlock<ApigeeApiProductOperationGroupElOperationConfigsElOperationsEl>>,
    quota: Option<DynamicBlock<ApigeeApiProductOperationGroupElOperationConfigsElQuotaEl>>,
}
#[derive(Serialize)]
pub struct ApigeeApiProductOperationGroupElOperationConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<ApigeeApiProductOperationGroupElOperationConfigsElAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operations: Option<Vec<ApigeeApiProductOperationGroupElOperationConfigsElOperationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<Vec<ApigeeApiProductOperationGroupElOperationConfigsElQuotaEl>>,
    dynamic: ApigeeApiProductOperationGroupElOperationConfigsElDynamic,
}
impl ApigeeApiProductOperationGroupElOperationConfigsEl {
    #[doc = "Set the field `api_source`.\nRequired. Name of the API proxy or remote service with which the resources, methods, and quota are associated."]
    pub fn set_api_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.api_source = Some(v.into());
        self
    }
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(
        mut self,
        v: impl Into<BlockAssignable<ApigeeApiProductOperationGroupElOperationConfigsElAttributesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.attributes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl Into<BlockAssignable<ApigeeApiProductOperationGroupElOperationConfigsElOperationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `quota`.\n"]
    pub fn set_quota(
        mut self,
        v: impl Into<BlockAssignable<ApigeeApiProductOperationGroupElOperationConfigsElQuotaEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.quota = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.quota = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeApiProductOperationGroupElOperationConfigsEl {
    type O = BlockAssignable<ApigeeApiProductOperationGroupElOperationConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductOperationGroupElOperationConfigsEl {}
impl BuildApigeeApiProductOperationGroupElOperationConfigsEl {
    pub fn build(self) -> ApigeeApiProductOperationGroupElOperationConfigsEl {
        ApigeeApiProductOperationGroupElOperationConfigsEl {
            api_source: core::default::Default::default(),
            attributes: core::default::Default::default(),
            operations: core::default::Default::default(),
            quota: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeApiProductOperationGroupElOperationConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductOperationGroupElOperationConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApigeeApiProductOperationGroupElOperationConfigsElRef {
        ApigeeApiProductOperationGroupElOperationConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductOperationGroupElOperationConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_source` after provisioning.\nRequired. Name of the API proxy or remote service with which the resources, methods, and quota are associated."]
    pub fn api_source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_source", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> ListRef<ApigeeApiProductOperationGroupElOperationConfigsElOperationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> ListRef<ApigeeApiProductOperationGroupElOperationConfigsElQuotaElRef> {
        ListRef::new(self.shared().clone(), format!("{}.quota", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApigeeApiProductOperationGroupElDynamic {
    operation_configs: Option<DynamicBlock<ApigeeApiProductOperationGroupElOperationConfigsEl>>,
}
#[derive(Serialize)]
pub struct ApigeeApiProductOperationGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_config_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_configs: Option<Vec<ApigeeApiProductOperationGroupElOperationConfigsEl>>,
    dynamic: ApigeeApiProductOperationGroupElDynamic,
}
impl ApigeeApiProductOperationGroupEl {
    #[doc = "Set the field `operation_config_type`.\nFlag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values include proxy or remoteservice. Defaults to proxy. Set to proxy when Apigee API proxies are associated with the API product. Set to remoteservice when non-Apigee proxies like Istio-Envoy are associated with the API product. Possible values: [\"proxy\", \"remoteservice\"]"]
    pub fn set_operation_config_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operation_config_type = Some(v.into());
        self
    }
    #[doc = "Set the field `operation_configs`.\n"]
    pub fn set_operation_configs(
        mut self,
        v: impl Into<BlockAssignable<ApigeeApiProductOperationGroupElOperationConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operation_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operation_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApigeeApiProductOperationGroupEl {
    type O = BlockAssignable<ApigeeApiProductOperationGroupEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductOperationGroupEl {}
impl BuildApigeeApiProductOperationGroupEl {
    pub fn build(self) -> ApigeeApiProductOperationGroupEl {
        ApigeeApiProductOperationGroupEl {
            operation_config_type: core::default::Default::default(),
            operation_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApigeeApiProductOperationGroupElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductOperationGroupElRef {
    fn new(shared: StackShared, base: String) -> ApigeeApiProductOperationGroupElRef {
        ApigeeApiProductOperationGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductOperationGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `operation_config_type` after provisioning.\nFlag that specifes whether the configuration is for Apigee API proxy or a remote service. Valid values include proxy or remoteservice. Defaults to proxy. Set to proxy when Apigee API proxies are associated with the API product. Set to remoteservice when non-Apigee proxies like Istio-Envoy are associated with the API product. Possible values: [\"proxy\", \"remoteservice\"]"]
    pub fn operation_config_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.operation_config_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApigeeApiProductTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApigeeApiProductTimeoutsEl {
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
impl ToListMappable for ApigeeApiProductTimeoutsEl {
    type O = BlockAssignable<ApigeeApiProductTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApigeeApiProductTimeoutsEl {}
impl BuildApigeeApiProductTimeoutsEl {
    pub fn build(self) -> ApigeeApiProductTimeoutsEl {
        ApigeeApiProductTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApigeeApiProductTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApigeeApiProductTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeApiProductTimeoutsElRef {
        ApigeeApiProductTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApigeeApiProductTimeoutsElRef {
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
struct ApigeeApiProductDynamic {
    attributes: Option<DynamicBlock<ApigeeApiProductAttributesEl>>,
    graphql_operation_group: Option<DynamicBlock<ApigeeApiProductGraphqlOperationGroupEl>>,
    grpc_operation_group: Option<DynamicBlock<ApigeeApiProductGrpcOperationGroupEl>>,
    operation_group: Option<DynamicBlock<ApigeeApiProductOperationGroupEl>>,
}
