use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeRegionNetworkEndpointGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
}
struct DataComputeRegionNetworkEndpointGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeRegionNetworkEndpointGroupData>,
}
#[derive(Clone)]
pub struct DataComputeRegionNetworkEndpointGroup(Rc<DataComputeRegionNetworkEndpointGroup_>);
impl DataComputeRegionNetworkEndpointGroup {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nName of the resource; provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035. Specifically, the name must be 1-63 characters long and match\nthe regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means the\nfirst character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the last\ncharacter, which cannot be a dash."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nA reference to the region where the regional NEGs reside."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `self_link`.\n"]
    pub fn set_self_link(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().self_link = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app_engine` after provisioning.\nThis field is only used for SERVERLESS NEGs.\n\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn app_engine(&self) -> ListRef<DataComputeRegionNetworkEndpointGroupAppEngineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\nThis field is only used for SERVERLESS NEGs.\n\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn cloud_function(
        &self,
    ) -> ListRef<DataComputeRegionNetworkEndpointGroupCloudFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_function", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_run` after provisioning.\nThis field is only used for SERVERLESS NEGs.\n\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn cloud_run(&self) -> ListRef<DataComputeRegionNetworkEndpointGroupCloudRunElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_run", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `psc_data` after provisioning.\nThis field is only used for PSC NEGs."]
    pub fn psc_data(&self) -> ListRef<DataComputeRegionNetworkEndpointGroupPscDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_data", self.extract_ref()),
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
}
impl Referable for DataComputeRegionNetworkEndpointGroup {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeRegionNetworkEndpointGroup {}
impl ToListMappable for DataComputeRegionNetworkEndpointGroup {
    type O = ListRef<DataComputeRegionNetworkEndpointGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeRegionNetworkEndpointGroup_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_region_network_endpoint_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeRegionNetworkEndpointGroup {
    pub tf_id: String,
}
impl BuildDataComputeRegionNetworkEndpointGroup {
    pub fn build(self, stack: &mut Stack) -> DataComputeRegionNetworkEndpointGroup {
        let out = DataComputeRegionNetworkEndpointGroup(Rc::new(
            DataComputeRegionNetworkEndpointGroup_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataComputeRegionNetworkEndpointGroupData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    name: core::default::Default::default(),
                    project: core::default::Default::default(),
                    region: core::default::Default::default(),
                    self_link: core::default::Default::default(),
                }),
            },
        ));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeRegionNetworkEndpointGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionNetworkEndpointGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeRegionNetworkEndpointGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `app_engine` after provisioning.\nThis field is only used for SERVERLESS NEGs.\n\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn app_engine(&self) -> ListRef<DataComputeRegionNetworkEndpointGroupAppEngineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.app_engine", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_function` after provisioning.\nThis field is only used for SERVERLESS NEGs.\n\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn cloud_function(
        &self,
    ) -> ListRef<DataComputeRegionNetworkEndpointGroupCloudFunctionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_function", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cloud_run` after provisioning.\nThis field is only used for SERVERLESS NEGs.\n\nOnly one of cloud_run, app_engine, cloud_function or serverless_deployment may be set."]
    pub fn cloud_run(&self) -> ListRef<DataComputeRegionNetworkEndpointGroupCloudRunElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_run", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `psc_data` after provisioning.\nThis field is only used for PSC NEGs."]
    pub fn psc_data(&self) -> ListRef<DataComputeRegionNetworkEndpointGroupPscDataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_data", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataComputeRegionNetworkEndpointGroupAppEngineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_mask: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}
