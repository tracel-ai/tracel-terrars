use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataEc2ClientVpnEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_vpn_endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2ClientVpnEndpointFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2ClientVpnEndpointTimeoutsEl>,
    dynamic: DataEc2ClientVpnEndpointDynamic,
}
struct DataEc2ClientVpnEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2ClientVpnEndpointData>,
}
#[derive(Clone)]
pub struct DataEc2ClientVpnEndpoint(Rc<DataEc2ClientVpnEndpoint_>);
impl DataEc2ClientVpnEndpoint {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `client_vpn_endpoint_id`.\n"]
    pub fn set_client_vpn_endpoint_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_vpn_endpoint_id = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl Into<BlockAssignable<DataEc2ClientVpnEndpointFilterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataEc2ClientVpnEndpointTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `authentication_options` after provisioning.\n"]
    pub fn authentication_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointAuthenticationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authentication_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointClientConnectOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_connect_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_login_banner_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_route_enforcement_options` after provisioning.\n"]
    pub fn client_route_enforcement_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointClientRouteEnforcementOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_route_enforcement_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_vpn_endpoint_id` after provisioning.\n"]
    pub fn client_vpn_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_vpn_endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointConnectionLogOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_log_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_servers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_ip_address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_service_portal` after provisioning.\n"]
    pub fn self_service_portal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_service_portal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_service_portal_url` after provisioning.\n"]
    pub fn self_service_portal_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_service_portal_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_certificate_arn` after provisioning.\n"]
    pub fn server_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_certificate_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_timeout_hours` after provisioning.\n"]
    pub fn session_timeout_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_timeout_hours", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `split_tunnel` after provisioning.\n"]
    pub fn split_tunnel(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.split_tunnel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic_ip_address_type` after provisioning.\n"]
    pub fn traffic_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_ip_address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transport_protocol` after provisioning.\n"]
    pub fn transport_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transport_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpn_port` after provisioning.\n"]
    pub fn vpn_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpn_port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2ClientVpnEndpointTimeoutsElRef {
        DataEc2ClientVpnEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataEc2ClientVpnEndpoint {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataEc2ClientVpnEndpoint {}
impl ToListMappable for DataEc2ClientVpnEndpoint {
    type O = ListRef<DataEc2ClientVpnEndpointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataEc2ClientVpnEndpoint_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_client_vpn_endpoint".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataEc2ClientVpnEndpoint {
    pub tf_id: String,
}
impl BuildDataEc2ClientVpnEndpoint {
    pub fn build(self, stack: &mut Stack) -> DataEc2ClientVpnEndpoint {
        let out = DataEc2ClientVpnEndpoint(Rc::new(DataEc2ClientVpnEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2ClientVpnEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                client_vpn_endpoint_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataEc2ClientVpnEndpointRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2ClientVpnEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataEc2ClientVpnEndpointRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `authentication_options` after provisioning.\n"]
    pub fn authentication_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointAuthenticationOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authentication_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_cidr_block", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointClientConnectOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_connect_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_login_banner_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_route_enforcement_options` after provisioning.\n"]
    pub fn client_route_enforcement_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointClientRouteEnforcementOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_route_enforcement_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `client_vpn_endpoint_id` after provisioning.\n"]
    pub fn client_vpn_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_vpn_endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(
        &self,
    ) -> ListRef<DataEc2ClientVpnEndpointConnectionLogOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.connection_log_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_servers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_ip_address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_service_portal` after provisioning.\n"]
    pub fn self_service_portal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_service_portal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_service_portal_url` after provisioning.\n"]
    pub fn self_service_portal_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_service_portal_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_certificate_arn` after provisioning.\n"]
    pub fn server_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.server_certificate_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_timeout_hours` after provisioning.\n"]
    pub fn session_timeout_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_timeout_hours", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `split_tunnel` after provisioning.\n"]
    pub fn split_tunnel(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.split_tunnel", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `traffic_ip_address_type` after provisioning.\n"]
    pub fn traffic_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.traffic_ip_address_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `transport_protocol` after provisioning.\n"]
    pub fn transport_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transport_protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpn_port` after provisioning.\n"]
    pub fn vpn_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpn_port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2ClientVpnEndpointTimeoutsElRef {
        DataEc2ClientVpnEndpointTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpointAuthenticationOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_certificate_chain_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml_provider_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_service_saml_provider_arn: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl DataEc2ClientVpnEndpointAuthenticationOptionsEl {
    #[doc = "Set the field `active_directory_id`.\n"]
    pub fn set_active_directory_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.active_directory_id = Some(v.into());
        self
    }
    #[doc = "Set the field `root_certificate_chain_arn`.\n"]
    pub fn set_root_certificate_chain_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.root_certificate_chain_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `saml_provider_arn`.\n"]
    pub fn set_saml_provider_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.saml_provider_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `self_service_saml_provider_arn`.\n"]
    pub fn set_self_service_saml_provider_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.self_service_saml_provider_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataEc2ClientVpnEndpointAuthenticationOptionsEl {
    type O = BlockAssignable<DataEc2ClientVpnEndpointAuthenticationOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2ClientVpnEndpointAuthenticationOptionsEl {}
impl BuildDataEc2ClientVpnEndpointAuthenticationOptionsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointAuthenticationOptionsEl {
        DataEc2ClientVpnEndpointAuthenticationOptionsEl {
            active_directory_id: core::default::Default::default(),
            root_certificate_chain_arn: core::default::Default::default(),
            saml_provider_arn: core::default::Default::default(),
            self_service_saml_provider_arn: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
        DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2ClientVpnEndpointAuthenticationOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `active_directory_id` after provisioning.\n"]
    pub fn active_directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `root_certificate_chain_arn` after provisioning.\n"]
    pub fn root_certificate_chain_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.root_certificate_chain_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `saml_provider_arn` after provisioning.\n"]
    pub fn saml_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.saml_provider_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `self_service_saml_provider_arn` after provisioning.\n"]
    pub fn self_service_saml_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_service_saml_provider_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpointClientConnectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_arn: Option<PrimField<String>>,
}
impl DataEc2ClientVpnEndpointClientConnectOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `lambda_function_arn`.\n"]
    pub fn set_lambda_function_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.lambda_function_arn = Some(v.into());
        self
    }
}
impl ToListMappable for DataEc2ClientVpnEndpointClientConnectOptionsEl {
    type O = BlockAssignable<DataEc2ClientVpnEndpointClientConnectOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2ClientVpnEndpointClientConnectOptionsEl {}
impl BuildDataEc2ClientVpnEndpointClientConnectOptionsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointClientConnectOptionsEl {
        DataEc2ClientVpnEndpointClientConnectOptionsEl {
            enabled: core::default::Default::default(),
            lambda_function_arn: core::default::Default::default(),
        }
    }
}
pub struct DataEc2ClientVpnEndpointClientConnectOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2ClientVpnEndpointClientConnectOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2ClientVpnEndpointClientConnectOptionsElRef {
        DataEc2ClientVpnEndpointClientConnectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2ClientVpnEndpointClientConnectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `lambda_function_arn` after provisioning.\n"]
    pub fn lambda_function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.lambda_function_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    banner_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
    #[doc = "Set the field `banner_text`.\n"]
    pub fn set_banner_text(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.banner_text = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
    type O = BlockAssignable<DataEc2ClientVpnEndpointClientLoginBannerOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2ClientVpnEndpointClientLoginBannerOptionsEl {}
impl BuildDataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
        DataEc2ClientVpnEndpointClientLoginBannerOptionsEl {
            banner_text: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
        DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2ClientVpnEndpointClientLoginBannerOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `banner_text` after provisioning.\n"]
    pub fn banner_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.banner_text", self.base))
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpointClientRouteEnforcementOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforced: Option<PrimField<bool>>,
}
impl DataEc2ClientVpnEndpointClientRouteEnforcementOptionsEl {
    #[doc = "Set the field `enforced`.\n"]
    pub fn set_enforced(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforced = Some(v.into());
        self
    }
}
impl ToListMappable for DataEc2ClientVpnEndpointClientRouteEnforcementOptionsEl {
    type O = BlockAssignable<DataEc2ClientVpnEndpointClientRouteEnforcementOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2ClientVpnEndpointClientRouteEnforcementOptionsEl {}
impl BuildDataEc2ClientVpnEndpointClientRouteEnforcementOptionsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointClientRouteEnforcementOptionsEl {
        DataEc2ClientVpnEndpointClientRouteEnforcementOptionsEl {
            enforced: core::default::Default::default(),
        }
    }
}
pub struct DataEc2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataEc2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
        DataEc2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enforced` after provisioning.\n"]
    pub fn enforced(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforced", self.base))
    }
}
#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpointConnectionLogOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_stream: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataEc2ClientVpnEndpointConnectionLogOptionsEl {
    #[doc = "Set the field `cloudwatch_log_group`.\n"]
    pub fn set_cloudwatch_log_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloudwatch_log_group = Some(v.into());
        self
    }
    #[doc = "Set the field `cloudwatch_log_stream`.\n"]
    pub fn set_cloudwatch_log_stream(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cloudwatch_log_stream = Some(v.into());
        self
    }
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataEc2ClientVpnEndpointConnectionLogOptionsEl {
    type O = BlockAssignable<DataEc2ClientVpnEndpointConnectionLogOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2ClientVpnEndpointConnectionLogOptionsEl {}
impl BuildDataEc2ClientVpnEndpointConnectionLogOptionsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointConnectionLogOptionsEl {
        DataEc2ClientVpnEndpointConnectionLogOptionsEl {
            cloudwatch_log_group: core::default::Default::default(),
            cloudwatch_log_stream: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
        DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2ClientVpnEndpointConnectionLogOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloudwatch_log_group` after provisioning.\n"]
    pub fn cloudwatch_log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_log_group", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cloudwatch_log_stream` after provisioning.\n"]
    pub fn cloudwatch_log_stream(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cloudwatch_log_stream", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpointFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}
impl DataEc2ClientVpnEndpointFilterEl {}
impl ToListMappable for DataEc2ClientVpnEndpointFilterEl {
    type O = BlockAssignable<DataEc2ClientVpnEndpointFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2ClientVpnEndpointFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}
impl BuildDataEc2ClientVpnEndpointFilterEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointFilterEl {
        DataEc2ClientVpnEndpointFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}
pub struct DataEc2ClientVpnEndpointFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2ClientVpnEndpointFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2ClientVpnEndpointFilterElRef {
        DataEc2ClientVpnEndpointFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2ClientVpnEndpointFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataEc2ClientVpnEndpointTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataEc2ClientVpnEndpointTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataEc2ClientVpnEndpointTimeoutsEl {
    type O = BlockAssignable<DataEc2ClientVpnEndpointTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataEc2ClientVpnEndpointTimeoutsEl {}
impl BuildDataEc2ClientVpnEndpointTimeoutsEl {
    pub fn build(self) -> DataEc2ClientVpnEndpointTimeoutsEl {
        DataEc2ClientVpnEndpointTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataEc2ClientVpnEndpointTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataEc2ClientVpnEndpointTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2ClientVpnEndpointTimeoutsElRef {
        DataEc2ClientVpnEndpointTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataEc2ClientVpnEndpointTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataEc2ClientVpnEndpointDynamic {
    filter: Option<DynamicBlock<DataEc2ClientVpnEndpointFilterEl>>,
}
