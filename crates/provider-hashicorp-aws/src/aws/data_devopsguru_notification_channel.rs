use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataDevopsguruNotificationChannelData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<Vec<DataDevopsguruNotificationChannelFiltersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sns: Option<Vec<DataDevopsguruNotificationChannelSnsEl>>,
    dynamic: DataDevopsguruNotificationChannelDynamic,
}

struct DataDevopsguruNotificationChannel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDevopsguruNotificationChannelData>,
}

#[derive(Clone)]
pub struct DataDevopsguruNotificationChannel(Rc<DataDevopsguruNotificationChannel_>);

impl DataDevopsguruNotificationChannel {
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

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        self,
        v: impl Into<BlockAssignable<DataDevopsguruNotificationChannelFiltersEl>>,
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
        v: impl Into<BlockAssignable<DataDevopsguruNotificationChannelSnsEl>>,
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
    pub fn filters(&self) -> ListRef<DataDevopsguruNotificationChannelFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> ListRef<DataDevopsguruNotificationChannelSnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sns", self.extract_ref()))
    }
}

impl Referable for DataDevopsguruNotificationChannel {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataDevopsguruNotificationChannel {}

impl ToListMappable for DataDevopsguruNotificationChannel {
    type O = ListRef<DataDevopsguruNotificationChannelRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDevopsguruNotificationChannel_ {
    fn extract_datasource_type(&self) -> String {
        "aws_devopsguru_notification_channel".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDevopsguruNotificationChannel {
    pub tf_id: String,
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildDataDevopsguruNotificationChannel {
    pub fn build(self, stack: &mut Stack) -> DataDevopsguruNotificationChannel {
        let out = DataDevopsguruNotificationChannel(Rc::new(DataDevopsguruNotificationChannel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDevopsguruNotificationChannelData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: self.id,
                region: core::default::Default::default(),
                filters: core::default::Default::default(),
                sns: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDevopsguruNotificationChannelRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDevopsguruNotificationChannelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataDevopsguruNotificationChannelRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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
    pub fn filters(&self) -> ListRef<DataDevopsguruNotificationChannelFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sns` after provisioning.\n"]
    pub fn sns(&self) -> ListRef<DataDevopsguruNotificationChannelSnsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sns", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDevopsguruNotificationChannelFiltersEl {}

impl DataDevopsguruNotificationChannelFiltersEl {}

impl ToListMappable for DataDevopsguruNotificationChannelFiltersEl {
    type O = BlockAssignable<DataDevopsguruNotificationChannelFiltersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDevopsguruNotificationChannelFiltersEl {}

impl BuildDataDevopsguruNotificationChannelFiltersEl {
    pub fn build(self) -> DataDevopsguruNotificationChannelFiltersEl {
        DataDevopsguruNotificationChannelFiltersEl {}
    }
}

pub struct DataDevopsguruNotificationChannelFiltersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDevopsguruNotificationChannelFiltersElRef {
    fn new(shared: StackShared, base: String) -> DataDevopsguruNotificationChannelFiltersElRef {
        DataDevopsguruNotificationChannelFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDevopsguruNotificationChannelFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message_types` after provisioning.\n"]
    pub fn message_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.message_types", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `severities` after provisioning.\n"]
    pub fn severities(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.severities", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDevopsguruNotificationChannelSnsEl {}

impl DataDevopsguruNotificationChannelSnsEl {}

impl ToListMappable for DataDevopsguruNotificationChannelSnsEl {
    type O = BlockAssignable<DataDevopsguruNotificationChannelSnsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDevopsguruNotificationChannelSnsEl {}

impl BuildDataDevopsguruNotificationChannelSnsEl {
    pub fn build(self) -> DataDevopsguruNotificationChannelSnsEl {
        DataDevopsguruNotificationChannelSnsEl {}
    }
}

pub struct DataDevopsguruNotificationChannelSnsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDevopsguruNotificationChannelSnsElRef {
    fn new(shared: StackShared, base: String) -> DataDevopsguruNotificationChannelSnsElRef {
        DataDevopsguruNotificationChannelSnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDevopsguruNotificationChannelSnsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataDevopsguruNotificationChannelDynamic {
    filters: Option<DynamicBlock<DataDevopsguruNotificationChannelFiltersEl>>,
    sns: Option<DynamicBlock<DataDevopsguruNotificationChannelSnsEl>>,
}
