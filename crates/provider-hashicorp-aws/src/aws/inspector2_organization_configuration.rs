use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Inspector2OrganizationConfigurationData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_enable: Option<Vec<Inspector2OrganizationConfigurationAutoEnableEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Inspector2OrganizationConfigurationTimeoutsEl>,
    dynamic: Inspector2OrganizationConfigurationDynamic,
}

struct Inspector2OrganizationConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Inspector2OrganizationConfigurationData>,
}

#[derive(Clone)]
pub struct Inspector2OrganizationConfiguration(Rc<Inspector2OrganizationConfiguration_>);

impl Inspector2OrganizationConfiguration {
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

    #[doc = "Set the field `auto_enable`.\n"]
    pub fn set_auto_enable(
        self,
        v: impl Into<BlockAssignable<Inspector2OrganizationConfigurationAutoEnableEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_enable = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_enable = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Inspector2OrganizationConfigurationTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_account_limit_reached` after provisioning.\n"]
    pub fn max_account_limit_reached(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_account_limit_reached", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> ListRef<Inspector2OrganizationConfigurationAutoEnableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Inspector2OrganizationConfigurationTimeoutsElRef {
        Inspector2OrganizationConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for Inspector2OrganizationConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Inspector2OrganizationConfiguration { }

impl ToListMappable for Inspector2OrganizationConfiguration {
    type O = ListRef<Inspector2OrganizationConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Inspector2OrganizationConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_inspector2_organization_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInspector2OrganizationConfiguration {
    pub tf_id: String,
}

impl BuildInspector2OrganizationConfiguration {
    pub fn build(self, stack: &mut Stack) -> Inspector2OrganizationConfiguration {
        let out = Inspector2OrganizationConfiguration(Rc::new(Inspector2OrganizationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Inspector2OrganizationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                auto_enable: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Inspector2OrganizationConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2OrganizationConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Inspector2OrganizationConfigurationRef {
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

    #[doc = "Get a reference to the value of field `max_account_limit_reached` after provisioning.\n"]
    pub fn max_account_limit_reached(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_account_limit_reached", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> ListRef<Inspector2OrganizationConfigurationAutoEnableElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Inspector2OrganizationConfigurationTimeoutsElRef {
        Inspector2OrganizationConfigurationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Inspector2OrganizationConfigurationAutoEnableEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<PrimField<bool>>,
    ec2: PrimField<bool>,
    ecr: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lambda_code: Option<PrimField<bool>>,
}

impl Inspector2OrganizationConfigurationAutoEnableEl {
    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.code_repository = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda`.\n"]
    pub fn set_lambda(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.lambda = Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_code`.\n"]
    pub fn set_lambda_code(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.lambda_code = Some(v.into());
        self
    }
}

impl ToListMappable for Inspector2OrganizationConfigurationAutoEnableEl {
    type O = BlockAssignable<Inspector2OrganizationConfigurationAutoEnableEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2OrganizationConfigurationAutoEnableEl {
    #[doc = ""]
    pub ec2: PrimField<bool>,
    #[doc = ""]
    pub ecr: PrimField<bool>,
}

impl BuildInspector2OrganizationConfigurationAutoEnableEl {
    pub fn build(self) -> Inspector2OrganizationConfigurationAutoEnableEl {
        Inspector2OrganizationConfigurationAutoEnableEl {
            code_repository: core::default::Default::default(),
            ec2: self.ec2,
            ecr: self.ecr,
            lambda: core::default::Default::default(),
            lambda_code: core::default::Default::default(),
        }
    }
}

pub struct Inspector2OrganizationConfigurationAutoEnableElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2OrganizationConfigurationAutoEnableElRef {
    fn new(shared: StackShared, base: String) -> Inspector2OrganizationConfigurationAutoEnableElRef {
        Inspector2OrganizationConfigurationAutoEnableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2OrganizationConfigurationAutoEnableElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code_repository` after provisioning.\n"]
    pub fn code_repository(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.code_repository", self.base))
    }

    #[doc = "Get a reference to the value of field `ec2` after provisioning.\n"]
    pub fn ec2(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ec2", self.base))
    }

    #[doc = "Get a reference to the value of field `ecr` after provisioning.\n"]
    pub fn ecr(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ecr", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda` after provisioning.\n"]
    pub fn lambda(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_code` after provisioning.\n"]
    pub fn lambda_code(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_code", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2OrganizationConfigurationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl Inspector2OrganizationConfigurationTimeoutsEl {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for Inspector2OrganizationConfigurationTimeoutsEl {
    type O = BlockAssignable<Inspector2OrganizationConfigurationTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2OrganizationConfigurationTimeoutsEl {}

impl BuildInspector2OrganizationConfigurationTimeoutsEl {
    pub fn build(self) -> Inspector2OrganizationConfigurationTimeoutsEl {
        Inspector2OrganizationConfigurationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct Inspector2OrganizationConfigurationTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Inspector2OrganizationConfigurationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Inspector2OrganizationConfigurationTimeoutsElRef {
        Inspector2OrganizationConfigurationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2OrganizationConfigurationTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct Inspector2OrganizationConfigurationDynamic {
    auto_enable: Option<DynamicBlock<Inspector2OrganizationConfigurationAutoEnableEl>>,
}
