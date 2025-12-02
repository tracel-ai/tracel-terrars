use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct EmrStudioSessionMappingData {
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
    identity_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_name: Option<PrimField<String>>,
    identity_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    session_policy_arn: PrimField<String>,
    studio_id: PrimField<String>,
}
struct EmrStudioSessionMapping_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmrStudioSessionMappingData>,
}
#[derive(Clone)]
pub struct EmrStudioSessionMapping(Rc<EmrStudioSessionMapping_>);
impl EmrStudioSessionMapping {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_id`.\n"]
    pub fn set_identity_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().identity_id = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_name`.\n"]
    pub fn set_identity_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().identity_name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `identity_id` after provisioning.\n"]
    pub fn identity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_name` after provisioning.\n"]
    pub fn identity_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_policy_arn` after provisioning.\n"]
    pub fn session_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_policy_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `studio_id` after provisioning.\n"]
    pub fn studio_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.studio_id", self.extract_ref()),
        )
    }
}
impl Referable for EmrStudioSessionMapping {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for EmrStudioSessionMapping {}
impl ToListMappable for EmrStudioSessionMapping {
    type O = ListRef<EmrStudioSessionMappingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for EmrStudioSessionMapping_ {
    fn extract_resource_type(&self) -> String {
        "aws_emr_studio_session_mapping".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildEmrStudioSessionMapping {
    pub tf_id: String,
    #[doc = ""]
    pub identity_type: PrimField<String>,
    #[doc = ""]
    pub session_policy_arn: PrimField<String>,
    #[doc = ""]
    pub studio_id: PrimField<String>,
}
impl BuildEmrStudioSessionMapping {
    pub fn build(self, stack: &mut Stack) -> EmrStudioSessionMapping {
        let out = EmrStudioSessionMapping(Rc::new(EmrStudioSessionMapping_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmrStudioSessionMappingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                identity_id: core::default::Default::default(),
                identity_name: core::default::Default::default(),
                identity_type: self.identity_type,
                region: core::default::Default::default(),
                session_policy_arn: self.session_policy_arn,
                studio_id: self.studio_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct EmrStudioSessionMappingRef {
    shared: StackShared,
    base: String,
}
impl Ref for EmrStudioSessionMappingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl EmrStudioSessionMappingRef {
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
    #[doc = "Get a reference to the value of field `identity_id` after provisioning.\n"]
    pub fn identity_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_name` after provisioning.\n"]
    pub fn identity_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `session_policy_arn` after provisioning.\n"]
    pub fn session_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.session_policy_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `studio_id` after provisioning.\n"]
    pub fn studio_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.studio_id", self.extract_ref()),
        )
    }
}
