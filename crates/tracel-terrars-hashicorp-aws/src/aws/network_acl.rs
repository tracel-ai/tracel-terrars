use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkAclData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress: Option<SetField<NetworkAclEgressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress: Option<SetField<NetworkAclIngressEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    vpc_id: PrimField<String>,
}

struct NetworkAcl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkAclData>,
}

#[derive(Clone)]
pub struct NetworkAcl(Rc<NetworkAcl_>);

impl NetworkAcl {
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

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(self, v: impl Into<SetField<NetworkAclEgressEl>>) -> Self {
        self.0.data.borrow_mut().egress = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `ingress`.\n"]
    pub fn set_ingress(self, v: impl Into<SetField<NetworkAclIngressEl>>) -> Self {
        self.0.data.borrow_mut().ingress = Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> SetRef<NetworkAclEgressElRef> {
        SetRef::new(self.shared().clone(), format!("{}.egress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> SetRef<NetworkAclIngressElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
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

impl Referable for NetworkAcl {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for NetworkAcl { }

impl ToListMappable for NetworkAcl {
    type O = ListRef<NetworkAclRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkAcl_ {
    fn extract_resource_type(&self) -> String {
        "aws_network_acl".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkAcl {
    pub tf_id: String,
    #[doc = ""]
    pub vpc_id: PrimField<String>,
}

impl BuildNetworkAcl {
    pub fn build(self, stack: &mut Stack) -> NetworkAcl {
        let out = NetworkAcl(Rc::new(NetworkAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkAclData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                egress: core::default::Default::default(),
                id: core::default::Default::default(),
                ingress: core::default::Default::default(),
                region: core::default::Default::default(),
                subnet_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: self.vpc_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkAclRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkAclRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl NetworkAclRef {
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

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> SetRef<NetworkAclEgressElRef> {
        SetRef::new(self.shared().clone(), format!("{}.egress", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress` after provisioning.\n"]
    pub fn ingress(&self) -> SetRef<NetworkAclIngressElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ingress", self.extract_ref()))
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
pub struct NetworkAclEgressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_type: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_no: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl NetworkAclEgressEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
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

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_no`.\n"]
    pub fn set_rule_no(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rule_no = Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkAclEgressEl {
    type O = BlockAssignable<NetworkAclEgressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkAclEgressEl {}

impl BuildNetworkAclEgressEl {
    pub fn build(self) -> NetworkAclEgressEl {
        NetworkAclEgressEl {
            action: core::default::Default::default(),
            cidr_block: core::default::Default::default(),
            from_port: core::default::Default::default(),
            icmp_code: core::default::Default::default(),
            icmp_type: core::default::Default::default(),
            ipv6_cidr_block: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_no: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct NetworkAclEgressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkAclEgressElRef {
    fn new(shared: StackShared, base: String) -> NetworkAclEgressElRef {
        NetworkAclEgressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkAclEgressElRef {
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
pub struct NetworkAclIngressEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icmp_type: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_cidr_block: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_no: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_port: Option<PrimField<f64>>,
}

impl NetworkAclIngressEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr_block = Some(v.into());
        self
    }

    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.from_port = Some(v.into());
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

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc = "Set the field `rule_no`.\n"]
    pub fn set_rule_no(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rule_no = Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.to_port = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkAclIngressEl {
    type O = BlockAssignable<NetworkAclIngressEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkAclIngressEl {}

impl BuildNetworkAclIngressEl {
    pub fn build(self) -> NetworkAclIngressEl {
        NetworkAclIngressEl {
            action: core::default::Default::default(),
            cidr_block: core::default::Default::default(),
            from_port: core::default::Default::default(),
            icmp_code: core::default::Default::default(),
            icmp_type: core::default::Default::default(),
            ipv6_cidr_block: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_no: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

pub struct NetworkAclIngressElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkAclIngressElRef {
    fn new(shared: StackShared, base: String) -> NetworkAclIngressElRef {
        NetworkAclIngressElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkAclIngressElRef {
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
