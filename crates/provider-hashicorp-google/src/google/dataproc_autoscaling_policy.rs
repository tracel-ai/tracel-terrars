use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataprocAutoscalingPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    policy_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_algorithm: Option<Vec<DataprocAutoscalingPolicyBasicAlgorithmEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_worker_config: Option<Vec<DataprocAutoscalingPolicySecondaryWorkerConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataprocAutoscalingPolicyTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_config: Option<Vec<DataprocAutoscalingPolicyWorkerConfigEl>>,
    dynamic: DataprocAutoscalingPolicyDynamic,
}
struct DataprocAutoscalingPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataprocAutoscalingPolicyData>,
}
#[derive(Clone)]
pub struct DataprocAutoscalingPolicy(Rc<DataprocAutoscalingPolicy_>);
impl DataprocAutoscalingPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe  location where the autoscaling policy should reside.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `basic_algorithm`.\n"]
    pub fn set_basic_algorithm(
        self,
        v: impl Into<BlockAssignable<DataprocAutoscalingPolicyBasicAlgorithmEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().basic_algorithm = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.basic_algorithm = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `secondary_worker_config`.\n"]
    pub fn set_secondary_worker_config(
        self,
        v: impl Into<BlockAssignable<DataprocAutoscalingPolicySecondaryWorkerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_worker_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_worker_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataprocAutoscalingPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `worker_config`.\n"]
    pub fn set_worker_config(
        self,
        v: impl Into<BlockAssignable<DataprocAutoscalingPolicyWorkerConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().worker_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.worker_config = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe  location where the autoscaling policy should reside.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe \"resource name\" of the autoscaling policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 50 characters."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_algorithm` after provisioning.\n"]
    pub fn basic_algorithm(&self) -> ListRef<DataprocAutoscalingPolicyBasicAlgorithmElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_worker_config` after provisioning.\n"]
    pub fn secondary_worker_config(
        &self,
    ) -> ListRef<DataprocAutoscalingPolicySecondaryWorkerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_worker_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocAutoscalingPolicyTimeoutsElRef {
        DataprocAutoscalingPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(&self) -> ListRef<DataprocAutoscalingPolicyWorkerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.worker_config", self.extract_ref()),
        )
    }
}
impl Referable for DataprocAutoscalingPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataprocAutoscalingPolicy {}
impl ToListMappable for DataprocAutoscalingPolicy {
    type O = ListRef<DataprocAutoscalingPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataprocAutoscalingPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_dataproc_autoscaling_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataprocAutoscalingPolicy {
    pub tf_id: String,
    #[doc = "The policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 50 characters."]
    pub policy_id: PrimField<String>,
}
impl BuildDataprocAutoscalingPolicy {
    pub fn build(self, stack: &mut Stack) -> DataprocAutoscalingPolicy {
        let out = DataprocAutoscalingPolicy(Rc::new(DataprocAutoscalingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataprocAutoscalingPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                policy_id: self.policy_id,
                project: core::default::Default::default(),
                basic_algorithm: core::default::Default::default(),
                secondary_worker_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                worker_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataprocAutoscalingPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocAutoscalingPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataprocAutoscalingPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe  location where the autoscaling policy should reside.\nThe default value is 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe \"resource name\" of the autoscaling policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\nThe policy id. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 50 characters."]
    pub fn policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic_algorithm` after provisioning.\n"]
    pub fn basic_algorithm(&self) -> ListRef<DataprocAutoscalingPolicyBasicAlgorithmElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic_algorithm", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `secondary_worker_config` after provisioning.\n"]
    pub fn secondary_worker_config(
        &self,
    ) -> ListRef<DataprocAutoscalingPolicySecondaryWorkerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.secondary_worker_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocAutoscalingPolicyTimeoutsElRef {
        DataprocAutoscalingPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `worker_config` after provisioning.\n"]
    pub fn worker_config(&self) -> ListRef<DataprocAutoscalingPolicyWorkerConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.worker_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl {
    graceful_decommission_timeout: PrimField<String>,
    scale_down_factor: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_down_min_worker_fraction: Option<PrimField<f64>>,
    scale_up_factor: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_up_min_worker_fraction: Option<PrimField<f64>>,
}
impl DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl {
    #[doc = "Set the field `scale_down_min_worker_fraction`.\nMinimum scale-down threshold as a fraction of total cluster size before scaling occurs.\nFor example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must\nrecommend at least a 2 worker scale-down for the cluster to scale. A threshold of 0\nmeans the autoscaler will scale down on any recommended change.\n\nBounds: [0.0, 1.0]. Default: 0.0."]
    pub fn set_scale_down_min_worker_fraction(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_down_min_worker_fraction = Some(v.into());
        self
    }
    #[doc = "Set the field `scale_up_min_worker_fraction`.\nMinimum scale-up threshold as a fraction of total cluster size before scaling\noccurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler\nmust recommend at least a 2-worker scale-up for the cluster to scale. A threshold of\n0 means the autoscaler will scale up on any recommended change.\n\nBounds: [0.0, 1.0]. Default: 0.0."]
    pub fn set_scale_up_min_worker_fraction(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scale_up_min_worker_fraction = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl {
    type O = BlockAssignable<DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl {
    #[doc = "Timeout for YARN graceful decommissioning of Node Managers. Specifies the\nduration to wait for jobs to complete before forcefully removing workers\n(and potentially interrupting jobs). Only applicable to downscaling operations.\n\nBounds: [0s, 1d]."]
    pub graceful_decommission_timeout: PrimField<String>,
    #[doc = "Fraction of average pending memory in the last cooldown period for which to\nremove workers. A scale-down factor of 1 will result in scaling down so that there\nis no available memory remaining after the update (more aggressive scaling).\nA scale-down factor of 0 disables removing workers, which can be beneficial for\nautoscaling a single job.\n\nBounds: [0.0, 1.0]."]
    pub scale_down_factor: PrimField<f64>,
    #[doc = "Fraction of average pending memory in the last cooldown period for which to\nadd workers. A scale-up factor of 1.0 will result in scaling up so that there\nis no pending memory remaining after the update (more aggressive scaling).\nA scale-up factor closer to 0 will result in a smaller magnitude of scaling up\n(less aggressive scaling).\n\nBounds: [0.0, 1.0]."]
    pub scale_up_factor: PrimField<f64>,
}
impl BuildDataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl {
    pub fn build(self) -> DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl {
        DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl {
            graceful_decommission_timeout: self.graceful_decommission_timeout,
            scale_down_factor: self.scale_down_factor,
            scale_down_min_worker_fraction: core::default::Default::default(),
            scale_up_factor: self.scale_up_factor,
            scale_up_min_worker_fraction: core::default::Default::default(),
        }
    }
}
pub struct DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigElRef {
        DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `graceful_decommission_timeout` after provisioning.\nTimeout for YARN graceful decommissioning of Node Managers. Specifies the\nduration to wait for jobs to complete before forcefully removing workers\n(and potentially interrupting jobs). Only applicable to downscaling operations.\n\nBounds: [0s, 1d]."]
    pub fn graceful_decommission_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.graceful_decommission_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scale_down_factor` after provisioning.\nFraction of average pending memory in the last cooldown period for which to\nremove workers. A scale-down factor of 1 will result in scaling down so that there\nis no available memory remaining after the update (more aggressive scaling).\nA scale-down factor of 0 disables removing workers, which can be beneficial for\nautoscaling a single job.\n\nBounds: [0.0, 1.0]."]
    pub fn scale_down_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_down_factor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scale_down_min_worker_fraction` after provisioning.\nMinimum scale-down threshold as a fraction of total cluster size before scaling occurs.\nFor example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler must\nrecommend at least a 2 worker scale-down for the cluster to scale. A threshold of 0\nmeans the autoscaler will scale down on any recommended change.\n\nBounds: [0.0, 1.0]. Default: 0.0."]
    pub fn scale_down_min_worker_fraction(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_down_min_worker_fraction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scale_up_factor` after provisioning.\nFraction of average pending memory in the last cooldown period for which to\nadd workers. A scale-up factor of 1.0 will result in scaling up so that there\nis no pending memory remaining after the update (more aggressive scaling).\nA scale-up factor closer to 0 will result in a smaller magnitude of scaling up\n(less aggressive scaling).\n\nBounds: [0.0, 1.0]."]
    pub fn scale_up_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_up_factor", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `scale_up_min_worker_fraction` after provisioning.\nMinimum scale-up threshold as a fraction of total cluster size before scaling\noccurs. For example, in a 20-worker cluster, a threshold of 0.1 means the autoscaler\nmust recommend at least a 2-worker scale-up for the cluster to scale. A threshold of\n0 means the autoscaler will scale up on any recommended change.\n\nBounds: [0.0, 1.0]. Default: 0.0."]
    pub fn scale_up_min_worker_fraction(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scale_up_min_worker_fraction", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataprocAutoscalingPolicyBasicAlgorithmElDynamic {
    yarn_config: Option<DynamicBlock<DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl>>,
}
#[derive(Serialize)]
pub struct DataprocAutoscalingPolicyBasicAlgorithmEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cooldown_period: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    yarn_config: Option<Vec<DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl>>,
    dynamic: DataprocAutoscalingPolicyBasicAlgorithmElDynamic,
}
impl DataprocAutoscalingPolicyBasicAlgorithmEl {
    #[doc = "Set the field `cooldown_period`.\nDuration between scaling events. A scaling period starts after the\nupdate operation from the previous event has completed.\n\nBounds: [2m, 1d]. Default: 2m."]
    pub fn set_cooldown_period(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cooldown_period = Some(v.into());
        self
    }
    #[doc = "Set the field `yarn_config`.\n"]
    pub fn set_yarn_config(
        mut self,
        v: impl Into<BlockAssignable<DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.yarn_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.yarn_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataprocAutoscalingPolicyBasicAlgorithmEl {
    type O = BlockAssignable<DataprocAutoscalingPolicyBasicAlgorithmEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocAutoscalingPolicyBasicAlgorithmEl {}
impl BuildDataprocAutoscalingPolicyBasicAlgorithmEl {
    pub fn build(self) -> DataprocAutoscalingPolicyBasicAlgorithmEl {
        DataprocAutoscalingPolicyBasicAlgorithmEl {
            cooldown_period: core::default::Default::default(),
            yarn_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataprocAutoscalingPolicyBasicAlgorithmElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocAutoscalingPolicyBasicAlgorithmElRef {
    fn new(shared: StackShared, base: String) -> DataprocAutoscalingPolicyBasicAlgorithmElRef {
        DataprocAutoscalingPolicyBasicAlgorithmElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocAutoscalingPolicyBasicAlgorithmElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cooldown_period` after provisioning.\nDuration between scaling events. A scaling period starts after the\nupdate operation from the previous event has completed.\n\nBounds: [2m, 1d]. Default: 2m."]
    pub fn cooldown_period(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cooldown_period", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `yarn_config` after provisioning.\n"]
    pub fn yarn_config(&self) -> ListRef<DataprocAutoscalingPolicyBasicAlgorithmElYarnConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.yarn_config", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocAutoscalingPolicySecondaryWorkerConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl DataprocAutoscalingPolicySecondaryWorkerConfigEl {
    #[doc = "Set the field `max_instances`.\nMaximum number of instances for this group. Note that by default, clusters will not use\nsecondary workers. Required for secondary workers if the minimum secondary instances is set.\nBounds: [minInstances, ). Defaults to 0."]
    pub fn set_max_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `min_instances`.\nMinimum number of instances for this group. Bounds: [0, maxInstances]. Defaults to 0."]
    pub fn set_min_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\nWeight for the instance group, which is used to determine the fraction of total workers\nin the cluster from this instance group. For example, if primary workers have weight 2,\nand secondary workers have weight 1, the cluster will have approximately 2 primary workers\nfor each secondary worker.\n\nThe cluster may not reach the specified balance if constrained by min/max bounds or other\nautoscaling settings. For example, if maxInstances for secondary workers is 0, then only\nprimary workers will be added. The cluster can also be out of balance when created.\n\nIf weight is not set on any instance group, the cluster will default to equal weight for\nall groups: the cluster will attempt to maintain an equal number of workers in each group\nwithin the configured size bounds for each group. If weight is set for one group only,\nthe cluster will default to zero weight on the unset group. For example if weight is set\nonly on primary workers, the cluster will use primary workers only and no secondary workers."]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocAutoscalingPolicySecondaryWorkerConfigEl {
    type O = BlockAssignable<DataprocAutoscalingPolicySecondaryWorkerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocAutoscalingPolicySecondaryWorkerConfigEl {}
impl BuildDataprocAutoscalingPolicySecondaryWorkerConfigEl {
    pub fn build(self) -> DataprocAutoscalingPolicySecondaryWorkerConfigEl {
        DataprocAutoscalingPolicySecondaryWorkerConfigEl {
            max_instances: core::default::Default::default(),
            min_instances: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct DataprocAutoscalingPolicySecondaryWorkerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocAutoscalingPolicySecondaryWorkerConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataprocAutoscalingPolicySecondaryWorkerConfigElRef {
        DataprocAutoscalingPolicySecondaryWorkerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocAutoscalingPolicySecondaryWorkerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nMaximum number of instances for this group. Note that by default, clusters will not use\nsecondary workers. Required for secondary workers if the minimum secondary instances is set.\nBounds: [minInstances, ). Defaults to 0."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\nMinimum number of instances for this group. Bounds: [0, maxInstances]. Defaults to 0."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nWeight for the instance group, which is used to determine the fraction of total workers\nin the cluster from this instance group. For example, if primary workers have weight 2,\nand secondary workers have weight 1, the cluster will have approximately 2 primary workers\nfor each secondary worker.\n\nThe cluster may not reach the specified balance if constrained by min/max bounds or other\nautoscaling settings. For example, if maxInstances for secondary workers is 0, then only\nprimary workers will be added. The cluster can also be out of balance when created.\n\nIf weight is not set on any instance group, the cluster will default to equal weight for\nall groups: the cluster will attempt to maintain an equal number of workers in each group\nwithin the configured size bounds for each group. If weight is set for one group only,\nthe cluster will default to zero weight on the unset group. For example if weight is set\nonly on primary workers, the cluster will use primary workers only and no secondary workers."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocAutoscalingPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataprocAutoscalingPolicyTimeoutsEl {
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
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocAutoscalingPolicyTimeoutsEl {
    type O = BlockAssignable<DataprocAutoscalingPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocAutoscalingPolicyTimeoutsEl {}
impl BuildDataprocAutoscalingPolicyTimeoutsEl {
    pub fn build(self) -> DataprocAutoscalingPolicyTimeoutsEl {
        DataprocAutoscalingPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataprocAutoscalingPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocAutoscalingPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataprocAutoscalingPolicyTimeoutsElRef {
        DataprocAutoscalingPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocAutoscalingPolicyTimeoutsElRef {
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
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize)]
pub struct DataprocAutoscalingPolicyWorkerConfigEl {
    max_instances: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_instances: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<PrimField<f64>>,
}
impl DataprocAutoscalingPolicyWorkerConfigEl {
    #[doc = "Set the field `min_instances`.\nMinimum number of instances for this group. Bounds: [2, maxInstances]. Defaults to 2."]
    pub fn set_min_instances(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_instances = Some(v.into());
        self
    }
    #[doc = "Set the field `weight`.\nWeight for the instance group, which is used to determine the fraction of total workers\nin the cluster from this instance group. For example, if primary workers have weight 2,\nand secondary workers have weight 1, the cluster will have approximately 2 primary workers\nfor each secondary worker.\n\nThe cluster may not reach the specified balance if constrained by min/max bounds or other\nautoscaling settings. For example, if maxInstances for secondary workers is 0, then only\nprimary workers will be added. The cluster can also be out of balance when created.\n\nIf weight is not set on any instance group, the cluster will default to equal weight for\nall groups: the cluster will attempt to maintain an equal number of workers in each group\nwithin the configured size bounds for each group. If weight is set for one group only,\nthe cluster will default to zero weight on the unset group. For example if weight is set\nonly on primary workers, the cluster will use primary workers only and no secondary workers."]
    pub fn set_weight(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weight = Some(v.into());
        self
    }
}
impl ToListMappable for DataprocAutoscalingPolicyWorkerConfigEl {
    type O = BlockAssignable<DataprocAutoscalingPolicyWorkerConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataprocAutoscalingPolicyWorkerConfigEl {
    #[doc = "Maximum number of instances for this group."]
    pub max_instances: PrimField<f64>,
}
impl BuildDataprocAutoscalingPolicyWorkerConfigEl {
    pub fn build(self) -> DataprocAutoscalingPolicyWorkerConfigEl {
        DataprocAutoscalingPolicyWorkerConfigEl {
            max_instances: self.max_instances,
            min_instances: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}
pub struct DataprocAutoscalingPolicyWorkerConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataprocAutoscalingPolicyWorkerConfigElRef {
    fn new(shared: StackShared, base: String) -> DataprocAutoscalingPolicyWorkerConfigElRef {
        DataprocAutoscalingPolicyWorkerConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataprocAutoscalingPolicyWorkerConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nMaximum number of instances for this group."]
    pub fn max_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\nMinimum number of instances for this group. Bounds: [2, maxInstances]. Defaults to 2."]
    pub fn min_instances(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_instances", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `weight` after provisioning.\nWeight for the instance group, which is used to determine the fraction of total workers\nin the cluster from this instance group. For example, if primary workers have weight 2,\nand secondary workers have weight 1, the cluster will have approximately 2 primary workers\nfor each secondary worker.\n\nThe cluster may not reach the specified balance if constrained by min/max bounds or other\nautoscaling settings. For example, if maxInstances for secondary workers is 0, then only\nprimary workers will be added. The cluster can also be out of balance when created.\n\nIf weight is not set on any instance group, the cluster will default to equal weight for\nall groups: the cluster will attempt to maintain an equal number of workers in each group\nwithin the configured size bounds for each group. If weight is set for one group only,\nthe cluster will default to zero weight on the unset group. For example if weight is set\nonly on primary workers, the cluster will use primary workers only and no secondary workers."]
    pub fn weight(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataprocAutoscalingPolicyDynamic {
    basic_algorithm: Option<DynamicBlock<DataprocAutoscalingPolicyBasicAlgorithmEl>>,
    secondary_worker_config: Option<DynamicBlock<DataprocAutoscalingPolicySecondaryWorkerConfigEl>>,
    worker_config: Option<DynamicBlock<DataprocAutoscalingPolicyWorkerConfigEl>>,
}
