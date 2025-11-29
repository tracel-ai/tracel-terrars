use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmcontactsPlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    contact_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataSsmcontactsPlan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsmcontactsPlanData>,
}

#[derive(Clone)]
pub struct DataSsmcontactsPlan(Rc<DataSsmcontactsPlan_>);

impl DataSsmcontactsPlan {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> ListRef<DataSsmcontactsPlanStageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

impl Referable for DataSsmcontactsPlan {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSsmcontactsPlan { }

impl ToListMappable for DataSsmcontactsPlan {
    type O = ListRef<DataSsmcontactsPlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsmcontactsPlan_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssmcontacts_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmcontactsPlan {
    pub tf_id: String,
    #[doc = ""]
    pub contact_id: PrimField<String>,
}

impl BuildDataSsmcontactsPlan {
    pub fn build(self, stack: &mut Stack) -> DataSsmcontactsPlan {
        let out = DataSsmcontactsPlan(Rc::new(DataSsmcontactsPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsmcontactsPlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                contact_id: self.contact_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmcontactsPlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsPlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSsmcontactsPlanRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> ListRef<DataSsmcontactsPlanStageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.stage", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsPlanStageElTargetElChannelTargetInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_channel_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_interval_in_minutes: Option<PrimField<f64>>,
}

impl DataSsmcontactsPlanStageElTargetElChannelTargetInfoEl {
    #[doc = "Set the field `contact_channel_id`.\n"]
    pub fn set_contact_channel_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_channel_id = Some(v.into());
        self
    }

    #[doc = "Set the field `retry_interval_in_minutes`.\n"]
    pub fn set_retry_interval_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.retry_interval_in_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsPlanStageElTargetElChannelTargetInfoEl {
    type O = BlockAssignable<DataSsmcontactsPlanStageElTargetElChannelTargetInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsPlanStageElTargetElChannelTargetInfoEl {}

impl BuildDataSsmcontactsPlanStageElTargetElChannelTargetInfoEl {
    pub fn build(self) -> DataSsmcontactsPlanStageElTargetElChannelTargetInfoEl {
        DataSsmcontactsPlanStageElTargetElChannelTargetInfoEl {
            contact_channel_id: core::default::Default::default(),
            retry_interval_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
    fn new(shared: StackShared, base: String) -> DataSsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
        DataSsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsPlanStageElTargetElChannelTargetInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_channel_id` after provisioning.\n"]
    pub fn contact_channel_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_channel_id", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_interval_in_minutes` after provisioning.\n"]
    pub fn retry_interval_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.retry_interval_in_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsPlanStageElTargetElContactTargetInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_essential: Option<PrimField<bool>>,
}

impl DataSsmcontactsPlanStageElTargetElContactTargetInfoEl {
    #[doc = "Set the field `contact_id`.\n"]
    pub fn set_contact_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.contact_id = Some(v.into());
        self
    }

    #[doc = "Set the field `is_essential`.\n"]
    pub fn set_is_essential(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_essential = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsPlanStageElTargetElContactTargetInfoEl {
    type O = BlockAssignable<DataSsmcontactsPlanStageElTargetElContactTargetInfoEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsPlanStageElTargetElContactTargetInfoEl {}

impl BuildDataSsmcontactsPlanStageElTargetElContactTargetInfoEl {
    pub fn build(self) -> DataSsmcontactsPlanStageElTargetElContactTargetInfoEl {
        DataSsmcontactsPlanStageElTargetElContactTargetInfoEl {
            contact_id: core::default::Default::default(),
            is_essential: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsPlanStageElTargetElContactTargetInfoElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsPlanStageElTargetElContactTargetInfoElRef {
    fn new(shared: StackShared, base: String) -> DataSsmcontactsPlanStageElTargetElContactTargetInfoElRef {
        DataSsmcontactsPlanStageElTargetElContactTargetInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsPlanStageElTargetElContactTargetInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contact_id` after provisioning.\n"]
    pub fn contact_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_id", self.base))
    }

    #[doc = "Get a reference to the value of field `is_essential` after provisioning.\n"]
    pub fn is_essential(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_essential", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsPlanStageElTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_target_info: Option<ListField<DataSsmcontactsPlanStageElTargetElChannelTargetInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_target_info: Option<ListField<DataSsmcontactsPlanStageElTargetElContactTargetInfoEl>>,
}

impl DataSsmcontactsPlanStageElTargetEl {
    #[doc = "Set the field `channel_target_info`.\n"]
    pub fn set_channel_target_info(
        mut self,
        v: impl Into<ListField<DataSsmcontactsPlanStageElTargetElChannelTargetInfoEl>>,
    ) -> Self {
        self.channel_target_info = Some(v.into());
        self
    }

    #[doc = "Set the field `contact_target_info`.\n"]
    pub fn set_contact_target_info(
        mut self,
        v: impl Into<ListField<DataSsmcontactsPlanStageElTargetElContactTargetInfoEl>>,
    ) -> Self {
        self.contact_target_info = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsPlanStageElTargetEl {
    type O = BlockAssignable<DataSsmcontactsPlanStageElTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsPlanStageElTargetEl {}

impl BuildDataSsmcontactsPlanStageElTargetEl {
    pub fn build(self) -> DataSsmcontactsPlanStageElTargetEl {
        DataSsmcontactsPlanStageElTargetEl {
            channel_target_info: core::default::Default::default(),
            contact_target_info: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsPlanStageElTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsPlanStageElTargetElRef {
    fn new(shared: StackShared, base: String) -> DataSsmcontactsPlanStageElTargetElRef {
        DataSsmcontactsPlanStageElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsPlanStageElTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_target_info` after provisioning.\n"]
    pub fn channel_target_info(&self) -> ListRef<DataSsmcontactsPlanStageElTargetElChannelTargetInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.channel_target_info", self.base))
    }

    #[doc = "Get a reference to the value of field `contact_target_info` after provisioning.\n"]
    pub fn contact_target_info(&self) -> ListRef<DataSsmcontactsPlanStageElTargetElContactTargetInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.contact_target_info", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmcontactsPlanStageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_in_minutes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<ListField<DataSsmcontactsPlanStageElTargetEl>>,
}

impl DataSsmcontactsPlanStageEl {
    #[doc = "Set the field `duration_in_minutes`.\n"]
    pub fn set_duration_in_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.duration_in_minutes = Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl Into<ListField<DataSsmcontactsPlanStageElTargetEl>>) -> Self {
        self.target = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmcontactsPlanStageEl {
    type O = BlockAssignable<DataSsmcontactsPlanStageEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmcontactsPlanStageEl {}

impl BuildDataSsmcontactsPlanStageEl {
    pub fn build(self) -> DataSsmcontactsPlanStageEl {
        DataSsmcontactsPlanStageEl {
            duration_in_minutes: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

pub struct DataSsmcontactsPlanStageElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmcontactsPlanStageElRef {
    fn new(shared: StackShared, base: String) -> DataSsmcontactsPlanStageElRef {
        DataSsmcontactsPlanStageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmcontactsPlanStageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration_in_minutes` after provisioning.\n"]
    pub fn duration_in_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<DataSsmcontactsPlanStageElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
