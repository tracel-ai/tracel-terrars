use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct QuicksightKeyRegistrationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_registration: Option<Vec<QuicksightKeyRegistrationKeyRegistrationEl>>,
    dynamic: QuicksightKeyRegistrationDynamic,
}
struct QuicksightKeyRegistration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<QuicksightKeyRegistrationData>,
}
#[derive(Clone)]
pub struct QuicksightKeyRegistration(Rc<QuicksightKeyRegistration_>);
impl QuicksightKeyRegistration {
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
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `key_registration`.\n"]
    pub fn set_key_registration(
        self,
        v: impl Into<BlockAssignable<QuicksightKeyRegistrationKeyRegistrationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().key_registration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.key_registration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
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
impl Referable for QuicksightKeyRegistration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for QuicksightKeyRegistration {}
impl ToListMappable for QuicksightKeyRegistration {
    type O = ListRef<QuicksightKeyRegistrationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for QuicksightKeyRegistration_ {
    fn extract_resource_type(&self) -> String {
        "aws_quicksight_key_registration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildQuicksightKeyRegistration {
    pub tf_id: String,
}
impl BuildQuicksightKeyRegistration {
    pub fn build(self, stack: &mut Stack) -> QuicksightKeyRegistration {
        let out = QuicksightKeyRegistration(Rc::new(QuicksightKeyRegistration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(QuicksightKeyRegistrationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aws_account_id: core::default::Default::default(),
                region: core::default::Default::default(),
                key_registration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct QuicksightKeyRegistrationRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightKeyRegistrationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl QuicksightKeyRegistrationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
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
pub struct QuicksightKeyRegistrationKeyRegistrationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_key: Option<PrimField<bool>>,
    key_arn: PrimField<String>,
}
impl QuicksightKeyRegistrationKeyRegistrationEl {
    #[doc = "Set the field `default_key`.\n"]
    pub fn set_default_key(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.default_key = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightKeyRegistrationKeyRegistrationEl {
    type O = BlockAssignable<QuicksightKeyRegistrationKeyRegistrationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightKeyRegistrationKeyRegistrationEl {
    #[doc = ""]
    pub key_arn: PrimField<String>,
}
impl BuildQuicksightKeyRegistrationKeyRegistrationEl {
    pub fn build(self) -> QuicksightKeyRegistrationKeyRegistrationEl {
        QuicksightKeyRegistrationKeyRegistrationEl {
            default_key: core::default::Default::default(),
            key_arn: self.key_arn,
        }
    }
}
pub struct QuicksightKeyRegistrationKeyRegistrationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightKeyRegistrationKeyRegistrationElRef {
    fn new(shared: StackShared, base: String) -> QuicksightKeyRegistrationKeyRegistrationElRef {
        QuicksightKeyRegistrationKeyRegistrationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightKeyRegistrationKeyRegistrationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_key` after provisioning.\n"]
    pub fn default_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_key", self.base))
    }
    #[doc = "Get a reference to the value of field `key_arn` after provisioning.\n"]
    pub fn key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightKeyRegistrationDynamic {
    key_registration: Option<DynamicBlock<QuicksightKeyRegistrationKeyRegistrationEl>>,
}
