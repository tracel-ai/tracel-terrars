use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudbuildWorkerPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_config: Option<Vec<CloudbuildWorkerPoolNetworkConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_service_connect: Option<Vec<CloudbuildWorkerPoolPrivateServiceConnectEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudbuildWorkerPoolTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_config: Option<Vec<CloudbuildWorkerPoolWorkerConfigEl>>,
    dynamic: CloudbuildWorkerPoolDynamic,
}
struct CloudbuildWorkerPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudbuildWorkerPoolData>,
}
#[derive(Clone)]
pub struct CloudbuildWorkerPool(Rc<CloudbuildWorkerPool_>);
impl CloudbuildWorkerPool {
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
    #[doc = "Set the field `annotations`.\nUser specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn set_annotations(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nA user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl Into<BlockAssignable<CloudbuildWorkerPoolNetworkConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `private_service_connect`.\n"]
    pub fn set_private_service_connect(
        self,
        v: impl Into<BlockAssignable<CloudbuildWorkerPoolPrivateServiceConnectEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_service_connect = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_service_connect = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudbuildWorkerPoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_config`.\n"]
    pub fn set_worker_config(
        self,
        v: impl Into<BlockAssignable<CloudbuildWorkerPoolWorkerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().worker_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.worker_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the request to create the `WorkerPool` was received."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. Time at which the request to delete the `WorkerPool` was received."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser-defined name of the `WorkerPool`."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. `WorkerPool` state. Possible values: STATE_UNSPECIFIED, PENDING, APPROVED, REJECTED, CANCELLED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A unique identifier for the `WorkerPool`."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the request to update the `WorkerPool` was received."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<CloudbuildWorkerPoolNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_service_connect` after provisioning.\n"]
    pub fn private_service_connect(
        &self,
    ) -> ListRef<CloudbuildWorkerPoolPrivateServiceConnectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_service_connect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudbuildWorkerPoolTimeoutsElRef {
        CloudbuildWorkerPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(&self) -> ListRef<CloudbuildWorkerPoolWorkerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.worker_config", self.extract_ref()),
        )
    }
}
impl Referable for CloudbuildWorkerPool {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudbuildWorkerPool {}
impl ToListMappable for CloudbuildWorkerPool {
    type O = ListRef<CloudbuildWorkerPoolRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudbuildWorkerPool_ {
    fn extract_resource_type(&self) -> String {
        "google_cloudbuild_worker_pool".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudbuildWorkerPool {
    pub tf_id: String,
    #[doc = "The location for the resource"]
    pub location: PrimField<String>,
    #[doc = "User-defined name of the `WorkerPool`."]
    pub name: PrimField<String>,
}
impl BuildCloudbuildWorkerPool {
    pub fn build(self, stack: &mut Stack) -> CloudbuildWorkerPool {
        let out = CloudbuildWorkerPool(Rc::new(CloudbuildWorkerPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudbuildWorkerPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                network_config: core::default::Default::default(),
                private_service_connect: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                worker_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudbuildWorkerPoolRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildWorkerPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudbuildWorkerPoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nUser specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field `effective_annotations` for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time at which the request to create the `WorkerPool` was received."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. Time at which the request to delete the `WorkerPool` was received."]
    pub fn delete_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nA user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nUser-defined name of the `WorkerPool`."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. `WorkerPool` state. Possible values: STATE_UNSPECIFIED, PENDING, APPROVED, REJECTED, CANCELLED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A unique identifier for the `WorkerPool`."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time at which the request to update the `WorkerPool` was received."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> ListRef<CloudbuildWorkerPoolNetworkConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_service_connect` after provisioning.\n"]
    pub fn private_service_connect(
        &self,
    ) -> ListRef<CloudbuildWorkerPoolPrivateServiceConnectElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.private_service_connect", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudbuildWorkerPoolTimeoutsElRef {
        CloudbuildWorkerPoolTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(&self) -> ListRef<CloudbuildWorkerPoolWorkerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.worker_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildWorkerPoolNetworkConfigEl {
    peered_network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    peered_network_ip_range: Option<PrimField<String>>,
}
impl CloudbuildWorkerPoolNetworkConfigEl {
    #[doc = "Set the field `peered_network_ip_range`.\nOptional. Immutable. Subnet IP range within the peered network. This is specified in CIDR notation with a slash and the subnet prefix size. You can optionally specify an IP address before the subnet prefix value. e.g. `192.168.0.0/29` would specify an IP range starting at 192.168.0.0 with a prefix size of 29 bits. `/16` would specify a prefix size of 16 bits, with an automatically determined IP within the peered VPC. If unspecified, a value of `/24` will be used."]
    pub fn set_peered_network_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.peered_network_ip_range = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildWorkerPoolNetworkConfigEl {
    type O = BlockAssignable<CloudbuildWorkerPoolNetworkConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildWorkerPoolNetworkConfigEl {
    #[doc = "Required. Immutable. The network definition that the workers are peered to. If this section is left empty, the workers will be peered to `WorkerPool.project_id` on the service producer network. Must be in the format `projects/{project}/global/networks/{network}`, where `{project}` is a project number, such as `12345`, and `{network}` is the name of a VPC network in the project. See [Understanding network configuration options](https://cloud.google.com/cloud-build/docs/custom-workers/set-up-custom-worker-pool-environment#understanding_the_network_configuration_options)"]
    pub peered_network: PrimField<String>,
}
impl BuildCloudbuildWorkerPoolNetworkConfigEl {
    pub fn build(self) -> CloudbuildWorkerPoolNetworkConfigEl {
        CloudbuildWorkerPoolNetworkConfigEl {
            peered_network: self.peered_network,
            peered_network_ip_range: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildWorkerPoolNetworkConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildWorkerPoolNetworkConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildWorkerPoolNetworkConfigElRef {
        CloudbuildWorkerPoolNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildWorkerPoolNetworkConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `peered_network` after provisioning.\nRequired. Immutable. The network definition that the workers are peered to. If this section is left empty, the workers will be peered to `WorkerPool.project_id` on the service producer network. Must be in the format `projects/{project}/global/networks/{network}`, where `{project}` is a project number, such as `12345`, and `{network}` is the name of a VPC network in the project. See [Understanding network configuration options](https://cloud.google.com/cloud-build/docs/custom-workers/set-up-custom-worker-pool-environment#understanding_the_network_configuration_options)"]
    pub fn peered_network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peered_network", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `peered_network_ip_range` after provisioning.\nOptional. Immutable. Subnet IP range within the peered network. This is specified in CIDR notation with a slash and the subnet prefix size. You can optionally specify an IP address before the subnet prefix value. e.g. `192.168.0.0/29` would specify an IP range starting at 192.168.0.0 with a prefix size of 29 bits. `/16` would specify a prefix size of 16 bits, with an automatically determined IP within the peered VPC. If unspecified, a value of `/24` will be used."]
    pub fn peered_network_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.peered_network_ip_range", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildWorkerPoolPrivateServiceConnectEl {
    network_attachment: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_all_traffic: Option<PrimField<bool>>,
}
impl CloudbuildWorkerPoolPrivateServiceConnectEl {
    #[doc = "Set the field `route_all_traffic`.\nImmutable. Route all traffic through PSC interface. Enable this if you want full control of traffic in the private pool. Configure Cloud NAT for the subnet of network attachment if you need to access public Internet. If false, Only route private IPs, e.g. 10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16 through PSC interface."]
    pub fn set_route_all_traffic(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.route_all_traffic = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildWorkerPoolPrivateServiceConnectEl {
    type O = BlockAssignable<CloudbuildWorkerPoolPrivateServiceConnectEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildWorkerPoolPrivateServiceConnectEl {
    #[doc = "Required. Immutable. The network attachment that the worker network interface is connected to. Must be in the format `projects/{project}/regions/{region}/networkAttachments/{networkAttachment}`. The region of network attachment must be the same as the worker pool. See [Network Attachments](https://cloud.google.com/vpc/docs/about-network-attachments)"]
    pub network_attachment: PrimField<String>,
}
impl BuildCloudbuildWorkerPoolPrivateServiceConnectEl {
    pub fn build(self) -> CloudbuildWorkerPoolPrivateServiceConnectEl {
        CloudbuildWorkerPoolPrivateServiceConnectEl {
            network_attachment: self.network_attachment,
            route_all_traffic: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildWorkerPoolPrivateServiceConnectElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildWorkerPoolPrivateServiceConnectElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildWorkerPoolPrivateServiceConnectElRef {
        CloudbuildWorkerPoolPrivateServiceConnectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildWorkerPoolPrivateServiceConnectElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network_attachment` after provisioning.\nRequired. Immutable. The network attachment that the worker network interface is connected to. Must be in the format `projects/{project}/regions/{region}/networkAttachments/{networkAttachment}`. The region of network attachment must be the same as the worker pool. See [Network Attachments](https://cloud.google.com/vpc/docs/about-network-attachments)"]
    pub fn network_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_attachment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `route_all_traffic` after provisioning.\nImmutable. Route all traffic through PSC interface. Enable this if you want full control of traffic in the private pool. Configure Cloud NAT for the subnet of network attachment if you need to access public Internet. If false, Only route private IPs, e.g. 10.0.0.0/8, 172.16.0.0/12, and 192.168.0.0/16 through PSC interface."]
    pub fn route_all_traffic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.route_all_traffic", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudbuildWorkerPoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl CloudbuildWorkerPoolTimeoutsEl {
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
impl ToListMappable for CloudbuildWorkerPoolTimeoutsEl {
    type O = BlockAssignable<CloudbuildWorkerPoolTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildWorkerPoolTimeoutsEl {}
impl BuildCloudbuildWorkerPoolTimeoutsEl {
    pub fn build(self) -> CloudbuildWorkerPoolTimeoutsEl {
        CloudbuildWorkerPoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildWorkerPoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildWorkerPoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildWorkerPoolTimeoutsElRef {
        CloudbuildWorkerPoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildWorkerPoolTimeoutsElRef {
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
#[derive(Serialize)]
pub struct CloudbuildWorkerPoolWorkerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_external_ip: Option<PrimField<bool>>,
}
impl CloudbuildWorkerPoolWorkerConfigEl {
    #[doc = "Set the field `disk_size_gb`.\nSize of the disk attached to the worker, in GB. See [Worker pool config file](https://cloud.google.com/cloud-build/docs/custom-workers/worker-pool-config-file). Specify a value of up to 1000. If `0` is specified, Cloud Build will use a standard disk size."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_nested_virtualization`.\nEnable nested virtualization on the worker, if supported by the machine type. See [Worker pool config file](https://cloud.google.com/build/docs/private-pools/worker-pool-config-file-schema). If left blank, Cloud Build will set this to false."]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nMachine type of a worker, such as `n1-standard-1`. See [Worker pool config file](https://cloud.google.com/cloud-build/docs/custom-workers/worker-pool-config-file). If left blank, Cloud Build will use `n1-standard-1`."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
    #[doc = "Set the field `no_external_ip`.\nIf true, workers are created without any public address, which prevents network egress to public IPs."]
    pub fn set_no_external_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.no_external_ip = Some(v.into());
        self
    }
}
impl ToListMappable for CloudbuildWorkerPoolWorkerConfigEl {
    type O = BlockAssignable<CloudbuildWorkerPoolWorkerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudbuildWorkerPoolWorkerConfigEl {}
impl BuildCloudbuildWorkerPoolWorkerConfigEl {
    pub fn build(self) -> CloudbuildWorkerPoolWorkerConfigEl {
        CloudbuildWorkerPoolWorkerConfigEl {
            disk_size_gb: core::default::Default::default(),
            enable_nested_virtualization: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            no_external_ip: core::default::Default::default(),
        }
    }
}
pub struct CloudbuildWorkerPoolWorkerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudbuildWorkerPoolWorkerConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudbuildWorkerPoolWorkerConfigElRef {
        CloudbuildWorkerPoolWorkerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudbuildWorkerPoolWorkerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nSize of the disk attached to the worker, in GB. See [Worker pool config file](https://cloud.google.com/cloud-build/docs/custom-workers/worker-pool-config-file). Specify a value of up to 1000. If `0` is specified, Cloud Build will use a standard disk size."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nEnable nested virtualization on the worker, if supported by the machine type. See [Worker pool config file](https://cloud.google.com/build/docs/private-pools/worker-pool-config-file-schema). If left blank, Cloud Build will set this to false."]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_nested_virtualization", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nMachine type of a worker, such as `n1-standard-1`. See [Worker pool config file](https://cloud.google.com/cloud-build/docs/custom-workers/worker-pool-config-file). If left blank, Cloud Build will use `n1-standard-1`."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `no_external_ip` after provisioning.\nIf true, workers are created without any public address, which prevents network egress to public IPs."]
    pub fn no_external_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.no_external_ip", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudbuildWorkerPoolDynamic {
    network_config: Option<DynamicBlock<CloudbuildWorkerPoolNetworkConfigEl>>,
    private_service_connect: Option<DynamicBlock<CloudbuildWorkerPoolPrivateServiceConnectEl>>,
    worker_config: Option<DynamicBlock<CloudbuildWorkerPoolWorkerConfigEl>>,
}
