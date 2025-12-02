use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataMedialiveInputData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataMedialiveInput_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMedialiveInputData>,
}
#[derive(Clone)]
pub struct DataMedialiveInput(Rc<DataMedialiveInput_>);
impl DataMedialiveInput {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `attached_channels` after provisioning.\n"]
    pub fn attached_channels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attached_channels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> ListRef<DataMedialiveInputDestinationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destinations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_class` after provisioning.\n"]
    pub fn input_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_devices` after provisioning.\n"]
    pub fn input_devices(&self) -> ListRef<DataMedialiveInputInputDevicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_devices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_partner_ids` after provisioning.\n"]
    pub fn input_partner_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_partner_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_source_type` after provisioning.\n"]
    pub fn input_source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_source_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `media_connect_flows` after provisioning.\n"]
    pub fn media_connect_flows(&self) -> ListRef<DataMedialiveInputMediaConnectFlowsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.media_connect_flows", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> ListRef<DataMedialiveInputSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
impl Referable for DataMedialiveInput {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataMedialiveInput {}
impl ToListMappable for DataMedialiveInput {
    type O = ListRef<DataMedialiveInputRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataMedialiveInput_ {
    fn extract_datasource_type(&self) -> String {
        "aws_medialive_input".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataMedialiveInput {
    pub tf_id: String,
    #[doc = ""]
    pub id: PrimField<String>,
}
impl BuildDataMedialiveInput {
    pub fn build(self, stack: &mut Stack) -> DataMedialiveInput {
        let out = DataMedialiveInput(Rc::new(DataMedialiveInput_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMedialiveInputData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataMedialiveInputRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMedialiveInputRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataMedialiveInputRef {
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
    #[doc = "Get a reference to the value of field `attached_channels` after provisioning.\n"]
    pub fn attached_channels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attached_channels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> ListRef<DataMedialiveInputDestinationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.destinations", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `input_class` after provisioning.\n"]
    pub fn input_class(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_class", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_devices` after provisioning.\n"]
    pub fn input_devices(&self) -> ListRef<DataMedialiveInputInputDevicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_devices", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_partner_ids` after provisioning.\n"]
    pub fn input_partner_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_partner_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `input_source_type` after provisioning.\n"]
    pub fn input_source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.input_source_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `media_connect_flows` after provisioning.\n"]
    pub fn media_connect_flows(&self) -> ListRef<DataMedialiveInputMediaConnectFlowsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.media_connect_flows", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_groups", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> ListRef<DataMedialiveInputSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sources", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataMedialiveInputDestinationsElVpcEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface_id: Option<PrimField<String>>,
}
impl DataMedialiveInputDestinationsElVpcEl {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_interface_id = Some(v.into());
        self
    }
}
impl ToListMappable for DataMedialiveInputDestinationsElVpcEl {
    type O = BlockAssignable<DataMedialiveInputDestinationsElVpcEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMedialiveInputDestinationsElVpcEl {}
impl BuildDataMedialiveInputDestinationsElVpcEl {
    pub fn build(self) -> DataMedialiveInputDestinationsElVpcEl {
        DataMedialiveInputDestinationsElVpcEl {
            availability_zone: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
        }
    }
}
pub struct DataMedialiveInputDestinationsElVpcElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMedialiveInputDestinationsElVpcElRef {
    fn new(shared: StackShared, base: String) -> DataMedialiveInputDestinationsElVpcElRef {
        DataMedialiveInputDestinationsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMedialiveInputDestinationsElVpcElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.network_interface_id", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataMedialiveInputDestinationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc: Option<ListField<DataMedialiveInputDestinationsElVpcEl>>,
}
impl DataMedialiveInputDestinationsEl {
    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip = Some(v.into());
        self
    }
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl Into<ListField<DataMedialiveInputDestinationsElVpcEl>>,
    ) -> Self {
        self.vpc = Some(v.into());
        self
    }
}
impl ToListMappable for DataMedialiveInputDestinationsEl {
    type O = BlockAssignable<DataMedialiveInputDestinationsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMedialiveInputDestinationsEl {}
impl BuildDataMedialiveInputDestinationsEl {
    pub fn build(self) -> DataMedialiveInputDestinationsEl {
        DataMedialiveInputDestinationsEl {
            ip: core::default::Default::default(),
            port: core::default::Default::default(),
            url: core::default::Default::default(),
            vpc: core::default::Default::default(),
        }
    }
}
pub struct DataMedialiveInputDestinationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMedialiveInputDestinationsElRef {
    fn new(shared: StackShared, base: String) -> DataMedialiveInputDestinationsElRef {
        DataMedialiveInputDestinationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMedialiveInputDestinationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> ListRef<DataMedialiveInputDestinationsElVpcElRef> {
        ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMedialiveInputInputDevicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}
impl DataMedialiveInputInputDevicesEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
}
impl ToListMappable for DataMedialiveInputInputDevicesEl {
    type O = BlockAssignable<DataMedialiveInputInputDevicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMedialiveInputInputDevicesEl {}
impl BuildDataMedialiveInputInputDevicesEl {
    pub fn build(self) -> DataMedialiveInputInputDevicesEl {
        DataMedialiveInputInputDevicesEl {
            id: core::default::Default::default(),
        }
    }
}
pub struct DataMedialiveInputInputDevicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMedialiveInputInputDevicesElRef {
    fn new(shared: StackShared, base: String) -> DataMedialiveInputInputDevicesElRef {
        DataMedialiveInputInputDevicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMedialiveInputInputDevicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMedialiveInputMediaConnectFlowsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    flow_arn: Option<PrimField<String>>,
}
impl DataMedialiveInputMediaConnectFlowsEl {
    #[doc = "Set the field `flow_arn`.\n"]
    pub fn set_flow_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flow_arn = Some(v.into());
        self
    }
}
impl ToListMappable for DataMedialiveInputMediaConnectFlowsEl {
    type O = BlockAssignable<DataMedialiveInputMediaConnectFlowsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMedialiveInputMediaConnectFlowsEl {}
impl BuildDataMedialiveInputMediaConnectFlowsEl {
    pub fn build(self) -> DataMedialiveInputMediaConnectFlowsEl {
        DataMedialiveInputMediaConnectFlowsEl {
            flow_arn: core::default::Default::default(),
        }
    }
}
pub struct DataMedialiveInputMediaConnectFlowsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMedialiveInputMediaConnectFlowsElRef {
    fn new(shared: StackShared, base: String) -> DataMedialiveInputMediaConnectFlowsElRef {
        DataMedialiveInputMediaConnectFlowsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMedialiveInputMediaConnectFlowsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `flow_arn` after provisioning.\n"]
    pub fn flow_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_arn", self.base))
    }
}
#[derive(Serialize)]
pub struct DataMedialiveInputSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password_param: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}
impl DataMedialiveInputSourcesEl {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password_param = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}
impl ToListMappable for DataMedialiveInputSourcesEl {
    type O = BlockAssignable<DataMedialiveInputSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataMedialiveInputSourcesEl {}
impl BuildDataMedialiveInputSourcesEl {
    pub fn build(self) -> DataMedialiveInputSourcesEl {
        DataMedialiveInputSourcesEl {
            password_param: core::default::Default::default(),
            url: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}
pub struct DataMedialiveInputSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataMedialiveInputSourcesElRef {
    fn new(shared: StackShared, base: String) -> DataMedialiveInputSourcesElRef {
        DataMedialiveInputSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataMedialiveInputSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.password_param", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
