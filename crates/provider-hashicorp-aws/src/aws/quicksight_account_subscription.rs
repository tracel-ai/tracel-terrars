use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct QuicksightAccountSubscriptionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    account_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_directory_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admin_pro_group: Option<ListField<PrimField<String>>>,
    authentication_method: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author_pro_group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_number: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_id: Option<PrimField<String>>,
    edition: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iam_identity_center_instance_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<PrimField<String>>,
    notification_email: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reader_group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reader_pro_group: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    realm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<QuicksightAccountSubscriptionTimeoutsEl>,
}
struct QuicksightAccountSubscription_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<QuicksightAccountSubscriptionData>,
}
#[derive(Clone)]
pub struct QuicksightAccountSubscription(Rc<QuicksightAccountSubscription_>);
impl QuicksightAccountSubscription {
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
    #[doc = "Set the field `active_directory_name`.\n"]
    pub fn set_active_directory_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().active_directory_name = Some(v.into());
        self
    }
    #[doc = "Set the field `admin_group`.\n"]
    pub fn set_admin_group(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().admin_group = Some(v.into());
        self
    }
    #[doc = "Set the field `admin_pro_group`.\n"]
    pub fn set_admin_pro_group(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().admin_pro_group = Some(v.into());
        self
    }
    #[doc = "Set the field `author_group`.\n"]
    pub fn set_author_group(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().author_group = Some(v.into());
        self
    }
    #[doc = "Set the field `author_pro_group`.\n"]
    pub fn set_author_pro_group(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().author_pro_group = Some(v.into());
        self
    }
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `contact_number`.\n"]
    pub fn set_contact_number(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().contact_number = Some(v.into());
        self
    }
    #[doc = "Set the field `directory_id`.\n"]
    pub fn set_directory_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().directory_id = Some(v.into());
        self
    }
    #[doc = "Set the field `email_address`.\n"]
    pub fn set_email_address(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().email_address = Some(v.into());
        self
    }
    #[doc = "Set the field `first_name`.\n"]
    pub fn set_first_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().first_name = Some(v.into());
        self
    }
    #[doc = "Set the field `iam_identity_center_instance_arn`.\n"]
    pub fn set_iam_identity_center_instance_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().iam_identity_center_instance_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `last_name`.\n"]
    pub fn set_last_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().last_name = Some(v.into());
        self
    }
    #[doc = "Set the field `reader_group`.\n"]
    pub fn set_reader_group(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().reader_group = Some(v.into());
        self
    }
    #[doc = "Set the field `reader_pro_group`.\n"]
    pub fn set_reader_pro_group(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().reader_pro_group = Some(v.into());
        self
    }
    #[doc = "Set the field `realm`.\n"]
    pub fn set_realm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().realm = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<QuicksightAccountSubscriptionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `account_name` after provisioning.\n"]
    pub fn account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `account_subscription_status` after provisioning.\n"]
    pub fn account_subscription_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_subscription_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `active_directory_name` after provisioning.\n"]
    pub fn active_directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_group` after provisioning.\n"]
    pub fn admin_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_pro_group` after provisioning.\n"]
    pub fn admin_pro_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_pro_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `author_group` after provisioning.\n"]
    pub fn author_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.author_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `author_pro_group` after provisioning.\n"]
    pub fn author_pro_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.author_pro_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `contact_number` after provisioning.\n"]
    pub fn contact_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.first_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_identity_center_instance_arn` after provisioning.\n"]
    pub fn iam_identity_center_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_identity_center_instance_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_email` after provisioning.\n"]
    pub fn notification_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reader_group` after provisioning.\n"]
    pub fn reader_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reader_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reader_pro_group` after provisioning.\n"]
    pub fn reader_pro_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reader_pro_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `realm` after provisioning.\n"]
    pub fn realm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.realm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QuicksightAccountSubscriptionTimeoutsElRef {
        QuicksightAccountSubscriptionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for QuicksightAccountSubscription {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for QuicksightAccountSubscription {}
impl ToListMappable for QuicksightAccountSubscription {
    type O = ListRef<QuicksightAccountSubscriptionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for QuicksightAccountSubscription_ {
    fn extract_resource_type(&self) -> String {
        "aws_quicksight_account_subscription".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildQuicksightAccountSubscription {
    pub tf_id: String,
    #[doc = ""]
    pub account_name: PrimField<String>,
    #[doc = ""]
    pub authentication_method: PrimField<String>,
    #[doc = ""]
    pub edition: PrimField<String>,
    #[doc = ""]
    pub notification_email: PrimField<String>,
}
impl BuildQuicksightAccountSubscription {
    pub fn build(self, stack: &mut Stack) -> QuicksightAccountSubscription {
        let out = QuicksightAccountSubscription(Rc::new(QuicksightAccountSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(QuicksightAccountSubscriptionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                account_name: self.account_name,
                active_directory_name: core::default::Default::default(),
                admin_group: core::default::Default::default(),
                admin_pro_group: core::default::Default::default(),
                authentication_method: self.authentication_method,
                author_group: core::default::Default::default(),
                author_pro_group: core::default::Default::default(),
                aws_account_id: core::default::Default::default(),
                contact_number: core::default::Default::default(),
                directory_id: core::default::Default::default(),
                edition: self.edition,
                email_address: core::default::Default::default(),
                first_name: core::default::Default::default(),
                iam_identity_center_instance_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                last_name: core::default::Default::default(),
                notification_email: self.notification_email,
                reader_group: core::default::Default::default(),
                reader_pro_group: core::default::Default::default(),
                realm: core::default::Default::default(),
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct QuicksightAccountSubscriptionRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightAccountSubscriptionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl QuicksightAccountSubscriptionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_name` after provisioning.\n"]
    pub fn account_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `account_subscription_status` after provisioning.\n"]
    pub fn account_subscription_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_subscription_status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `active_directory_name` after provisioning.\n"]
    pub fn active_directory_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.active_directory_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_group` after provisioning.\n"]
    pub fn admin_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `admin_pro_group` after provisioning.\n"]
    pub fn admin_pro_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.admin_pro_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `authentication_method` after provisioning.\n"]
    pub fn authentication_method(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.authentication_method", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `author_group` after provisioning.\n"]
    pub fn author_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.author_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `author_pro_group` after provisioning.\n"]
    pub fn author_pro_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.author_pro_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `contact_number` after provisioning.\n"]
    pub fn contact_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.directory_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.email_address", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `first_name` after provisioning.\n"]
    pub fn first_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.first_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iam_identity_center_instance_arn` after provisioning.\n"]
    pub fn iam_identity_center_instance_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iam_identity_center_instance_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_name` after provisioning.\n"]
    pub fn last_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `notification_email` after provisioning.\n"]
    pub fn notification_email(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notification_email", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reader_group` after provisioning.\n"]
    pub fn reader_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reader_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reader_pro_group` after provisioning.\n"]
    pub fn reader_pro_group(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reader_pro_group", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `realm` after provisioning.\n"]
    pub fn realm(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.realm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> QuicksightAccountSubscriptionTimeoutsElRef {
        QuicksightAccountSubscriptionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct QuicksightAccountSubscriptionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl QuicksightAccountSubscriptionTimeoutsEl {
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
impl ToListMappable for QuicksightAccountSubscriptionTimeoutsEl {
    type O = BlockAssignable<QuicksightAccountSubscriptionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildQuicksightAccountSubscriptionTimeoutsEl {}
impl BuildQuicksightAccountSubscriptionTimeoutsEl {
    pub fn build(self) -> QuicksightAccountSubscriptionTimeoutsEl {
        QuicksightAccountSubscriptionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
        }
    }
}
pub struct QuicksightAccountSubscriptionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for QuicksightAccountSubscriptionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> QuicksightAccountSubscriptionTimeoutsElRef {
        QuicksightAccountSubscriptionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl QuicksightAccountSubscriptionTimeoutsElRef {
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
