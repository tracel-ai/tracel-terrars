use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_info: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    applications: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configurations: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configurations_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_ami_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_root_volume_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_job_flow_alive_when_no_steps: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    list_steps_states: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_encryption_kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_uri: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_release_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_group_config: Option<ListField<EmrClusterPlacementGroupConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    release_label: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale_down_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_configuration: Option<PrimField<String>>,
    service_role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<ListField<EmrClusterStepEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_concurrency_level: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    termination_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_node_replacement: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible_to_all_users: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_termination_policy: Option<Vec<EmrClusterAutoTerminationPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bootstrap_action: Option<Vec<EmrClusterBootstrapActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_instance_fleet: Option<Vec<EmrClusterCoreInstanceFleetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_instance_group: Option<Vec<EmrClusterCoreInstanceGroupEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_attributes: Option<Vec<EmrClusterEc2AttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kerberos_attributes: Option<Vec<EmrClusterKerberosAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_instance_fleet: Option<Vec<EmrClusterMasterInstanceFleetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    master_instance_group: Option<Vec<EmrClusterMasterInstanceGroupEl>>,
    dynamic: EmrClusterDynamic,
}

struct EmrCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmrClusterData>,
}

#[derive(Clone)]
pub struct EmrCluster(Rc<EmrCluster_>);

impl EmrCluster {
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

    #[doc = "Set the field `additional_info`.\n"]
    pub fn set_additional_info(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().additional_info = Some(v.into());
        self
    }

