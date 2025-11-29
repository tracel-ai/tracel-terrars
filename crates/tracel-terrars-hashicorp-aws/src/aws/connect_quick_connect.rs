use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectQuickConnectData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quick_connect_config: Option<Vec<ConnectQuickConnectQuickConnectConfigEl>>,
    dynamic: ConnectQuickConnectDynamic,
}

struct ConnectQuickConnect_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConnectQuickConnectData>,
}

#[derive(Clone)]
pub struct ConnectQuickConnect(Rc<ConnectQuickConnect_>);

impl ConnectQuickConnect {
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

    #[doc = "Set the field `quick_connect_config`.\n"]
    pub fn set_quick_connect_config(
        self,
        v: impl Into<BlockAssignable<ConnectQuickConnectQuickConnectConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quick_connect_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quick_connect_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_id` after provisioning.\n"]
    pub fn quick_connect_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> ListRef<ConnectQuickConnectQuickConnectConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.quick_connect_config", self.extract_ref()))
    }
}

impl Referable for ConnectQuickConnect {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ConnectQuickConnect { }

impl ToListMappable for ConnectQuickConnect {
    type O = ListRef<ConnectQuickConnectRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ConnectQuickConnect_ {
    fn extract_resource_type(&self) -> String {
        "aws_connect_quick_connect".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectQuickConnect {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildConnectQuickConnect {
    pub fn build(self, stack: &mut Stack) -> ConnectQuickConnect {
        let out = ConnectQuickConnect(Rc::new(ConnectQuickConnect_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConnectQuickConnectData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                quick_connect_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectQuickConnectRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectQuickConnectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl ConnectQuickConnectRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_id` after provisioning.\n"]
    pub fn quick_connect_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> ListRef<ConnectQuickConnectQuickConnectConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.quick_connect_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    phone_number: PrimField<String>,
}

impl ConnectQuickConnectQuickConnectConfigElPhoneConfigEl { }

impl ToListMappable for ConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    type O = BlockAssignable<ConnectQuickConnectQuickConnectConfigElPhoneConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    #[doc = ""]
    pub phone_number: PrimField<String>,
}

impl BuildConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    pub fn build(self) -> ConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
        ConnectQuickConnectQuickConnectConfigElPhoneConfigEl { phone_number: self.phone_number }
    }
}

pub struct ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
    fn new(shared: StackShared, base: String) -> ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
        ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    contact_flow_id: PrimField<String>,
    queue_id: PrimField<String>,
}

impl ConnectQuickConnectQuickConnectConfigElQueueConfigEl { }

impl ToListMappable for ConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    type O = BlockAssignable<ConnectQuickConnectQuickConnectConfigElQueueConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    #[doc = ""]
    pub contact_flow_id: PrimField<String>,
    #[doc = ""]
    pub queue_id: PrimField<String>,
}

impl BuildConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    pub fn build(self) -> ConnectQuickConnectQuickConnectConfigElQueueConfigEl {
        ConnectQuickConnectQuickConnectConfigElQueueConfigEl {
            contact_flow_id: self.contact_flow_id,
            queue_id: self.queue_id,
        }
    }
}

