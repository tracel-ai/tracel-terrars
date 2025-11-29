use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2ClientVpnEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disconnect_on_session_timeout: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_servers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_service_portal: Option<PrimField<String>>,
    server_certificate_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_timeout_hours: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_tunnel: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    traffic_ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transport_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpn_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_options: Option<Vec<Ec2ClientVpnEndpointAuthenticationOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_connect_options: Option<Vec<Ec2ClientVpnEndpointClientConnectOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_login_banner_options: Option<Vec<Ec2ClientVpnEndpointClientLoginBannerOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_route_enforcement_options: Option<Vec<Ec2ClientVpnEndpointClientRouteEnforcementOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_log_options: Option<Vec<Ec2ClientVpnEndpointConnectionLogOptionsEl>>,
    dynamic: Ec2ClientVpnEndpointDynamic,
}

struct Ec2ClientVpnEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Ec2ClientVpnEndpointData>,
}

#[derive(Clone)]
pub struct Ec2ClientVpnEndpoint(Rc<Ec2ClientVpnEndpoint_>);

impl Ec2ClientVpnEndpoint {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = "Set the field `client_cidr_block`.\n"]
    pub fn set_client_cidr_block(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().client_cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `disconnect_on_session_timeout`.\n"]
    pub fn set_disconnect_on_session_timeout(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disconnect_on_session_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `dns_servers`.\n"]
    pub fn set_dns_servers(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().dns_servers = Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_ip_address_type`.\n"]
    pub fn set_endpoint_ip_address_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().endpoint_ip_address_type = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().security_group_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `self_service_portal`.\n"]
    pub fn set_self_service_portal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().self_service_portal = Some(v.into());
        self
    }

    #[doc = "Set the field `session_timeout_hours`.\n"]
    pub fn set_session_timeout_hours(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().session_timeout_hours = Some(v.into());
        self
    }

    #[doc = "Set the field `split_tunnel`.\n"]
    pub fn set_split_tunnel(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().split_tunnel = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_ip_address_type`.\n"]
    pub fn set_traffic_ip_address_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().traffic_ip_address_type = Some(v.into());
        self
    }

    #[doc = "Set the field `transport_protocol`.\n"]
    pub fn set_transport_protocol(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transport_protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_port`.\n"]
    pub fn set_vpn_port(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().vpn_port = Some(v.into());
        self
    }

    #[doc = "Set the field `authentication_options`.\n"]
    pub fn set_authentication_options(
        self,
        v: impl Into<BlockAssignable<Ec2ClientVpnEndpointAuthenticationOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authentication_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authentication_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_connect_options`.\n"]
    pub fn set_client_connect_options(
        self,
        v: impl Into<BlockAssignable<Ec2ClientVpnEndpointClientConnectOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_connect_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_connect_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_login_banner_options`.\n"]
    pub fn set_client_login_banner_options(
        self,
        v: impl Into<BlockAssignable<Ec2ClientVpnEndpointClientLoginBannerOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_login_banner_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_login_banner_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_route_enforcement_options`.\n"]
    pub fn set_client_route_enforcement_options(
        self,
        v: impl Into<BlockAssignable<Ec2ClientVpnEndpointClientRouteEnforcementOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_route_enforcement_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_route_enforcement_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connection_log_options`.\n"]
    pub fn set_connection_log_options(
        self,
        v: impl Into<BlockAssignable<Ec2ClientVpnEndpointConnectionLogOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_log_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_log_options = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disconnect_on_session_timeout` after provisioning.\n"]
    pub fn disconnect_on_session_timeout(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disconnect_on_session_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal` after provisioning.\n"]
    pub fn self_service_portal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal_url` after provisioning.\n"]
    pub fn self_service_portal_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate_arn` after provisioning.\n"]
    pub fn server_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_timeout_hours` after provisioning.\n"]
    pub fn session_timeout_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_timeout_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `split_tunnel` after provisioning.\n"]
    pub fn split_tunnel(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.split_tunnel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_ip_address_type` after provisioning.\n"]
    pub fn traffic_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.traffic_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport_protocol` after provisioning.\n"]
    pub fn transport_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transport_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_port` after provisioning.\n"]
    pub fn vpn_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> ListRef<Ec2ClientVpnEndpointClientConnectOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> ListRef<Ec2ClientVpnEndpointClientLoginBannerOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_route_enforcement_options` after provisioning.\n"]
    pub fn client_route_enforcement_options(&self) -> ListRef<Ec2ClientVpnEndpointClientRouteEnforcementOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_route_enforcement_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> ListRef<Ec2ClientVpnEndpointConnectionLogOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connection_log_options", self.extract_ref()))
    }
}

impl Referable for Ec2ClientVpnEndpoint {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Ec2ClientVpnEndpoint { }

impl ToListMappable for Ec2ClientVpnEndpoint {
    type O = ListRef<Ec2ClientVpnEndpointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Ec2ClientVpnEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "aws_ec2_client_vpn_endpoint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2ClientVpnEndpoint {
    pub tf_id: String,
    #[doc = ""]
    pub server_certificate_arn: PrimField<String>,
}

impl BuildEc2ClientVpnEndpoint {
    pub fn build(self, stack: &mut Stack) -> Ec2ClientVpnEndpoint {
        let out = Ec2ClientVpnEndpoint(Rc::new(Ec2ClientVpnEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Ec2ClientVpnEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                client_cidr_block: core::default::Default::default(),
                description: core::default::Default::default(),
                disconnect_on_session_timeout: core::default::Default::default(),
                dns_servers: core::default::Default::default(),
                endpoint_ip_address_type: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                self_service_portal: core::default::Default::default(),
                server_certificate_arn: self.server_certificate_arn,
                session_timeout_hours: core::default::Default::default(),
                split_tunnel: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                traffic_ip_address_type: core::default::Default::default(),
                transport_protocol: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                vpn_port: core::default::Default::default(),
                authentication_options: core::default::Default::default(),
                client_connect_options: core::default::Default::default(),
                client_login_banner_options: core::default::Default::default(),
                client_route_enforcement_options: core::default::Default::default(),
                connection_log_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2ClientVpnEndpointRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2ClientVpnEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Ec2ClientVpnEndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_cidr_block` after provisioning.\n"]
    pub fn client_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disconnect_on_session_timeout` after provisioning.\n"]
    pub fn disconnect_on_session_timeout(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disconnect_on_session_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_servers` after provisioning.\n"]
    pub fn dns_servers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dns_servers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal` after provisioning.\n"]
    pub fn self_service_portal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_service_portal_url` after provisioning.\n"]
    pub fn self_service_portal_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_service_portal_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_certificate_arn` after provisioning.\n"]
    pub fn server_certificate_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.server_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `session_timeout_hours` after provisioning.\n"]
    pub fn session_timeout_hours(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.session_timeout_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `split_tunnel` after provisioning.\n"]
    pub fn split_tunnel(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.split_tunnel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_ip_address_type` after provisioning.\n"]
    pub fn traffic_ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.traffic_ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transport_protocol` after provisioning.\n"]
    pub fn transport_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transport_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_port` after provisioning.\n"]
    pub fn vpn_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpn_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_connect_options` after provisioning.\n"]
    pub fn client_connect_options(&self) -> ListRef<Ec2ClientVpnEndpointClientConnectOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_connect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_login_banner_options` after provisioning.\n"]
    pub fn client_login_banner_options(&self) -> ListRef<Ec2ClientVpnEndpointClientLoginBannerOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_login_banner_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_route_enforcement_options` after provisioning.\n"]
    pub fn client_route_enforcement_options(&self) -> ListRef<Ec2ClientVpnEndpointClientRouteEnforcementOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.client_route_enforcement_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_log_options` after provisioning.\n"]
    pub fn connection_log_options(&self) -> ListRef<Ec2ClientVpnEndpointConnectionLogOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connection_log_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpointAuthenticationOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_certificate_chain_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saml_provider_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_service_saml_provider_arn: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl Ec2ClientVpnEndpointAuthenticationOptionsEl {
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
}

impl ToListMappable for Ec2ClientVpnEndpointAuthenticationOptionsEl {
    type O = BlockAssignable<Ec2ClientVpnEndpointAuthenticationOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpointAuthenticationOptionsEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildEc2ClientVpnEndpointAuthenticationOptionsEl {
    pub fn build(self) -> Ec2ClientVpnEndpointAuthenticationOptionsEl {
        Ec2ClientVpnEndpointAuthenticationOptionsEl {
            active_directory_id: core::default::Default::default(),
            root_certificate_chain_arn: core::default::Default::default(),
            saml_provider_arn: core::default::Default::default(),
            self_service_saml_provider_arn: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct Ec2ClientVpnEndpointAuthenticationOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2ClientVpnEndpointAuthenticationOptionsElRef {
    fn new(shared: StackShared, base: String) -> Ec2ClientVpnEndpointAuthenticationOptionsElRef {
        Ec2ClientVpnEndpointAuthenticationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpointAuthenticationOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_directory_id` after provisioning.\n"]
    pub fn active_directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.active_directory_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_certificate_chain_arn` after provisioning.\n"]
    pub fn root_certificate_chain_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_certificate_chain_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `saml_provider_arn` after provisioning.\n"]
    pub fn saml_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.saml_provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `self_service_saml_provider_arn` after provisioning.\n"]
    pub fn self_service_saml_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_service_saml_provider_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpointClientConnectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_function_arn: Option<PrimField<String>>,
}

impl Ec2ClientVpnEndpointClientConnectOptionsEl {
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

impl ToListMappable for Ec2ClientVpnEndpointClientConnectOptionsEl {
    type O = BlockAssignable<Ec2ClientVpnEndpointClientConnectOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpointClientConnectOptionsEl {}

impl BuildEc2ClientVpnEndpointClientConnectOptionsEl {
    pub fn build(self) -> Ec2ClientVpnEndpointClientConnectOptionsEl {
        Ec2ClientVpnEndpointClientConnectOptionsEl {
            enabled: core::default::Default::default(),
            lambda_function_arn: core::default::Default::default(),
        }
    }
}

pub struct Ec2ClientVpnEndpointClientConnectOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2ClientVpnEndpointClientConnectOptionsElRef {
    fn new(shared: StackShared, base: String) -> Ec2ClientVpnEndpointClientConnectOptionsElRef {
        Ec2ClientVpnEndpointClientConnectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpointClientConnectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_function_arn` after provisioning.\n"]
    pub fn lambda_function_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_function_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    banner_text: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
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

impl ToListMappable for Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
    type O = BlockAssignable<Ec2ClientVpnEndpointClientLoginBannerOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpointClientLoginBannerOptionsEl {}

impl BuildEc2ClientVpnEndpointClientLoginBannerOptionsEl {
    pub fn build(self) -> Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
        Ec2ClientVpnEndpointClientLoginBannerOptionsEl {
            banner_text: core::default::Default::default(),
            enabled: core::default::Default::default(),
        }
    }
}

pub struct Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
    fn new(shared: StackShared, base: String) -> Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
        Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpointClientLoginBannerOptionsElRef {
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
pub struct Ec2ClientVpnEndpointClientRouteEnforcementOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enforced: Option<PrimField<bool>>,
}

impl Ec2ClientVpnEndpointClientRouteEnforcementOptionsEl {
    #[doc = "Set the field `enforced`.\n"]
    pub fn set_enforced(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforced = Some(v.into());
        self
    }
}

impl ToListMappable for Ec2ClientVpnEndpointClientRouteEnforcementOptionsEl {
    type O = BlockAssignable<Ec2ClientVpnEndpointClientRouteEnforcementOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpointClientRouteEnforcementOptionsEl {}

impl BuildEc2ClientVpnEndpointClientRouteEnforcementOptionsEl {
    pub fn build(self) -> Ec2ClientVpnEndpointClientRouteEnforcementOptionsEl {
        Ec2ClientVpnEndpointClientRouteEnforcementOptionsEl { enforced: core::default::Default::default() }
    }
}

pub struct Ec2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
    fn new(shared: StackShared, base: String) -> Ec2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
        Ec2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpointClientRouteEnforcementOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforced` after provisioning.\n"]
    pub fn enforced(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforced", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2ClientVpnEndpointConnectionLogOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_stream: Option<PrimField<String>>,
    enabled: PrimField<bool>,
}

impl Ec2ClientVpnEndpointConnectionLogOptionsEl {
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
}

impl ToListMappable for Ec2ClientVpnEndpointConnectionLogOptionsEl {
    type O = BlockAssignable<Ec2ClientVpnEndpointConnectionLogOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2ClientVpnEndpointConnectionLogOptionsEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
}

impl BuildEc2ClientVpnEndpointConnectionLogOptionsEl {
    pub fn build(self) -> Ec2ClientVpnEndpointConnectionLogOptionsEl {
        Ec2ClientVpnEndpointConnectionLogOptionsEl {
            cloudwatch_log_group: core::default::Default::default(),
            cloudwatch_log_stream: core::default::Default::default(),
            enabled: self.enabled,
        }
    }
}

pub struct Ec2ClientVpnEndpointConnectionLogOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Ec2ClientVpnEndpointConnectionLogOptionsElRef {
    fn new(shared: StackShared, base: String) -> Ec2ClientVpnEndpointConnectionLogOptionsElRef {
        Ec2ClientVpnEndpointConnectionLogOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2ClientVpnEndpointConnectionLogOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group` after provisioning.\n"]
    pub fn cloudwatch_log_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_stream` after provisioning.\n"]
    pub fn cloudwatch_log_stream(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_stream", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct Ec2ClientVpnEndpointDynamic {
    authentication_options: Option<DynamicBlock<Ec2ClientVpnEndpointAuthenticationOptionsEl>>,
    client_connect_options: Option<DynamicBlock<Ec2ClientVpnEndpointClientConnectOptionsEl>>,
    client_login_banner_options: Option<DynamicBlock<Ec2ClientVpnEndpointClientLoginBannerOptionsEl>>,
    client_route_enforcement_options: Option<DynamicBlock<Ec2ClientVpnEndpointClientRouteEnforcementOptionsEl>>,
    connection_log_options: Option<DynamicBlock<Ec2ClientVpnEndpointConnectionLogOptionsEl>>,
}
