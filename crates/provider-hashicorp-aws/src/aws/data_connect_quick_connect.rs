use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataConnectQuickConnectData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quick_connect_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataConnectQuickConnect_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataConnectQuickConnectData>,
}

#[derive(Clone)]
pub struct DataConnectQuickConnect(Rc<DataConnectQuickConnect_>);

impl DataConnectQuickConnect {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Set the field `quick_connect_id`.\n"]
    pub fn set_quick_connect_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().quick_connect_id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> ListRef<DataConnectQuickConnectQuickConnectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quick_connect_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `quick_connect_id` after provisioning.\n"]
    pub fn quick_connect_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quick_connect_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

impl Referable for DataConnectQuickConnect {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataConnectQuickConnect {}

impl ToListMappable for DataConnectQuickConnect {
    type O = ListRef<DataConnectQuickConnectRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataConnectQuickConnect_ {
    fn extract_datasource_type(&self) -> String {
        "aws_connect_quick_connect".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataConnectQuickConnect {
    pub tf_id: String,
    #[doc = ""]
    pub instance_id: PrimField<String>,
}

impl BuildDataConnectQuickConnect {
    pub fn build(self, stack: &mut Stack) -> DataConnectQuickConnect {
        let out = DataConnectQuickConnect(Rc::new(DataConnectQuickConnect_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataConnectQuickConnectData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: core::default::Default::default(),
                quick_connect_id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataConnectQuickConnectRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectQuickConnectRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataConnectQuickConnectRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `quick_connect_config` after provisioning.\n"]
    pub fn quick_connect_config(&self) -> ListRef<DataConnectQuickConnectQuickConnectConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.quick_connect_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `quick_connect_id` after provisioning.\n"]
    pub fn quick_connect_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quick_connect_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<PrimField<String>>,
}

impl DataConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.phone_number = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    type O = BlockAssignable<DataConnectQuickConnectQuickConnectConfigElPhoneConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectQuickConnectQuickConnectConfigElPhoneConfigEl {}

impl BuildDataConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
    pub fn build(self) -> DataConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
        DataConnectQuickConnectQuickConnectConfigElPhoneConfigEl {
            phone_number: core::default::Default::default(),
        }
    }
}

pub struct DataConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
        DataConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectQuickConnectQuickConnectConfigElPhoneConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_flow_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_id: Option<PrimField<String>>,
}

impl DataConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    #[doc = "Set the field `contact_flow_id`.\n"]
    pub fn set_contact_flow_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_flow_id = Some(v.into());
        self
    }

    #[doc = "Set the field `queue_id`.\n"]
    pub fn set_queue_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.queue_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    type O = BlockAssignable<DataConnectQuickConnectQuickConnectConfigElQueueConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectQuickConnectQuickConnectConfigElQueueConfigEl {}

impl BuildDataConnectQuickConnectQuickConnectConfigElQueueConfigEl {
    pub fn build(self) -> DataConnectQuickConnectQuickConnectConfigElQueueConfigEl {
        DataConnectQuickConnectQuickConnectConfigElQueueConfigEl {
            contact_flow_id: core::default::Default::default(),
            queue_id: core::default::Default::default(),
        }
    }
}

pub struct DataConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
        DataConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectQuickConnectQuickConnectConfigElQueueConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_flow_id` after provisioning.\n"]
    pub fn contact_flow_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_flow_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `queue_id` after provisioning.\n"]
    pub fn queue_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectQuickConnectQuickConnectConfigElUserConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_flow_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<PrimField<String>>,
}

impl DataConnectQuickConnectQuickConnectConfigElUserConfigEl {
    #[doc = "Set the field `contact_flow_id`.\n"]
    pub fn set_contact_flow_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_flow_id = Some(v.into());
        self
    }

    #[doc = "Set the field `user_id`.\n"]
    pub fn set_user_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_id = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectQuickConnectQuickConnectConfigElUserConfigEl {
    type O = BlockAssignable<DataConnectQuickConnectQuickConnectConfigElUserConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectQuickConnectQuickConnectConfigElUserConfigEl {}

impl BuildDataConnectQuickConnectQuickConnectConfigElUserConfigEl {
    pub fn build(self) -> DataConnectQuickConnectQuickConnectConfigElUserConfigEl {
        DataConnectQuickConnectQuickConnectConfigElUserConfigEl {
            contact_flow_id: core::default::Default::default(),
            user_id: core::default::Default::default(),
        }
    }
}

pub struct DataConnectQuickConnectQuickConnectConfigElUserConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectQuickConnectQuickConnectConfigElUserConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataConnectQuickConnectQuickConnectConfigElUserConfigElRef {
        DataConnectQuickConnectQuickConnectConfigElUserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectQuickConnectQuickConnectConfigElUserConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_flow_id` after provisioning.\n"]
    pub fn contact_flow_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.contact_flow_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_id` after provisioning.\n"]
    pub fn user_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataConnectQuickConnectQuickConnectConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_config: Option<ListField<DataConnectQuickConnectQuickConnectConfigElPhoneConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_config: Option<ListField<DataConnectQuickConnectQuickConnectConfigElQueueConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quick_connect_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_config: Option<ListField<DataConnectQuickConnectQuickConnectConfigElUserConfigEl>>,
}

impl DataConnectQuickConnectQuickConnectConfigEl {
    #[doc = "Set the field `phone_config`.\n"]
    pub fn set_phone_config(
        mut self,
        v: impl Into<ListField<DataConnectQuickConnectQuickConnectConfigElPhoneConfigEl>>,
    ) -> Self {
        self.phone_config = Some(v.into());
        self
    }

    #[doc = "Set the field `queue_config`.\n"]
    pub fn set_queue_config(
        mut self,
        v: impl Into<ListField<DataConnectQuickConnectQuickConnectConfigElQueueConfigEl>>,
    ) -> Self {
        self.queue_config = Some(v.into());
        self
    }

    #[doc = "Set the field `quick_connect_type`.\n"]
    pub fn set_quick_connect_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.quick_connect_type = Some(v.into());
        self
    }

    #[doc = "Set the field `user_config`.\n"]
    pub fn set_user_config(
        mut self,
        v: impl Into<ListField<DataConnectQuickConnectQuickConnectConfigElUserConfigEl>>,
    ) -> Self {
        self.user_config = Some(v.into());
        self
    }
}

impl ToListMappable for DataConnectQuickConnectQuickConnectConfigEl {
    type O = BlockAssignable<DataConnectQuickConnectQuickConnectConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataConnectQuickConnectQuickConnectConfigEl {}

impl BuildDataConnectQuickConnectQuickConnectConfigEl {
    pub fn build(self) -> DataConnectQuickConnectQuickConnectConfigEl {
        DataConnectQuickConnectQuickConnectConfigEl {
            phone_config: core::default::Default::default(),
            queue_config: core::default::Default::default(),
            quick_connect_type: core::default::Default::default(),
            user_config: core::default::Default::default(),
        }
    }
}

pub struct DataConnectQuickConnectQuickConnectConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataConnectQuickConnectQuickConnectConfigElRef {
    fn new(shared: StackShared, base: String) -> DataConnectQuickConnectQuickConnectConfigElRef {
        DataConnectQuickConnectQuickConnectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataConnectQuickConnectQuickConnectConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `phone_config` after provisioning.\n"]
    pub fn phone_config(
        &self,
    ) -> ListRef<DataConnectQuickConnectQuickConnectConfigElPhoneConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.phone_config", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_config` after provisioning.\n"]
    pub fn queue_config(
        &self,
    ) -> ListRef<DataConnectQuickConnectQuickConnectConfigElQueueConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.queue_config", self.base))
    }

    #[doc = "Get a reference to the value of field `quick_connect_type` after provisioning.\n"]
    pub fn quick_connect_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.quick_connect_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_config` after provisioning.\n"]
    pub fn user_config(
        &self,
    ) -> ListRef<DataConnectQuickConnectQuickConnectConfigElUserConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.user_config", self.base))
    }
}
