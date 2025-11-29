use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDirectoryServiceDirectoryData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    directory_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataDirectoryServiceDirectory_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDirectoryServiceDirectoryData>,
}

#[derive(Clone)]
pub struct DataDirectoryServiceDirectory(Rc<DataDirectoryServiceDirectory_>);

impl DataDirectoryServiceDirectory {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `access_url` after provisioning.\n"]
    pub fn access_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_settings` after provisioning.\n"]
    pub fn connect_settings(&self) -> ListRef<DataDirectoryServiceDirectoryConnectSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connect_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_sso` after provisioning.\n"]
    pub fn enable_sso(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_sso", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_settings` after provisioning.\n"]
    pub fn radius_settings(&self) -> ListRef<DataDirectoryServiceDirectoryRadiusSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.radius_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> ListRef<DataDirectoryServiceDirectoryVpcSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_settings", self.extract_ref()))
    }
}

impl Referable for DataDirectoryServiceDirectory {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataDirectoryServiceDirectory { }

impl ToListMappable for DataDirectoryServiceDirectory {
    type O = ListRef<DataDirectoryServiceDirectoryRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDirectoryServiceDirectory_ {
    fn extract_datasource_type(&self) -> String {
        "aws_directory_service_directory".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDirectoryServiceDirectory {
    pub tf_id: String,
    #[doc = ""]
    pub directory_id: PrimField<String>,
}

impl BuildDataDirectoryServiceDirectory {
    pub fn build(self, stack: &mut Stack) -> DataDirectoryServiceDirectory {
        let out = DataDirectoryServiceDirectory(Rc::new(DataDirectoryServiceDirectory_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDirectoryServiceDirectoryData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                directory_id: self.directory_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDirectoryServiceDirectoryRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDirectoryServiceDirectoryRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataDirectoryServiceDirectoryRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_url` after provisioning.\n"]
    pub fn access_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alias` after provisioning.\n"]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connect_settings` after provisioning.\n"]
    pub fn connect_settings(&self) -> ListRef<DataDirectoryServiceDirectoryConnectSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.connect_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_ip_addresses` after provisioning.\n"]
    pub fn dns_ip_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.dns_ip_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.edition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_sso` after provisioning.\n"]
    pub fn enable_sso(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_sso", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `radius_settings` after provisioning.\n"]
    pub fn radius_settings(&self) -> ListRef<DataDirectoryServiceDirectoryRadiusSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.radius_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `short_name` after provisioning.\n"]
    pub fn short_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.short_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_settings` after provisioning.\n"]
    pub fn vpc_settings(&self) -> ListRef<DataDirectoryServiceDirectoryVpcSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDirectoryServiceDirectoryConnectSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zones: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_dns_ips: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_username: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl DataDirectoryServiceDirectoryConnectSettingsEl {
    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.availability_zones = Some(v.into());
        self
    }

    #[doc = "Set the field `connect_ips`.\n"]
    pub fn set_connect_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.connect_ips = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_dns_ips`.\n"]
    pub fn set_customer_dns_ips(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.customer_dns_ips = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_username`.\n"]
    pub fn set_customer_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.customer_username = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataDirectoryServiceDirectoryConnectSettingsEl {
    type O = BlockAssignable<DataDirectoryServiceDirectoryConnectSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDirectoryServiceDirectoryConnectSettingsEl {}

impl BuildDataDirectoryServiceDirectoryConnectSettingsEl {
    pub fn build(self) -> DataDirectoryServiceDirectoryConnectSettingsEl {
        DataDirectoryServiceDirectoryConnectSettingsEl {
            availability_zones: core::default::Default::default(),
            connect_ips: core::default::Default::default(),
            customer_dns_ips: core::default::Default::default(),
            customer_username: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct DataDirectoryServiceDirectoryConnectSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDirectoryServiceDirectoryConnectSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataDirectoryServiceDirectoryConnectSettingsElRef {
        DataDirectoryServiceDirectoryConnectSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDirectoryServiceDirectoryConnectSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `connect_ips` after provisioning.\n"]
    pub fn connect_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.connect_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_dns_ips` after provisioning.\n"]
    pub fn customer_dns_ips(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.customer_dns_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_username` after provisioning.\n"]
    pub fn customer_username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_username", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDirectoryServiceDirectoryRadiusSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius_retries: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius_servers: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_same_username: Option<PrimField<bool>>,
}

impl DataDirectoryServiceDirectoryRadiusSettingsEl {
    #[doc = "Set the field `authentication_protocol`.\n"]
    pub fn set_authentication_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authentication_protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `display_label`.\n"]
    pub fn set_display_label(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_label = Some(v.into());
        self
    }

    #[doc = "Set the field `radius_port`.\n"]
    pub fn set_radius_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.radius_port = Some(v.into());
        self
    }

    #[doc = "Set the field `radius_retries`.\n"]
    pub fn set_radius_retries(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.radius_retries = Some(v.into());
        self
    }

    #[doc = "Set the field `radius_servers`.\n"]
    pub fn set_radius_servers(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.radius_servers = Some(v.into());
        self
    }

    #[doc = "Set the field `radius_timeout`.\n"]
    pub fn set_radius_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.radius_timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `use_same_username`.\n"]
    pub fn set_use_same_username(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_same_username = Some(v.into());
        self
    }
}

impl ToListMappable for DataDirectoryServiceDirectoryRadiusSettingsEl {
    type O = BlockAssignable<DataDirectoryServiceDirectoryRadiusSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDirectoryServiceDirectoryRadiusSettingsEl {}

impl BuildDataDirectoryServiceDirectoryRadiusSettingsEl {
    pub fn build(self) -> DataDirectoryServiceDirectoryRadiusSettingsEl {
        DataDirectoryServiceDirectoryRadiusSettingsEl {
            authentication_protocol: core::default::Default::default(),
            display_label: core::default::Default::default(),
            radius_port: core::default::Default::default(),
            radius_retries: core::default::Default::default(),
            radius_servers: core::default::Default::default(),
            radius_timeout: core::default::Default::default(),
            use_same_username: core::default::Default::default(),
        }
    }
}

pub struct DataDirectoryServiceDirectoryRadiusSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDirectoryServiceDirectoryRadiusSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataDirectoryServiceDirectoryRadiusSettingsElRef {
        DataDirectoryServiceDirectoryRadiusSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDirectoryServiceDirectoryRadiusSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_protocol` after provisioning.\n"]
    pub fn authentication_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.authentication_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `display_label` after provisioning.\n"]
    pub fn display_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_label", self.base))
    }

    #[doc = "Get a reference to the value of field `radius_port` after provisioning.\n"]
    pub fn radius_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.radius_port", self.base))
    }

    #[doc = "Get a reference to the value of field `radius_retries` after provisioning.\n"]
    pub fn radius_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.radius_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `radius_servers` after provisioning.\n"]
    pub fn radius_servers(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.radius_servers", self.base))
    }

    #[doc = "Get a reference to the value of field `radius_timeout` after provisioning.\n"]
    pub fn radius_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.radius_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `use_same_username` after provisioning.\n"]
    pub fn use_same_username(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_same_username", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDirectoryServiceDirectoryVpcSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zones: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_id: Option<PrimField<String>>,
}

impl DataDirectoryServiceDirectoryVpcSettingsEl {
    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.availability_zones = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataDirectoryServiceDirectoryVpcSettingsEl {
    type O = BlockAssignable<DataDirectoryServiceDirectoryVpcSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDirectoryServiceDirectoryVpcSettingsEl {}

impl BuildDataDirectoryServiceDirectoryVpcSettingsEl {
    pub fn build(self) -> DataDirectoryServiceDirectoryVpcSettingsEl {
        DataDirectoryServiceDirectoryVpcSettingsEl {
            availability_zones: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

pub struct DataDirectoryServiceDirectoryVpcSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDirectoryServiceDirectoryVpcSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataDirectoryServiceDirectoryVpcSettingsElRef {
        DataDirectoryServiceDirectoryVpcSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDirectoryServiceDirectoryVpcSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