impl DataComputeRegionNetworkEndpointGroupAppEngineEl {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `url_mask`.\n"]
    pub fn set_url_mask(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url_mask = Some(v.into());
        self
    }
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionNetworkEndpointGroupAppEngineEl {
    type O = BlockAssignable<DataComputeRegionNetworkEndpointGroupAppEngineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionNetworkEndpointGroupAppEngineEl {}
impl BuildDataComputeRegionNetworkEndpointGroupAppEngineEl {
    pub fn build(self) -> DataComputeRegionNetworkEndpointGroupAppEngineEl {
        DataComputeRegionNetworkEndpointGroupAppEngineEl {
            service: core::default::Default::default(),
            url_mask: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionNetworkEndpointGroupAppEngineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionNetworkEndpointGroupAppEngineElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionNetworkEndpointGroupAppEngineElRef {
        DataComputeRegionNetworkEndpointGroupAppEngineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionNetworkEndpointGroupAppEngineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `url_mask` after provisioning.\n"]
    pub fn url_mask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    function: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_mask: Option<PrimField<String>>,
}
impl DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
    #[doc = "Set the field `function`.\n"]
    pub fn set_function(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.function = Some(v.into());
        self
    }
    #[doc = "Set the field `url_mask`.\n"]
    pub fn set_url_mask(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url_mask = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
    type O = BlockAssignable<DataComputeRegionNetworkEndpointGroupCloudFunctionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionNetworkEndpointGroupCloudFunctionEl {}
impl BuildDataComputeRegionNetworkEndpointGroupCloudFunctionEl {
    pub fn build(self) -> DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
        DataComputeRegionNetworkEndpointGroupCloudFunctionEl {
            function: core::default::Default::default(),
            url_mask: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
        DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionNetworkEndpointGroupCloudFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `function` after provisioning.\n"]
    pub fn function(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.function", self.base))
    }
    #[doc = "Get a reference to the value of field `url_mask` after provisioning.\n"]
    pub fn url_mask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionNetworkEndpointGroupCloudRunEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_mask: Option<PrimField<String>>,
}
impl DataComputeRegionNetworkEndpointGroupCloudRunEl {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
    #[doc = "Set the field `url_mask`.\n"]
    pub fn set_url_mask(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url_mask = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionNetworkEndpointGroupCloudRunEl {
    type O = BlockAssignable<DataComputeRegionNetworkEndpointGroupCloudRunEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionNetworkEndpointGroupCloudRunEl {}
impl BuildDataComputeRegionNetworkEndpointGroupCloudRunEl {
    pub fn build(self) -> DataComputeRegionNetworkEndpointGroupCloudRunEl {
        DataComputeRegionNetworkEndpointGroupCloudRunEl {
            service: core::default::Default::default(),
            tag: core::default::Default::default(),
            url_mask: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionNetworkEndpointGroupCloudRunElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionNetworkEndpointGroupCloudRunElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeRegionNetworkEndpointGroupCloudRunElRef {
        DataComputeRegionNetworkEndpointGroupCloudRunElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionNetworkEndpointGroupCloudRunElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
    #[doc = "Get a reference to the value of field `url_mask` after provisioning.\n"]
    pub fn url_mask(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url_mask", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeRegionNetworkEndpointGroupPscDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    producer_port: Option<PrimField<String>>,
}
impl DataComputeRegionNetworkEndpointGroupPscDataEl {
    #[doc = "Set the field `producer_port`.\n"]
    pub fn set_producer_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.producer_port = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeRegionNetworkEndpointGroupPscDataEl {
    type O = BlockAssignable<DataComputeRegionNetworkEndpointGroupPscDataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeRegionNetworkEndpointGroupPscDataEl {}
impl BuildDataComputeRegionNetworkEndpointGroupPscDataEl {
    pub fn build(self) -> DataComputeRegionNetworkEndpointGroupPscDataEl {
        DataComputeRegionNetworkEndpointGroupPscDataEl {
            producer_port: core::default::Default::default(),
        }
    }
}
pub struct DataComputeRegionNetworkEndpointGroupPscDataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeRegionNetworkEndpointGroupPscDataElRef {
    fn new(shared: StackShared, base: String) -> DataComputeRegionNetworkEndpointGroupPscDataElRef {
        DataComputeRegionNetworkEndpointGroupPscDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeRegionNetworkEndpointGroupPscDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `producer_port` after provisioning.\n"]
    pub fn producer_port(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.producer_port", self.base),
        )
    }
}
