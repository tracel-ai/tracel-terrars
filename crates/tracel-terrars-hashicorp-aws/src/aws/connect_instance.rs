use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_resolve_best_voices_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_flow_logs_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_lens_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    directory_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    early_media_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    identity_management_type: PrimField<String>,
    inbound_calls_enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_alias: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_party_conference_enabled: Option<PrimField<bool>>,
    outbound_calls_enabled: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ConnectInstanceTimeoutsEl>,
}

struct ConnectInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConnectInstanceData>,
}

#[derive(Clone)]
pub struct ConnectInstance(Rc<ConnectInstance_>);

impl ConnectInstance {
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

    #[doc = "Set the field `auto_resolve_best_voices_enabled`.\n"]
    pub fn set_auto_resolve_best_voices_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_resolve_best_voices_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `contact_flow_logs_enabled`.\n"]
    pub fn set_contact_flow_logs_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().contact_flow_logs_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `contact_lens_enabled`.\n"]
    pub fn set_contact_lens_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().contact_lens_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `directory_id`.\n"]
    pub fn set_directory_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().directory_id = Some(v.into());
        self
    }

    #[doc = "Set the field `early_media_enabled`.\n"]
    pub fn set_early_media_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().early_media_enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_alias`.\n"]
    pub fn set_instance_alias(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_alias = Some(v.into());
        self
    }

    #[doc = "Set the field `multi_party_conference_enabled`.\n"]
    pub fn set_multi_party_conference_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_party_conference_enabled = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ConnectInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_resolve_best_voices_enabled` after provisioning.\n"]
    pub fn auto_resolve_best_voices_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_resolve_best_voices_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_flow_logs_enabled` after provisioning.\n"]
    pub fn contact_flow_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_logs_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_lens_enabled` after provisioning.\n"]
    pub fn contact_lens_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_lens_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `early_media_enabled` after provisioning.\n"]
    pub fn early_media_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.early_media_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_management_type` after provisioning.\n"]
    pub fn identity_management_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_management_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inbound_calls_enabled` after provisioning.\n"]
    pub fn inbound_calls_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inbound_calls_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_alias` after provisioning.\n"]
    pub fn instance_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_party_conference_enabled` after provisioning.\n"]
    pub fn multi_party_conference_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_party_conference_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outbound_calls_enabled` after provisioning.\n"]
    pub fn outbound_calls_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.outbound_calls_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ConnectInstanceTimeoutsElRef {
        ConnectInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ConnectInstance {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ConnectInstance { }

impl ToListMappable for ConnectInstance {
    type O = ListRef<ConnectInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ConnectInstance_ {
    fn extract_resource_type(&self) -> String {
        "aws_connect_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectInstance {
    pub tf_id: String,
    #[doc = ""]
    pub identity_management_type: PrimField<String>,
    #[doc = ""]
    pub inbound_calls_enabled: PrimField<bool>,
    #[doc = ""]
    pub outbound_calls_enabled: PrimField<bool>,
}

impl BuildConnectInstance {
    pub fn build(self, stack: &mut Stack) -> ConnectInstance {
        let out = ConnectInstance(Rc::new(ConnectInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConnectInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_resolve_best_voices_enabled: core::default::Default::default(),
                contact_flow_logs_enabled: core::default::Default::default(),
                contact_lens_enabled: core::default::Default::default(),
                directory_id: core::default::Default::default(),
                early_media_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                identity_management_type: self.identity_management_type,
                inbound_calls_enabled: self.inbound_calls_enabled,
                instance_alias: core::default::Default::default(),
                multi_party_conference_enabled: core::default::Default::default(),
                outbound_calls_enabled: self.outbound_calls_enabled,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ConnectInstanceRef {
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

    #[doc = "Get a reference to the value of field `auto_resolve_best_voices_enabled` after provisioning.\n"]
    pub fn auto_resolve_best_voices_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_resolve_best_voices_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_flow_logs_enabled` after provisioning.\n"]
    pub fn contact_flow_logs_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_logs_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `contact_lens_enabled` after provisioning.\n"]
    pub fn contact_lens_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_lens_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `early_media_enabled` after provisioning.\n"]
    pub fn early_media_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.early_media_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_management_type` after provisioning.\n"]
    pub fn identity_management_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.identity_management_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inbound_calls_enabled` after provisioning.\n"]
    pub fn inbound_calls_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inbound_calls_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_alias` after provisioning.\n"]
    pub fn instance_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_party_conference_enabled` after provisioning.\n"]
    pub fn multi_party_conference_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.multi_party_conference_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outbound_calls_enabled` after provisioning.\n"]
    pub fn outbound_calls_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.outbound_calls_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ConnectInstanceTimeoutsElRef {
        ConnectInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ConnectInstanceTimeoutsEl {
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
}

impl ToListMappable for ConnectInstanceTimeoutsEl {
    type O = BlockAssignable<ConnectInstanceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectInstanceTimeoutsEl {}

impl BuildConnectInstanceTimeoutsEl {
    pub fn build(self) -> ConnectInstanceTimeoutsEl {
        ConnectInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ConnectInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ConnectInstanceTimeoutsElRef {
        ConnectInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectInstanceTimeoutsElRef {
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
}
