use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDxRouterConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    router_type_identifier: PrimField<String>,
    virtual_interface_id: PrimField<String>,
}

struct DataDxRouterConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDxRouterConfigurationData>,
}

#[derive(Clone)]
pub struct DataDxRouterConfiguration(Rc<DataDxRouterConfiguration_>);

impl DataDxRouterConfiguration {
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

    #[doc = "Get a reference to the value of field `customer_router_config` after provisioning.\n"]
    pub fn customer_router_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_router_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `router` after provisioning.\n"]
    pub fn router(&self) -> ListRef<DataDxRouterConfigurationRouterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router_type_identifier` after provisioning.\n"]
    pub fn router_type_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.router_type_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_interface_id` after provisioning.\n"]
    pub fn virtual_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_interface_name` after provisioning.\n"]
    pub fn virtual_interface_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_interface_name", self.extract_ref()))
    }
}

impl Referable for DataDxRouterConfiguration {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataDxRouterConfiguration { }

impl ToListMappable for DataDxRouterConfiguration {
    type O = ListRef<DataDxRouterConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDxRouterConfiguration_ {
    fn extract_datasource_type(&self) -> String {
        "aws_dx_router_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDxRouterConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub router_type_identifier: PrimField<String>,
    #[doc = ""]
    pub virtual_interface_id: PrimField<String>,
}

impl BuildDataDxRouterConfiguration {
    pub fn build(self, stack: &mut Stack) -> DataDxRouterConfiguration {
        let out = DataDxRouterConfiguration(Rc::new(DataDxRouterConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDxRouterConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                router_type_identifier: self.router_type_identifier,
                virtual_interface_id: self.virtual_interface_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDxRouterConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDxRouterConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataDxRouterConfigurationRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `customer_router_config` after provisioning.\n"]
    pub fn customer_router_config(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_router_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `router` after provisioning.\n"]
    pub fn router(&self) -> ListRef<DataDxRouterConfigurationRouterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.router", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `router_type_identifier` after provisioning.\n"]
    pub fn router_type_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.router_type_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_interface_id` after provisioning.\n"]
    pub fn virtual_interface_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_interface_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_interface_name` after provisioning.\n"]
    pub fn virtual_interface_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_interface_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDxRouterConfigurationRouterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    router_type_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xslt_template_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xslt_template_name_for_mac_sec: Option<PrimField<String>>,
}

impl DataDxRouterConfigurationRouterEl {
    #[doc = "Set the field `platform`.\n"]
    pub fn set_platform(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.platform = Some(v.into());
        self
    }

    #[doc = "Set the field `router_type_identifier`.\n"]
    pub fn set_router_type_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.router_type_identifier = Some(v.into());
        self
    }

    #[doc = "Set the field `software`.\n"]
    pub fn set_software(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.software = Some(v.into());
        self
    }

    #[doc = "Set the field `vendor`.\n"]
    pub fn set_vendor(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vendor = Some(v.into());
        self
    }

    #[doc = "Set the field `xslt_template_name`.\n"]
    pub fn set_xslt_template_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.xslt_template_name = Some(v.into());
        self
    }

    #[doc = "Set the field `xslt_template_name_for_mac_sec`.\n"]
    pub fn set_xslt_template_name_for_mac_sec(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.xslt_template_name_for_mac_sec = Some(v.into());
        self
    }
}

impl ToListMappable for DataDxRouterConfigurationRouterEl {
    type O = BlockAssignable<DataDxRouterConfigurationRouterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDxRouterConfigurationRouterEl {}

impl BuildDataDxRouterConfigurationRouterEl {
    pub fn build(self) -> DataDxRouterConfigurationRouterEl {
        DataDxRouterConfigurationRouterEl {
            platform: core::default::Default::default(),
            router_type_identifier: core::default::Default::default(),
            software: core::default::Default::default(),
            vendor: core::default::Default::default(),
            xslt_template_name: core::default::Default::default(),
            xslt_template_name_for_mac_sec: core::default::Default::default(),
        }
    }
}

pub struct DataDxRouterConfigurationRouterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDxRouterConfigurationRouterElRef {
    fn new(shared: StackShared, base: String) -> DataDxRouterConfigurationRouterElRef {
        DataDxRouterConfigurationRouterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDxRouterConfigurationRouterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.platform", self.base))
    }

    #[doc = "Get a reference to the value of field `router_type_identifier` after provisioning.\n"]
    pub fn router_type_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.router_type_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `software` after provisioning.\n"]
    pub fn software(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.software", self.base))
    }

    #[doc = "Get a reference to the value of field `vendor` after provisioning.\n"]
    pub fn vendor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vendor", self.base))
    }

    #[doc = "Get a reference to the value of field `xslt_template_name` after provisioning.\n"]
    pub fn xslt_template_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.xslt_template_name", self.base))
    }

    #[doc = "Get a reference to the value of field `xslt_template_name_for_mac_sec` after provisioning.\n"]
    pub fn xslt_template_name_for_mac_sec(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.xslt_template_name_for_mac_sec", self.base))
    }
}
