use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeRegionNetworkEndpointGroupData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_endpoint_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_target_service: Option<PrimField<String>>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_engine: Option<Vec<ComputeRegionNetworkEndpointGroupAppEngineEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_function: Option<Vec<ComputeRegionNetworkEndpointGroupCloudFunctionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_run: Option<Vec<ComputeRegionNetworkEndpointGroupCloudRunEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    psc_data: Option<Vec<ComputeRegionNetworkEndpointGroupPscDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionNetworkEndpointGroupTimeoutsEl>,
    dynamic: ComputeRegionNetworkEndpointGroupDynamic,
}
struct ComputeRegionNetworkEndpointGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionNetworkEndpointGroupData>,
}
#[derive(Clone)]
pub struct ComputeRegionNetworkEndpointGroup(Rc<ComputeRegionNetworkEndpointGroup_>);
impl ComputeRegionNetworkEndpointGroup {
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
    #[doc = "Set the field `description`.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe URL of the network to which all network endpoints in the NEG belong. Uses\n\"default\" project network if unspecified."]
    pub fn set_network(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network = Some(v.into());
        self
    }
    #[doc = "Set the field `network_endpoint_type`.\nType of network endpoints in this network endpoint group. Defaults to SERVERLESS. Default value: \"SERVERLESS\" Possible values: [\"SERVERLESS\", \"PRIVATE_SERVICE_CONNECT\", \"INTERNET_IP_PORT\", \"INTERNET_FQDN_PORT\", \"GCE_VM_IP_PORTMAP\"]"]
    pub fn set_network_endpoint_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().network_endpoint_type = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `psc_target_service`.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe target service url used to set up private service connection to\na Google API or a PSC Producer Service Attachment."]
    pub fn set_psc_target_service(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().psc_target_service = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThis field is only used for PSC NEGs.\n\nOptional URL of the subnetwork to which all network endpoints in the NEG belong."]
    pub fn set_subnetwork(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subnetwork = Some(v.into());
        self
    }
    #[doc = "Set the field `app_engine`.\n"]
    pub fn set_app_engine(
        self,
        v: impl Into<BlockAssignable<ComputeRegionNetworkEndpointGroupAppEngineEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().app_engine = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.app_engine = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_function`.\n"]
    pub fn set_cloud_function(
        self,
        v: impl Into<BlockAssignable<ComputeRegionNetworkEndpointGroupCloudFunctionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_function = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_function = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cloud_run`.\n"]
    pub fn set_cloud_run(
        self,
        v: impl Into<BlockAssignable<ComputeRegionNetworkEndpointGroupCloudRunEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_run = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_run = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `psc_data`.\n"]
    pub fn set_psc_data(
        self,
        v: impl Into<BlockAssignable<ComputeRegionNetworkEndpointGroupPscDataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().psc_data = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.psc_data = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionNetworkEndpointGroupTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe URL of the network to which all network endpoints in the NEG belong. Uses\n\"default\" project network if unspecified."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_endpoint_type` after provisioning.\nType of network endpoints in this network endpoint group. Defaults to SERVERLESS. Default value: \"SERVERLESS\" Possible values: [\"SERVERLESS\", \"PRIVATE_SERVICE_CONNECT\", \"INTERNET_IP_PORT\", \"INTERNET_FQDN_PORT\", \"GCE_VM_IP_PORTMAP\"]"]
    pub fn network_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_endpoint_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_target_service` after provisioning.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe target service url used to set up private service connection to\na Google API or a PSC Producer Service Attachment."]
    pub fn psc_target_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_target_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the regional NEGs reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThis field is only used for PSC NEGs.\n\nOptional URL of the subnetwork to which all network endpoints in the NEG belong."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_engine` after provisioning.\n"]
    pub fn app_engine(&self) -> ListRef<ComputeRegionNetworkEndpointGroupAppEngineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\n"]
    pub fn cloud_function(&self) -> ListRef<ComputeRegionNetworkEndpointGroupCloudFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_function", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_run` after provisioning.\n"]
    pub fn cloud_run(&self) -> ListRef<ComputeRegionNetworkEndpointGroupCloudRunElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_run", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_data` after provisioning.\n"]
    pub fn psc_data(&self) -> ListRef<ComputeRegionNetworkEndpointGroupPscDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionNetworkEndpointGroupTimeoutsElRef {
        ComputeRegionNetworkEndpointGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeRegionNetworkEndpointGroup {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeRegionNetworkEndpointGroup {}
impl ToListMappable for ComputeRegionNetworkEndpointGroup {
    type O = ListRef<ComputeRegionNetworkEndpointGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeRegionNetworkEndpointGroup_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_network_endpoint_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeRegionNetworkEndpointGroup {
    pub tf_id: String,
    #[doc = "Name of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub name: PrimField<String>,
    #[doc = "A reference to the region where the regional NEGs reside."]
    pub region: PrimField<String>,
}
impl BuildComputeRegionNetworkEndpointGroup {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionNetworkEndpointGroup {
        let out = ComputeRegionNetworkEndpointGroup(Rc::new(ComputeRegionNetworkEndpointGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionNetworkEndpointGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                network_endpoint_type: core::default::Default::default(),
                project: core::default::Default::default(),
                psc_target_service: core::default::Default::default(),
                region: self.region,
                subnetwork: core::default::Default::default(),
                app_engine: core::default::Default::default(),
                cloud_function: core::default::Default::default(),
                cloud_run: core::default::Default::default(),
                psc_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeRegionNetworkEndpointGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkEndpointGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeRegionNetworkEndpointGroupRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource. Provide this property when\nyou create the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe URL of the network to which all network endpoints in the NEG belong. Uses\n\"default\" project network if unspecified."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_endpoint_type` after provisioning.\nType of network endpoints in this network endpoint group. Defaults to SERVERLESS. Default value: \"SERVERLESS\" Possible values: [\"SERVERLESS\", \"PRIVATE_SERVICE_CONNECT\", \"INTERNET_IP_PORT\", \"INTERNET_FQDN_PORT\", \"GCE_VM_IP_PORTMAP\"]"]
    pub fn network_endpoint_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_endpoint_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_target_service` after provisioning.\nThis field is only used for PSC and INTERNET NEGs.\n\nThe target service url used to set up private service connection to\na Google API or a PSC Producer Service Attachment."]
    pub fn psc_target_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.psc_target_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nA reference to the region where the regional NEGs reside."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThis field is only used for PSC NEGs.\n\nOptional URL of the subnetwork to which all network endpoints in the NEG belong."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subnetwork", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `app_engine` after provisioning.\n"]
    pub fn app_engine(&self) -> ListRef<ComputeRegionNetworkEndpointGroupAppEngineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\n"]
    pub fn cloud_function(&self) -> ListRef<ComputeRegionNetworkEndpointGroupCloudFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_function", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_run` after provisioning.\n"]
    pub fn cloud_run(&self) -> ListRef<ComputeRegionNetworkEndpointGroupCloudRunElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_run", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `psc_data` after provisioning.\n"]
    pub fn psc_data(&self) -> ListRef<ComputeRegionNetworkEndpointGroupPscDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_data", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionNetworkEndpointGroupTimeoutsElRef {
        ComputeRegionNetworkEndpointGroupTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroupAppEngineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_mask: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl ComputeRegionNetworkEndpointGroupAppEngineEl {
    #[doc = "Set the field `service`.\nOptional serving service.\nThe service name must be 1-63 characters long, and comply with RFC1035.\nExample value: \"default\", \"my-service\"."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `url_mask`.\nA template to parse service and version fields from a request URL.\nURL mask allows for routing to multiple App Engine services without\nhaving to create multiple Network Endpoint Groups and backend services.\n\nFor example, the request URLs \"foo1-dot-appname.appspot.com/v1\" and\n\"foo1-dot-appname.appspot.com/v2\" can be backed by the same Serverless NEG with\nURL mask \"-dot-appname.appspot.com/\". The URL mask will parse\nthem to { service = \"foo1\", version = \"v1\" } and { service = \"foo1\", version = \"v2\" } respectively."]
    pub fn set_url_mask(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url_mask = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\nOptional serving version.\nThe version must be 1-63 characters long, and comply with RFC1035.\nExample value: \"v1\", \"v2\"."]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionNetworkEndpointGroupAppEngineEl {
    type O = BlockAssignable<ComputeRegionNetworkEndpointGroupAppEngineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkEndpointGroupAppEngineEl {}
impl BuildComputeRegionNetworkEndpointGroupAppEngineEl {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroupAppEngineEl {
        ComputeRegionNetworkEndpointGroupAppEngineEl {
            service: core::default::Default::default(),
            url_mask: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkEndpointGroupAppEngineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkEndpointGroupAppEngineElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionNetworkEndpointGroupAppEngineElRef {
        ComputeRegionNetworkEndpointGroupAppEngineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkEndpointGroupAppEngineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nOptional serving service.\nThe service name must be 1-63 characters long, and comply with RFC1035.\nExample value: \"default\", \"my-service\"."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `url_mask` after provisioning.\nA template to parse service and version fields from a request URL.\nURL mask allows for routing to multiple App Engine services without\nhaving to create multiple Network Endpoint Groups and backend services.\n\nFor example, the request URLs \"foo1-dot-appname.appspot.com/v1\" and\n\"foo1-dot-appname.appspot.com/v2\" can be backed by the same Serverless NEG with\nURL mask \"-dot-appname.appspot.com/\". The URL mask will parse\nthem to { service = \"foo1\", version = \"v1\" } and { service = \"foo1\", version = \"v2\" } respectively."]
    pub fn url_mask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\nOptional serving version.\nThe version must be 1-63 characters long, and comply with RFC1035.\nExample value: \"v1\", \"v2\"."]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroupCloudFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    function: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_mask: Option<PrimField<String>>,
}
impl ComputeRegionNetworkEndpointGroupCloudFunctionEl {
    #[doc = "Set the field `function`.\nA user-defined name of the Cloud Function.\nThe function name is case-sensitive and must be 1-63 characters long.\nExample value: \"func1\"."]
    pub fn set_function(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.function = Some(v.into());
        self
    }
    #[doc = "Set the field `url_mask`.\nA template to parse function field from a request URL. URL mask allows\nfor routing to multiple Cloud Functions without having to create\nmultiple Network Endpoint Groups and backend services.\n\nFor example, request URLs \"mydomain.com/function1\" and \"mydomain.com/function2\"\ncan be backed by the same Serverless NEG with URL mask \"/\". The URL mask\nwill parse them to { function = \"function1\" } and { function = \"function2\" } respectively."]
    pub fn set_url_mask(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url_mask = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionNetworkEndpointGroupCloudFunctionEl {
    type O = BlockAssignable<ComputeRegionNetworkEndpointGroupCloudFunctionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkEndpointGroupCloudFunctionEl {}
impl BuildComputeRegionNetworkEndpointGroupCloudFunctionEl {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroupCloudFunctionEl {
        ComputeRegionNetworkEndpointGroupCloudFunctionEl {
            function: core::default::Default::default(),
            url_mask: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkEndpointGroupCloudFunctionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkEndpointGroupCloudFunctionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionNetworkEndpointGroupCloudFunctionElRef {
        ComputeRegionNetworkEndpointGroupCloudFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkEndpointGroupCloudFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `function` after provisioning.\nA user-defined name of the Cloud Function.\nThe function name is case-sensitive and must be 1-63 characters long.\nExample value: \"func1\"."]
    pub fn function(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function", self.base))
    }
    #[doc = "Get a reference to the value of field `url_mask` after provisioning.\nA template to parse function field from a request URL. URL mask allows\nfor routing to multiple Cloud Functions without having to create\nmultiple Network Endpoint Groups and backend services.\n\nFor example, request URLs \"mydomain.com/function1\" and \"mydomain.com/function2\"\ncan be backed by the same Serverless NEG with URL mask \"/\". The URL mask\nwill parse them to { function = \"function1\" } and { function = \"function2\" } respectively."]
    pub fn url_mask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroupCloudRunEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_mask: Option<PrimField<String>>,
}
impl ComputeRegionNetworkEndpointGroupCloudRunEl {
    #[doc = "Set the field `service`.\nCloud Run service is the main resource of Cloud Run.\nThe service must be 1-63 characters long, and comply with RFC1035.\nExample value: \"run-service\"."]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\nCloud Run tag represents the \"named-revision\" to provide\nadditional fine-grained traffic routing information.\nThe tag must be 1-63 characters long, and comply with RFC1035.\nExample value: \"revision-0010\"."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `url_mask`.\nA template to parse service and tag fields from a request URL.\nURL mask allows for routing to multiple Run services without having\nto create multiple network endpoint groups and backend services.\n\nFor example, request URLs \"foo1.domain.com/bar1\" and \"foo1.domain.com/bar2\"\nan be backed by the same Serverless Network Endpoint Group (NEG) with\nURL mask \".domain.com/\". The URL mask will parse them to { service=\"bar1\", tag=\"foo1\" }\nand { service=\"bar2\", tag=\"foo2\" } respectively."]
    pub fn set_url_mask(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url_mask = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionNetworkEndpointGroupCloudRunEl {
    type O = BlockAssignable<ComputeRegionNetworkEndpointGroupCloudRunEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkEndpointGroupCloudRunEl {}
impl BuildComputeRegionNetworkEndpointGroupCloudRunEl {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroupCloudRunEl {
        ComputeRegionNetworkEndpointGroupCloudRunEl {
            service: core::default::Default::default(),
            tag: core::default::Default::default(),
            url_mask: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkEndpointGroupCloudRunElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkEndpointGroupCloudRunElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionNetworkEndpointGroupCloudRunElRef {
        ComputeRegionNetworkEndpointGroupCloudRunElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkEndpointGroupCloudRunElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\nCloud Run service is the main resource of Cloud Run.\nThe service must be 1-63 characters long, and comply with RFC1035.\nExample value: \"run-service\"."]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nCloud Run tag represents the \"named-revision\" to provide\nadditional fine-grained traffic routing information.\nThe tag must be 1-63 characters long, and comply with RFC1035.\nExample value: \"revision-0010\"."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `url_mask` after provisioning.\nA template to parse service and tag fields from a request URL.\nURL mask allows for routing to multiple Run services without having\nto create multiple network endpoint groups and backend services.\n\nFor example, request URLs \"foo1.domain.com/bar1\" and \"foo1.domain.com/bar2\"\nan be backed by the same Serverless Network Endpoint Group (NEG) with\nURL mask \".domain.com/\". The URL mask will parse them to { service=\"bar1\", tag=\"foo1\" }\nand { service=\"bar2\", tag=\"foo2\" } respectively."]
    pub fn url_mask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroupPscDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    producer_port: Option<PrimField<String>>,
}
impl ComputeRegionNetworkEndpointGroupPscDataEl {
    #[doc = "Set the field `producer_port`.\nThe PSC producer port to use when consumer PSC NEG connects to a producer. If\nthis flag isn't specified for a PSC NEG with endpoint type\nprivate-service-connect, then PSC NEG will be connected to a first port in the\navailable PSC producer port range."]
    pub fn set_producer_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.producer_port = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeRegionNetworkEndpointGroupPscDataEl {
    type O = BlockAssignable<ComputeRegionNetworkEndpointGroupPscDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkEndpointGroupPscDataEl {}
impl BuildComputeRegionNetworkEndpointGroupPscDataEl {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroupPscDataEl {
        ComputeRegionNetworkEndpointGroupPscDataEl {
            producer_port: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkEndpointGroupPscDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkEndpointGroupPscDataElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionNetworkEndpointGroupPscDataElRef {
        ComputeRegionNetworkEndpointGroupPscDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkEndpointGroupPscDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `producer_port` after provisioning.\nThe PSC producer port to use when consumer PSC NEG connects to a producer. If\nthis flag isn't specified for a PSC NEG with endpoint type\nprivate-service-connect, then PSC NEG will be connected to a first port in the\navailable PSC producer port range."]
    pub fn producer_port(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.producer_port", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeRegionNetworkEndpointGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ComputeRegionNetworkEndpointGroupTimeoutsEl {
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
impl ToListMappable for ComputeRegionNetworkEndpointGroupTimeoutsEl {
    type O = BlockAssignable<ComputeRegionNetworkEndpointGroupTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeRegionNetworkEndpointGroupTimeoutsEl {}
impl BuildComputeRegionNetworkEndpointGroupTimeoutsEl {
    pub fn build(self) -> ComputeRegionNetworkEndpointGroupTimeoutsEl {
        ComputeRegionNetworkEndpointGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ComputeRegionNetworkEndpointGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeRegionNetworkEndpointGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionNetworkEndpointGroupTimeoutsElRef {
        ComputeRegionNetworkEndpointGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeRegionNetworkEndpointGroupTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct ComputeRegionNetworkEndpointGroupDynamic {
    app_engine: Option<DynamicBlock<ComputeRegionNetworkEndpointGroupAppEngineEl>>,
    cloud_function: Option<DynamicBlock<ComputeRegionNetworkEndpointGroupCloudFunctionEl>>,
    cloud_run: Option<DynamicBlock<ComputeRegionNetworkEndpointGroupCloudRunEl>>,
    psc_data: Option<DynamicBlock<ComputeRegionNetworkEndpointGroupPscDataEl>>,
}
