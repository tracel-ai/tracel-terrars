use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EksNodeGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ami_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_type: Option<PrimField<String>>,
    cluster_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_update_version: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_group_name_prefix: Option<PrimField<String>>,
    node_role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    release_version: Option<PrimField<String>>,
    subnet_ids: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    launch_template: Option<Vec<EksNodeGroupLaunchTemplateEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_repair_config: Option<Vec<EksNodeGroupNodeRepairConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_access: Option<Vec<EksNodeGroupRemoteAccessEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scaling_config: Option<Vec<EksNodeGroupScalingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taint: Option<Vec<EksNodeGroupTaintEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<EksNodeGroupTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_config: Option<Vec<EksNodeGroupUpdateConfigEl>>,
    dynamic: EksNodeGroupDynamic,
}

struct EksNodeGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EksNodeGroupData>,
}

#[derive(Clone)]
pub struct EksNodeGroup(Rc<EksNodeGroup_>);

impl EksNodeGroup {
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

    #[doc = "Set the field `ami_type`.\n"]
    pub fn set_ami_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().ami_type = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_type`.\n"]
    pub fn set_capacity_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().capacity_type = Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size`.\n"]
    pub fn set_disk_size(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().disk_size = Some(v.into());
        self
    }

    #[doc = "Set the field `force_update_version`.\n"]
    pub fn set_force_update_version(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_update_version = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_types`.\n"]
    pub fn set_instance_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().instance_types = Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc = "Set the field `node_group_name`.\n"]
    pub fn set_node_group_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().node_group_name = Some(v.into());
        self
    }

    #[doc = "Set the field `node_group_name_prefix`.\n"]
    pub fn set_node_group_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().node_group_name_prefix = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `release_version`.\n"]
    pub fn set_release_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().release_version = Some(v.into());
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

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().version = Some(v.into());
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(self, v: impl Into<BlockAssignable<EksNodeGroupLaunchTemplateEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_repair_config`.\n"]
    pub fn set_node_repair_config(self, v: impl Into<BlockAssignable<EksNodeGroupNodeRepairConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_repair_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_repair_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_access`.\n"]
    pub fn set_remote_access(self, v: impl Into<BlockAssignable<EksNodeGroupRemoteAccessEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_access = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_access = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_config`.\n"]
    pub fn set_scaling_config(self, v: impl Into<BlockAssignable<EksNodeGroupScalingConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `taint`.\n"]
    pub fn set_taint(self, v: impl Into<BlockAssignable<EksNodeGroupTaintEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().taint = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.taint = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<EksNodeGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `update_config`.\n"]
    pub fn set_update_config(self, v: impl Into<BlockAssignable<EksNodeGroupUpdateConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().update_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.update_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `ami_type` after provisioning.\n"]
    pub fn ami_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ami_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_type` after provisioning.\n"]
    pub fn capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_update_version` after provisioning.\n"]
    pub fn force_update_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_update_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instance_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name` after provisioning.\n"]
    pub fn node_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name_prefix` after provisioning.\n"]
    pub fn node_group_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_role_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<EksNodeGroupResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<EksNodeGroupLaunchTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_repair_config` after provisioning.\n"]
    pub fn node_repair_config(&self) -> ListRef<EksNodeGroupNodeRepairConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_repair_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> ListRef<EksNodeGroupRemoteAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> ListRef<EksNodeGroupScalingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksNodeGroupTimeoutsElRef {
        EksNodeGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> ListRef<EksNodeGroupUpdateConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }
}

impl Referable for EksNodeGroup {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for EksNodeGroup { }

impl ToListMappable for EksNodeGroup {
    type O = ListRef<EksNodeGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EksNodeGroup_ {
    fn extract_resource_type(&self) -> String {
        "aws_eks_node_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEksNodeGroup {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_name: PrimField<String>,
    #[doc = ""]
    pub node_role_arn: PrimField<String>,
    #[doc = ""]
    pub subnet_ids: SetField<PrimField<String>>,
}

impl BuildEksNodeGroup {
    pub fn build(self, stack: &mut Stack) -> EksNodeGroup {
        let out = EksNodeGroup(Rc::new(EksNodeGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EksNodeGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                ami_type: core::default::Default::default(),
                capacity_type: core::default::Default::default(),
                cluster_name: self.cluster_name,
                disk_size: core::default::Default::default(),
                force_update_version: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_types: core::default::Default::default(),
                labels: core::default::Default::default(),
                node_group_name: core::default::Default::default(),
                node_group_name_prefix: core::default::Default::default(),
                node_role_arn: self.node_role_arn,
                region: core::default::Default::default(),
                release_version: core::default::Default::default(),
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                version: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                node_repair_config: core::default::Default::default(),
                remote_access: core::default::Default::default(),
                scaling_config: core::default::Default::default(),
                taint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                update_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EksNodeGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl EksNodeGroupRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami_type` after provisioning.\n"]
    pub fn ami_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ami_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_type` after provisioning.\n"]
    pub fn capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.capacity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_update_version` after provisioning.\n"]
    pub fn force_update_version(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.force_update_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.instance_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name` after provisioning.\n"]
    pub fn node_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_group_name_prefix` after provisioning.\n"]
    pub fn node_group_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_group_name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_role_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<EksNodeGroupResourcesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<EksNodeGroupLaunchTemplateElRef> {
        ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_repair_config` after provisioning.\n"]
    pub fn node_repair_config(&self) -> ListRef<EksNodeGroupNodeRepairConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_repair_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> ListRef<EksNodeGroupRemoteAccessElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remote_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> ListRef<EksNodeGroupScalingConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EksNodeGroupTimeoutsElRef {
        EksNodeGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_config` after provisioning.\n"]
    pub fn update_config(&self) -> ListRef<EksNodeGroupUpdateConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.update_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupResourcesElAutoscalingGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl EksNodeGroupResourcesElAutoscalingGroupsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for EksNodeGroupResourcesElAutoscalingGroupsEl {
    type O = BlockAssignable<EksNodeGroupResourcesElAutoscalingGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupResourcesElAutoscalingGroupsEl {}

impl BuildEksNodeGroupResourcesElAutoscalingGroupsEl {
    pub fn build(self) -> EksNodeGroupResourcesElAutoscalingGroupsEl {
        EksNodeGroupResourcesElAutoscalingGroupsEl { name: core::default::Default::default() }
    }
}

pub struct EksNodeGroupResourcesElAutoscalingGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupResourcesElAutoscalingGroupsElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupResourcesElAutoscalingGroupsElRef {
        EksNodeGroupResourcesElAutoscalingGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupResourcesElAutoscalingGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_groups: Option<ListField<EksNodeGroupResourcesElAutoscalingGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_access_security_group_id: Option<PrimField<String>>,
}

impl EksNodeGroupResourcesEl {
    #[doc = "Set the field `autoscaling_groups`.\n"]
    pub fn set_autoscaling_groups(
        mut self,
        v: impl Into<ListField<EksNodeGroupResourcesElAutoscalingGroupsEl>>,
    ) -> Self {
        self.autoscaling_groups = Some(v.into());
        self
    }

    #[doc = "Set the field `remote_access_security_group_id`.\n"]
    pub fn set_remote_access_security_group_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.remote_access_security_group_id = Some(v.into());
        self
    }
}

impl ToListMappable for EksNodeGroupResourcesEl {
    type O = BlockAssignable<EksNodeGroupResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupResourcesEl {}

impl BuildEksNodeGroupResourcesEl {
    pub fn build(self) -> EksNodeGroupResourcesEl {
        EksNodeGroupResourcesEl {
            autoscaling_groups: core::default::Default::default(),
            remote_access_security_group_id: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupResourcesElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupResourcesElRef {
        EksNodeGroupResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> ListRef<EksNodeGroupResourcesElAutoscalingGroupsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.autoscaling_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_access_security_group_id` after provisioning.\n"]
    pub fn remote_access_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.remote_access_security_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    version: PrimField<String>,
}

impl EksNodeGroupLaunchTemplateEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for EksNodeGroupLaunchTemplateEl {
    type O = BlockAssignable<EksNodeGroupLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupLaunchTemplateEl {
    #[doc = ""]
    pub version: PrimField<String>,
}

impl BuildEksNodeGroupLaunchTemplateEl {
    pub fn build(self) -> EksNodeGroupLaunchTemplateEl {
        EksNodeGroupLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: self.version,
        }
    }
}

pub struct EksNodeGroupLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupLaunchTemplateElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupLaunchTemplateElRef {
        EksNodeGroupLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupLaunchTemplateElRef {
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl {
    min_repair_wait_time_mins: PrimField<f64>,
    node_monitoring_condition: PrimField<String>,
    node_unhealthy_reason: PrimField<String>,
    repair_action: PrimField<String>,
}

impl EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl { }

impl ToListMappable for EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl {
    type O = BlockAssignable<EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl {
    #[doc = ""]
    pub min_repair_wait_time_mins: PrimField<f64>,
    #[doc = ""]
    pub node_monitoring_condition: PrimField<String>,
    #[doc = ""]
    pub node_unhealthy_reason: PrimField<String>,
    #[doc = ""]
    pub repair_action: PrimField<String>,
}

impl BuildEksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl {
    pub fn build(self) -> EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl {
        EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl {
            min_repair_wait_time_mins: self.min_repair_wait_time_mins,
            node_monitoring_condition: self.node_monitoring_condition,
            node_unhealthy_reason: self.node_unhealthy_reason,
            repair_action: self.repair_action,
        }
    }
}

pub struct EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesElRef {
        EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `min_repair_wait_time_mins` after provisioning.\n"]
    pub fn min_repair_wait_time_mins(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_repair_wait_time_mins", self.base))
    }

    #[doc = "Get a reference to the value of field `node_monitoring_condition` after provisioning.\n"]
    pub fn node_monitoring_condition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_monitoring_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `node_unhealthy_reason` after provisioning.\n"]
    pub fn node_unhealthy_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_unhealthy_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `repair_action` after provisioning.\n"]
    pub fn repair_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repair_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksNodeGroupNodeRepairConfigElDynamic {
    node_repair_config_overrides: Option<DynamicBlock<EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl>>,
}

#[derive(Serialize)]
pub struct EksNodeGroupNodeRepairConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_parallel_nodes_repaired_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_parallel_nodes_repaired_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unhealthy_node_threshold_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unhealthy_node_threshold_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_repair_config_overrides: Option<Vec<EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl>>,
    dynamic: EksNodeGroupNodeRepairConfigElDynamic,
}

impl EksNodeGroupNodeRepairConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_nodes_repaired_count`.\n"]
    pub fn set_max_parallel_nodes_repaired_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_parallel_nodes_repaired_count = Some(v.into());
        self
    }

    #[doc = "Set the field `max_parallel_nodes_repaired_percentage`.\n"]
    pub fn set_max_parallel_nodes_repaired_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_parallel_nodes_repaired_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `max_unhealthy_node_threshold_count`.\n"]
    pub fn set_max_unhealthy_node_threshold_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unhealthy_node_threshold_count = Some(v.into());
        self
    }

    #[doc = "Set the field `max_unhealthy_node_threshold_percentage`.\n"]
    pub fn set_max_unhealthy_node_threshold_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unhealthy_node_threshold_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `node_repair_config_overrides`.\n"]
    pub fn set_node_repair_config_overrides(
        mut self,
        v: impl Into<BlockAssignable<EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_repair_config_overrides = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_repair_config_overrides = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EksNodeGroupNodeRepairConfigEl {
    type O = BlockAssignable<EksNodeGroupNodeRepairConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupNodeRepairConfigEl {}

impl BuildEksNodeGroupNodeRepairConfigEl {
    pub fn build(self) -> EksNodeGroupNodeRepairConfigEl {
        EksNodeGroupNodeRepairConfigEl {
            enabled: core::default::Default::default(),
            max_parallel_nodes_repaired_count: core::default::Default::default(),
            max_parallel_nodes_repaired_percentage: core::default::Default::default(),
            max_unhealthy_node_threshold_count: core::default::Default::default(),
            max_unhealthy_node_threshold_percentage: core::default::Default::default(),
            node_repair_config_overrides: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EksNodeGroupNodeRepairConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupNodeRepairConfigElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupNodeRepairConfigElRef {
        EksNodeGroupNodeRepairConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupNodeRepairConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `max_parallel_nodes_repaired_count` after provisioning.\n"]
    pub fn max_parallel_nodes_repaired_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_nodes_repaired_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_parallel_nodes_repaired_percentage` after provisioning.\n"]
    pub fn max_parallel_nodes_repaired_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_parallel_nodes_repaired_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `max_unhealthy_node_threshold_count` after provisioning.\n"]
    pub fn max_unhealthy_node_threshold_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_unhealthy_node_threshold_count", self.base))
    }

    #[doc = "Get a reference to the value of field `max_unhealthy_node_threshold_percentage` after provisioning.\n"]
    pub fn max_unhealthy_node_threshold_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_unhealthy_node_threshold_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `node_repair_config_overrides` after provisioning.\n"]
    pub fn node_repair_config_overrides(&self) -> ListRef<EksNodeGroupNodeRepairConfigElNodeRepairConfigOverridesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_repair_config_overrides", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupRemoteAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_ssh_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_security_group_ids: Option<SetField<PrimField<String>>>,
}

impl EksNodeGroupRemoteAccessEl {
    #[doc = "Set the field `ec2_ssh_key`.\n"]
    pub fn set_ec2_ssh_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ec2_ssh_key = Some(v.into());
        self
    }

    #[doc = "Set the field `source_security_group_ids`.\n"]
    pub fn set_source_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.source_security_group_ids = Some(v.into());
        self
    }
}

impl ToListMappable for EksNodeGroupRemoteAccessEl {
    type O = BlockAssignable<EksNodeGroupRemoteAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupRemoteAccessEl {}

impl BuildEksNodeGroupRemoteAccessEl {
    pub fn build(self) -> EksNodeGroupRemoteAccessEl {
        EksNodeGroupRemoteAccessEl {
            ec2_ssh_key: core::default::Default::default(),
            source_security_group_ids: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupRemoteAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupRemoteAccessElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupRemoteAccessElRef {
        EksNodeGroupRemoteAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupRemoteAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ec2_ssh_key` after provisioning.\n"]
    pub fn ec2_ssh_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ec2_ssh_key", self.base))
    }

    #[doc = "Get a reference to the value of field `source_security_group_ids` after provisioning.\n"]
    pub fn source_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.source_security_group_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupScalingConfigEl {
    desired_size: PrimField<f64>,
    max_size: PrimField<f64>,
    min_size: PrimField<f64>,
}

impl EksNodeGroupScalingConfigEl { }

impl ToListMappable for EksNodeGroupScalingConfigEl {
    type O = BlockAssignable<EksNodeGroupScalingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupScalingConfigEl {
    #[doc = ""]
    pub desired_size: PrimField<f64>,
    #[doc = ""]
    pub max_size: PrimField<f64>,
    #[doc = ""]
    pub min_size: PrimField<f64>,
}

impl BuildEksNodeGroupScalingConfigEl {
    pub fn build(self) -> EksNodeGroupScalingConfigEl {
        EksNodeGroupScalingConfigEl {
            desired_size: self.desired_size,
            max_size: self.max_size,
            min_size: self.min_size,
        }
    }
}

pub struct EksNodeGroupScalingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupScalingConfigElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupScalingConfigElRef {
        EksNodeGroupScalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupScalingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_size` after provisioning.\n"]
    pub fn desired_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_size", self.base))
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.base))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupTaintEl {
    effect: PrimField<String>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl EksNodeGroupTaintEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for EksNodeGroupTaintEl {
    type O = BlockAssignable<EksNodeGroupTaintEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupTaintEl {
    #[doc = ""]
    pub effect: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
}

impl BuildEksNodeGroupTaintEl {
    pub fn build(self) -> EksNodeGroupTaintEl {
        EksNodeGroupTaintEl {
            effect: self.effect,
            key: self.key,
            value: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupTaintElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupTaintElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupTaintElRef {
        EksNodeGroupTaintElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupTaintElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct EksNodeGroupTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl EksNodeGroupTimeoutsEl {
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

impl ToListMappable for EksNodeGroupTimeoutsEl {
    type O = BlockAssignable<EksNodeGroupTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupTimeoutsEl {}

impl BuildEksNodeGroupTimeoutsEl {
    pub fn build(self) -> EksNodeGroupTimeoutsEl {
        EksNodeGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupTimeoutsElRef {
        EksNodeGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupTimeoutsElRef {
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
pub struct EksNodeGroupUpdateConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_unavailable_percentage: Option<PrimField<f64>>,
}

impl EksNodeGroupUpdateConfigEl {
    #[doc = "Set the field `max_unavailable`.\n"]
    pub fn set_max_unavailable(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable = Some(v.into());
        self
    }

    #[doc = "Set the field `max_unavailable_percentage`.\n"]
    pub fn set_max_unavailable_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_unavailable_percentage = Some(v.into());
        self
    }
}

impl ToListMappable for EksNodeGroupUpdateConfigEl {
    type O = BlockAssignable<EksNodeGroupUpdateConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEksNodeGroupUpdateConfigEl {}

impl BuildEksNodeGroupUpdateConfigEl {
    pub fn build(self) -> EksNodeGroupUpdateConfigEl {
        EksNodeGroupUpdateConfigEl {
            max_unavailable: core::default::Default::default(),
            max_unavailable_percentage: core::default::Default::default(),
        }
    }
}

pub struct EksNodeGroupUpdateConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EksNodeGroupUpdateConfigElRef {
    fn new(shared: StackShared, base: String) -> EksNodeGroupUpdateConfigElRef {
        EksNodeGroupUpdateConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EksNodeGroupUpdateConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_unavailable` after provisioning.\n"]
    pub fn max_unavailable(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable", self.base))
    }

    #[doc = "Get a reference to the value of field `max_unavailable_percentage` after provisioning.\n"]
    pub fn max_unavailable_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable_percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct EksNodeGroupDynamic {
    launch_template: Option<DynamicBlock<EksNodeGroupLaunchTemplateEl>>,
    node_repair_config: Option<DynamicBlock<EksNodeGroupNodeRepairConfigEl>>,
    remote_access: Option<DynamicBlock<EksNodeGroupRemoteAccessEl>>,
    scaling_config: Option<DynamicBlock<EksNodeGroupScalingConfigEl>>,
    taint: Option<DynamicBlock<EksNodeGroupTaintEl>>,
    update_config: Option<DynamicBlock<EksNodeGroupUpdateConfigEl>>,
}
