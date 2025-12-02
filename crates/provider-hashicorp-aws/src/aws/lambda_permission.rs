use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LambdaPermissionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_source_token: Option<PrimField<String>>,
    function_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_url_auth_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    principal: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principal_org_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qualifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statement_id_prefix: Option<PrimField<String>>,
}
struct LambdaPermission_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LambdaPermissionData>,
}
#[derive(Clone)]
pub struct LambdaPermission(Rc<LambdaPermission_>);
impl LambdaPermission {
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
    #[doc = "Set the field `event_source_token`.\n"]
    pub fn set_event_source_token(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().event_source_token = Some(v.into());
        self
    }
    #[doc = "Set the field `function_url_auth_type`.\n"]
    pub fn set_function_url_auth_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().function_url_auth_type = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `principal_org_id`.\n"]
    pub fn set_principal_org_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().principal_org_id = Some(v.into());
        self
    }
    #[doc = "Set the field `qualifier`.\n"]
    pub fn set_qualifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().qualifier = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `source_account`.\n"]
    pub fn set_source_account(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_account = Some(v.into());
        self
    }
    #[doc = "Set the field `source_arn`.\n"]
    pub fn set_source_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `statement_id`.\n"]
    pub fn set_statement_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().statement_id = Some(v.into());
        self
    }
    #[doc = "Set the field `statement_id_prefix`.\n"]
    pub fn set_statement_id_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().statement_id_prefix = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_source_token` after provisioning.\n"]
    pub fn event_source_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_source_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `function_url_auth_type` after provisioning.\n"]
    pub fn function_url_auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_url_auth_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_org_id` after provisioning.\n"]
    pub fn principal_org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_account` after provisioning.\n"]
    pub fn source_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statement_id` after provisioning.\n"]
    pub fn statement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statement_id_prefix` after provisioning.\n"]
    pub fn statement_id_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_id_prefix", self.extract_ref()),
        )
    }
}
impl Referable for LambdaPermission {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LambdaPermission {}
impl ToListMappable for LambdaPermission {
    type O = ListRef<LambdaPermissionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LambdaPermission_ {
    fn extract_resource_type(&self) -> String {
        "aws_lambda_permission".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLambdaPermission {
    pub tf_id: String,
    #[doc = ""]
    pub action: PrimField<String>,
    #[doc = ""]
    pub function_name: PrimField<String>,
    #[doc = ""]
    pub principal: PrimField<String>,
}
impl BuildLambdaPermission {
    pub fn build(self, stack: &mut Stack) -> LambdaPermission {
        let out = LambdaPermission(Rc::new(LambdaPermission_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LambdaPermissionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                event_source_token: core::default::Default::default(),
                function_name: self.function_name,
                function_url_auth_type: core::default::Default::default(),
                id: core::default::Default::default(),
                principal: self.principal,
                principal_org_id: core::default::Default::default(),
                qualifier: core::default::Default::default(),
                region: core::default::Default::default(),
                source_account: core::default::Default::default(),
                source_arn: core::default::Default::default(),
                statement_id: core::default::Default::default(),
                statement_id_prefix: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LambdaPermissionRef {
    shared: StackShared,
    base: String,
}
impl Ref for LambdaPermissionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LambdaPermissionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `event_source_token` after provisioning.\n"]
    pub fn event_source_token(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_source_token", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `function_url_auth_type` after provisioning.\n"]
    pub fn function_url_auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.function_url_auth_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal_org_id` after provisioning.\n"]
    pub fn principal_org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal_org_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `qualifier` after provisioning.\n"]
    pub fn qualifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.qualifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_account` after provisioning.\n"]
    pub fn source_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statement_id` after provisioning.\n"]
    pub fn statement_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `statement_id_prefix` after provisioning.\n"]
    pub fn statement_id_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.statement_id_prefix", self.extract_ref()),
        )
    }
}