pub struct ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
    fn new(shared: StackShared, base: String) -> ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
        ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_flow_id` after provisioning.\n"]
    pub fn contact_flow_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_id", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\n"]
    pub fn queue_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ConnectQuickConnectQuickConnectConfigElUserConfigEl {
    contact_flow_id: PrimField<String>,
    user_id: PrimField<String>,
}

impl ConnectQuickConnectQuickConnectConfigElUserConfigEl { }

impl ToListMappable for ConnectQuickConnectQuickConnectConfigElUserConfigEl {
    type O = BlockAssignable<ConnectQuickConnectQuickConnectConfigElUserConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnectQuickConnectConfigElUserConfigEl {
    #[doc = ""]
    pub contact_flow_id: PrimField<String>,
    #[doc = ""]
    pub user_id: PrimField<String>,
}

impl BuildConnectQuickConnectQuickConnectConfigElUserConfigEl {
    pub fn build(self) -> ConnectQuickConnectQuickConnectConfigElUserConfigEl {
        ConnectQuickConnectQuickConnectConfigElUserConfigEl {
            contact_flow_id: self.contact_flow_id,
            user_id: self.user_id,
        }
    }
}

pub struct ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
    fn new(shared: StackShared, base: String) -> ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
        ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnectQuickConnectConfigElUserConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_flow_id` after provisioning.\n"]
    pub fn contact_flow_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_id", self.base))
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectQuickConnectQuickConnectConfigElDynamic {
    phone_config: Option<DynamicBlock<ConnectQuickConnectQuickConnectConfigElPhoneConfigEl>>,
    queue_config: Option<DynamicBlock<ConnectQuickConnectQuickConnectConfigElQueueConfigEl>>,
    user_config: Option<DynamicBlock<ConnectQuickConnectQuickConnectConfigElUserConfigEl>>,
}

#[derive(Serialize)]
pub struct ConnectQuickConnectQuickConnectConfigEl {
    quick_connect_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_config: Option<Vec<ConnectQuickConnectQuickConnectConfigElPhoneConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_config: Option<Vec<ConnectQuickConnectQuickConnectConfigElQueueConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_config: Option<Vec<ConnectQuickConnectQuickConnectConfigElUserConfigEl>>,
    dynamic: ConnectQuickConnectQuickConnectConfigElDynamic,
}

impl ConnectQuickConnectQuickConnectConfigEl {
    #[doc = "Set the field `phone_config`.\n"]
    pub fn set_phone_config(
        mut self,
        v: impl Into<BlockAssignable<ConnectQuickConnectQuickConnectConfigElPhoneConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.phone_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.phone_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `queue_config`.\n"]
    pub fn set_queue_config(
        mut self,
        v: impl Into<BlockAssignable<ConnectQuickConnectQuickConnectConfigElQueueConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.queue_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.queue_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_config`.\n"]
    pub fn set_user_config(
        mut self,
        v: impl Into<BlockAssignable<ConnectQuickConnectQuickConnectConfigElUserConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ConnectQuickConnectQuickConnectConfigEl {
    type O = BlockAssignable<ConnectQuickConnectQuickConnectConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConnectQuickConnectQuickConnectConfigEl {
    #[doc = ""]
    pub quick_connect_type: PrimField<String>,
}

impl BuildConnectQuickConnectQuickConnectConfigEl {
    pub fn build(self) -> ConnectQuickConnectQuickConnectConfigEl {
        ConnectQuickConnectQuickConnectConfigEl {
            quick_connect_type: self.quick_connect_type,
            phone_config: core::default::Default::default(),
            queue_config: core::default::Default::default(),
            user_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ConnectQuickConnectQuickConnectConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectQuickConnectQuickConnectConfigElRef {
    fn new(shared: StackShared, base: String) -> ConnectQuickConnectQuickConnectConfigElRef {
        ConnectQuickConnectQuickConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConnectQuickConnectQuickConnectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `quick_connect_type` after provisioning.\n"]
    pub fn quick_connect_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.quick_connect_type", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(&self) -> ListRef<ConnectQuickConnectQuickConnectConfigElPhoneConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.phone_config", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_config` after provisioning.\n"]
    pub fn queue_config(&self) -> ListRef<ConnectQuickConnectQuickConnectConfigElQueueConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.queue_config", self.base))
    }

    #[doc = "Get a reference to the value of field `user_config` after provisioning.\n"]
    pub fn user_config(&self) -> ListRef<ConnectQuickConnectQuickConnectConfigElUserConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConnectQuickConnectDynamic {
    quick_connect_config: Option<DynamicBlock<ConnectQuickConnectQuickConnectConfigEl>>,
}