    #[doc = "Set the field `applications`.\n"]
    pub fn set_applications(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().applications = Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_role`.\n"]
    pub fn set_autoscaling_role(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().autoscaling_role = Some(v.into());
        self
    }

    #[doc = "Set the field `configurations`.\n"]
    pub fn set_configurations(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().configurations = Some(v.into());
        self
    }

    #[doc = "Set the field `configurations_json`.\n"]
    pub fn set_configurations_json(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().configurations_json = Some(v.into());
        self
    }

    #[doc = "Set the field `custom_ami_id`.\n"]
    pub fn set_custom_ami_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_ami_id = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_root_volume_size`.\n"]
    pub fn set_ebs_root_volume_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ebs_root_volume_size = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `keep_job_flow_alive_when_no_steps`.\n"]
    pub fn set_keep_job_flow_alive_when_no_steps(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().keep_job_flow_alive_when_no_steps = Some(v.into());
        self
    }

    #[doc = "Set the field `list_steps_states`.\n"]
    pub fn set_list_steps_states(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().list_steps_states = Some(v.into());
        self
    }

    #[doc = "Set the field `log_encryption_kms_key_id`.\n"]
    pub fn set_log_encryption_kms_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().log_encryption_kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `log_uri`.\n"]
    pub fn set_log_uri(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().log_uri = Some(v.into());
        self
    }

    #[doc = "Set the field `os_release_label`.\n"]
    pub fn set_os_release_label(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().os_release_label = Some(v.into());
        self
    }

    #[doc = "Set the field `placement_group_config`.\n"]
    pub fn set_placement_group_config(self, v: impl Into<ListField<EmrClusterPlacementGroupConfigEl>>) -> Self {
        self.0.data.borrow_mut().placement_group_config = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `scale_down_behavior`.\n"]
    pub fn set_scale_down_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().scale_down_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `security_configuration`.\n"]
    pub fn set_security_configuration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_configuration = Some(v.into());
        self
    }

    #[doc = "Set the field `step`.\n"]
    pub fn set_step(self, v: impl Into<ListField<EmrClusterStepEl>>) -> Self {
        self.0.data.borrow_mut().step = Some(v.into());
        self
    }

    #[doc = "Set the field `step_concurrency_level`.\n"]
    pub fn set_step_concurrency_level(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().step_concurrency_level = Some(v.into());
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

    #[doc = "Set the field `termination_protection`.\n"]
    pub fn set_termination_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().termination_protection = Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_node_replacement`.\n"]
    pub fn set_unhealthy_node_replacement(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().unhealthy_node_replacement = Some(v.into());
        self
    }

    #[doc = "Set the field `visible_to_all_users`.\n"]
    pub fn set_visible_to_all_users(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().visible_to_all_users = Some(v.into());
        self
    }

    #[doc = "Set the field `auto_termination_policy`.\n"]
    pub fn set_auto_termination_policy(self, v: impl Into<BlockAssignable<EmrClusterAutoTerminationPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_termination_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_termination_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bootstrap_action`.\n"]
    pub fn set_bootstrap_action(self, v: impl Into<BlockAssignable<EmrClusterBootstrapActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bootstrap_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bootstrap_action = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `core_instance_fleet`.\n"]
    pub fn set_core_instance_fleet(self, v: impl Into<BlockAssignable<EmrClusterCoreInstanceFleetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().core_instance_fleet = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.core_instance_fleet = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `core_instance_group`.\n"]
    pub fn set_core_instance_group(self, v: impl Into<BlockAssignable<EmrClusterCoreInstanceGroupEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().core_instance_group = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.core_instance_group = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_attributes`.\n"]
    pub fn set_ec2_attributes(self, v: impl Into<BlockAssignable<EmrClusterEc2AttributesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_attributes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_attributes = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kerberos_attributes`.\n"]
    pub fn set_kerberos_attributes(self, v: impl Into<BlockAssignable<EmrClusterKerberosAttributesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kerberos_attributes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kerberos_attributes = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_instance_fleet`.\n"]
    pub fn set_master_instance_fleet(self, v: impl Into<BlockAssignable<EmrClusterMasterInstanceFleetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().master_instance_fleet = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.master_instance_fleet = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_instance_group`.\n"]
    pub fn set_master_instance_group(self, v: impl Into<BlockAssignable<EmrClusterMasterInstanceGroupEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().master_instance_group = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.master_instance_group = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `additional_info` after provisioning.\n"]
    pub fn additional_info(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `applications` after provisioning.\n"]
    pub fn applications(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.applications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_role` after provisioning.\n"]
    pub fn autoscaling_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configurations` after provisioning.\n"]
    pub fn configurations(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configurations_json` after provisioning.\n"]
    pub fn configurations_json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configurations_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_ami_id` after provisioning.\n"]
    pub fn custom_ami_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_ami_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_root_volume_size` after provisioning.\n"]
    pub fn ebs_root_volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_root_volume_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keep_job_flow_alive_when_no_steps` after provisioning.\n"]
    pub fn keep_job_flow_alive_when_no_steps(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_job_flow_alive_when_no_steps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `list_steps_states` after provisioning.\n"]
    pub fn list_steps_states(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.list_steps_states", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_encryption_kms_key_id` after provisioning.\n"]
    pub fn log_encryption_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_encryption_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_uri` after provisioning.\n"]
    pub fn log_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_public_dns` after provisioning.\n"]
    pub fn master_public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_release_label` after provisioning.\n"]
    pub fn os_release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group_config` after provisioning.\n"]
    pub fn placement_group_config(&self) -> ListRef<EmrClusterPlacementGroupConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.placement_group_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scale_down_behavior` after provisioning.\n"]
    pub fn scale_down_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_down_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> ListRef<EmrClusterStepElRef> {
        ListRef::new(self.shared().clone(), format!("{}.step", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step_concurrency_level` after provisioning.\n"]
    pub fn step_concurrency_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.step_concurrency_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_protection` after provisioning.\n"]
    pub fn termination_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.termination_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `unhealthy_node_replacement` after provisioning.\n"]
    pub fn unhealthy_node_replacement(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_node_replacement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visible_to_all_users` after provisioning.\n"]
    pub fn visible_to_all_users(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.visible_to_all_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_termination_policy` after provisioning.\n"]
    pub fn auto_termination_policy(&self) -> ListRef<EmrClusterAutoTerminationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_termination_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_action` after provisioning.\n"]
    pub fn bootstrap_action(&self) -> ListRef<EmrClusterBootstrapActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bootstrap_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_instance_fleet` after provisioning.\n"]
    pub fn core_instance_fleet(&self) -> ListRef<EmrClusterCoreInstanceFleetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.core_instance_fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_instance_group` after provisioning.\n"]
    pub fn core_instance_group(&self) -> ListRef<EmrClusterCoreInstanceGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.core_instance_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_attributes` after provisioning.\n"]
    pub fn ec2_attributes(&self) -> ListRef<EmrClusterEc2AttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ec2_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_attributes` after provisioning.\n"]
    pub fn kerberos_attributes(&self) -> ListRef<EmrClusterKerberosAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kerberos_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_instance_fleet` after provisioning.\n"]
    pub fn master_instance_fleet(&self) -> ListRef<EmrClusterMasterInstanceFleetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_instance_fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_instance_group` after provisioning.\n"]
    pub fn master_instance_group(&self) -> ListRef<EmrClusterMasterInstanceGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_instance_group", self.extract_ref()))
    }
}

impl Referable for EmrCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EmrCluster { }

impl ToListMappable for EmrCluster {
    type O = ListRef<EmrClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmrCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_emr_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrCluster {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub release_label: PrimField<String>,
    #[doc = ""]
    pub service_role: PrimField<String>,
}

impl BuildEmrCluster {
    pub fn build(self, stack: &mut Stack) -> EmrCluster {
        let out = EmrCluster(Rc::new(EmrCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmrClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                additional_info: core::default::Default::default(),
                applications: core::default::Default::default(),
                autoscaling_role: core::default::Default::default(),
                configurations: core::default::Default::default(),
                configurations_json: core::default::Default::default(),
                custom_ami_id: core::default::Default::default(),
                ebs_root_volume_size: core::default::Default::default(),
                id: core::default::Default::default(),
                keep_job_flow_alive_when_no_steps: core::default::Default::default(),
                list_steps_states: core::default::Default::default(),
                log_encryption_kms_key_id: core::default::Default::default(),
                log_uri: core::default::Default::default(),
                name: self.name,
                os_release_label: core::default::Default::default(),
                placement_group_config: core::default::Default::default(),
                region: core::default::Default::default(),
                release_label: self.release_label,
                scale_down_behavior: core::default::Default::default(),
                security_configuration: core::default::Default::default(),
                service_role: self.service_role,
                step: core::default::Default::default(),
                step_concurrency_level: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                termination_protection: core::default::Default::default(),
                unhealthy_node_replacement: core::default::Default::default(),
                visible_to_all_users: core::default::Default::default(),
                auto_termination_policy: core::default::Default::default(),
                bootstrap_action: core::default::Default::default(),
                core_instance_fleet: core::default::Default::default(),
                core_instance_group: core::default::Default::default(),
                ec2_attributes: core::default::Default::default(),
                kerberos_attributes: core::default::Default::default(),
                master_instance_fleet: core::default::Default::default(),
                master_instance_group: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EmrClusterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_info` after provisioning.\n"]
    pub fn additional_info(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `applications` after provisioning.\n"]
    pub fn applications(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.applications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_role` after provisioning.\n"]
    pub fn autoscaling_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configurations` after provisioning.\n"]
    pub fn configurations(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configurations_json` after provisioning.\n"]
    pub fn configurations_json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configurations_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_ami_id` after provisioning.\n"]
    pub fn custom_ami_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_ami_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_root_volume_size` after provisioning.\n"]
    pub fn ebs_root_volume_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_root_volume_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keep_job_flow_alive_when_no_steps` after provisioning.\n"]
    pub fn keep_job_flow_alive_when_no_steps(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.keep_job_flow_alive_when_no_steps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `list_steps_states` after provisioning.\n"]
    pub fn list_steps_states(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.list_steps_states", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_encryption_kms_key_id` after provisioning.\n"]
    pub fn log_encryption_kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_encryption_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_uri` after provisioning.\n"]
    pub fn log_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_public_dns` after provisioning.\n"]
    pub fn master_public_dns(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.master_public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_release_label` after provisioning.\n"]
    pub fn os_release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group_config` after provisioning.\n"]
    pub fn placement_group_config(&self) -> ListRef<EmrClusterPlacementGroupConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.placement_group_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scale_down_behavior` after provisioning.\n"]
    pub fn scale_down_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scale_down_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> ListRef<EmrClusterStepElRef> {
        ListRef::new(self.shared().clone(), format!("{}.step", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step_concurrency_level` after provisioning.\n"]
    pub fn step_concurrency_level(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.step_concurrency_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_protection` after provisioning.\n"]
    pub fn termination_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.termination_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `unhealthy_node_replacement` after provisioning.\n"]
    pub fn unhealthy_node_replacement(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_node_replacement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visible_to_all_users` after provisioning.\n"]
    pub fn visible_to_all_users(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.visible_to_all_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_termination_policy` after provisioning.\n"]
    pub fn auto_termination_policy(&self) -> ListRef<EmrClusterAutoTerminationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_termination_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_action` after provisioning.\n"]
    pub fn bootstrap_action(&self) -> ListRef<EmrClusterBootstrapActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bootstrap_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_instance_fleet` after provisioning.\n"]
    pub fn core_instance_fleet(&self) -> ListRef<EmrClusterCoreInstanceFleetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.core_instance_fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_instance_group` after provisioning.\n"]
    pub fn core_instance_group(&self) -> ListRef<EmrClusterCoreInstanceGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.core_instance_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_attributes` after provisioning.\n"]
    pub fn ec2_attributes(&self) -> ListRef<EmrClusterEc2AttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ec2_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_attributes` after provisioning.\n"]
    pub fn kerberos_attributes(&self) -> ListRef<EmrClusterKerberosAttributesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kerberos_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_instance_fleet` after provisioning.\n"]
    pub fn master_instance_fleet(&self) -> ListRef<EmrClusterMasterInstanceFleetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_instance_fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_instance_group` after provisioning.\n"]
    pub fn master_instance_group(&self) -> ListRef<EmrClusterMasterInstanceGroupElRef> {
        ListRef::new(self.shared().clone(), format!("{}.master_instance_group", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmrClusterPlacementGroupConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_role: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placement_strategy: Option<PrimField<String>>,
}

impl EmrClusterPlacementGroupConfigEl {
    #[doc = "Set the field `instance_role`.\n"]
    pub fn set_instance_role(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_role = Some(v.into());
        self
    }

    #[doc = "Set the field `placement_strategy`.\n"]
    pub fn set_placement_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.placement_strategy = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterPlacementGroupConfigEl {
    type O = BlockAssignable<EmrClusterPlacementGroupConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterPlacementGroupConfigEl {}

impl BuildEmrClusterPlacementGroupConfigEl {
    pub fn build(self) -> EmrClusterPlacementGroupConfigEl {
        EmrClusterPlacementGroupConfigEl {
            instance_role: core::default::Default::default(),
            placement_strategy: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterPlacementGroupConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterPlacementGroupConfigElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterPlacementGroupConfigElRef {
        EmrClusterPlacementGroupConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterPlacementGroupConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_role` after provisioning.\n"]
    pub fn instance_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_role", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_strategy` after provisioning.\n"]
    pub fn placement_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.placement_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterStepElHadoopJarStepEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jar: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
}

impl EmrClusterStepElHadoopJarStepEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }

    #[doc = "Set the field `jar`.\n"]
    pub fn set_jar(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.jar = Some(v.into());
        self
    }

    #[doc = "Set the field `main_class`.\n"]
    pub fn set_main_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_class = Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterStepElHadoopJarStepEl {
    type O = BlockAssignable<EmrClusterStepElHadoopJarStepEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterStepElHadoopJarStepEl {}

impl BuildEmrClusterStepElHadoopJarStepEl {
    pub fn build(self) -> EmrClusterStepElHadoopJarStepEl {
        EmrClusterStepElHadoopJarStepEl {
            args: core::default::Default::default(),
            jar: core::default::Default::default(),
            main_class: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterStepElHadoopJarStepElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterStepElHadoopJarStepElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterStepElHadoopJarStepElRef {
        EmrClusterStepElHadoopJarStepElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterStepElHadoopJarStepElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `jar` after provisioning.\n"]
    pub fn jar(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jar", self.base))
    }

    #[doc = "Get a reference to the value of field `main_class` after provisioning.\n"]
    pub fn main_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterStepEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_on_failure: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hadoop_jar_step: Option<ListField<EmrClusterStepElHadoopJarStepEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl EmrClusterStepEl {
    #[doc = "Set the field `action_on_failure`.\n"]
    pub fn set_action_on_failure(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action_on_failure = Some(v.into());
        self
    }

    #[doc = "Set the field `hadoop_jar_step`.\n"]
    pub fn set_hadoop_jar_step(mut self, v: impl Into<ListField<EmrClusterStepElHadoopJarStepEl>>) -> Self {
        self.hadoop_jar_step = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterStepEl {
    type O = BlockAssignable<EmrClusterStepEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterStepEl {}

impl BuildEmrClusterStepEl {
    pub fn build(self) -> EmrClusterStepEl {
        EmrClusterStepEl {
            action_on_failure: core::default::Default::default(),
            hadoop_jar_step: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterStepElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterStepElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterStepElRef {
        EmrClusterStepElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterStepElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_on_failure` after provisioning.\n"]
    pub fn action_on_failure(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_on_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `hadoop_jar_step` after provisioning.\n"]
    pub fn hadoop_jar_step(&self) -> ListRef<EmrClusterStepElHadoopJarStepElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hadoop_jar_step", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterAutoTerminationPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout: Option<PrimField<f64>>,
}

impl EmrClusterAutoTerminationPolicyEl {
    #[doc = "Set the field `idle_timeout`.\n"]
    pub fn set_idle_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterAutoTerminationPolicyEl {
    type O = BlockAssignable<EmrClusterAutoTerminationPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterAutoTerminationPolicyEl {}

impl BuildEmrClusterAutoTerminationPolicyEl {
    pub fn build(self) -> EmrClusterAutoTerminationPolicyEl {
        EmrClusterAutoTerminationPolicyEl { idle_timeout: core::default::Default::default() }
    }
}

pub struct EmrClusterAutoTerminationPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterAutoTerminationPolicyElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterAutoTerminationPolicyElRef {
        EmrClusterAutoTerminationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterAutoTerminationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterBootstrapActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    args: Option<ListField<PrimField<String>>>,
    name: PrimField<String>,
    path: PrimField<String>,
}

impl EmrClusterBootstrapActionEl {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.args = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterBootstrapActionEl {
    type O = BlockAssignable<EmrClusterBootstrapActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterBootstrapActionEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub path: PrimField<String>,
}

impl BuildEmrClusterBootstrapActionEl {
    pub fn build(self) -> EmrClusterBootstrapActionEl {
        EmrClusterBootstrapActionEl {
            args: core::default::Default::default(),
            name: self.name,
            path: self.path,
        }
    }
}

pub struct EmrClusterBootstrapActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterBootstrapActionElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterBootstrapActionElRef {
        EmrClusterBootstrapActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterBootstrapActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
}

impl EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
    #[doc = "Set the field `classification`.\n"]
    pub fn set_classification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.classification = Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
    type O = BlockAssignable<EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl {}

impl BuildEmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
    pub fn build(self) -> EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
        EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
            classification: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
        EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    size: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes_per_instance: Option<PrimField<f64>>,
}

impl EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `volumes_per_instance`.\n"]
    pub fn set_volumes_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volumes_per_instance = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    type O = BlockAssignable<EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    #[doc = ""]
    pub size: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildEmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    pub fn build(self) -> EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
        EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
            iops: core::default::Default::default(),
            size: self.size,
            type_: self.type_,
            volumes_per_instance: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
        EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes_per_instance` after provisioning.\n"]
    pub fn volumes_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volumes_per_instance", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterCoreInstanceFleetElInstanceTypeConfigsElDynamic {
    configurations: Option<DynamicBlock<EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl>>,
    ebs_config: Option<DynamicBlock<EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl>>,
}

#[derive(Serialize)]
pub struct EmrClusterCoreInstanceFleetElInstanceTypeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bid_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bid_price_as_percentage_of_on_demand_price: Option<PrimField<f64>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configurations: Option<Vec<EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_config: Option<Vec<EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl>>,
    dynamic: EmrClusterCoreInstanceFleetElInstanceTypeConfigsElDynamic,
}

impl EmrClusterCoreInstanceFleetElInstanceTypeConfigsEl {
    #[doc = "Set the field `bid_price`.\n"]
    pub fn set_bid_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bid_price = Some(v.into());
        self
    }

    #[doc = "Set the field `bid_price_as_percentage_of_on_demand_price`.\n"]
    pub fn set_bid_price_as_percentage_of_on_demand_price(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bid_price_as_percentage_of_on_demand_price = Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weighted_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `configurations`.\n"]
    pub fn set_configurations(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterCoreInstanceFleetElInstanceTypeConfigsElConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.configurations = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.configurations = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ebs_config`.\n"]
    pub fn set_ebs_config(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterCoreInstanceFleetElInstanceTypeConfigsElEbsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrClusterCoreInstanceFleetElInstanceTypeConfigsEl {
    type O = BlockAssignable<EmrClusterCoreInstanceFleetElInstanceTypeConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterCoreInstanceFleetElInstanceTypeConfigsEl {
    #[doc = ""]
    pub instance_type: PrimField<String>,
}

impl BuildEmrClusterCoreInstanceFleetElInstanceTypeConfigsEl {
    pub fn build(self) -> EmrClusterCoreInstanceFleetElInstanceTypeConfigsEl {
        EmrClusterCoreInstanceFleetElInstanceTypeConfigsEl {
            bid_price: core::default::Default::default(),
            bid_price_as_percentage_of_on_demand_price: core::default::Default::default(),
            instance_type: self.instance_type,
            weighted_capacity: core::default::Default::default(),
            configurations: core::default::Default::default(),
            ebs_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrClusterCoreInstanceFleetElInstanceTypeConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterCoreInstanceFleetElInstanceTypeConfigsElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterCoreInstanceFleetElInstanceTypeConfigsElRef {
        EmrClusterCoreInstanceFleetElInstanceTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterCoreInstanceFleetElInstanceTypeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bid_price` after provisioning.\n"]
    pub fn bid_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bid_price", self.base))
    }

    #[doc = "Get a reference to the value of field `bid_price_as_percentage_of_on_demand_price` after provisioning.\n"]
    pub fn bid_price_as_percentage_of_on_demand_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bid_price_as_percentage_of_on_demand_price", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
    allocation_strategy: PrimField<String>,
}

impl EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl { }

impl ToListMappable for EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
    type O = BlockAssignable<EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
    #[doc = ""]
    pub allocation_strategy: PrimField<String>,
}

impl BuildEmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
    pub fn build(self) -> EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
        EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
            allocation_strategy: self.allocation_strategy,
        }
    }
}

pub struct EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
        EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    allocation_strategy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_duration_minutes: Option<PrimField<f64>>,
    timeout_action: PrimField<String>,
    timeout_duration_minutes: PrimField<f64>,
}

impl EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    #[doc = "Set the field `block_duration_minutes`.\n"]
    pub fn set_block_duration_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.block_duration_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    type O = BlockAssignable<EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    #[doc = ""]
    pub allocation_strategy: PrimField<String>,
    #[doc = ""]
    pub timeout_action: PrimField<String>,
    #[doc = ""]
    pub timeout_duration_minutes: PrimField<f64>,
}

impl BuildEmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    pub fn build(self) -> EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
        EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
            allocation_strategy: self.allocation_strategy,
            block_duration_minutes: core::default::Default::default(),
            timeout_action: self.timeout_action,
            timeout_duration_minutes: self.timeout_duration_minutes,
        }
    }
}

pub struct EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
        EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `block_duration_minutes` after provisioning.\n"]
    pub fn block_duration_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_duration_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_action` after provisioning.\n"]
    pub fn timeout_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_action", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_duration_minutes` after provisioning.\n"]
    pub fn timeout_duration_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_duration_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterCoreInstanceFleetElLaunchSpecificationsElDynamic {
    on_demand_specification: Option<
        DynamicBlock<EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl>,
    >,
    spot_specification: Option<
        DynamicBlock<EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl>,
    >,
}

#[derive(Serialize)]
pub struct EmrClusterCoreInstanceFleetElLaunchSpecificationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_specification: Option<Vec<EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_specification: Option<Vec<EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl>>,
    dynamic: EmrClusterCoreInstanceFleetElLaunchSpecificationsElDynamic,
}

impl EmrClusterCoreInstanceFleetElLaunchSpecificationsEl {
    #[doc = "Set the field `on_demand_specification`.\n"]
    pub fn set_on_demand_specification(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_demand_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_demand_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spot_specification`.\n"]
    pub fn set_spot_specification(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spot_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spot_specification = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrClusterCoreInstanceFleetElLaunchSpecificationsEl {
    type O = BlockAssignable<EmrClusterCoreInstanceFleetElLaunchSpecificationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterCoreInstanceFleetElLaunchSpecificationsEl {}

impl BuildEmrClusterCoreInstanceFleetElLaunchSpecificationsEl {
    pub fn build(self) -> EmrClusterCoreInstanceFleetElLaunchSpecificationsEl {
        EmrClusterCoreInstanceFleetElLaunchSpecificationsEl {
            on_demand_specification: core::default::Default::default(),
            spot_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrClusterCoreInstanceFleetElLaunchSpecificationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterCoreInstanceFleetElLaunchSpecificationsElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterCoreInstanceFleetElLaunchSpecificationsElRef {
        EmrClusterCoreInstanceFleetElLaunchSpecificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterCoreInstanceFleetElLaunchSpecificationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_demand_specification` after provisioning.\n"]
    pub fn on_demand_specification(
        &self,
    ) -> ListRef<EmrClusterCoreInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_demand_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_specification` after provisioning.\n"]
    pub fn spot_specification(
        &self,
    ) -> ListRef<EmrClusterCoreInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spot_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterCoreInstanceFleetElDynamic {
    instance_type_configs: Option<DynamicBlock<EmrClusterCoreInstanceFleetElInstanceTypeConfigsEl>>,
    launch_specifications: Option<DynamicBlock<EmrClusterCoreInstanceFleetElLaunchSpecificationsEl>>,
}

#[derive(Serialize)]
pub struct EmrClusterCoreInstanceFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_on_demand_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_spot_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type_configs: Option<Vec<EmrClusterCoreInstanceFleetElInstanceTypeConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_specifications: Option<Vec<EmrClusterCoreInstanceFleetElLaunchSpecificationsEl>>,
    dynamic: EmrClusterCoreInstanceFleetElDynamic,
}

impl EmrClusterCoreInstanceFleetEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `target_on_demand_capacity`.\n"]
    pub fn set_target_on_demand_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_on_demand_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `target_spot_capacity`.\n"]
    pub fn set_target_spot_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_spot_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type_configs`.\n"]
    pub fn set_instance_type_configs(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterCoreInstanceFleetElInstanceTypeConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_type_configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_type_configs = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_specifications`.\n"]
    pub fn set_launch_specifications(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterCoreInstanceFleetElLaunchSpecificationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_specifications = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_specifications = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrClusterCoreInstanceFleetEl {
    type O = BlockAssignable<EmrClusterCoreInstanceFleetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterCoreInstanceFleetEl {}

impl BuildEmrClusterCoreInstanceFleetEl {
    pub fn build(self) -> EmrClusterCoreInstanceFleetEl {
        EmrClusterCoreInstanceFleetEl {
            name: core::default::Default::default(),
            target_on_demand_capacity: core::default::Default::default(),
            target_spot_capacity: core::default::Default::default(),
            instance_type_configs: core::default::Default::default(),
            launch_specifications: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrClusterCoreInstanceFleetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterCoreInstanceFleetElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterCoreInstanceFleetElRef {
        EmrClusterCoreInstanceFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterCoreInstanceFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_on_demand_capacity` after provisioning.\n"]
    pub fn provisioned_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_on_demand_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_spot_capacity` after provisioning.\n"]
    pub fn provisioned_spot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_spot_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `target_on_demand_capacity` after provisioning.\n"]
    pub fn target_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_on_demand_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `target_spot_capacity` after provisioning.\n"]
    pub fn target_spot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_spot_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_specifications` after provisioning.\n"]
    pub fn launch_specifications(&self) -> ListRef<EmrClusterCoreInstanceFleetElLaunchSpecificationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_specifications", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterCoreInstanceGroupElEbsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    size: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes_per_instance: Option<PrimField<f64>>,
}

impl EmrClusterCoreInstanceGroupElEbsConfigEl {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `volumes_per_instance`.\n"]
    pub fn set_volumes_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volumes_per_instance = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterCoreInstanceGroupElEbsConfigEl {
    type O = BlockAssignable<EmrClusterCoreInstanceGroupElEbsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterCoreInstanceGroupElEbsConfigEl {
    #[doc = ""]
    pub size: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildEmrClusterCoreInstanceGroupElEbsConfigEl {
    pub fn build(self) -> EmrClusterCoreInstanceGroupElEbsConfigEl {
        EmrClusterCoreInstanceGroupElEbsConfigEl {
            iops: core::default::Default::default(),
            size: self.size,
            throughput: core::default::Default::default(),
            type_: self.type_,
            volumes_per_instance: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterCoreInstanceGroupElEbsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterCoreInstanceGroupElEbsConfigElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterCoreInstanceGroupElEbsConfigElRef {
        EmrClusterCoreInstanceGroupElEbsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterCoreInstanceGroupElEbsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes_per_instance` after provisioning.\n"]
    pub fn volumes_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volumes_per_instance", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterCoreInstanceGroupElDynamic {
    ebs_config: Option<DynamicBlock<EmrClusterCoreInstanceGroupElEbsConfigEl>>,
}

#[derive(Serialize)]
pub struct EmrClusterCoreInstanceGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bid_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_config: Option<Vec<EmrClusterCoreInstanceGroupElEbsConfigEl>>,
    dynamic: EmrClusterCoreInstanceGroupElDynamic,
}

impl EmrClusterCoreInstanceGroupEl {
    #[doc = "Set the field `autoscaling_policy`.\n"]
    pub fn set_autoscaling_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.autoscaling_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `bid_price`.\n"]
    pub fn set_bid_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bid_price = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_config`.\n"]
    pub fn set_ebs_config(mut self, v: impl Into<BlockAssignable<EmrClusterCoreInstanceGroupElEbsConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrClusterCoreInstanceGroupEl {
    type O = BlockAssignable<EmrClusterCoreInstanceGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterCoreInstanceGroupEl {
    #[doc = ""]
    pub instance_type: PrimField<String>,
}

impl BuildEmrClusterCoreInstanceGroupEl {
    pub fn build(self) -> EmrClusterCoreInstanceGroupEl {
        EmrClusterCoreInstanceGroupEl {
            autoscaling_policy: core::default::Default::default(),
            bid_price: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: self.instance_type,
            name: core::default::Default::default(),
            ebs_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrClusterCoreInstanceGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterCoreInstanceGroupElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterCoreInstanceGroupElRef {
        EmrClusterCoreInstanceGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterCoreInstanceGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `bid_price` after provisioning.\n"]
    pub fn bid_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bid_price", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterEc2AttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_master_security_groups: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additional_slave_security_groups: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emr_managed_master_security_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emr_managed_slave_security_group: Option<PrimField<String>>,
    instance_profile: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_access_security_group: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
}

impl EmrClusterEc2AttributesEl {
    #[doc = "Set the field `additional_master_security_groups`.\n"]
    pub fn set_additional_master_security_groups(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_master_security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `additional_slave_security_groups`.\n"]
    pub fn set_additional_slave_security_groups(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.additional_slave_security_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `emr_managed_master_security_group`.\n"]
    pub fn set_emr_managed_master_security_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.emr_managed_master_security_group = Some(v.into());
        self
    }

    #[doc = "Set the field `emr_managed_slave_security_group`.\n"]
    pub fn set_emr_managed_slave_security_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.emr_managed_slave_security_group = Some(v.into());
        self
    }

    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_name = Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_security_group`.\n"]
    pub fn set_service_access_security_group(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_access_security_group = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterEc2AttributesEl {
    type O = BlockAssignable<EmrClusterEc2AttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterEc2AttributesEl {
    #[doc = ""]
    pub instance_profile: PrimField<String>,
}

impl BuildEmrClusterEc2AttributesEl {
    pub fn build(self) -> EmrClusterEc2AttributesEl {
        EmrClusterEc2AttributesEl {
            additional_master_security_groups: core::default::Default::default(),
            additional_slave_security_groups: core::default::Default::default(),
            emr_managed_master_security_group: core::default::Default::default(),
            emr_managed_slave_security_group: core::default::Default::default(),
            instance_profile: self.instance_profile,
            key_name: core::default::Default::default(),
            service_access_security_group: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterEc2AttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterEc2AttributesElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterEc2AttributesElRef {
        EmrClusterEc2AttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterEc2AttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_master_security_groups` after provisioning.\n"]
    pub fn additional_master_security_groups(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_master_security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_slave_security_groups` after provisioning.\n"]
    pub fn additional_slave_security_groups(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.additional_slave_security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_managed_master_security_group` after provisioning.\n"]
    pub fn emr_managed_master_security_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.emr_managed_master_security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_managed_slave_security_group` after provisioning.\n"]
    pub fn emr_managed_slave_security_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.emr_managed_slave_security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_profile` after provisioning.\n"]
    pub fn instance_profile(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_access_security_group` after provisioning.\n"]
    pub fn service_access_security_group(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_access_security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterKerberosAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ad_domain_join_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ad_domain_join_user: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cross_realm_trust_principal_password: Option<PrimField<String>>,
    kdc_admin_password: PrimField<String>,
    realm: PrimField<String>,
}

impl EmrClusterKerberosAttributesEl {
    #[doc = "Set the field `ad_domain_join_password`.\n"]
    pub fn set_ad_domain_join_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ad_domain_join_password = Some(v.into());
        self
    }

    #[doc = "Set the field `ad_domain_join_user`.\n"]
    pub fn set_ad_domain_join_user(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ad_domain_join_user = Some(v.into());
        self
    }

    #[doc = "Set the field `cross_realm_trust_principal_password`.\n"]
    pub fn set_cross_realm_trust_principal_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cross_realm_trust_principal_password = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterKerberosAttributesEl {
    type O = BlockAssignable<EmrClusterKerberosAttributesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterKerberosAttributesEl {
    #[doc = ""]
    pub kdc_admin_password: PrimField<String>,
    #[doc = ""]
    pub realm: PrimField<String>,
}

impl BuildEmrClusterKerberosAttributesEl {
    pub fn build(self) -> EmrClusterKerberosAttributesEl {
        EmrClusterKerberosAttributesEl {
            ad_domain_join_password: core::default::Default::default(),
            ad_domain_join_user: core::default::Default::default(),
            cross_realm_trust_principal_password: core::default::Default::default(),
            kdc_admin_password: self.kdc_admin_password,
            realm: self.realm,
        }
    }
}

pub struct EmrClusterKerberosAttributesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterKerberosAttributesElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterKerberosAttributesElRef {
        EmrClusterKerberosAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterKerberosAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ad_domain_join_password` after provisioning.\n"]
    pub fn ad_domain_join_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ad_domain_join_password", self.base))
    }

    #[doc = "Get a reference to the value of field `ad_domain_join_user` after provisioning.\n"]
    pub fn ad_domain_join_user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ad_domain_join_user", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_realm_trust_principal_password` after provisioning.\n"]
    pub fn cross_realm_trust_principal_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_principal_password", self.base))
    }

    #[doc = "Get a reference to the value of field `kdc_admin_password` after provisioning.\n"]
    pub fn kdc_admin_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kdc_admin_password", self.base))
    }

    #[doc = "Get a reference to the value of field `realm` after provisioning.\n"]
    pub fn realm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.realm", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    classification: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
}

impl EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
    #[doc = "Set the field `classification`.\n"]
    pub fn set_classification(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.classification = Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
    type O = BlockAssignable<EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl {}

impl BuildEmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
    pub fn build(self) -> EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
        EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl {
            classification: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
        EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    size: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes_per_instance: Option<PrimField<f64>>,
}

impl EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `volumes_per_instance`.\n"]
    pub fn set_volumes_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volumes_per_instance = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    type O = BlockAssignable<EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    #[doc = ""]
    pub size: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildEmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
    pub fn build(self) -> EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
        EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl {
            iops: core::default::Default::default(),
            size: self.size,
            type_: self.type_,
            volumes_per_instance: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
        EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes_per_instance` after provisioning.\n"]
    pub fn volumes_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volumes_per_instance", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterMasterInstanceFleetElInstanceTypeConfigsElDynamic {
    configurations: Option<DynamicBlock<EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl>>,
    ebs_config: Option<DynamicBlock<EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl>>,
}

#[derive(Serialize)]
pub struct EmrClusterMasterInstanceFleetElInstanceTypeConfigsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bid_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bid_price_as_percentage_of_on_demand_price: Option<PrimField<f64>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weighted_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configurations: Option<Vec<EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_config: Option<Vec<EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl>>,
    dynamic: EmrClusterMasterInstanceFleetElInstanceTypeConfigsElDynamic,
}

impl EmrClusterMasterInstanceFleetElInstanceTypeConfigsEl {
    #[doc = "Set the field `bid_price`.\n"]
    pub fn set_bid_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bid_price = Some(v.into());
        self
    }

    #[doc = "Set the field `bid_price_as_percentage_of_on_demand_price`.\n"]
    pub fn set_bid_price_as_percentage_of_on_demand_price(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bid_price_as_percentage_of_on_demand_price = Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.weighted_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `configurations`.\n"]
    pub fn set_configurations(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterMasterInstanceFleetElInstanceTypeConfigsElConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.configurations = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.configurations = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ebs_config`.\n"]
    pub fn set_ebs_config(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterMasterInstanceFleetElInstanceTypeConfigsElEbsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrClusterMasterInstanceFleetElInstanceTypeConfigsEl {
    type O = BlockAssignable<EmrClusterMasterInstanceFleetElInstanceTypeConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterMasterInstanceFleetElInstanceTypeConfigsEl {
    #[doc = ""]
    pub instance_type: PrimField<String>,
}

impl BuildEmrClusterMasterInstanceFleetElInstanceTypeConfigsEl {
    pub fn build(self) -> EmrClusterMasterInstanceFleetElInstanceTypeConfigsEl {
        EmrClusterMasterInstanceFleetElInstanceTypeConfigsEl {
            bid_price: core::default::Default::default(),
            bid_price_as_percentage_of_on_demand_price: core::default::Default::default(),
            instance_type: self.instance_type,
            weighted_capacity: core::default::Default::default(),
            configurations: core::default::Default::default(),
            ebs_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrClusterMasterInstanceFleetElInstanceTypeConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterMasterInstanceFleetElInstanceTypeConfigsElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterMasterInstanceFleetElInstanceTypeConfigsElRef {
        EmrClusterMasterInstanceFleetElInstanceTypeConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterMasterInstanceFleetElInstanceTypeConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bid_price` after provisioning.\n"]
    pub fn bid_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bid_price", self.base))
    }

    #[doc = "Get a reference to the value of field `bid_price_as_percentage_of_on_demand_price` after provisioning.\n"]
    pub fn bid_price_as_percentage_of_on_demand_price(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bid_price_as_percentage_of_on_demand_price", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
    allocation_strategy: PrimField<String>,
}

impl EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl { }

impl ToListMappable for EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
    type O = BlockAssignable<EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
    #[doc = ""]
    pub allocation_strategy: PrimField<String>,
}

impl BuildEmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
    pub fn build(self) -> EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
        EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl {
            allocation_strategy: self.allocation_strategy,
        }
    }
}

pub struct EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
        EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    allocation_strategy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_duration_minutes: Option<PrimField<f64>>,
    timeout_action: PrimField<String>,
    timeout_duration_minutes: PrimField<f64>,
}

impl EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    #[doc = "Set the field `block_duration_minutes`.\n"]
    pub fn set_block_duration_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.block_duration_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    type O = BlockAssignable<EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    #[doc = ""]
    pub allocation_strategy: PrimField<String>,
    #[doc = ""]
    pub timeout_action: PrimField<String>,
    #[doc = ""]
    pub timeout_duration_minutes: PrimField<f64>,
}

impl BuildEmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
    pub fn build(self) -> EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
        EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl {
            allocation_strategy: self.allocation_strategy,
            block_duration_minutes: core::default::Default::default(),
            timeout_action: self.timeout_action,
            timeout_duration_minutes: self.timeout_duration_minutes,
        }
    }
}

pub struct EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
        EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `block_duration_minutes` after provisioning.\n"]
    pub fn block_duration_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.block_duration_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_action` after provisioning.\n"]
    pub fn timeout_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_action", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_duration_minutes` after provisioning.\n"]
    pub fn timeout_duration_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout_duration_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterMasterInstanceFleetElLaunchSpecificationsElDynamic {
    on_demand_specification: Option<
        DynamicBlock<EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl>,
    >,
    spot_specification: Option<
        DynamicBlock<EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl>,
    >,
}

#[derive(Serialize)]
pub struct EmrClusterMasterInstanceFleetElLaunchSpecificationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    on_demand_specification: Option<Vec<EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spot_specification: Option<Vec<EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl>>,
    dynamic: EmrClusterMasterInstanceFleetElLaunchSpecificationsElDynamic,
}

impl EmrClusterMasterInstanceFleetElLaunchSpecificationsEl {
    #[doc = "Set the field `on_demand_specification`.\n"]
    pub fn set_on_demand_specification(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.on_demand_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.on_demand_specification = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spot_specification`.\n"]
    pub fn set_spot_specification(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.spot_specification = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.spot_specification = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrClusterMasterInstanceFleetElLaunchSpecificationsEl {
    type O = BlockAssignable<EmrClusterMasterInstanceFleetElLaunchSpecificationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterMasterInstanceFleetElLaunchSpecificationsEl {}

impl BuildEmrClusterMasterInstanceFleetElLaunchSpecificationsEl {
    pub fn build(self) -> EmrClusterMasterInstanceFleetElLaunchSpecificationsEl {
        EmrClusterMasterInstanceFleetElLaunchSpecificationsEl {
            on_demand_specification: core::default::Default::default(),
            spot_specification: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrClusterMasterInstanceFleetElLaunchSpecificationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterMasterInstanceFleetElLaunchSpecificationsElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterMasterInstanceFleetElLaunchSpecificationsElRef {
        EmrClusterMasterInstanceFleetElLaunchSpecificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterMasterInstanceFleetElLaunchSpecificationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_demand_specification` after provisioning.\n"]
    pub fn on_demand_specification(
        &self,
    ) -> ListRef<EmrClusterMasterInstanceFleetElLaunchSpecificationsElOnDemandSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.on_demand_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_specification` after provisioning.\n"]
    pub fn spot_specification(
        &self,
    ) -> ListRef<EmrClusterMasterInstanceFleetElLaunchSpecificationsElSpotSpecificationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spot_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterMasterInstanceFleetElDynamic {
    instance_type_configs: Option<DynamicBlock<EmrClusterMasterInstanceFleetElInstanceTypeConfigsEl>>,
    launch_specifications: Option<DynamicBlock<EmrClusterMasterInstanceFleetElLaunchSpecificationsEl>>,
}

#[derive(Serialize)]
pub struct EmrClusterMasterInstanceFleetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_on_demand_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_spot_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_type_configs: Option<Vec<EmrClusterMasterInstanceFleetElInstanceTypeConfigsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_specifications: Option<Vec<EmrClusterMasterInstanceFleetElLaunchSpecificationsEl>>,
    dynamic: EmrClusterMasterInstanceFleetElDynamic,
}

impl EmrClusterMasterInstanceFleetEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `target_on_demand_capacity`.\n"]
    pub fn set_target_on_demand_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_on_demand_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `target_spot_capacity`.\n"]
    pub fn set_target_spot_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_spot_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type_configs`.\n"]
    pub fn set_instance_type_configs(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterMasterInstanceFleetElInstanceTypeConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.instance_type_configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_type_configs = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_specifications`.\n"]
    pub fn set_launch_specifications(
        mut self,
        v: impl Into<BlockAssignable<EmrClusterMasterInstanceFleetElLaunchSpecificationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.launch_specifications = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_specifications = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrClusterMasterInstanceFleetEl {
    type O = BlockAssignable<EmrClusterMasterInstanceFleetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterMasterInstanceFleetEl {}

impl BuildEmrClusterMasterInstanceFleetEl {
    pub fn build(self) -> EmrClusterMasterInstanceFleetEl {
        EmrClusterMasterInstanceFleetEl {
            name: core::default::Default::default(),
            target_on_demand_capacity: core::default::Default::default(),
            target_spot_capacity: core::default::Default::default(),
            instance_type_configs: core::default::Default::default(),
            launch_specifications: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrClusterMasterInstanceFleetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterMasterInstanceFleetElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterMasterInstanceFleetElRef {
        EmrClusterMasterInstanceFleetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterMasterInstanceFleetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_on_demand_capacity` after provisioning.\n"]
    pub fn provisioned_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_on_demand_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_spot_capacity` after provisioning.\n"]
    pub fn provisioned_spot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_spot_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `target_on_demand_capacity` after provisioning.\n"]
    pub fn target_on_demand_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_on_demand_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `target_spot_capacity` after provisioning.\n"]
    pub fn target_spot_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_spot_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_specifications` after provisioning.\n"]
    pub fn launch_specifications(&self) -> ListRef<EmrClusterMasterInstanceFleetElLaunchSpecificationsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_specifications", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrClusterMasterInstanceGroupElEbsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    size: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<PrimField<f64>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes_per_instance: Option<PrimField<f64>>,
}

impl EmrClusterMasterInstanceGroupElEbsConfigEl {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.throughput = Some(v.into());
        self
    }

    #[doc = "Set the field `volumes_per_instance`.\n"]
    pub fn set_volumes_per_instance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volumes_per_instance = Some(v.into());
        self
    }
}

impl ToListMappable for EmrClusterMasterInstanceGroupElEbsConfigEl {
    type O = BlockAssignable<EmrClusterMasterInstanceGroupElEbsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterMasterInstanceGroupElEbsConfigEl {
    #[doc = ""]
    pub size: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildEmrClusterMasterInstanceGroupElEbsConfigEl {
    pub fn build(self) -> EmrClusterMasterInstanceGroupElEbsConfigEl {
        EmrClusterMasterInstanceGroupElEbsConfigEl {
            iops: core::default::Default::default(),
            size: self.size,
            throughput: core::default::Default::default(),
            type_: self.type_,
            volumes_per_instance: core::default::Default::default(),
        }
    }
}

pub struct EmrClusterMasterInstanceGroupElEbsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterMasterInstanceGroupElEbsConfigElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterMasterInstanceGroupElEbsConfigElRef {
        EmrClusterMasterInstanceGroupElEbsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterMasterInstanceGroupElEbsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes_per_instance` after provisioning.\n"]
    pub fn volumes_per_instance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volumes_per_instance", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterMasterInstanceGroupElDynamic {
    ebs_config: Option<DynamicBlock<EmrClusterMasterInstanceGroupElEbsConfigEl>>,
}

#[derive(Serialize)]
pub struct EmrClusterMasterInstanceGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bid_price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_config: Option<Vec<EmrClusterMasterInstanceGroupElEbsConfigEl>>,
    dynamic: EmrClusterMasterInstanceGroupElDynamic,
}

impl EmrClusterMasterInstanceGroupEl {
    #[doc = "Set the field `bid_price`.\n"]
    pub fn set_bid_price(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bid_price = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_config`.\n"]
    pub fn set_ebs_config(mut self, v: impl Into<BlockAssignable<EmrClusterMasterInstanceGroupElEbsConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrClusterMasterInstanceGroupEl {
    type O = BlockAssignable<EmrClusterMasterInstanceGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrClusterMasterInstanceGroupEl {
    #[doc = ""]
    pub instance_type: PrimField<String>,
}

impl BuildEmrClusterMasterInstanceGroupEl {
    pub fn build(self) -> EmrClusterMasterInstanceGroupEl {
        EmrClusterMasterInstanceGroupEl {
            bid_price: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: self.instance_type,
            name: core::default::Default::default(),
            ebs_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrClusterMasterInstanceGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrClusterMasterInstanceGroupElRef {
    fn new(shared: StackShared, base: String) -> EmrClusterMasterInstanceGroupElRef {
        EmrClusterMasterInstanceGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrClusterMasterInstanceGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bid_price` after provisioning.\n"]
    pub fn bid_price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bid_price", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterDynamic {
    auto_termination_policy: Option<DynamicBlock<EmrClusterAutoTerminationPolicyEl>>,
    bootstrap_action: Option<DynamicBlock<EmrClusterBootstrapActionEl>>,
    core_instance_fleet: Option<DynamicBlock<EmrClusterCoreInstanceFleetEl>>,
    core_instance_group: Option<DynamicBlock<EmrClusterCoreInstanceGroupEl>>,
    ec2_attributes: Option<DynamicBlock<EmrClusterEc2AttributesEl>>,
    kerberos_attributes: Option<DynamicBlock<EmrClusterKerberosAttributesEl>>,
    master_instance_fleet: Option<DynamicBlock<EmrClusterMasterInstanceFleetEl>>,
    master_instance_group: Option<DynamicBlock<EmrClusterMasterInstanceGroupEl>>,
}
