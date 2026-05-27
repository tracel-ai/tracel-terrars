use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataBeyondcorpSecurityGatewayData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    security_gateway_id: PrimField<String>,
}
struct DataBeyondcorpSecurityGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBeyondcorpSecurityGatewayData>,
}
#[derive(Clone)]
pub struct DataBeyondcorpSecurityGateway(Rc<DataBeyondcorpSecurityGateway_>);
impl DataBeyondcorpSecurityGateway {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delegating_service_account` after provisioning.\nService account used for operations that involve resources in consumer projects."]
    pub fn delegating_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delegating_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the SecurityGateway.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_ips` after provisioning.\nOutput only. IP addresses that will be used for establishing\nconnection to the endpoints."]
    pub fn external_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hubs` after provisioning.\nOptional. Map of Hubs that represents regional data path deployment with GCP region\nas a key."]
    pub fn hubs(&self) -> SetRef<DataBeyondcorpSecurityGatewayHubsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.hubs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Must be omitted or set to 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\nSettings related to Cloud Logging."]
    pub fn logging(&self) -> ListRef<DataBeyondcorpSecurityGatewayLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
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
    #[doc = "Get a reference to the value of field `proxy_protocol_config` after provisioning.\nShared proxy configuration for all apps."]
    pub fn proxy_protocol_config(
        &self,
    ) -> ListRef<DataBeyondcorpSecurityGatewayProxyProtocolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_protocol_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_gateway_id` after provisioning.\nOptional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn security_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\nSettings related to the Service Discovery."]
    pub fn service_discovery(&self) -> ListRef<DataBeyondcorpSecurityGatewayServiceDiscoveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_discovery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The operational state of the SecurityGateway.\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nUPDATING\nDELETING\nRUNNING\nDOWN\nERROR"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
impl Referable for DataBeyondcorpSecurityGateway {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataBeyondcorpSecurityGateway {}
impl ToListMappable for DataBeyondcorpSecurityGateway {
    type O = ListRef<DataBeyondcorpSecurityGatewayRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataBeyondcorpSecurityGateway_ {
    fn extract_datasource_type(&self) -> String {
        "google_beyondcorp_security_gateway".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataBeyondcorpSecurityGateway {
    pub tf_id: String,
    #[doc = "Optional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub security_gateway_id: PrimField<String>,
}
impl BuildDataBeyondcorpSecurityGateway {
    pub fn build(self, stack: &mut Stack) -> DataBeyondcorpSecurityGateway {
        let out = DataBeyondcorpSecurityGateway(Rc::new(DataBeyondcorpSecurityGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBeyondcorpSecurityGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                security_gateway_id: self.security_gateway_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataBeyondcorpSecurityGatewayRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataBeyondcorpSecurityGatewayRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delegating_service_account` after provisioning.\nService account used for operations that involve resources in consumer projects."]
    pub fn delegating_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delegating_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the SecurityGateway.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_ips` after provisioning.\nOutput only. IP addresses that will be used for establishing\nconnection to the endpoints."]
    pub fn external_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hubs` after provisioning.\nOptional. Map of Hubs that represents regional data path deployment with GCP region\nas a key."]
    pub fn hubs(&self) -> SetRef<DataBeyondcorpSecurityGatewayHubsElRef> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.hubs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Must be omitted or set to 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\nSettings related to Cloud Logging."]
    pub fn logging(&self) -> ListRef<DataBeyondcorpSecurityGatewayLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
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
    #[doc = "Get a reference to the value of field `proxy_protocol_config` after provisioning.\nShared proxy configuration for all apps."]
    pub fn proxy_protocol_config(
        &self,
    ) -> ListRef<DataBeyondcorpSecurityGatewayProxyProtocolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_protocol_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_gateway_id` after provisioning.\nOptional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn security_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\nSettings related to the Service Discovery."]
    pub fn service_discovery(&self) -> ListRef<DataBeyondcorpSecurityGatewayServiceDiscoveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_discovery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The operational state of the SecurityGateway.\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nUPDATING\nDELETING\nRUNNING\nDOWN\nERROR"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayHubsElInternetGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    assigned_ips: Option<ListField<PrimField<String>>>,
}
impl DataBeyondcorpSecurityGatewayHubsElInternetGatewayEl {
    #[doc = "Set the field `assigned_ips`.\n"]
    pub fn set_assigned_ips(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.assigned_ips = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpSecurityGatewayHubsElInternetGatewayEl {
    type O = BlockAssignable<DataBeyondcorpSecurityGatewayHubsElInternetGatewayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayHubsElInternetGatewayEl {}
impl BuildDataBeyondcorpSecurityGatewayHubsElInternetGatewayEl {
    pub fn build(self) -> DataBeyondcorpSecurityGatewayHubsElInternetGatewayEl {
        DataBeyondcorpSecurityGatewayHubsElInternetGatewayEl {
            assigned_ips: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
        DataBeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assigned_ips` after provisioning.\n"]
    pub fn assigned_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.assigned_ips", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayHubsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    internet_gateway: Option<ListField<DataBeyondcorpSecurityGatewayHubsElInternetGatewayEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl DataBeyondcorpSecurityGatewayHubsEl {
    #[doc = "Set the field `internet_gateway`.\n"]
    pub fn set_internet_gateway(
        mut self,
        v: impl Into<ListField<DataBeyondcorpSecurityGatewayHubsElInternetGatewayEl>>,
    ) -> Self {
        self.internet_gateway = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpSecurityGatewayHubsEl {
    type O = BlockAssignable<DataBeyondcorpSecurityGatewayHubsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayHubsEl {}
impl BuildDataBeyondcorpSecurityGatewayHubsEl {
    pub fn build(self) -> DataBeyondcorpSecurityGatewayHubsEl {
        DataBeyondcorpSecurityGatewayHubsEl {
            internet_gateway: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayHubsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayHubsElRef {
    fn new(shared: StackShared, base: String) -> DataBeyondcorpSecurityGatewayHubsElRef {
        DataBeyondcorpSecurityGatewayHubsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayHubsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `internet_gateway` after provisioning.\n"]
    pub fn internet_gateway(
        &self,
    ) -> ListRef<DataBeyondcorpSecurityGatewayHubsElInternetGatewayElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.internet_gateway", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayLoggingEl {}
impl DataBeyondcorpSecurityGatewayLoggingEl {}
impl ToListMappable for DataBeyondcorpSecurityGatewayLoggingEl {
    type O = BlockAssignable<DataBeyondcorpSecurityGatewayLoggingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayLoggingEl {}
impl BuildDataBeyondcorpSecurityGatewayLoggingEl {
    pub fn build(self) -> DataBeyondcorpSecurityGatewayLoggingEl {
        DataBeyondcorpSecurityGatewayLoggingEl {}
    }
}
pub struct DataBeyondcorpSecurityGatewayLoggingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayLoggingElRef {
    fn new(shared: StackShared, base: String) -> DataBeyondcorpSecurityGatewayLoggingElRef {
        DataBeyondcorpSecurityGatewayLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
    #[doc = "Set the field `output_type`.\n"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl
{
    type O = BlockAssignable<
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
}
impl BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
    pub fn build(
        self,
    ) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
            output_type: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
    #[doc = "Set the field `output_type`.\n"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl
{
    type O = BlockAssignable<
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
}
impl BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
    pub fn build(
        self,
    ) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
            output_type: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
    #[doc = "Set the field `output_type`.\n"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl
{
    type O = BlockAssignable<
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {}
impl BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
    pub fn build(
        self,
    ) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
            output_type: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_info: Option<
        ListField<
            DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_info: Option<
        ListField<DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_info: Option<
        ListField<DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl>,
    >,
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
    #[doc = "Set the field `device_info`.\n"]
    pub fn set_device_info(
        mut self,
        v: impl Into<
            ListField<
                DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl,
            >,
        >,
    ) -> Self {
        self.device_info = Some(v.into());
        self
    }
    #[doc = "Set the field `group_info`.\n"]
    pub fn set_group_info(
        mut self,
        v: impl Into<
            ListField<
                DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl,
            >,
        >,
    ) -> Self {
        self.group_info = Some(v.into());
        self
    }
    #[doc = "Set the field `output_type`.\n"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
    #[doc = "Set the field `user_info`.\n"]
    pub fn set_user_info(
        mut self,
        v: impl Into<
            ListField<
                DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl,
            >,
        >,
    ) -> Self {
        self.user_info = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
    type O = BlockAssignable<DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {}
impl BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
    pub fn build(self) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
            device_info: core::default::Default::default(),
            group_info: core::default::Default::default(),
            output_type: core::default::Default::default(),
            user_info: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `device_info` after provisioning.\n"]
    pub fn device_info(
        &self,
    ) -> ListRef<DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.device_info", self.base))
    }
    #[doc = "Get a reference to the value of field `group_info` after provisioning.\n"]
    pub fn group_info(
        &self,
    ) -> ListRef<DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.group_info", self.base))
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
    #[doc = "Get a reference to the value of field `user_info` after provisioning.\n"]
    pub fn user_info(
        &self,
    ) -> ListRef<DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.user_info", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_client_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contextual_headers:
        Option<ListField<DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_identity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_headers: Option<RecField<PrimField<String>>>,
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigEl {
    #[doc = "Set the field `allowed_client_headers`.\n"]
    pub fn set_allowed_client_headers(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_client_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `client_ip`.\n"]
    pub fn set_client_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.client_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `contextual_headers`.\n"]
    pub fn set_contextual_headers(
        mut self,
        v: impl Into<ListField<DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl>>,
    ) -> Self {
        self.contextual_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `gateway_identity`.\n"]
    pub fn set_gateway_identity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway_identity = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_headers`.\n"]
    pub fn set_metadata_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata_headers = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpSecurityGatewayProxyProtocolConfigEl {
    type O = BlockAssignable<DataBeyondcorpSecurityGatewayProxyProtocolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigEl {}
impl BuildDataBeyondcorpSecurityGatewayProxyProtocolConfigEl {
    pub fn build(self) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigEl {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigEl {
            allowed_client_headers: core::default::Default::default(),
            client_ip: core::default::Default::default(),
            contextual_headers: core::default::Default::default(),
            gateway_identity: core::default::Default::default(),
            metadata_headers: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayProxyProtocolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayProxyProtocolConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpSecurityGatewayProxyProtocolConfigElRef {
        DataBeyondcorpSecurityGatewayProxyProtocolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayProxyProtocolConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_client_headers` after provisioning.\n"]
    pub fn allowed_client_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_client_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_ip` after provisioning.\n"]
    pub fn client_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `contextual_headers` after provisioning.\n"]
    pub fn contextual_headers(
        &self,
    ) -> ListRef<DataBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contextual_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gateway_identity` after provisioning.\n"]
    pub fn gateway_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_identity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_headers` after provisioning.\n"]
    pub fn metadata_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata_headers", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl
{
    type O = BlockAssignable<
        DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {}
impl BuildDataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
    pub fn build(
        self,
    ) -> DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
        DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
            path: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
        DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_override: Option<
        ListField<DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl>,
    >,
}
impl DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
    #[doc = "Set the field `resource_override`.\n"]
    pub fn set_resource_override(
        mut self,
        v: impl Into<
            ListField<
                DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl,
            >,
        >,
    ) -> Self {
        self.resource_override = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
    type O = BlockAssignable<DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {}
impl BuildDataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
    pub fn build(self) -> DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
        DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
            resource_override: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
        DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_override` after provisioning.\n"]
    pub fn resource_override(
        &self,
    ) -> ListRef<DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_override", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataBeyondcorpSecurityGatewayServiceDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_gateway: Option<ListField<DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl>>,
}
impl DataBeyondcorpSecurityGatewayServiceDiscoveryEl {
    #[doc = "Set the field `api_gateway`.\n"]
    pub fn set_api_gateway(
        mut self,
        v: impl Into<ListField<DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl>>,
    ) -> Self {
        self.api_gateway = Some(v.into());
        self
    }
}
impl ToListMappable for DataBeyondcorpSecurityGatewayServiceDiscoveryEl {
    type O = BlockAssignable<DataBeyondcorpSecurityGatewayServiceDiscoveryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataBeyondcorpSecurityGatewayServiceDiscoveryEl {}
impl BuildDataBeyondcorpSecurityGatewayServiceDiscoveryEl {
    pub fn build(self) -> DataBeyondcorpSecurityGatewayServiceDiscoveryEl {
        DataBeyondcorpSecurityGatewayServiceDiscoveryEl {
            api_gateway: core::default::Default::default(),
        }
    }
}
pub struct DataBeyondcorpSecurityGatewayServiceDiscoveryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataBeyondcorpSecurityGatewayServiceDiscoveryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataBeyondcorpSecurityGatewayServiceDiscoveryElRef {
        DataBeyondcorpSecurityGatewayServiceDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataBeyondcorpSecurityGatewayServiceDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_gateway` after provisioning.\n"]
    pub fn api_gateway(
        &self,
    ) -> ListRef<DataBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.api_gateway", self.base))
    }
}
