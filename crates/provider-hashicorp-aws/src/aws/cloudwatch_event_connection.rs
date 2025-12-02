use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudwatchEventConnectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    authorization_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_identifier: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_parameters: Option<Vec<CloudwatchEventConnectionAuthParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_connectivity_parameters:
        Option<Vec<CloudwatchEventConnectionInvocationConnectivityParametersEl>>,
    dynamic: CloudwatchEventConnectionDynamic,
}
struct CloudwatchEventConnection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudwatchEventConnectionData>,
}
#[derive(Clone)]
pub struct CloudwatchEventConnection(Rc<CloudwatchEventConnection_>);
impl CloudwatchEventConnection {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_identifier`.\n"]
    pub fn set_kms_key_identifier(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `auth_parameters`.\n"]
    pub fn set_auth_parameters(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventConnectionAuthParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `invocation_connectivity_parameters`.\n"]
    pub fn set_invocation_connectivity_parameters(
        self,
        v: impl Into<BlockAssignable<CloudwatchEventConnectionInvocationConnectivityParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().invocation_connectivity_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .invocation_connectivity_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_parameters` after provisioning.\n"]
    pub fn auth_parameters(&self) -> ListRef<CloudwatchEventConnectionAuthParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auth_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `invocation_connectivity_parameters` after provisioning.\n"]
    pub fn invocation_connectivity_parameters(
        &self,
    ) -> ListRef<CloudwatchEventConnectionInvocationConnectivityParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.invocation_connectivity_parameters", self.extract_ref()),
        )
    }
}
impl Referable for CloudwatchEventConnection {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudwatchEventConnection {}
impl ToListMappable for CloudwatchEventConnection {
    type O = ListRef<CloudwatchEventConnectionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudwatchEventConnection_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudwatch_event_connection".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudwatchEventConnection {
    pub tf_id: String,
    #[doc = ""]
    pub authorization_type: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildCloudwatchEventConnection {
    pub fn build(self, stack: &mut Stack) -> CloudwatchEventConnection {
        let out = CloudwatchEventConnection(Rc::new(CloudwatchEventConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudwatchEventConnectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                authorization_type: self.authorization_type,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_identifier: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                auth_parameters: core::default::Default::default(),
                invocation_connectivity_parameters: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudwatchEventConnectionRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudwatchEventConnectionRef {
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
    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secret_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `auth_parameters` after provisioning.\n"]
    pub fn auth_parameters(&self) -> ListRef<CloudwatchEventConnectionAuthParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.auth_parameters", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `invocation_connectivity_parameters` after provisioning.\n"]
    pub fn invocation_connectivity_parameters(
        &self,
    ) -> ListRef<CloudwatchEventConnectionInvocationConnectivityParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.invocation_connectivity_parameters", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElApiKeyEl {
    key: PrimField<String>,
    value: PrimField<String>,
}
impl CloudwatchEventConnectionAuthParametersElApiKeyEl {}
impl ToListMappable for CloudwatchEventConnectionAuthParametersElApiKeyEl {
    type O = BlockAssignable<CloudwatchEventConnectionAuthParametersElApiKeyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElApiKeyEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildCloudwatchEventConnectionAuthParametersElApiKeyEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElApiKeyEl {
        CloudwatchEventConnectionAuthParametersElApiKeyEl {
            key: self.key,
            value: self.value,
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElApiKeyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElApiKeyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElApiKeyElRef {
        CloudwatchEventConnectionAuthParametersElApiKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElApiKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElBasicEl {
    password: PrimField<String>,
    username: PrimField<String>,
}
impl CloudwatchEventConnectionAuthParametersElBasicEl {}
impl ToListMappable for CloudwatchEventConnectionAuthParametersElBasicEl {
    type O = BlockAssignable<CloudwatchEventConnectionAuthParametersElBasicEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElBasicEl {
    #[doc = ""]
    pub password: PrimField<String>,
    #[doc = ""]
    pub username: PrimField<String>,
}
impl BuildCloudwatchEventConnectionAuthParametersElBasicEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElBasicEl {
        CloudwatchEventConnectionAuthParametersElBasicEl {
            password: self.password,
            username: self.username,
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElBasicElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElBasicElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElBasicElRef {
        CloudwatchEventConnectionAuthParametersElBasicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElBasicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }
    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_value_secret: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_value_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
    type O =
        BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {}
impl BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
    pub fn build(
        self,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_value_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_value_secret: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_value_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl
{
    type O = BlockAssignable<
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {}
impl BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
    pub fn build(
        self,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_value_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_value_secret: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_value_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl
{
    type O = BlockAssignable<
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
}
impl BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
    pub fn build(
        self,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_value_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElDynamic {
    body: Option<
        DynamicBlock<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl>,
    >,
    header: Option<
        DynamicBlock<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl>,
    >,
    query_string: Option<
        DynamicBlock<
            CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Vec<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header:
        Option<Vec<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string: Option<
        Vec<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl>,
    >,
    dynamic: CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElDynamic,
}
impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
    #[doc = "Set the field `body`.\n"]
    pub fn set_body(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.body = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.body = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_string = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
    type O = BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {}
impl BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
            body: core::default::Default::default(),
            header: core::default::Default::default(),
            query_string: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(
        &self,
    ) -> ListRef<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.body", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> ListRef<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> ListRef<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
    client_id: PrimField<String>,
    client_secret: PrimField<String>,
}
impl CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {}
impl ToListMappable for CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
    type O = BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
    #[doc = ""]
    pub client_id: PrimField<String>,
    #[doc = ""]
    pub client_secret: PrimField<String>,
}
impl BuildCloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
        CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
            client_id: self.client_id,
            client_secret: self.client_secret,
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
        CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }
    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.client_secret", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_value_secret: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_value_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl
{
    type O = BlockAssignable<
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {}
impl BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
    pub fn build(
        self,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_value_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_value_secret: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_value_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl
{
    type O = BlockAssignable<
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {}
impl BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
    pub fn build(
        self,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_value_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_value_secret: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_value_secret = Some(v.into());
        self
    }
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable
    for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl
{
    type O = BlockAssignable<
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl
{}
impl BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
    pub fn build(
        self,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_value_secret", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElDynamic {
    body: Option<
        DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl>,
    >,
    header: Option<
        DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl>,
    >,
    query_string: Option<
        DynamicBlock<
            CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Vec<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header:
        Option<Vec<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string: Option<
        Vec<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl>,
    >,
    dynamic: CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElDynamic,
}
impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
    #[doc = "Set the field `body`.\n"]
    pub fn set_body(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.body = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.body = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_string = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
    type O = BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {}
impl BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
            body: core::default::Default::default(),
            header: core::default::Default::default(),
            query_string: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(
        &self,
    ) -> ListRef<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.body", self.base))
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> ListRef<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }
    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> ListRef<
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudwatchEventConnectionAuthParametersElOauthElDynamic {
    client_parameters:
        Option<DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl>>,
    oauth_http_parameters:
        Option<DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl>>,
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthEl {
    authorization_endpoint: PrimField<String>,
    http_method: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_parameters:
        Option<Vec<CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth_http_parameters:
        Option<Vec<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl>>,
    dynamic: CloudwatchEventConnectionAuthParametersElOauthElDynamic,
}
impl CloudwatchEventConnectionAuthParametersElOauthEl {
    #[doc = "Set the field `client_parameters`.\n"]
    pub fn set_client_parameters(
        mut self,
        v: impl Into<
            BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.client_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.client_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth_http_parameters`.\n"]
    pub fn set_oauth_http_parameters(
        mut self,
        v: impl Into<
            BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth_http_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_http_parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudwatchEventConnectionAuthParametersElOauthEl {
    type O = BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersElOauthEl {
    #[doc = ""]
    pub authorization_endpoint: PrimField<String>,
    #[doc = ""]
    pub http_method: PrimField<String>,
}
impl BuildCloudwatchEventConnectionAuthParametersElOauthEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElOauthEl {
        CloudwatchEventConnectionAuthParametersElOauthEl {
            authorization_endpoint: self.authorization_endpoint,
            http_method: self.http_method,
            client_parameters: core::default::Default::default(),
            oauth_http_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElOauthElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElOauthElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElRef {
        CloudwatchEventConnectionAuthParametersElOauthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElOauthElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_endpoint", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }
    #[doc = "Get a reference to the value of field `client_parameters` after provisioning.\n"]
    pub fn client_parameters(
        &self,
    ) -> ListRef<CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.client_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth_http_parameters` after provisioning.\n"]
    pub fn oauth_http_parameters(
        &self,
    ) -> ListRef<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.oauth_http_parameters", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudwatchEventConnectionAuthParametersElDynamic {
    api_key: Option<DynamicBlock<CloudwatchEventConnectionAuthParametersElApiKeyEl>>,
    basic: Option<DynamicBlock<CloudwatchEventConnectionAuthParametersElBasicEl>>,
    invocation_http_parameters:
        Option<DynamicBlock<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl>>,
    oauth: Option<DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthEl>>,
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_key: Option<Vec<CloudwatchEventConnectionAuthParametersElApiKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic: Option<Vec<CloudwatchEventConnectionAuthParametersElBasicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_http_parameters:
        Option<Vec<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth: Option<Vec<CloudwatchEventConnectionAuthParametersElOauthEl>>,
    dynamic: CloudwatchEventConnectionAuthParametersElDynamic,
}
impl CloudwatchEventConnectionAuthParametersEl {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventConnectionAuthParametersElApiKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_key = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `basic`.\n"]
    pub fn set_basic(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventConnectionAuthParametersElBasicEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.basic = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.basic = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `invocation_http_parameters`.\n"]
    pub fn set_invocation_http_parameters(
        mut self,
        v: impl Into<
            BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.invocation_http_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.invocation_http_parameters = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `oauth`.\n"]
    pub fn set_oauth(
        mut self,
        v: impl Into<BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oauth = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudwatchEventConnectionAuthParametersEl {
    type O = BlockAssignable<CloudwatchEventConnectionAuthParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionAuthParametersEl {}
impl BuildCloudwatchEventConnectionAuthParametersEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersEl {
        CloudwatchEventConnectionAuthParametersEl {
            api_key: core::default::Default::default(),
            basic: core::default::Default::default(),
            invocation_http_parameters: core::default::Default::default(),
            oauth: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionAuthParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionAuthParametersElRef {
    fn new(shared: StackShared, base: String) -> CloudwatchEventConnectionAuthParametersElRef {
        CloudwatchEventConnectionAuthParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionAuthParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> ListRef<CloudwatchEventConnectionAuthParametersElApiKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(&self) -> ListRef<CloudwatchEventConnectionAuthParametersElBasicElRef> {
        ListRef::new(self.shared().clone(), format!("{}.basic", self.base))
    }
    #[doc = "Get a reference to the value of field `invocation_http_parameters` after provisioning.\n"]
    pub fn invocation_http_parameters(
        &self,
    ) -> ListRef<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.invocation_http_parameters", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oauth` after provisioning.\n"]
    pub fn oauth(&self) -> ListRef<CloudwatchEventConnectionAuthParametersElOauthElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oauth", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl {
    resource_configuration_arn: PrimField<String>,
}
impl CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl {}
impl ToListMappable
    for CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl
{
    type O = BlockAssignable<
        CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl {
    #[doc = ""]
    pub resource_configuration_arn: PrimField<String>,
}
impl BuildCloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl {
    pub fn build(
        self,
    ) -> CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl {
        CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl {
            resource_configuration_arn: self.resource_configuration_arn,
        }
    }
}
pub struct CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersElRef {
        CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_association_arn` after provisioning.\n"]
    pub fn resource_association_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_association_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_configuration_arn` after provisioning.\n"]
    pub fn resource_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_configuration_arn", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudwatchEventConnectionInvocationConnectivityParametersElDynamic {
    resource_parameters: Option<
        DynamicBlock<
            CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct CloudwatchEventConnectionInvocationConnectivityParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_parameters: Option<
        Vec<CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl>,
    >,
    dynamic: CloudwatchEventConnectionInvocationConnectivityParametersElDynamic,
}
impl CloudwatchEventConnectionInvocationConnectivityParametersEl {
    #[doc = "Set the field `resource_parameters`.\n"]
    pub fn set_resource_parameters(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_parameters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudwatchEventConnectionInvocationConnectivityParametersEl {
    type O = BlockAssignable<CloudwatchEventConnectionInvocationConnectivityParametersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudwatchEventConnectionInvocationConnectivityParametersEl {}
impl BuildCloudwatchEventConnectionInvocationConnectivityParametersEl {
    pub fn build(self) -> CloudwatchEventConnectionInvocationConnectivityParametersEl {
        CloudwatchEventConnectionInvocationConnectivityParametersEl {
            resource_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudwatchEventConnectionInvocationConnectivityParametersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudwatchEventConnectionInvocationConnectivityParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudwatchEventConnectionInvocationConnectivityParametersElRef {
        CloudwatchEventConnectionInvocationConnectivityParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudwatchEventConnectionInvocationConnectivityParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_parameters` after provisioning.\n"]
    pub fn resource_parameters(
        &self,
    ) -> ListRef<CloudwatchEventConnectionInvocationConnectivityParametersElResourceParametersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_parameters", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudwatchEventConnectionDynamic {
    auth_parameters: Option<DynamicBlock<CloudwatchEventConnectionAuthParametersEl>>,
    invocation_connectivity_parameters:
        Option<DynamicBlock<CloudwatchEventConnectionInvocationConnectivityParametersEl>>,
}
