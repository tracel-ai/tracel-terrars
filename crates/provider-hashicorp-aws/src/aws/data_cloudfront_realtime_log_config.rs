use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataCloudfrontRealtimeLogConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataCloudfrontRealtimeLogConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudfrontRealtimeLogConfigData>,
}

#[derive(Clone)]
pub struct DataCloudfrontRealtimeLogConfig(Rc<DataCloudfrontRealtimeLogConfig_>);

impl DataCloudfrontRealtimeLogConfig {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<DataCloudfrontRealtimeLogConfigEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
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
}

impl Referable for DataCloudfrontRealtimeLogConfig {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataCloudfrontRealtimeLogConfig {}

impl ToListMappable for DataCloudfrontRealtimeLogConfig {
    type O = ListRef<DataCloudfrontRealtimeLogConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudfrontRealtimeLogConfig_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudfront_realtime_log_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontRealtimeLogConfig {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildDataCloudfrontRealtimeLogConfig {
    pub fn build(self, stack: &mut Stack) -> DataCloudfrontRealtimeLogConfig {
        let out = DataCloudfrontRealtimeLogConfig(Rc::new(DataCloudfrontRealtimeLogConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudfrontRealtimeLogConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfrontRealtimeLogConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontRealtimeLogConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataCloudfrontRealtimeLogConfigRef {
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

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> ListRef<DataCloudfrontRealtimeLogConfigEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint", self.extract_ref()),
        )
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
}

#[derive(Serialize)]
pub struct DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_arn: Option<PrimField<String>>,
}

impl DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_arn = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    type O = BlockAssignable<DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {}

impl BuildDataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
    pub fn build(self) -> DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
        DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl {
            role_arn: core::default::Default::default(),
            stream_arn: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
        DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef {
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

#[derive(Serialize)]
pub struct DataCloudfrontRealtimeLogConfigEndpointEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kinesis_stream_config:
        Option<ListField<DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_type: Option<PrimField<String>>,
}

impl DataCloudfrontRealtimeLogConfigEndpointEl {
    #[doc = "Set the field `kinesis_stream_config`.\n"]
    pub fn set_kinesis_stream_config(
        mut self,
        v: impl Into<ListField<DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigEl>>,
    ) -> Self {
        self.kinesis_stream_config = Some(v.into());
        self
    }

    #[doc = "Set the field `stream_type`.\n"]
    pub fn set_stream_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stream_type = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudfrontRealtimeLogConfigEndpointEl {
    type O = BlockAssignable<DataCloudfrontRealtimeLogConfigEndpointEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontRealtimeLogConfigEndpointEl {}

impl BuildDataCloudfrontRealtimeLogConfigEndpointEl {
    pub fn build(self) -> DataCloudfrontRealtimeLogConfigEndpointEl {
        DataCloudfrontRealtimeLogConfigEndpointEl {
            kinesis_stream_config: core::default::Default::default(),
            stream_type: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontRealtimeLogConfigEndpointElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontRealtimeLogConfigEndpointElRef {
    fn new(shared: StackShared, base: String) -> DataCloudfrontRealtimeLogConfigEndpointElRef {
        DataCloudfrontRealtimeLogConfigEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontRealtimeLogConfigEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kinesis_stream_config` after provisioning.\n"]
    pub fn kinesis_stream_config(
        &self,
    ) -> ListRef<DataCloudfrontRealtimeLogConfigEndpointElKinesisStreamConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kinesis_stream_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `stream_type` after provisioning.\n"]
    pub fn stream_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stream_type", self.base))
    }
}
