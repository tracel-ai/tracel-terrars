use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct QuicksightIamPolicyAssignmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    assignment_name: PrimField<String>,
    assignment_status: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    policy_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identities: Option<Vec<QuicksightIamPolicyAssignmentIdentitiesEl>>,
    dynamic: QuicksightIamPolicyAssignmentDynamic,
}
struct QuicksightIamPolicyAssignment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<QuicksightIamPolicyAssignmentData>,
}
#[derive(Clone)]
pub struct QuicksightIamPolicyAssignment(Rc<QuicksightIamPolicyAssignment_>);
impl QuicksightIamPolicyAssignment {
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
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().namespace = Some(v.into());
        self
    }
    #[doc = "Set the field `policy_arn`.\n"]
    pub fn set_policy_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().policy_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `identities`.\n"]
    pub fn set_identities(
        self,
        v: impl Into<BlockAssignable<QuicksightIamPolicyAssignmentIdentitiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identities = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identities = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `assignment_id` after provisioning.\n"]
    pub fn assignment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assignment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `assignment_name` after provisioning.\n"]
    pub fn assignment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assignment_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `assignment_status` after provisioning.\n"]
    pub fn assignment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assignment_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_arn` after provisioning.\n"]
    pub fn policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\n"]
    pub fn identities(&self) -> ListRef<QuicksightIamPolicyAssignmentIdentitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identities", self.extract_ref()),
        )
    }
}
impl Referable for QuicksightIamPolicyAssignment {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for QuicksightIamPolicyAssignment {}
impl ToListMappable for QuicksightIamPolicyAssignment {
    type O = ListRef<QuicksightIamPolicyAssignmentRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for QuicksightIamPolicyAssignment_ {
    fn extract_resource_type(&self) -> String {
        "aws_quicksight_iam_policy_assignment".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildQuicksightIamPolicyAssignment {
    pub tf_id: String,
    #[doc = ""]
    pub assignment_name: PrimField<String>,
    #[doc = ""]
    pub assignment_status: PrimField<String>,
}
impl BuildQuicksightIamPolicyAssignment {
    pub fn build(self, stack: &mut Stack) -> QuicksightIamPolicyAssignment {
        let out = QuicksightIamPolicyAssignment(Rc::new(QuicksightIamPolicyAssignment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(QuicksightIamPolicyAssignmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                assignment_name: self.assignment_name,
                assignment_status: self.assignment_status,
                aws_account_id: core::default::Default::default(),
                namespace: core::default::Default::default(),
                policy_arn: core::default::Default::default(),
                region: core::default::Default::default(),
                identities: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct QuicksightIamPolicyAssignmentRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightIamPolicyAssignmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl QuicksightIamPolicyAssignmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assignment_id` after provisioning.\n"]
    pub fn assignment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assignment_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `assignment_name` after provisioning.\n"]
    pub fn assignment_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assignment_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `assignment_status` after provisioning.\n"]
    pub fn assignment_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.assignment_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_arn` after provisioning.\n"]
    pub fn policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\n"]
    pub fn identities(&self) -> ListRef<QuicksightIamPolicyAssignmentIdentitiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.identities", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightIamPolicyAssignmentIdentitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<SetField<PrimField<String>>>,
}
impl QuicksightIamPolicyAssignmentIdentitiesEl {
    #[doc = "Set the field `group`.\n"]
    pub fn set_group(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.group = Some(v.into());
        self
    }
    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.user = Some(v.into());
        self
    }
}
impl ToListMappable for QuicksightIamPolicyAssignmentIdentitiesEl {
    type O = BlockAssignable<QuicksightIamPolicyAssignmentIdentitiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightIamPolicyAssignmentIdentitiesEl {}
impl BuildQuicksightIamPolicyAssignmentIdentitiesEl {
    pub fn build(self) -> QuicksightIamPolicyAssignmentIdentitiesEl {
        QuicksightIamPolicyAssignmentIdentitiesEl {
            group: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}
pub struct QuicksightIamPolicyAssignmentIdentitiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightIamPolicyAssignmentIdentitiesElRef {
    fn new(shared: StackShared, base: String) -> QuicksightIamPolicyAssignmentIdentitiesElRef {
        QuicksightIamPolicyAssignmentIdentitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightIamPolicyAssignmentIdentitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.group", self.base))
    }
    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.user", self.base))
    }
}
#[derive(Serialize, Default)]
struct QuicksightIamPolicyAssignmentDynamic {
    identities: Option<DynamicBlock<QuicksightIamPolicyAssignmentIdentitiesEl>>,
}
