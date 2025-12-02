use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVpclatticeListenerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    listener_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_identifier: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}
struct DataVpclatticeListener_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpclatticeListenerData>,
}
#[derive(Clone)]
pub struct DataVpclatticeListener(Rc<DataVpclatticeListener_>);
impl DataVpclatticeListener {
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
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<DataVpclatticeListenerDefaultActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_at` after provisioning.\n"]
    pub fn last_updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listener_id` after provisioning.\n"]
    pub fn listener_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listener_identifier` after provisioning.\n"]
    pub fn listener_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\n"]
    pub fn service_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_identifier", self.extract_ref()),
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
impl Referable for DataVpclatticeListener {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVpclatticeListener {}
impl ToListMappable for DataVpclatticeListener {
    type O = ListRef<DataVpclatticeListenerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVpclatticeListener_ {
    fn extract_datasource_type(&self) -> String {
        "aws_vpclattice_listener".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVpclatticeListener {
    pub tf_id: String,
    #[doc = ""]
    pub listener_identifier: PrimField<String>,
    #[doc = ""]
    pub service_identifier: PrimField<String>,
}
impl BuildDataVpclatticeListener {
    pub fn build(self, stack: &mut Stack) -> DataVpclatticeListener {
        let out = DataVpclatticeListener(Rc::new(DataVpclatticeListener_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpclatticeListenerData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                listener_identifier: self.listener_identifier,
                region: core::default::Default::default(),
                service_identifier: self.service_identifier,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVpclatticeListenerRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpclatticeListenerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVpclatticeListenerRef {
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
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<DataVpclatticeListenerDefaultActionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_action", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `last_updated_at` after provisioning.\n"]
    pub fn last_updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listener_id` after provisioning.\n"]
    pub fn listener_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `listener_identifier` after provisioning.\n"]
    pub fn listener_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.listener_identifier", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.port", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.protocol", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\n"]
    pub fn service_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_identifier", self.extract_ref()),
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
pub struct DataVpclatticeListenerDefaultActionElFixedResponseEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<PrimField<f64>>,
}
impl DataVpclatticeListenerDefaultActionElFixedResponseEl {
    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.status_code = Some(v.into());
        self
    }
}
impl ToListMappable for DataVpclatticeListenerDefaultActionElFixedResponseEl {
    type O = BlockAssignable<DataVpclatticeListenerDefaultActionElFixedResponseEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpclatticeListenerDefaultActionElFixedResponseEl {}
impl BuildDataVpclatticeListenerDefaultActionElFixedResponseEl {
    pub fn build(self) -> DataVpclatticeListenerDefaultActionElFixedResponseEl {
        DataVpclatticeListenerDefaultActionElFixedResponseEl {
            status_code: core::default::Default::default(),
        }
    }
}
pub struct DataVpclatticeListenerDefaultActionElFixedResponseElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpclatticeListenerDefaultActionElFixedResponseElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVpclatticeListenerDefaultActionElFixedResponseElRef {
        DataVpclatticeListenerDefaultActionElFixedResponseElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpclatticeListenerDefaultActionElFixedResponseElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_group_identifier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl DataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
    #[doc = "Set the field `target_group_identifier`.\n"]
    pub fn set_target_group_identifier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_group_identifier = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for DataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
    type O = BlockAssignable<DataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl {}
impl BuildDataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
    pub fn build(self) -> DataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
        DataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl {
            target_group_identifier: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct DataVpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataVpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
        DataVpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpclatticeListenerDefaultActionElForwardElTargetGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_group_identifier` after provisioning.\n"]
    pub fn target_group_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_group_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVpclatticeListenerDefaultActionElForwardEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target_groups: Option<ListField<DataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl>>,
}
impl DataVpclatticeListenerDefaultActionElForwardEl {
    #[doc = "Set the field `target_groups`.\n"]
    pub fn set_target_groups(
        mut self,
        v: impl Into<ListField<DataVpclatticeListenerDefaultActionElForwardElTargetGroupsEl>>,
    ) -> Self {
        self.target_groups = Some(v.into());
        self
    }
}
impl ToListMappable for DataVpclatticeListenerDefaultActionElForwardEl {
    type O = BlockAssignable<DataVpclatticeListenerDefaultActionElForwardEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpclatticeListenerDefaultActionElForwardEl {}
impl BuildDataVpclatticeListenerDefaultActionElForwardEl {
    pub fn build(self) -> DataVpclatticeListenerDefaultActionElForwardEl {
        DataVpclatticeListenerDefaultActionElForwardEl {
            target_groups: core::default::Default::default(),
        }
    }
}
pub struct DataVpclatticeListenerDefaultActionElForwardElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpclatticeListenerDefaultActionElForwardElRef {
    fn new(shared: StackShared, base: String) -> DataVpclatticeListenerDefaultActionElForwardElRef {
        DataVpclatticeListenerDefaultActionElForwardElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpclatticeListenerDefaultActionElForwardElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_groups` after provisioning.\n"]
    pub fn target_groups(
        &self,
    ) -> ListRef<DataVpclatticeListenerDefaultActionElForwardElTargetGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_groups", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataVpclatticeListenerDefaultActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_response: Option<ListField<DataVpclatticeListenerDefaultActionElFixedResponseEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward: Option<ListField<DataVpclatticeListenerDefaultActionElForwardEl>>,
}
impl DataVpclatticeListenerDefaultActionEl {
    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl Into<ListField<DataVpclatticeListenerDefaultActionElFixedResponseEl>>,
    ) -> Self {
        self.fixed_response = Some(v.into());
        self
    }
    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(
        mut self,
        v: impl Into<ListField<DataVpclatticeListenerDefaultActionElForwardEl>>,
    ) -> Self {
        self.forward = Some(v.into());
        self
    }
}
impl ToListMappable for DataVpclatticeListenerDefaultActionEl {
    type O = BlockAssignable<DataVpclatticeListenerDefaultActionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpclatticeListenerDefaultActionEl {}
impl BuildDataVpclatticeListenerDefaultActionEl {
    pub fn build(self) -> DataVpclatticeListenerDefaultActionEl {
        DataVpclatticeListenerDefaultActionEl {
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
        }
    }
}
pub struct DataVpclatticeListenerDefaultActionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpclatticeListenerDefaultActionElRef {
    fn new(shared: StackShared, base: String) -> DataVpclatticeListenerDefaultActionElRef {
        DataVpclatticeListenerDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpclatticeListenerDefaultActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(
        &self,
    ) -> ListRef<DataVpclatticeListenerDefaultActionElFixedResponseElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fixed_response", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> ListRef<DataVpclatticeListenerDefaultActionElForwardElRef> {
        ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }
}
