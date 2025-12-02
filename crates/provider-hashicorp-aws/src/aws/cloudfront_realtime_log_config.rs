use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudfrontRealtimeLogConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    fields: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    sampling_rate: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<Vec<CloudfrontRealtimeLogConfigEndpointEl>>,
    dynamic: CloudfrontRealtimeLogConfigDynamic,
}
struct CloudfrontRealtimeLogConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontRealtimeLogConfigData>,
}
#[derive(Clone)]
pub struct CloudfrontRealtimeLogConfig(Rc<CloudfrontRealtimeLogConfig_>);
impl CloudfrontRealtimeLogConfig {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(
        self,
        v: impl Into<BlockAssignable<CloudfrontRealtimeLogConfigEndpointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.fields", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sampling_rate` after provisioning.\n"]
    pub fn sampling_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sampling_rate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<CloudfrontRealtimeLogConfigEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
}
impl Referable for CloudfrontRealtimeLogConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudfrontRealtimeLogConfig {}
impl ToListMappable for CloudfrontRealtimeLogConfig {
    type O = ListRef<CloudfrontRealtimeLogConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudfrontRealtimeLogConfig_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_realtime_log_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudfrontRealtimeLogConfig {
    pub tf_id: String,
    #[doc = ""]
    pub fields: SetField<PrimField<String>>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub sampling_rate: PrimField<f64>,
}
impl BuildCloudfrontRealtimeLogConfig {
    pub fn build(self, stack: &mut Stack) -> CloudfrontRealtimeLogConfig {
        let out = CloudfrontRealtimeLogConfig(Rc::new(CloudfrontRealtimeLogConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontRealtimeLogConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                fields: self.fields,
                id: core::default::Default::default(),
                name: self.name,
                sampling_rate: self.sampling_rate,
                endpoint: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudfrontRealtimeLogConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontRealtimeLogConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudfrontRealtimeLogConfigRef {
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
    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.fields", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `sampling_rate` after provisioning.\n"]
    pub fn sampling_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sampling_rate", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<CloudfrontRealtimeLogConfigEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    role_arn: PrimField<String>,
    stream_arn: PrimField<String>,
}
impl CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {}
impl ToListMappable for CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    type O = BlockAssignable<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    #[doc = ""]
    pub role_arn: PrimField<String>,
    #[doc = ""]
    pub stream_arn: PrimField<String>,
}
impl BuildCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    pub fn build(self) -> CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
        CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
            role_arn: self.role_arn,
            stream_arn: self.stream_arn,
        }
    }
}
pub struct CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
        CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfrontRealtimeLogConfigEndpointElDynamic {
    kinesis_stream_config:
        Option<DynamicBlock<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>>,
}
#[derive(Serialize)]
pub struct CloudfrontRealtimeLogConfigEndpointEl {
    stream_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_stream_config: Option<Vec<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>>,
    dynamic: CloudfrontRealtimeLogConfigEndpointElDynamic,
}
impl CloudfrontRealtimeLogConfigEndpointEl {
    #[doc = "Set the field `kinesis_stream_config`.\n"]
    pub fn set_kinesis_stream_config(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kinesis_stream_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kinesis_stream_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontRealtimeLogConfigEndpointEl {
    type O = BlockAssignable<CloudfrontRealtimeLogConfigEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontRealtimeLogConfigEndpointEl {
    #[doc = ""]
    pub stream_type: PrimField<String>,
}
impl BuildCloudfrontRealtimeLogConfigEndpointEl {
    pub fn build(self) -> CloudfrontRealtimeLogConfigEndpointEl {
        CloudfrontRealtimeLogConfigEndpointEl {
            stream_type: self.stream_type,
            kinesis_stream_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontRealtimeLogConfigEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontRealtimeLogConfigEndpointElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontRealtimeLogConfigEndpointElRef {
        CloudfrontRealtimeLogConfigEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontRealtimeLogConfigEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `stream_type` after provisioning.\n"]
    pub fn stream_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_type", self.base))
    }
    #[doc = "Get a reference to the value of field `kinesis_stream_config` after provisioning.\n"]
    pub fn kinesis_stream_config(
        &self,
    ) -> ListRef<CloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_stream_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudfrontRealtimeLogConfigDynamic {
    endpoint: Option<DynamicBlock<CloudfrontRealtimeLogConfigEndpointEl>>,
}
