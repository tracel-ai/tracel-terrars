use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NetworkConnectivityServiceConnectionPolicyData {
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
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service_class: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_config: Option<Vec<NetworkConnectivityServiceConnectionPolicyPscConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NetworkConnectivityServiceConnectionPolicyTimeoutsEl>,
    dynamic: NetworkConnectivityServiceConnectionPolicyDynamic,
}
struct NetworkConnectivityServiceConnectionPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkConnectivityServiceConnectionPolicyData>,
}
#[derive(Clone)]
pub struct NetworkConnectivityServiceConnectionPolicy(
    Rc<NetworkConnectivityServiceConnectionPolicy_>,
);
impl NetworkConnectivityServiceConnectionPolicy {
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
    #[doc = "Set the field `description`.\nFree-text description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_config`.\n"]
    pub fn set_psc_config(
        self,
        v: impl Into<BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<NetworkConnectivityServiceConnectionPolicyTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nFree-text description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `infrastructure` after provisioning.\nThe type of underlying resources used to create the connection."]
    pub fn infrastructure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.infrastructure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the ServiceConnectionPolicy."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connections` after provisioning.\nInformation about each Private Service Connect connection."]
    pub fn psc_connections(
        &self,
    ) -> ListRef<NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_class` after provisioning.\nThe service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.\nIt is provided by the Service Producer. Google services have a prefix of gcp. For example, google-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx.\n\nFor a list of supported services, see [Supported Services](https://docs.cloud.google.com/vpc/docs/about-service-connectivity-automation#supported-services)."]
    pub fn service_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> ListRef<NetworkConnectivityServiceConnectionPolicyPscConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
        NetworkConnectivityServiceConnectionPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for NetworkConnectivityServiceConnectionPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NetworkConnectivityServiceConnectionPolicy {}
impl ToListMappable for NetworkConnectivityServiceConnectionPolicy {
    type O = ListRef<NetworkConnectivityServiceConnectionPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NetworkConnectivityServiceConnectionPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_network_connectivity_service_connection_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNetworkConnectivityServiceConnectionPolicy {
    pub tf_id: String,
    #[doc = "The location of the ServiceConnectionPolicy."]
    pub location: PrimField<String>,
    #[doc = "The name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names"]
    pub name: PrimField<String>,
    #[doc = "The resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}."]
    pub network: PrimField<String>,
    #[doc = "The service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.\nIt is provided by the Service Producer. Google services have a prefix of gcp. For example, google-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx.\n\nFor a list of supported services, see [Supported Services](https://docs.cloud.google.com/vpc/docs/about-service-connectivity-automation#supported-services)."]
    pub service_class: PrimField<String>,
}
impl BuildNetworkConnectivityServiceConnectionPolicy {
    pub fn build(self, stack: &mut Stack) -> NetworkConnectivityServiceConnectionPolicy {
        let out = NetworkConnectivityServiceConnectionPolicy(Rc::new(
            NetworkConnectivityServiceConnectionPolicy_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(NetworkConnectivityServiceConnectionPolicyData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    labels: core::default::Default::default(),
                    location: self.location,
                    name: self.name,
                    network: self.network,
                    project: core::default::Default::default(),
                    service_class: self.service_class,
                    psc_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NetworkConnectivityServiceConnectionPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityServiceConnectionPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NetworkConnectivityServiceConnectionPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp when the resource was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nFree-text description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag is computed by the server, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `infrastructure` after provisioning.\nThe type of underlying resources used to create the connection."]
    pub fn infrastructure(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.infrastructure", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the ServiceConnectionPolicy."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of a ServiceConnectionPolicy. Format: projects/{project}/locations/{location}/serviceConnectionPolicies/{service_connection_policy} See: https://google.aip.dev/122#fields-representing-resource-names"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe resource path of the consumer network. Example: - projects/{projectNumOrId}/global/networks/{resourceId}."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connections` after provisioning.\nInformation about each Private Service Connect connection."]
    pub fn psc_connections(
        &self,
    ) -> ListRef<NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_connections", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_class` after provisioning.\nThe service class identifier for which this ServiceConnectionPolicy is for. The service class identifier is a unique, symbolic representation of a ServiceClass.\nIt is provided by the Service Producer. Google services have a prefix of gcp. For example, google-cloud-sql. 3rd party services do not. For example, test-service-a3dfcx.\n\nFor a list of supported services, see [Supported Services](https://docs.cloud.google.com/vpc/docs/about-service-connectivity-automation#supported-services)."]
    pub fn service_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp when the resource was updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_config` after provisioning.\n"]
    pub fn psc_config(&self) -> ListRef<NetworkConnectivityServiceConnectionPolicyPscConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
        NetworkConnectivityServiceConnectionPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ListField<RecField<PrimField<String>>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(mut self, v: impl Into<ListField<RecField<PrimField<String>>>>) -> Self {
        self.details = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
    type O = BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {}
impl BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> ListRef<RecRef<PrimExpr<String>>> {
        ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
}
impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
    type O = BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {}
impl BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl {
            domain: core::default::Default::default(),
            metadata: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_forwarding_rule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    consumer_target_project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ListField<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_info:
        Option<ListField<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gce_operation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_connection_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}
impl NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
    #[doc = "Set the field `consumer_address`.\n"]
    pub fn set_consumer_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_address = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_forwarding_rule`.\n"]
    pub fn set_consumer_forwarding_rule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_forwarding_rule = Some(v.into());
        self
    }
    #[doc = "Set the field `consumer_target_project`.\n"]
    pub fn set_consumer_target_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.consumer_target_project = Some(v.into());
        self
    }
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl Into<ListField<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorEl>>,
    ) -> Self {
        self.error = Some(v.into());
        self
    }
    #[doc = "Set the field `error_info`.\n"]
    pub fn set_error_info(
        mut self,
        v: impl Into<ListField<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoEl>>,
    ) -> Self {
        self.error_info = Some(v.into());
        self
    }
    #[doc = "Set the field `error_type`.\n"]
    pub fn set_error_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.error_type = Some(v.into());
        self
    }
    #[doc = "Set the field `gce_operation`.\n"]
    pub fn set_gce_operation(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gce_operation = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_connection_id`.\n"]
    pub fn set_psc_connection_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.psc_connection_id = Some(v.into());
        self
    }
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
    type O = BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConnectionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsEl {}
impl BuildNetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsEl {
            consumer_address: core::default::Default::default(),
            consumer_forwarding_rule: core::default::Default::default(),
            consumer_target_project: core::default::Default::default(),
            error: core::default::Default::default(),
            error_info: core::default::Default::default(),
            error_type: core::default::Default::default(),
            gce_operation: core::default::Default::default(),
            psc_connection_id: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
        NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityServiceConnectionPolicyPscConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `consumer_address` after provisioning.\n"]
    pub fn consumer_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_forwarding_rule` after provisioning.\n"]
    pub fn consumer_forwarding_rule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_forwarding_rule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `consumer_target_project` after provisioning.\n"]
    pub fn consumer_target_project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.consumer_target_project", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(
        &self,
    ) -> ListRef<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }
    #[doc = "Get a reference to the value of field `error_info` after provisioning.\n"]
    pub fn error_info(
        &self,
    ) -> ListRef<NetworkConnectivityServiceConnectionPolicyPscConnectionsElErrorInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error_info", self.base))
    }
    #[doc = "Get a reference to the value of field `error_type` after provisioning.\n"]
    pub fn error_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_type", self.base))
    }
    #[doc = "Get a reference to the value of field `gce_operation` after provisioning.\n"]
    pub fn gce_operation(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gce_operation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\n"]
    pub fn psc_connection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_connection_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyPscConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_google_producers_resource_hierarchy_level: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    producer_instance_location: Option<PrimField<String>>,
    subnetworks: ListField<PrimField<String>>,
}
impl NetworkConnectivityServiceConnectionPolicyPscConfigEl {
    #[doc = "Set the field `allowed_google_producers_resource_hierarchy_level`.\nList of Projects, Folders, or Organizations from where the Producer instance can be within. For example,\na network administrator can provide both 'organizations/foo' and 'projects/bar' as\nallowed_google_producers_resource_hierarchy_levels. This allowlists this network to connect with any Producer\ninstance within the 'foo' organization or the 'bar' project. By default,\nallowedGoogleProducersResourceHierarchyLevel is empty. The format for each\nallowedGoogleProducersResourceHierarchyLevel is / where is one of 'projects', 'folders', or 'organizations'\nand is either the ID or the number of the resource type. Format for each\nallowedGoogleProducersResourceHierarchyLevel value: 'projects/' or 'folders/' or 'organizations/' Eg.\n[projects/my-project-id, projects/567, folders/891, organizations/123]"]
    pub fn set_allowed_google_producers_resource_hierarchy_level(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_google_producers_resource_hierarchy_level = Some(v.into());
        self
    }
    #[doc = "Set the field `limit`.\nMax number of PSC connections for this policy."]
    pub fn set_limit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.limit = Some(v.into());
        self
    }
    #[doc = "Set the field `producer_instance_location`.\nProducerInstanceLocation is used to specify which authorization mechanism to use to determine which projects\nthe Producer instance can be within. Possible values: [\"PRODUCER_INSTANCE_LOCATION_UNSPECIFIED\", \"CUSTOM_RESOURCE_HIERARCHY_LEVELS\"]"]
    pub fn set_producer_instance_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.producer_instance_location = Some(v.into());
        self
    }
}
impl ToListMappable for NetworkConnectivityServiceConnectionPolicyPscConfigEl {
    type O = BlockAssignable<NetworkConnectivityServiceConnectionPolicyPscConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityServiceConnectionPolicyPscConfigEl {
    #[doc = "IDs of the subnetworks or fully qualified identifiers for the subnetworks"]
    pub subnetworks: ListField<PrimField<String>>,
}
impl BuildNetworkConnectivityServiceConnectionPolicyPscConfigEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyPscConfigEl {
        NetworkConnectivityServiceConnectionPolicyPscConfigEl {
            allowed_google_producers_resource_hierarchy_level: core::default::Default::default(),
            limit: core::default::Default::default(),
            producer_instance_location: core::default::Default::default(),
            subnetworks: self.subnetworks,
        }
    }
}
pub struct NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
        NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityServiceConnectionPolicyPscConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_google_producers_resource_hierarchy_level` after provisioning.\nList of Projects, Folders, or Organizations from where the Producer instance can be within. For example,\na network administrator can provide both 'organizations/foo' and 'projects/bar' as\nallowed_google_producers_resource_hierarchy_levels. This allowlists this network to connect with any Producer\ninstance within the 'foo' organization or the 'bar' project. By default,\nallowedGoogleProducersResourceHierarchyLevel is empty. The format for each\nallowedGoogleProducersResourceHierarchyLevel is / where is one of 'projects', 'folders', or 'organizations'\nand is either the ID or the number of the resource type. Format for each\nallowedGoogleProducersResourceHierarchyLevel value: 'projects/' or 'folders/' or 'organizations/' Eg.\n[projects/my-project-id, projects/567, folders/891, organizations/123]"]
    pub fn allowed_google_producers_resource_hierarchy_level(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.allowed_google_producers_resource_hierarchy_level",
                self.base
            ),
        )
    }
    #[doc = "Get a reference to the value of field `limit` after provisioning.\nMax number of PSC connections for this policy."]
    pub fn limit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }
    #[doc = "Get a reference to the value of field `producer_instance_location` after provisioning.\nProducerInstanceLocation is used to specify which authorization mechanism to use to determine which projects\nthe Producer instance can be within. Possible values: [\"PRODUCER_INSTANCE_LOCATION_UNSPECIFIED\", \"CUSTOM_RESOURCE_HIERARCHY_LEVELS\"]"]
    pub fn producer_instance_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.producer_instance_location", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnetworks` after provisioning.\nIDs of the subnetworks or fully qualified identifiers for the subnetworks"]
    pub fn subnetworks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.subnetworks", self.base))
    }
}
#[derive(Serialize)]
pub struct NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
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
impl ToListMappable for NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
    type O = BlockAssignable<NetworkConnectivityServiceConnectionPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNetworkConnectivityServiceConnectionPolicyTimeoutsEl {}
impl BuildNetworkConnectivityServiceConnectionPolicyTimeoutsEl {
    pub fn build(self) -> NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
        NetworkConnectivityServiceConnectionPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
        NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NetworkConnectivityServiceConnectionPolicyTimeoutsElRef {
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
struct NetworkConnectivityServiceConnectionPolicyDynamic {
    psc_config: Option<DynamicBlock<NetworkConnectivityServiceConnectionPolicyPscConfigEl>>,
}
