use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct OpensearchAuthorizeVpcEndpointAccessData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account: PrimField<String>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct OpensearchAuthorizeVpcEndpointAccess_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpensearchAuthorizeVpcEndpointAccessData>,
}

#[derive(Clone)]
pub struct OpensearchAuthorizeVpcEndpointAccess(Rc<OpensearchAuthorizeVpcEndpointAccess_>);

impl OpensearchAuthorizeVpcEndpointAccess {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorized_principal` after provisioning.\n"]
    pub fn authorized_principal(
        &self,
    ) -> ListRef<OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_principal", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}

impl Referable for OpensearchAuthorizeVpcEndpointAccess {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for OpensearchAuthorizeVpcEndpointAccess {}

impl ToListMappable for OpensearchAuthorizeVpcEndpointAccess {
    type O = ListRef<OpensearchAuthorizeVpcEndpointAccessRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for OpensearchAuthorizeVpcEndpointAccess_ {
    fn extract_resource_type(&self) -> String {
        "aws_opensearch_authorize_vpc_endpoint_access".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpensearchAuthorizeVpcEndpointAccess {
    pub tf_id: String,
    #[doc = ""]
    pub account: PrimField<String>,
    #[doc = ""]
    pub domain_name: PrimField<String>,
}

impl BuildOpensearchAuthorizeVpcEndpointAccess {
    pub fn build(self, stack: &mut Stack) -> OpensearchAuthorizeVpcEndpointAccess {
        let out =
            OpensearchAuthorizeVpcEndpointAccess(Rc::new(OpensearchAuthorizeVpcEndpointAccess_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(OpensearchAuthorizeVpcEndpointAccessData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    account: self.account,
                    domain_name: self.domain_name,
                    region: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpensearchAuthorizeVpcEndpointAccessRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchAuthorizeVpcEndpointAccessRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl OpensearchAuthorizeVpcEndpointAccessRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `authorized_principal` after provisioning.\n"]
    pub fn authorized_principal(
        &self,
    ) -> ListRef<OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authorized_principal", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_type: Option<PrimField<String>>,
}

impl OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalEl {
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal = Some(v.into());
        self
    }

    #[doc = "Set the field `principal_type`.\n"]
    pub fn set_principal_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal_type = Some(v.into());
        self
    }
}

impl ToListMappable for OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalEl {
    type O = BlockAssignable<OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalEl {}

impl BuildOpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalEl {
    pub fn build(self) -> OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalEl {
        OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalEl {
            principal: core::default::Default::default(),
            principal_type: core::default::Default::default(),
        }
    }
}

pub struct OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalElRef {
        OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpensearchAuthorizeVpcEndpointAccessAuthorizedPrincipalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_type", self.base),
        )
    }
}
