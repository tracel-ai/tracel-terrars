use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct Pinpointsmsvoicev2PhoneNumberData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_protection_enabled: Option<PrimField<bool>>,
    iso_country_code: PrimField<String>,
    message_type: PrimField<String>,
    number_capabilities: SetField<PrimField<String>>,
    number_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_out_list_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registration_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_managed_opt_outs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    two_way_channel_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    two_way_channel_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    two_way_channel_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<Pinpointsmsvoicev2PhoneNumberTimeoutsEl>,
}
struct Pinpointsmsvoicev2PhoneNumber_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Pinpointsmsvoicev2PhoneNumberData>,
}
#[derive(Clone)]
pub struct Pinpointsmsvoicev2PhoneNumber(Rc<Pinpointsmsvoicev2PhoneNumber_>);
impl Pinpointsmsvoicev2PhoneNumber {
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
    #[doc = "Set the field `deletion_protection_enabled`.\n"]
    pub fn set_deletion_protection_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `opt_out_list_name`.\n"]
    pub fn set_opt_out_list_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().opt_out_list_name = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `registration_id`.\n"]
    pub fn set_registration_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().registration_id = Some(v.into());
        self
    }
    #[doc = "Set the field `self_managed_opt_outs_enabled`.\n"]
    pub fn set_self_managed_opt_outs_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().self_managed_opt_outs_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `two_way_channel_arn`.\n"]
    pub fn set_two_way_channel_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().two_way_channel_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `two_way_channel_enabled`.\n"]
    pub fn set_two_way_channel_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().two_way_channel_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `two_way_channel_role`.\n"]
    pub fn set_two_way_channel_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().two_way_channel_role = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<Pinpointsmsvoicev2PhoneNumberTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `iso_country_code` after provisioning.\n"]
    pub fn iso_country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iso_country_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_type` after provisioning.\n"]
    pub fn message_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_leasing_price` after provisioning.\n"]
    pub fn monthly_leasing_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_leasing_price", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `number_capabilities` after provisioning.\n"]
    pub fn number_capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.number_capabilities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `number_type` after provisioning.\n"]
    pub fn number_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `opt_out_list_name` after provisioning.\n"]
    pub fn opt_out_list_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.opt_out_list_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.phone_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registration_id` after provisioning.\n"]
    pub fn registration_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registration_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_managed_opt_outs_enabled` after provisioning.\n"]
    pub fn self_managed_opt_outs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_managed_opt_outs_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `two_way_channel_arn` after provisioning.\n"]
    pub fn two_way_channel_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_way_channel_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `two_way_channel_enabled` after provisioning.\n"]
    pub fn two_way_channel_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_way_channel_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `two_way_channel_role` after provisioning.\n"]
    pub fn two_way_channel_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_way_channel_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Pinpointsmsvoicev2PhoneNumberTimeoutsElRef {
        Pinpointsmsvoicev2PhoneNumberTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for Pinpointsmsvoicev2PhoneNumber {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for Pinpointsmsvoicev2PhoneNumber {}
impl ToListMappable for Pinpointsmsvoicev2PhoneNumber {
    type O = ListRef<Pinpointsmsvoicev2PhoneNumberRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for Pinpointsmsvoicev2PhoneNumber_ {
    fn extract_resource_type(&self) -> String {
        "aws_pinpointsmsvoicev2_phone_number".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildPinpointsmsvoicev2PhoneNumber {
    pub tf_id: String,
    #[doc = ""]
    pub iso_country_code: PrimField<String>,
    #[doc = ""]
    pub message_type: PrimField<String>,
    #[doc = ""]
    pub number_capabilities: SetField<PrimField<String>>,
    #[doc = ""]
    pub number_type: PrimField<String>,
}
impl BuildPinpointsmsvoicev2PhoneNumber {
    pub fn build(self, stack: &mut Stack) -> Pinpointsmsvoicev2PhoneNumber {
        let out = Pinpointsmsvoicev2PhoneNumber(Rc::new(Pinpointsmsvoicev2PhoneNumber_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Pinpointsmsvoicev2PhoneNumberData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_protection_enabled: core::default::Default::default(),
                iso_country_code: self.iso_country_code,
                message_type: self.message_type,
                number_capabilities: self.number_capabilities,
                number_type: self.number_type,
                opt_out_list_name: core::default::Default::default(),
                region: core::default::Default::default(),
                registration_id: core::default::Default::default(),
                self_managed_opt_outs_enabled: core::default::Default::default(),
                tags: core::default::Default::default(),
                two_way_channel_arn: core::default::Default::default(),
                two_way_channel_enabled: core::default::Default::default(),
                two_way_channel_role: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct Pinpointsmsvoicev2PhoneNumberRef {
    shared: StackShared,
    base: String,
}
impl Ref for Pinpointsmsvoicev2PhoneNumberRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl Pinpointsmsvoicev2PhoneNumberRef {
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
    #[doc = "Get a reference to the value of field `deletion_protection_enabled` after provisioning.\n"]
    pub fn deletion_protection_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_protection_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `iso_country_code` after provisioning.\n"]
    pub fn iso_country_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.iso_country_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `message_type` after provisioning.\n"]
    pub fn message_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.message_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `monthly_leasing_price` after provisioning.\n"]
    pub fn monthly_leasing_price(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.monthly_leasing_price", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `number_capabilities` after provisioning.\n"]
    pub fn number_capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.number_capabilities", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `number_type` after provisioning.\n"]
    pub fn number_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `opt_out_list_name` after provisioning.\n"]
    pub fn opt_out_list_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.opt_out_list_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.phone_number", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `registration_id` after provisioning.\n"]
    pub fn registration_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.registration_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_managed_opt_outs_enabled` after provisioning.\n"]
    pub fn self_managed_opt_outs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_managed_opt_outs_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `two_way_channel_arn` after provisioning.\n"]
    pub fn two_way_channel_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_way_channel_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `two_way_channel_enabled` after provisioning.\n"]
    pub fn two_way_channel_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_way_channel_enabled", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `two_way_channel_role` after provisioning.\n"]
    pub fn two_way_channel_role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.two_way_channel_role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Pinpointsmsvoicev2PhoneNumberTimeoutsElRef {
        Pinpointsmsvoicev2PhoneNumberTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct Pinpointsmsvoicev2PhoneNumberTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl Pinpointsmsvoicev2PhoneNumberTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for Pinpointsmsvoicev2PhoneNumberTimeoutsEl {
    type O = BlockAssignable<Pinpointsmsvoicev2PhoneNumberTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildPinpointsmsvoicev2PhoneNumberTimeoutsEl {}
impl BuildPinpointsmsvoicev2PhoneNumberTimeoutsEl {
    pub fn build(self) -> Pinpointsmsvoicev2PhoneNumberTimeoutsEl {
        Pinpointsmsvoicev2PhoneNumberTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct Pinpointsmsvoicev2PhoneNumberTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for Pinpointsmsvoicev2PhoneNumberTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> Pinpointsmsvoicev2PhoneNumberTimeoutsElRef {
        Pinpointsmsvoicev2PhoneNumberTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl Pinpointsmsvoicev2PhoneNumberTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
