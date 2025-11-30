use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DevopsguruNotificationChannelData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Vec<DevopsguruNotificationChannelFiltersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns: Option<Vec<DevopsguruNotificationChannelSnsEl>>,
    dynamic: DevopsguruNotificationChannelDynamic,
}

struct DevopsguruNotificationChannel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DevopsguruNotificationChannelData>,
}

#[derive(Clone)]
pub struct DevopsguruNotificationChannel(Rc<DevopsguruNotificationChannel_>);

impl DevopsguruNotificationChannel {
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

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        self,
        v: impl Into<BlockAssignable<DevopsguruNotificationChannelFiltersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `sns`.\n"]
    pub fn set_sns(
        self,
        v: impl Into<BlockAssignable<DevopsguruNotificationChannelSnsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns = Some(d);
            }
        }
        self
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

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<DevopsguruNotificationChannelFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> ListRef<DevopsguruNotificationChannelSnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sns", self.extract_ref()))
    }
}

impl Referable for DevopsguruNotificationChannel {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DevopsguruNotificationChannel {}

impl ToListMappable for DevopsguruNotificationChannel {
    type O = ListRef<DevopsguruNotificationChannelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DevopsguruNotificationChannel_ {
    fn extract_resource_type(&self) -> String {
        "aws_devopsguru_notification_channel".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevopsguruNotificationChannel {
    pub tf_id: String,
}

impl BuildDevopsguruNotificationChannel {
    pub fn build(self, stack: &mut Stack) -> DevopsguruNotificationChannel {
        let out = DevopsguruNotificationChannel(Rc::new(DevopsguruNotificationChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DevopsguruNotificationChannelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                filters: core::default::Default::default(),
                sns: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevopsguruNotificationChannelRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruNotificationChannelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DevopsguruNotificationChannelRef {
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<DevopsguruNotificationChannelFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> ListRef<DevopsguruNotificationChannelSnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sns", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DevopsguruNotificationChannelFiltersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_types: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    severities: Option<SetField<PrimField<String>>>,
}

impl DevopsguruNotificationChannelFiltersEl {
    #[doc = "Set the field `message_types`.\n"]
    pub fn set_message_types(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.message_types = Some(v.into());
        self
    }

    #[doc = "Set the field `severities`.\n"]
    pub fn set_severities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.severities = Some(v.into());
        self
    }
}

impl ToListMappable for DevopsguruNotificationChannelFiltersEl {
    type O = BlockAssignable<DevopsguruNotificationChannelFiltersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruNotificationChannelFiltersEl {}

impl BuildDevopsguruNotificationChannelFiltersEl {
    pub fn build(self) -> DevopsguruNotificationChannelFiltersEl {
        DevopsguruNotificationChannelFiltersEl {
            message_types: core::default::Default::default(),
            severities: core::default::Default::default(),
        }
    }
}

pub struct DevopsguruNotificationChannelFiltersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruNotificationChannelFiltersElRef {
    fn new(shared: StackShared, base: String) -> DevopsguruNotificationChannelFiltersElRef {
        DevopsguruNotificationChannelFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruNotificationChannelFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_types` after provisioning.\n"]
    pub fn message_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.message_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `severities` after provisioning.\n"]
    pub fn severities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.severities", self.base))
    }
}

#[derive(Serialize)]
pub struct DevopsguruNotificationChannelSnsEl {
    topic_arn: PrimField<String>,
}

impl DevopsguruNotificationChannelSnsEl {}

impl ToListMappable for DevopsguruNotificationChannelSnsEl {
    type O = BlockAssignable<DevopsguruNotificationChannelSnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruNotificationChannelSnsEl {
    #[doc = ""]
    pub topic_arn: PrimField<String>,
}

impl BuildDevopsguruNotificationChannelSnsEl {
    pub fn build(self) -> DevopsguruNotificationChannelSnsEl {
        DevopsguruNotificationChannelSnsEl {
            topic_arn: self.topic_arn,
        }
    }
}

pub struct DevopsguruNotificationChannelSnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruNotificationChannelSnsElRef {
    fn new(shared: StackShared, base: String) -> DevopsguruNotificationChannelSnsElRef {
        DevopsguruNotificationChannelSnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruNotificationChannelSnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct DevopsguruNotificationChannelDynamic {
    filters: Option<DynamicBlock<DevopsguruNotificationChannelFiltersEl>>,
    sns: Option<DynamicBlock<DevopsguruNotificationChannelSnsEl>>,
}
