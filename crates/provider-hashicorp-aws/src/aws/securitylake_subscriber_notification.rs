use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SecuritylakeSubscriberNotificationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    subscriber_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<SecuritylakeSubscriberNotificationConfigurationEl>>,
    dynamic: SecuritylakeSubscriberNotificationDynamic,
}
struct SecuritylakeSubscriberNotification_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SecuritylakeSubscriberNotificationData>,
}
#[derive(Clone)]
pub struct SecuritylakeSubscriberNotification(Rc<SecuritylakeSubscriberNotification_>);
impl SecuritylakeSubscriberNotification {
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
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<SecuritylakeSubscriberNotificationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscriber_endpoint` after provisioning.\n"]
    pub fn subscriber_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscriber_id` after provisioning.\n"]
    pub fn subscriber_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<SecuritylakeSubscriberNotificationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}
impl Referable for SecuritylakeSubscriberNotification {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SecuritylakeSubscriberNotification {}
impl ToListMappable for SecuritylakeSubscriberNotification {
    type O = ListRef<SecuritylakeSubscriberNotificationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SecuritylakeSubscriberNotification_ {
    fn extract_resource_type(&self) -> String {
        "aws_securitylake_subscriber_notification".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSecuritylakeSubscriberNotification {
    pub tf_id: String,
    #[doc = ""]
    pub subscriber_id: PrimField<String>,
}
impl BuildSecuritylakeSubscriberNotification {
    pub fn build(self, stack: &mut Stack) -> SecuritylakeSubscriberNotification {
        let out =
            SecuritylakeSubscriberNotification(Rc::new(SecuritylakeSubscriberNotification_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(SecuritylakeSubscriberNotificationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    region: core::default::Default::default(),
                    subscriber_id: self.subscriber_id,
                    configuration: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SecuritylakeSubscriberNotificationRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeSubscriberNotificationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SecuritylakeSubscriberNotificationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscriber_endpoint` after provisioning.\n"]
    pub fn subscriber_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subscriber_id` after provisioning.\n"]
    pub fn subscriber_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscriber_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<SecuritylakeSubscriberNotificationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_api_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_api_key_value: Option<PrimField<String>>,
    endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_method: Option<PrimField<String>>,
    target_role_arn: PrimField<String>,
}
impl SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl {
    #[doc = "Set the field `authorization_api_key_name`.\n"]
    pub fn set_authorization_api_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_api_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `authorization_api_key_value`.\n"]
    pub fn set_authorization_api_key_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.authorization_api_key_value = Some(v.into());
        self
    }
    #[doc = "Set the field `http_method`.\n"]
    pub fn set_http_method(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.http_method = Some(v.into());
        self
    }
}
impl ToListMappable
    for SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl
{
    type O = BlockAssignable<
        SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl {
    #[doc = ""]
    pub endpoint: PrimField<String>,
    #[doc = ""]
    pub target_role_arn: PrimField<String>,
}
impl BuildSecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl {
    pub fn build(
        self,
    ) -> SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl {
        SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl {
            authorization_api_key_name: core::default::Default::default(),
            authorization_api_key_value: core::default::Default::default(),
            endpoint: self.endpoint,
            http_method: core::default::Default::default(),
            target_role_arn: self.target_role_arn,
        }
    }
}
pub struct SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationElRef {
        SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `authorization_api_key_name` after provisioning.\n"]
    pub fn authorization_api_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_api_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authorization_api_key_value` after provisioning.\n"]
    pub fn authorization_api_key_value(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authorization_api_key_value", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }
    #[doc = "Get a reference to the value of field `target_role_arn` after provisioning.\n"]
    pub fn target_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_role_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl {}
impl SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl {}
impl ToListMappable
    for SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl
{
    type O = BlockAssignable<
        SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl {}
impl BuildSecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl {
    pub fn build(
        self,
    ) -> SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl {
        SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl {}
    }
}
pub struct SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationElRef {
        SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize, Default)]
struct SecuritylakeSubscriberNotificationConfigurationElDynamic {
    https_notification_configuration: Option<
        DynamicBlock<
            SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl,
        >,
    >,
    sqs_notification_configuration: Option<
        DynamicBlock<
            SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct SecuritylakeSubscriberNotificationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    https_notification_configuration: Option<
        Vec<SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    sqs_notification_configuration: Option<
        Vec<SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl>,
    >,
    dynamic: SecuritylakeSubscriberNotificationConfigurationElDynamic,
}
impl SecuritylakeSubscriberNotificationConfigurationEl {
    #[doc = "Set the field `https_notification_configuration`.\n"]
    pub fn set_https_notification_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.https_notification_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.https_notification_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sqs_notification_configuration`.\n"]
    pub fn set_sqs_notification_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sqs_notification_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_notification_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for SecuritylakeSubscriberNotificationConfigurationEl {
    type O = BlockAssignable<SecuritylakeSubscriberNotificationConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSecuritylakeSubscriberNotificationConfigurationEl {}
impl BuildSecuritylakeSubscriberNotificationConfigurationEl {
    pub fn build(self) -> SecuritylakeSubscriberNotificationConfigurationEl {
        SecuritylakeSubscriberNotificationConfigurationEl {
            https_notification_configuration: core::default::Default::default(),
            sqs_notification_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct SecuritylakeSubscriberNotificationConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SecuritylakeSubscriberNotificationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> SecuritylakeSubscriberNotificationConfigurationElRef {
        SecuritylakeSubscriberNotificationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SecuritylakeSubscriberNotificationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `https_notification_configuration` after provisioning.\n"]
    pub fn https_notification_configuration(
        &self,
    ) -> ListRef<SecuritylakeSubscriberNotificationConfigurationElHttpsNotificationConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.https_notification_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sqs_notification_configuration` after provisioning.\n"]
    pub fn sqs_notification_configuration(
        &self,
    ) -> ListRef<SecuritylakeSubscriberNotificationConfigurationElSqsNotificationConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sqs_notification_configuration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct SecuritylakeSubscriberNotificationDynamic {
    configuration: Option<DynamicBlock<SecuritylakeSubscriberNotificationConfigurationEl>>,
}
