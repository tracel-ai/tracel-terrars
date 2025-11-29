use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ServicecatalogPrincipalPortfolioAssociationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accept_language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    portfolio_id: PrimField<String>,
    principal_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ServicecatalogPrincipalPortfolioAssociationTimeoutsEl>,
}

struct ServicecatalogPrincipalPortfolioAssociation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ServicecatalogPrincipalPortfolioAssociationData>,
}

#[derive(Clone)]
pub struct ServicecatalogPrincipalPortfolioAssociation(Rc<ServicecatalogPrincipalPortfolioAssociation_>);

impl ServicecatalogPrincipalPortfolioAssociation {
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

    #[doc = "Set the field `accept_language`.\n"]
    pub fn set_accept_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().accept_language = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `principal_type`.\n"]
    pub fn set_principal_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().principal_type = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ServicecatalogPrincipalPortfolioAssociationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.portfolio_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_arn` after provisioning.\n"]
    pub fn principal_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ServicecatalogPrincipalPortfolioAssociationTimeoutsElRef {
        ServicecatalogPrincipalPortfolioAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ServicecatalogPrincipalPortfolioAssociation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ServicecatalogPrincipalPortfolioAssociation { }

impl ToListMappable for ServicecatalogPrincipalPortfolioAssociation {
    type O = ListRef<ServicecatalogPrincipalPortfolioAssociationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ServicecatalogPrincipalPortfolioAssociation_ {
    fn extract_resource_type(&self) -> String {
        "aws_servicecatalog_principal_portfolio_association".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServicecatalogPrincipalPortfolioAssociation {
    pub tf_id: String,
    #[doc = ""]
    pub portfolio_id: PrimField<String>,
    #[doc = ""]
    pub principal_arn: PrimField<String>,
}

impl BuildServicecatalogPrincipalPortfolioAssociation {
    pub fn build(self, stack: &mut Stack) -> ServicecatalogPrincipalPortfolioAssociation {
        let out = ServicecatalogPrincipalPortfolioAssociation(Rc::new(ServicecatalogPrincipalPortfolioAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ServicecatalogPrincipalPortfolioAssociationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                accept_language: core::default::Default::default(),
                id: core::default::Default::default(),
                portfolio_id: self.portfolio_id,
                principal_arn: self.principal_arn,
                principal_type: core::default::Default::default(),
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServicecatalogPrincipalPortfolioAssociationRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicecatalogPrincipalPortfolioAssociationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ServicecatalogPrincipalPortfolioAssociationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accept_language` after provisioning.\n"]
    pub fn accept_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.accept_language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `portfolio_id` after provisioning.\n"]
    pub fn portfolio_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.portfolio_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_arn` after provisioning.\n"]
    pub fn principal_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ServicecatalogPrincipalPortfolioAssociationTimeoutsElRef {
        ServicecatalogPrincipalPortfolioAssociationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ServicecatalogPrincipalPortfolioAssociationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl ServicecatalogPrincipalPortfolioAssociationTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for ServicecatalogPrincipalPortfolioAssociationTimeoutsEl {
    type O = BlockAssignable<ServicecatalogPrincipalPortfolioAssociationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServicecatalogPrincipalPortfolioAssociationTimeoutsEl {}

impl BuildServicecatalogPrincipalPortfolioAssociationTimeoutsEl {
    pub fn build(self) -> ServicecatalogPrincipalPortfolioAssociationTimeoutsEl {
        ServicecatalogPrincipalPortfolioAssociationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
        }
    }
}

pub struct ServicecatalogPrincipalPortfolioAssociationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServicecatalogPrincipalPortfolioAssociationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ServicecatalogPrincipalPortfolioAssociationTimeoutsElRef {
        ServicecatalogPrincipalPortfolioAssociationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServicecatalogPrincipalPortfolioAssociationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
