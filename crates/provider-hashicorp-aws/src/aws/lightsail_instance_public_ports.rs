use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LightsailInstancePublicPortsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_info: Option<Vec<LightsailInstancePublicPortsPortInfoEl>>,
    dynamic: LightsailInstancePublicPortsDynamic,
}

struct LightsailInstancePublicPorts_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LightsailInstancePublicPortsData>,
}

#[derive(Clone)]
pub struct LightsailInstancePublicPorts(Rc<LightsailInstancePublicPorts_>);

impl LightsailInstancePublicPorts {
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

    #[doc = "Set the field `port_info`.\n"]
    pub fn set_port_info(self, v: impl Into<BlockAssignable<LightsailInstancePublicPortsPortInfoEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().port_info = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.port_info = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_name` after provisioning.\n"]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for LightsailInstancePublicPorts {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for LightsailInstancePublicPorts { }

impl ToListMappable for LightsailInstancePublicPorts {
    type O = ListRef<LightsailInstancePublicPortsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for LightsailInstancePublicPorts_ {
    fn extract_resource_type(&self) -> String {
        "aws_lightsail_instance_public_ports".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLightsailInstancePublicPorts {
    pub tf_id: String,
    #[doc = ""]
    pub instance_name: PrimField<String>,
}

impl BuildLightsailInstancePublicPorts {
    pub fn build(self, stack: &mut Stack) -> LightsailInstancePublicPorts {
        let out = LightsailInstancePublicPorts(Rc::new(LightsailInstancePublicPorts_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LightsailInstancePublicPortsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                instance_name: self.instance_name,
                region: core::default::Default::default(),
                port_info: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LightsailInstancePublicPortsRef {
    shared: StackShared,
    base: String,
}

impl Ref for LightsailInstancePublicPortsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl LightsailInstancePublicPortsRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_name` after provisioning.\n"]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LightsailInstancePublicPortsPortInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_list_aliases: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidrs: Option<SetField<PrimField<String>>>,
    from_port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidrs: Option<SetField<PrimField<String>>>,
    protocol: PrimField<String>,
    to_port: PrimField<f64>,
}

impl LightsailInstancePublicPortsPortInfoEl {
    #[doc = "Set the field `cidr_list_aliases`.\n"]
    pub fn set_cidr_list_aliases(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cidr_list_aliases = Some(v.into());
        self
    }

    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.cidrs = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidrs`.\n"]
    pub fn set_ipv6_cidrs(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.ipv6_cidrs = Some(v.into());
        self
    }
}

impl ToListMappable for LightsailInstancePublicPortsPortInfoEl {
    type O = BlockAssignable<LightsailInstancePublicPortsPortInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailInstancePublicPortsPortInfoEl {
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}

impl BuildLightsailInstancePublicPortsPortInfoEl {
    pub fn build(self) -> LightsailInstancePublicPortsPortInfoEl {
        LightsailInstancePublicPortsPortInfoEl {
            cidr_list_aliases: core::default::Default::default(),
            cidrs: core::default::Default::default(),
            from_port: self.from_port,
            ipv6_cidrs: core::default::Default::default(),
            protocol: self.protocol,
            to_port: self.to_port,
        }
    }
}

pub struct LightsailInstancePublicPortsPortInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LightsailInstancePublicPortsPortInfoElRef {
    fn new(shared: StackShared, base: String) -> LightsailInstancePublicPortsPortInfoElRef {
        LightsailInstancePublicPortsPortInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailInstancePublicPortsPortInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_list_aliases` after provisioning.\n"]
    pub fn cidr_list_aliases(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cidr_list_aliases", self.base))
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidrs` after provisioning.\n"]
    pub fn ipv6_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ipv6_cidrs", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct LightsailInstancePublicPortsDynamic {
    port_info: Option<DynamicBlock<LightsailInstancePublicPortsPortInfoEl>>,
}
