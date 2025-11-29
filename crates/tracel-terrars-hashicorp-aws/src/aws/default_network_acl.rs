use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DefaultNetworkAclData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    default_network_acl_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<Vec<DefaultNetworkAclEgressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress: Option<Vec<DefaultNetworkAclIngressEl>>,
    dynamic: DefaultNetworkAclDynamic,
}

struct DefaultNetworkAcl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DefaultNetworkAclData>,
}

#[derive(Clone)]
pub struct DefaultNetworkAcl(Rc<DefaultNetworkAcl_>);

impl DefaultNetworkAcl {
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

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().subnet_ids = Some(v.into());
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

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(self, v: impl Into<BlockAssignable<DefaultNetworkAclEgressEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().egress = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.egress = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(self, v: impl Into<BlockAssignable<DefaultNetworkAclIngressEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_network_acl_id` after provisioning.\n"]
    pub fn default_network_acl_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_network_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

impl Referable for DefaultNetworkAcl {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DefaultNetworkAcl { }

impl ToListMappable for DefaultNetworkAcl {
    type O = ListRef<DefaultNetworkAclRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DefaultNetworkAcl_ {
    fn extract_resource_type(&self) -> String {
        "aws_default_network_acl".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDefaultNetworkAcl {
    pub tf_id: String,
    #[doc = ""]
    pub default_network_acl_id: PrimField<String>,
}

impl BuildDefaultNetworkAcl {
    pub fn build(self, stack: &mut Stack) -> DefaultNetworkAcl {
        let out = DefaultNetworkAcl(Rc::new(DefaultNetworkAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DefaultNetworkAclData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                default_network_acl_id: self.default_network_acl_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                egress: core::default::Default::default(),
                ingress: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DefaultNetworkAclRef {
    shared: StackShared,
    base: String,
}

impl Ref for DefaultNetworkAclRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DefaultNetworkAclRef {
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

    #[doc = "Get a reference to the value of field `default_network_acl_id` after provisioning.\n"]
    pub fn default_network_acl_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_network_acl_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DefaultNetworkAclEgressEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    from_port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_type: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    protocol: PrimField<String>,
    rule_no: PrimField<f64>,
    to_port: PrimField<f64>,
}

impl DefaultNetworkAclEgressEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_code`.\n"]
    pub fn set_icmp_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.icmp_code = Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_type`.\n"]
    pub fn set_icmp_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.icmp_type = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_cidr_block = Some(v.into());
        self
    }
}

impl ToListMappable for DefaultNetworkAclEgressEl {
    type O = BlockAssignable<DefaultNetworkAclEgressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDefaultNetworkAclEgressEl {
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub rule_no: PrimField<f64>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}

impl BuildDefaultNetworkAclEgressEl {
    pub fn build(self) -> DefaultNetworkAclEgressEl {
        DefaultNetworkAclEgressEl {
            action: self.action,
            cidr_block: core::default::Default::default(),
            from_port: self.from_port,
            icmp_code: core::default::Default::default(),
            icmp_type: core::default::Default::default(),
            ipv6_cidr_block: core::default::Default::default(),
            protocol: self.protocol,
            rule_no: self.rule_no,
            to_port: self.to_port,
        }
    }
}

pub struct DefaultNetworkAclEgressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DefaultNetworkAclEgressElRef {
    fn new(shared: StackShared, base: String) -> DefaultNetworkAclEgressElRef {
        DefaultNetworkAclEgressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DefaultNetworkAclEgressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_code` after provisioning.\n"]
    pub fn icmp_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.icmp_code", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type` after provisioning.\n"]
    pub fn icmp_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.icmp_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_no` after provisioning.\n"]
    pub fn rule_no(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_no", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DefaultNetworkAclIngressEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    from_port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_type: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    protocol: PrimField<String>,
    rule_no: PrimField<f64>,
    to_port: PrimField<f64>,
}

impl DefaultNetworkAclIngressEl {
    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_code`.\n"]
    pub fn set_icmp_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.icmp_code = Some(v.into());
        self
    }

    #[doc = "Set the field `icmp_type`.\n"]
    pub fn set_icmp_type(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.icmp_type = Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_cidr_block = Some(v.into());
        self
    }
}

impl ToListMappable for DefaultNetworkAclIngressEl {
    type O = BlockAssignable<DefaultNetworkAclIngressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDefaultNetworkAclIngressEl {
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub from_port: PrimField<f64>,
    #[doc = ""]
    pub protocol: PrimField<String>,
    #[doc = ""]
    pub rule_no: PrimField<f64>,
    #[doc = ""]
    pub to_port: PrimField<f64>,
}

impl BuildDefaultNetworkAclIngressEl {
    pub fn build(self) -> DefaultNetworkAclIngressEl {
        DefaultNetworkAclIngressEl {
            action: self.action,
            cidr_block: core::default::Default::default(),
            from_port: self.from_port,
            icmp_code: core::default::Default::default(),
            icmp_type: core::default::Default::default(),
            ipv6_cidr_block: core::default::Default::default(),
            protocol: self.protocol,
            rule_no: self.rule_no,
            to_port: self.to_port,
        }
    }
}

pub struct DefaultNetworkAclIngressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DefaultNetworkAclIngressElRef {
    fn new(shared: StackShared, base: String) -> DefaultNetworkAclIngressElRef {
        DefaultNetworkAclIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DefaultNetworkAclIngressElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_code` after provisioning.\n"]
    pub fn icmp_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.icmp_code", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type` after provisioning.\n"]
    pub fn icmp_type(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.icmp_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_no` after provisioning.\n"]
    pub fn rule_no(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_no", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct DefaultNetworkAclDynamic {
    egress: Option<DynamicBlock<DefaultNetworkAclEgressEl>>,
    ingress: Option<DynamicBlock<DefaultNetworkAclIngressEl>>,
}
