use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCodeguruprofilerProfilingGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataCodeguruprofilerProfilingGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCodeguruprofilerProfilingGroupData>,
}
#[derive(Clone)]
pub struct DataCodeguruprofilerProfilingGroup(Rc<DataCodeguruprofilerProfilingGroup_>);
impl DataCodeguruprofilerProfilingGroup {
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
    #[doc = "Get a reference to the value of field `agent_orchestration_config` after provisioning.\n"]
    pub fn agent_orchestration_config(
        &self,
    ) -> ListRef<DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.agent_orchestration_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_platform", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `profiling_status` after provisioning.\n"]
    pub fn profiling_status(
        &self,
    ) -> ListRef<DataCodeguruprofilerProfilingGroupProfilingStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.profiling_status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
}
impl Referable for DataCodeguruprofilerProfilingGroup {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCodeguruprofilerProfilingGroup {}
impl ToListMappable for DataCodeguruprofilerProfilingGroup {
    type O = ListRef<DataCodeguruprofilerProfilingGroupRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCodeguruprofilerProfilingGroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_codeguruprofiler_profiling_group".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCodeguruprofilerProfilingGroup {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildDataCodeguruprofilerProfilingGroup {
    pub fn build(self, stack: &mut Stack) -> DataCodeguruprofilerProfilingGroup {
        let out =
            DataCodeguruprofilerProfilingGroup(Rc::new(DataCodeguruprofilerProfilingGroup_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataCodeguruprofilerProfilingGroupData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    name: self.name,
                    region: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataCodeguruprofilerProfilingGroupRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodeguruprofilerProfilingGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCodeguruprofilerProfilingGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `agent_orchestration_config` after provisioning.\n"]
    pub fn agent_orchestration_config(
        &self,
    ) -> ListRef<DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.agent_orchestration_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compute_platform", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `profiling_status` after provisioning.\n"]
    pub fn profiling_status(
        &self,
    ) -> ListRef<DataCodeguruprofilerProfilingGroupProfilingStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.profiling_status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    profiling_enabled: Option<PrimField<bool>>,
}
impl DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
    #[doc = "Set the field `profiling_enabled`.\n"]
    pub fn set_profiling_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.profiling_enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
    type O = BlockAssignable<DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {}
impl BuildDataCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
    pub fn build(self) -> DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
        DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigEl {
            profiling_enabled: core::default::Default::default(),
        }
    }
}
pub struct DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
        DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCodeguruprofilerProfilingGroupAgentOrchestrationConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `profiling_enabled` after provisioning.\n"]
    pub fn profiling_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.profiling_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<PrimField<String>>,
}
impl DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl {
    #[doc = "Set the field `period`.\n"]
    pub fn set_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.period = Some(v.into());
        self
    }
    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl
{
    type O = BlockAssignable<
        DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl {}
impl BuildDataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl {
    pub fn build(
        self,
    ) -> DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl {
        DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl {
            period: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}
pub struct DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileElRef {
        DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.base))
    }
    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCodeguruprofilerProfilingGroupProfilingStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_agent_orchestrated_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_agent_profile_reported_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latest_aggregated_profile: Option<
        ListField<DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl>,
    >,
}
impl DataCodeguruprofilerProfilingGroupProfilingStatusEl {
    #[doc = "Set the field `latest_agent_orchestrated_at`.\n"]
    pub fn set_latest_agent_orchestrated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.latest_agent_orchestrated_at = Some(v.into());
        self
    }
    #[doc = "Set the field `latest_agent_profile_reported_at`.\n"]
    pub fn set_latest_agent_profile_reported_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.latest_agent_profile_reported_at = Some(v.into());
        self
    }
    #[doc = "Set the field `latest_aggregated_profile`.\n"]
    pub fn set_latest_aggregated_profile(
        mut self,
        v: impl Into<
            ListField<DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileEl>,
        >,
    ) -> Self {
        self.latest_aggregated_profile = Some(v.into());
        self
    }
}
impl ToListMappable for DataCodeguruprofilerProfilingGroupProfilingStatusEl {
    type O = BlockAssignable<DataCodeguruprofilerProfilingGroupProfilingStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCodeguruprofilerProfilingGroupProfilingStatusEl {}
impl BuildDataCodeguruprofilerProfilingGroupProfilingStatusEl {
    pub fn build(self) -> DataCodeguruprofilerProfilingGroupProfilingStatusEl {
        DataCodeguruprofilerProfilingGroupProfilingStatusEl {
            latest_agent_orchestrated_at: core::default::Default::default(),
            latest_agent_profile_reported_at: core::default::Default::default(),
            latest_aggregated_profile: core::default::Default::default(),
        }
    }
}
pub struct DataCodeguruprofilerProfilingGroupProfilingStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCodeguruprofilerProfilingGroupProfilingStatusElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCodeguruprofilerProfilingGroupProfilingStatusElRef {
        DataCodeguruprofilerProfilingGroupProfilingStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCodeguruprofilerProfilingGroupProfilingStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `latest_agent_orchestrated_at` after provisioning.\n"]
    pub fn latest_agent_orchestrated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_agent_orchestrated_at", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `latest_agent_profile_reported_at` after provisioning.\n"]
    pub fn latest_agent_profile_reported_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.latest_agent_profile_reported_at", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `latest_aggregated_profile` after provisioning.\n"]
    pub fn latest_aggregated_profile(
        &self,
    ) -> ListRef<DataCodeguruprofilerProfilingGroupProfilingStatusElLatestAggregatedProfileElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.latest_aggregated_profile", self.base),
        )
    }
}
