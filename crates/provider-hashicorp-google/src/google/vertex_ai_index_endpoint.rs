use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiIndexEndpointData {
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
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_endpoint_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_spec: Option<Vec<VertexAiIndexEndpointEncryptionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_service_connect_config: Option<Vec<VertexAiIndexEndpointPrivateServiceConnectConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiIndexEndpointTimeoutsEl>,
    dynamic: VertexAiIndexEndpointDynamic,
}
struct VertexAiIndexEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiIndexEndpointData>,
}
#[derive(Clone)]
pub struct VertexAiIndexEndpoint(Rc<VertexAiIndexEndpoint_>);
impl VertexAiIndexEndpoint {
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
    #[doc = "Set the field `description`.\nThe description of the Index."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the index endpoint should be peered.\nPrivate services access must already be configured for the network. If left unspecified, the index endpoint is not peered with any network.\n[Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): 'projects/{project}/global/networks/{network}'.\nWhere '{project}' is a project number, as in '12345', and '{network}' is network name."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `public_endpoint_enabled`.\nIf true, the deployed index will be accessible through public endpoint."]
    pub fn set_public_endpoint_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_endpoint_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the index endpoint. eg us-central1"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl Into<BlockAssignable<VertexAiIndexEndpointEncryptionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `private_service_connect_config`.\n"]
    pub fn set_private_service_connect_config(
        self,
        v: impl Into<BlockAssignable<VertexAiIndexEndpointPrivateServiceConnectConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_service_connect_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .private_service_connect_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VertexAiIndexEndpointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Index."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Index."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the index endpoint should be peered.\nPrivate services access must already be configured for the network. If left unspecified, the index endpoint is not peered with any network.\n[Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): 'projects/{project}/global/networks/{network}'.\nWhere '{project}' is a project number, as in '12345', and '{network}' is network name."]
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
    #[doc = "Get a reference to the value of field `public_endpoint_domain_name` after provisioning.\nIf publicEndpointEnabled is true, this field will be populated with the domain name to use for this index endpoint."]
    pub fn public_endpoint_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_endpoint_domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_endpoint_enabled` after provisioning.\nIf true, the deployed index will be accessible through public endpoint."]
    pub fn public_endpoint_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_endpoint_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the index endpoint. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiIndexEndpointEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(
        &self,
    ) -> ListRef<VertexAiIndexEndpointPrivateServiceConnectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_service_connect_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndexEndpointTimeoutsElRef {
        VertexAiIndexEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiIndexEndpoint {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiIndexEndpoint {}
impl ToListMappable for VertexAiIndexEndpoint {
    type O = ListRef<VertexAiIndexEndpointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiIndexEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_index_endpoint".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiIndexEndpoint {
    pub tf_id: String,
    #[doc = "The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub display_name: PrimField<String>,
}
impl BuildVertexAiIndexEndpoint {
    pub fn build(self, stack: &mut Stack) -> VertexAiIndexEndpoint {
        let out = VertexAiIndexEndpoint(Rc::new(VertexAiIndexEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VertexAiIndexEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                network: core::default::Default::default(),
                project: core::default::Default::default(),
                public_endpoint_enabled: core::default::Default::default(),
                region: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                private_service_connect_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiIndexEndpointRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiIndexEndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Index."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Index."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe full name of the Google Compute Engine [network](https://cloud.google.com//compute/docs/networks-and-firewalls#networks) to which the index endpoint should be peered.\nPrivate services access must already be configured for the network. If left unspecified, the index endpoint is not peered with any network.\n[Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/insert): 'projects/{project}/global/networks/{network}'.\nWhere '{project}' is a project number, as in '12345', and '{network}' is network name."]
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
    #[doc = "Get a reference to the value of field `public_endpoint_domain_name` after provisioning.\nIf publicEndpointEnabled is true, this field will be populated with the domain name to use for this index endpoint."]
    pub fn public_endpoint_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_endpoint_domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_endpoint_enabled` after provisioning.\nIf true, the deployed index will be accessible through public endpoint."]
    pub fn public_endpoint_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_endpoint_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the index endpoint. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<VertexAiIndexEndpointEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(
        &self,
    ) -> ListRef<VertexAiIndexEndpointPrivateServiceConnectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_service_connect_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndexEndpointTimeoutsElRef {
        VertexAiIndexEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointEncryptionSpecEl {
    kms_key_name: PrimField<String>,
}
impl VertexAiIndexEndpointEncryptionSpecEl {}
impl ToListMappable for VertexAiIndexEndpointEncryptionSpecEl {
    type O = BlockAssignable<VertexAiIndexEndpointEncryptionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointEncryptionSpecEl {
    #[doc = "Required. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub kms_key_name: PrimField<String>,
}
impl BuildVertexAiIndexEndpointEncryptionSpecEl {
    pub fn build(self) -> VertexAiIndexEndpointEncryptionSpecEl {
        VertexAiIndexEndpointEncryptionSpecEl {
            kms_key_name: self.kms_key_name,
        }
    }
}
pub struct VertexAiIndexEndpointEncryptionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointEncryptionSpecElRef {
    fn new(shared: StackShared, base: String) -> VertexAiIndexEndpointEncryptionSpecElRef {
        VertexAiIndexEndpointEncryptionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointEncryptionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nRequired. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl {
    network: PrimField<String>,
    project_id: PrimField<String>,
}
impl VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl {}
impl ToListMappable for VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl {
    type O =
        BlockAssignable<VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl {
    #[doc = "The full name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/get): projects/{project}/global/networks/{network}."]
    pub network: PrimField<String>,
    #[doc = "Project id used to create forwarding rule."]
    pub project_id: PrimField<String>,
}
impl BuildVertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl {
    pub fn build(self) -> VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl {
        VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl {
            network: self.network,
            project_id: self.project_id,
        }
    }
}
pub struct VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsElRef {
        VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe full name of the Google Compute Engine [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks). [Format](https://cloud.google.com/compute/docs/reference/rest/v1/networks/get): projects/{project}/global/networks/{network}."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nProject id used to create forwarding rule."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiIndexEndpointPrivateServiceConnectConfigElDynamic {
    psc_automation_configs: Option<
        DynamicBlock<VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl>,
    >,
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointPrivateServiceConnectConfigEl {
    enable_private_service_connect: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_allowlist: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_automation_configs:
        Option<Vec<VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl>>,
    dynamic: VertexAiIndexEndpointPrivateServiceConnectConfigElDynamic,
}
impl VertexAiIndexEndpointPrivateServiceConnectConfigEl {
    #[doc = "Set the field `project_allowlist`.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn set_project_allowlist(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.project_allowlist = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_automation_configs`.\n"]
    pub fn set_psc_automation_configs(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.psc_automation_configs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_automation_configs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiIndexEndpointPrivateServiceConnectConfigEl {
    type O = BlockAssignable<VertexAiIndexEndpointPrivateServiceConnectConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointPrivateServiceConnectConfigEl {
    #[doc = "If set to true, the IndexEndpoint is created without private service access."]
    pub enable_private_service_connect: PrimField<bool>,
}
impl BuildVertexAiIndexEndpointPrivateServiceConnectConfigEl {
    pub fn build(self) -> VertexAiIndexEndpointPrivateServiceConnectConfigEl {
        VertexAiIndexEndpointPrivateServiceConnectConfigEl {
            enable_private_service_connect: self.enable_private_service_connect,
            project_allowlist: core::default::Default::default(),
            psc_automation_configs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointPrivateServiceConnectConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointPrivateServiceConnectConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexEndpointPrivateServiceConnectConfigElRef {
        VertexAiIndexEndpointPrivateServiceConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointPrivateServiceConnectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_private_service_connect` after provisioning.\nIf set to true, the IndexEndpoint is created without private service access."]
    pub fn enable_private_service_connect(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_service_connect", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `project_allowlist` after provisioning.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn project_allowlist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.project_allowlist", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_automation_configs` after provisioning.\n"]
    pub fn psc_automation_configs(
        &self,
    ) -> ListRef<VertexAiIndexEndpointPrivateServiceConnectConfigElPscAutomationConfigsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_automation_configs", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VertexAiIndexEndpointTimeoutsEl {
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
impl ToListMappable for VertexAiIndexEndpointTimeoutsEl {
    type O = BlockAssignable<VertexAiIndexEndpointTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointTimeoutsEl {}
impl BuildVertexAiIndexEndpointTimeoutsEl {
    pub fn build(self) -> VertexAiIndexEndpointTimeoutsEl {
        VertexAiIndexEndpointTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiIndexEndpointTimeoutsElRef {
        VertexAiIndexEndpointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointTimeoutsElRef {
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
struct VertexAiIndexEndpointDynamic {
    encryption_spec: Option<DynamicBlock<VertexAiIndexEndpointEncryptionSpecEl>>,
    private_service_connect_config:
        Option<DynamicBlock<VertexAiIndexEndpointPrivateServiceConnectConfigEl>>,
}
