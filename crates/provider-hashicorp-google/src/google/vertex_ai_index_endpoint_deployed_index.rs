use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct VertexAiIndexEndpointDeployedIndexData {
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
    deployed_index_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_access_logging: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    index: PrimField<String>,
    index_endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_resources: Option<Vec<VertexAiIndexEndpointDeployedIndexAutomaticResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dedicated_resources: Option<Vec<VertexAiIndexEndpointDeployedIndexDedicatedResourcesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployed_index_auth_config:
        Option<Vec<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<VertexAiIndexEndpointDeployedIndexTimeoutsEl>,
    dynamic: VertexAiIndexEndpointDeployedIndexDynamic,
}
struct VertexAiIndexEndpointDeployedIndex_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VertexAiIndexEndpointDeployedIndexData>,
}
#[derive(Clone)]
pub struct VertexAiIndexEndpointDeployedIndex(Rc<VertexAiIndexEndpointDeployedIndex_>);
impl VertexAiIndexEndpointDeployedIndex {
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
    #[doc = "Set the field `deployment_group`.\nThe deployment group can be no longer than 64 characters (eg: 'test', 'prod'). If not set, we will use the 'default' deployment group.\nCreating deployment_groups with reserved_ip_ranges is a recommended practice when the peered network has multiple peering ranges. This creates your deployments from predictable IP spaces for easier traffic administration. Also, one deployment_group (except 'default') can only be used with the same reserved_ip_ranges which means if the deployment_group has been used with reserved_ip_ranges: [a, b, c], using it with [a, b] or [d, e] is disallowed. [See the official documentation here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#DeployedIndex.FIELDS.deployment_group).\nNote: we only support up to 5 deployment groups (not including 'default')."]
    pub fn set_deployment_group(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_group = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_access_logging`.\nIf true, private endpoint's access logs are sent to Cloud Logging."]
    pub fn set_enable_access_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_access_logging = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nThe region of the index endpoint deployment. eg us-central1"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `reserved_ip_ranges`.\nA list of reserved ip ranges under the VPC network that can be used for this DeployedIndex.\nIf set, we will deploy the index within the provided ip ranges. Otherwise, the index might be deployed to any ip ranges under the provided VPC network.\n\nThe value should be the name of the address (https://cloud.google.com/compute/docs/reference/rest/v1/addresses) Example: ['vertex-ai-ip-range'].\n\nFor more information about subnets and network IP ranges, please see https://cloud.google.com/vpc/docs/subnets#manually_created_subnet_ip_ranges."]
    pub fn set_reserved_ip_ranges(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().reserved_ip_ranges = Some(v.into());
        self
    }
    #[doc = "Set the field `automatic_resources`.\n"]
    pub fn set_automatic_resources(
        self,
        v: impl Into<BlockAssignable<VertexAiIndexEndpointDeployedIndexAutomaticResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automatic_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automatic_resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dedicated_resources`.\n"]
    pub fn set_dedicated_resources(
        self,
        v: impl Into<BlockAssignable<VertexAiIndexEndpointDeployedIndexDedicatedResourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dedicated_resources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dedicated_resources = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `deployed_index_auth_config`.\n"]
    pub fn set_deployed_index_auth_config(
        self,
        v: impl Into<BlockAssignable<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployed_index_auth_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployed_index_auth_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<VertexAiIndexEndpointDeployedIndexTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `deployed_index_id` after provisioning.\nThe user specified ID of the DeployedIndex. The ID can be up to 128 characters long and must start with a letter and only contain letters, numbers, and underscores. The ID must be unique within the project it is created in."]
    pub fn deployed_index_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployed_index_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_group` after provisioning.\nThe deployment group can be no longer than 64 characters (eg: 'test', 'prod'). If not set, we will use the 'default' deployment group.\nCreating deployment_groups with reserved_ip_ranges is a recommended practice when the peered network has multiple peering ranges. This creates your deployments from predictable IP spaces for easier traffic administration. Also, one deployment_group (except 'default') can only be used with the same reserved_ip_ranges which means if the deployment_group has been used with reserved_ip_ranges: [a, b, c], using it with [a, b] or [d, e] is disallowed. [See the official documentation here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#DeployedIndex.FIELDS.deployment_group).\nNote: we only support up to 5 deployment groups (not including 'default')."]
    pub fn deployment_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_access_logging` after provisioning.\nIf true, private endpoint's access logs are sent to Cloud Logging."]
    pub fn enable_access_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_access_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `index` after provisioning.\nThe name of the Index this is the deployment of."]
    pub fn index(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `index_endpoint` after provisioning.\nIdentifies the index endpoint. Must be in the format\n'projects/{{project}}/locations/{{region}}/indexEndpoints/{{indexEndpoint}}'"]
    pub fn index_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `index_sync_time` after provisioning.\nThe DeployedIndex may depend on various data on its original Index. Additionally when certain changes to the original Index are being done (e.g. when what the Index contains is being changed) the DeployedIndex may be asynchronously updated in the background to reflect these changes. If this timestamp's value is at least the [Index.update_time](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexes#Index.FIELDS.update_time) of the original Index, it means that this DeployedIndex and the original Index are in sync. If this timestamp is older, then to see which updates this DeployedIndex already contains (and which it does not), one must [list](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.operations/list#google.longrunning.Operations.ListOperations) the operations that are running on the original Index. Only the successfully completed Operations with updateTime equal or before this sync time are contained in this DeployedIndex.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples:\u{a0}\"2014-10-02T15:01:23Z\"\u{a0}and\u{a0}\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn index_sync_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_sync_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the DeployedIndex resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_endpoints` after provisioning.\nProvides paths for users to send requests directly to the deployed index services running on Cloud via private services access. This field is populated if [network](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#IndexEndpoint.FIELDS.network) is configured."]
    pub fn private_endpoints(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexPrivateEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the index endpoint deployment. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_ip_ranges` after provisioning.\nA list of reserved ip ranges under the VPC network that can be used for this DeployedIndex.\nIf set, we will deploy the index within the provided ip ranges. Otherwise, the index might be deployed to any ip ranges under the provided VPC network.\n\nThe value should be the name of the address (https://cloud.google.com/compute/docs/reference/rest/v1/addresses) Example: ['vertex-ai-ip-range'].\n\nFor more information about subnets and network IP ranges, please see https://cloud.google.com/vpc/docs/subnets#manually_created_subnet_ip_ranges."]
    pub fn reserved_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reserved_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automatic_resources` after provisioning.\n"]
    pub fn automatic_resources(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexAutomaticResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automatic_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexDedicatedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployed_index_auth_config` after provisioning.\n"]
    pub fn deployed_index_auth_config(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployed_index_auth_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndexEndpointDeployedIndexTimeoutsElRef {
        VertexAiIndexEndpointDeployedIndexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for VertexAiIndexEndpointDeployedIndex {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for VertexAiIndexEndpointDeployedIndex {}
impl ToListMappable for VertexAiIndexEndpointDeployedIndex {
    type O = ListRef<VertexAiIndexEndpointDeployedIndexRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for VertexAiIndexEndpointDeployedIndex_ {
    fn extract_resource_type(&self) -> String {
        "google_vertex_ai_index_endpoint_deployed_index".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildVertexAiIndexEndpointDeployedIndex {
    pub tf_id: String,
    #[doc = "The user specified ID of the DeployedIndex. The ID can be up to 128 characters long and must start with a letter and only contain letters, numbers, and underscores. The ID must be unique within the project it is created in."]
    pub deployed_index_id: PrimField<String>,
    #[doc = "The name of the Index this is the deployment of."]
    pub index: PrimField<String>,
    #[doc = "Identifies the index endpoint. Must be in the format\n'projects/{{project}}/locations/{{region}}/indexEndpoints/{{indexEndpoint}}'"]
    pub index_endpoint: PrimField<String>,
}
impl BuildVertexAiIndexEndpointDeployedIndex {
    pub fn build(self, stack: &mut Stack) -> VertexAiIndexEndpointDeployedIndex {
        let out =
            VertexAiIndexEndpointDeployedIndex(Rc::new(VertexAiIndexEndpointDeployedIndex_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(VertexAiIndexEndpointDeployedIndexData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    deployed_index_id: self.deployed_index_id,
                    deployment_group: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    enable_access_logging: core::default::Default::default(),
                    id: core::default::Default::default(),
                    index: self.index,
                    index_endpoint: self.index_endpoint,
                    region: core::default::Default::default(),
                    reserved_ip_ranges: core::default::Default::default(),
                    automatic_resources: core::default::Default::default(),
                    dedicated_resources: core::default::Default::default(),
                    deployed_index_auth_config: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct VertexAiIndexEndpointDeployedIndexRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointDeployedIndexRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl VertexAiIndexEndpointDeployedIndexRef {
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
    #[doc = "Get a reference to the value of field `deployed_index_id` after provisioning.\nThe user specified ID of the DeployedIndex. The ID can be up to 128 characters long and must start with a letter and only contain letters, numbers, and underscores. The ID must be unique within the project it is created in."]
    pub fn deployed_index_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployed_index_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployment_group` after provisioning.\nThe deployment group can be no longer than 64 characters (eg: 'test', 'prod'). If not set, we will use the 'default' deployment group.\nCreating deployment_groups with reserved_ip_ranges is a recommended practice when the peered network has multiple peering ranges. This creates your deployments from predictable IP spaces for easier traffic administration. Also, one deployment_group (except 'default') can only be used with the same reserved_ip_ranges which means if the deployment_group has been used with reserved_ip_ranges: [a, b, c], using it with [a, b] or [d, e] is disallowed. [See the official documentation here](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#DeployedIndex.FIELDS.deployment_group).\nNote: we only support up to 5 deployment groups (not including 'default')."]
    pub fn deployment_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deployment_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_access_logging` after provisioning.\nIf true, private endpoint's access logs are sent to Cloud Logging."]
    pub fn enable_access_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_access_logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `index` after provisioning.\nThe name of the Index this is the deployment of."]
    pub fn index(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `index_endpoint` after provisioning.\nIdentifies the index endpoint. Must be in the format\n'projects/{{project}}/locations/{{region}}/indexEndpoints/{{indexEndpoint}}'"]
    pub fn index_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `index_sync_time` after provisioning.\nThe DeployedIndex may depend on various data on its original Index. Additionally when certain changes to the original Index are being done (e.g. when what the Index contains is being changed) the DeployedIndex may be asynchronously updated in the background to reflect these changes. If this timestamp's value is at least the [Index.update_time](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexes#Index.FIELDS.update_time) of the original Index, it means that this DeployedIndex and the original Index are in sync. If this timestamp is older, then to see which updates this DeployedIndex already contains (and which it does not), one must [list](https://cloud.google.com/vertex-ai/docs/reference/rest/v1beta1/projects.locations.operations/list#google.longrunning.Operations.ListOperations) the operations that are running on the original Index. Only the successfully completed Operations with updateTime equal or before this sync time are contained in this DeployedIndex.\n\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples:\u{a0}\"2014-10-02T15:01:23Z\"\u{a0}and\u{a0}\"2014-10-02T15:01:23.045123456Z\"."]
    pub fn index_sync_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_sync_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the DeployedIndex resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_endpoints` after provisioning.\nProvides paths for users to send requests directly to the deployed index services running on Cloud via private services access. This field is populated if [network](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.indexEndpoints#IndexEndpoint.FIELDS.network) is configured."]
    pub fn private_endpoints(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexPrivateEndpointsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the index endpoint deployment. eg us-central1"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reserved_ip_ranges` after provisioning.\nA list of reserved ip ranges under the VPC network that can be used for this DeployedIndex.\nIf set, we will deploy the index within the provided ip ranges. Otherwise, the index might be deployed to any ip ranges under the provided VPC network.\n\nThe value should be the name of the address (https://cloud.google.com/compute/docs/reference/rest/v1/addresses) Example: ['vertex-ai-ip-range'].\n\nFor more information about subnets and network IP ranges, please see https://cloud.google.com/vpc/docs/subnets#manually_created_subnet_ip_ranges."]
    pub fn reserved_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reserved_ip_ranges", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `automatic_resources` after provisioning.\n"]
    pub fn automatic_resources(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexAutomaticResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.automatic_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dedicated_resources` after provisioning.\n"]
    pub fn dedicated_resources(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexDedicatedResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_resources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deployed_index_auth_config` after provisioning.\n"]
    pub fn deployed_index_auth_config(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.deployed_index_auth_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndexEndpointDeployedIndexTimeoutsElRef {
        VertexAiIndexEndpointDeployedIndexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    match_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
}
impl VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl {
    #[doc = "Set the field `match_address`.\n"]
    pub fn set_match_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_address = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.project_id = Some(v.into());
        self
    }
}
impl ToListMappable
    for VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl
{
    type O = BlockAssignable<
        VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl {}
impl BuildVertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl {
    pub fn build(
        self,
    ) -> VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl {
        VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl {
            match_address: core::default::Default::default(),
            network: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsElRef {
        VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `match_address` after provisioning.\n"]
    pub fn match_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.match_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndexPrivateEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    match_grpc_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_automated_endpoints: Option<
        ListField<VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_attachment: Option<PrimField<String>>,
}
impl VertexAiIndexEndpointDeployedIndexPrivateEndpointsEl {
    #[doc = "Set the field `match_grpc_address`.\n"]
    pub fn set_match_grpc_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_grpc_address = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_automated_endpoints`.\n"]
    pub fn set_psc_automated_endpoints(
        mut self,
        v: impl Into<
            ListField<VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsEl>,
        >,
    ) -> Self {
        self.psc_automated_endpoints = Some(v.into());
        self
    }
    #[doc = "Set the field `service_attachment`.\n"]
    pub fn set_service_attachment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_attachment = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiIndexEndpointDeployedIndexPrivateEndpointsEl {
    type O = BlockAssignable<VertexAiIndexEndpointDeployedIndexPrivateEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointDeployedIndexPrivateEndpointsEl {}
impl BuildVertexAiIndexEndpointDeployedIndexPrivateEndpointsEl {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndexPrivateEndpointsEl {
        VertexAiIndexEndpointDeployedIndexPrivateEndpointsEl {
            match_grpc_address: core::default::Default::default(),
            psc_automated_endpoints: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointDeployedIndexPrivateEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointDeployedIndexPrivateEndpointsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexEndpointDeployedIndexPrivateEndpointsElRef {
        VertexAiIndexEndpointDeployedIndexPrivateEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointDeployedIndexPrivateEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `match_grpc_address` after provisioning.\n"]
    pub fn match_grpc_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.match_grpc_address", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `psc_automated_endpoints` after provisioning.\n"]
    pub fn psc_automated_endpoints(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexPrivateEndpointsElPscAutomatedEndpointsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_automated_endpoints", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\n"]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_attachment", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndexAutomaticResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_replica_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_replica_count: Option<PrimField<f64>>,
}
impl VertexAiIndexEndpointDeployedIndexAutomaticResourcesEl {
    #[doc = "Set the field `max_replica_count`.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount. The max allowed replica count is 1000.\n\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, a no upper bound for scaling under heavy traffic will be assume, though Vertex AI may be unable to scale beyond certain replica number."]
    pub fn set_max_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `min_replica_count`.\nThe minimum number of replicas this DeployedModel will be always deployed on. If minReplicaCount is not set, the default value is 2 (we don't provide SLA when minReplicaCount=1).\n\nIf traffic against it increases, it may dynamically be deployed onto more replicas up to [maxReplicaCount](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/AutomaticResources#FIELDS.max_replica_count), and as traffic decreases, some of these extra replicas may be freed. If the requested value is too large, the deployment will error."]
    pub fn set_min_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_replica_count = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiIndexEndpointDeployedIndexAutomaticResourcesEl {
    type O = BlockAssignable<VertexAiIndexEndpointDeployedIndexAutomaticResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointDeployedIndexAutomaticResourcesEl {}
impl BuildVertexAiIndexEndpointDeployedIndexAutomaticResourcesEl {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndexAutomaticResourcesEl {
        VertexAiIndexEndpointDeployedIndexAutomaticResourcesEl {
            max_replica_count: core::default::Default::default(),
            min_replica_count: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointDeployedIndexAutomaticResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointDeployedIndexAutomaticResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexEndpointDeployedIndexAutomaticResourcesElRef {
        VertexAiIndexEndpointDeployedIndexAutomaticResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointDeployedIndexAutomaticResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_replica_count` after provisioning.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount. The max allowed replica count is 1000.\n\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If the requested value is too large, the deployment will error, but if deployment succeeds then the ability to scale the model to that many replicas is guaranteed (barring service outages). If traffic against the DeployedModel increases beyond what its replicas at maximum may handle, a portion of the traffic will be dropped. If this value is not provided, a no upper bound for scaling under heavy traffic will be assume, though Vertex AI may be unable to scale beyond certain replica number."]
    pub fn max_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_replica_count` after provisioning.\nThe minimum number of replicas this DeployedModel will be always deployed on. If minReplicaCount is not set, the default value is 2 (we don't provide SLA when minReplicaCount=1).\n\nIf traffic against it increases, it may dynamically be deployed onto more replicas up to [maxReplicaCount](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/AutomaticResources#FIELDS.max_replica_count), and as traffic decreases, some of these extra replicas may be freed. If the requested value is too large, the deployment will error."]
    pub fn min_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_replica_count", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
}
impl VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl {
    #[doc = "Set the field `machine_type`.\nThe type of the machine.\n\nSee the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom training](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types).\n\nFor [DeployedModel](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.endpoints#DeployedModel) this field is optional, and the default value is n1-standard-2. For [BatchPredictionJob](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.batchPredictionJobs#BatchPredictionJob) or as part of [WorkerPoolSpec](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/CustomJobSpec#WorkerPoolSpec) this field is required."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl {
    type O = BlockAssignable<VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl {}
impl BuildVertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl {
        VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl {
            machine_type: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecElRef {
        VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe type of the machine.\n\nSee the [list of machine types supported for prediction](https://cloud.google.com/vertex-ai/docs/predictions/configure-compute#machine-types)\n\nSee the [list of machine types supported for custom training](https://cloud.google.com/vertex-ai/docs/training/configure-compute#machine-types).\n\nFor [DeployedModel](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.endpoints#DeployedModel) this field is optional, and the default value is n1-standard-2. For [BatchPredictionJob](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.batchPredictionJobs#BatchPredictionJob) or as part of [WorkerPoolSpec](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/CustomJobSpec#WorkerPoolSpec) this field is required."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiIndexEndpointDeployedIndexDedicatedResourcesElDynamic {
    machine_spec:
        Option<DynamicBlock<VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl>>,
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndexDedicatedResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_replica_count: Option<PrimField<f64>>,
    min_replica_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_spec: Option<Vec<VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl>>,
    dynamic: VertexAiIndexEndpointDeployedIndexDedicatedResourcesElDynamic,
}
impl VertexAiIndexEndpointDeployedIndexDedicatedResourcesEl {
    #[doc = "Set the field `max_replica_count`.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount"]
    pub fn set_max_replica_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_replica_count = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        mut self,
        v: impl Into<
            BlockAssignable<VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.machine_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_spec = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiIndexEndpointDeployedIndexDedicatedResourcesEl {
    type O = BlockAssignable<VertexAiIndexEndpointDeployedIndexDedicatedResourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointDeployedIndexDedicatedResourcesEl {
    #[doc = "The minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1."]
    pub min_replica_count: PrimField<f64>,
}
impl BuildVertexAiIndexEndpointDeployedIndexDedicatedResourcesEl {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndexDedicatedResourcesEl {
        VertexAiIndexEndpointDeployedIndexDedicatedResourcesEl {
            max_replica_count: core::default::Default::default(),
            min_replica_count: self.min_replica_count,
            machine_spec: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointDeployedIndexDedicatedResourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointDeployedIndexDedicatedResourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexEndpointDeployedIndexDedicatedResourcesElRef {
        VertexAiIndexEndpointDeployedIndexDedicatedResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointDeployedIndexDedicatedResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_replica_count` after provisioning.\nThe maximum number of replicas this DeployedModel may be deployed on when the traffic against it increases. If maxReplicaCount is not set, the default value is minReplicaCount"]
    pub fn max_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_replica_count` after provisioning.\nThe minimum number of machine replicas this DeployedModel will be always deployed on. This value must be greater than or equal to 1."]
    pub fn min_replica_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_replica_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexDedicatedResourcesElMachineSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.base))
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_issuers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audiences: Option<ListField<PrimField<String>>>,
}
impl VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl {
    #[doc = "Set the field `allowed_issuers`.\nA list of allowed JWT issuers. Each entry must be a valid Google service account, in the following format: service-account-name@project-id.iam.gserviceaccount.com"]
    pub fn set_allowed_issuers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_issuers = Some(v.into());
        self
    }
    #[doc = "Set the field `audiences`.\nThe list of JWT audiences. that are allowed to access. A JWT containing any of these audiences will be accepted."]
    pub fn set_audiences(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.audiences = Some(v.into());
        self
    }
}
impl ToListMappable for VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl {
    type O =
        BlockAssignable<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl {}
impl BuildVertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl {
    pub fn build(
        self,
    ) -> VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl {
        VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl {
            allowed_issuers: core::default::Default::default(),
            audiences: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderElRef {
        VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_issuers` after provisioning.\nA list of allowed JWT issuers. Each entry must be a valid Google service account, in the following format: service-account-name@project-id.iam.gserviceaccount.com"]
    pub fn allowed_issuers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_issuers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `audiences` after provisioning.\nThe list of JWT audiences. that are allowed to access. A JWT containing any of these audiences will be accepted."]
    pub fn audiences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.audiences", self.base))
    }
}
#[derive(Serialize, Default)]
struct VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElDynamic {
    auth_provider: Option<
        DynamicBlock<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl>,
    >,
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_provider:
        Option<Vec<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl>>,
    dynamic: VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElDynamic,
}
impl VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl {
    #[doc = "Set the field `auth_provider`.\n"]
    pub fn set_auth_provider(
        mut self,
        v: impl Into<
            BlockAssignable<
                VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.auth_provider = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_provider = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl {
    type O = BlockAssignable<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl {}
impl BuildVertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl {
        VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl {
            auth_provider: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElRef {
        VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auth_provider` after provisioning.\n"]
    pub fn auth_provider(
        &self,
    ) -> ListRef<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigElAuthProviderElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auth_provider", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct VertexAiIndexEndpointDeployedIndexTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl VertexAiIndexEndpointDeployedIndexTimeoutsEl {
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
impl ToListMappable for VertexAiIndexEndpointDeployedIndexTimeoutsEl {
    type O = BlockAssignable<VertexAiIndexEndpointDeployedIndexTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildVertexAiIndexEndpointDeployedIndexTimeoutsEl {}
impl BuildVertexAiIndexEndpointDeployedIndexTimeoutsEl {
    pub fn build(self) -> VertexAiIndexEndpointDeployedIndexTimeoutsEl {
        VertexAiIndexEndpointDeployedIndexTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct VertexAiIndexEndpointDeployedIndexTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for VertexAiIndexEndpointDeployedIndexTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> VertexAiIndexEndpointDeployedIndexTimeoutsElRef {
        VertexAiIndexEndpointDeployedIndexTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl VertexAiIndexEndpointDeployedIndexTimeoutsElRef {
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
struct VertexAiIndexEndpointDeployedIndexDynamic {
    automatic_resources:
        Option<DynamicBlock<VertexAiIndexEndpointDeployedIndexAutomaticResourcesEl>>,
    dedicated_resources:
        Option<DynamicBlock<VertexAiIndexEndpointDeployedIndexDedicatedResourcesEl>>,
    deployed_index_auth_config:
        Option<DynamicBlock<VertexAiIndexEndpointDeployedIndexDeployedIndexAuthConfigEl>>,
}
