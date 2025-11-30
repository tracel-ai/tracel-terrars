use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataEksNodeGroupData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    node_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataEksNodeGroup_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEksNodeGroupData>,
}

#[derive(Clone)]
pub struct DataEksNodeGroup(Rc<DataEksNodeGroup_>);

impl DataEksNodeGroup {
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

    #[doc = "Get a reference to the value of field `ami_type` after provisioning.\n"]
    pub fn ami_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ami_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_type` after provisioning.\n"]
    pub fn capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<DataEksNodeGroupLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_group_name` after provisioning.\n"]
    pub fn node_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> ListRef<DataEksNodeGroupRemoteAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remote_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<DataEksNodeGroupResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> ListRef<DataEksNodeGroupScalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(&self) -> ListRef<DataEksNodeGroupTaintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.taints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}

impl Referable for DataEksNodeGroup {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataEksNodeGroup {}

impl ToListMappable for DataEksNodeGroup {
    type O = ListRef<DataEksNodeGroupRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEksNodeGroup_ {
    fn extract_datasource_type(&self) -> String {
        "aws_eks_node_group".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEksNodeGroup {
    pub tf_id: String,
    #[doc = ""]
    pub cluster_name: PrimField<String>,
    #[doc = ""]
    pub node_group_name: PrimField<String>,
}

impl BuildDataEksNodeGroup {
    pub fn build(self, stack: &mut Stack) -> DataEksNodeGroup {
        let out = DataEksNodeGroup(Rc::new(DataEksNodeGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEksNodeGroupData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                node_group_name: self.node_group_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEksNodeGroupRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksNodeGroupRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataEksNodeGroupRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `ami_type` after provisioning.\n"]
    pub fn ami_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ami_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_type` after provisioning.\n"]
    pub fn capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cluster_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_size` after provisioning.\n"]
    pub fn disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_size", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_types` after provisioning.\n"]
    pub fn instance_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> ListRef<DataEksNodeGroupLaunchTemplateElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.launch_template", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_group_name` after provisioning.\n"]
    pub fn node_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_group_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `node_role_arn` after provisioning.\n"]
    pub fn node_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `release_version` after provisioning.\n"]
    pub fn release_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.release_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `remote_access` after provisioning.\n"]
    pub fn remote_access(&self) -> ListRef<DataEksNodeGroupRemoteAccessElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remote_access", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<DataEksNodeGroupResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> ListRef<DataEksNodeGroupScalingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scaling_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `taints` after provisioning.\n"]
    pub fn taints(&self) -> ListRef<DataEksNodeGroupTaintsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.taints", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEksNodeGroupLaunchTemplateEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<PrimField<String>>,
}

impl DataEksNodeGroupLaunchTemplateEl {
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

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.version = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksNodeGroupLaunchTemplateEl {
    type O = BlockAssignable<DataEksNodeGroupLaunchTemplateEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupLaunchTemplateEl {}

impl BuildDataEksNodeGroupLaunchTemplateEl {
    pub fn build(self) -> DataEksNodeGroupLaunchTemplateEl {
        DataEksNodeGroupLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupLaunchTemplateElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksNodeGroupLaunchTemplateElRef {
    fn new(shared: StackShared, base: String) -> DataEksNodeGroupLaunchTemplateElRef {
        DataEksNodeGroupLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupLaunchTemplateElRef {
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
pub struct DataEksNodeGroupRemoteAccessEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ec2_ssh_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_security_group_ids: Option<SetField<PrimField<String>>>,
}

impl DataEksNodeGroupRemoteAccessEl {
    #[doc = "Set the field `ec2_ssh_key`.\n"]
    pub fn set_ec2_ssh_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ec2_ssh_key = Some(v.into());
        self
    }

    #[doc = "Set the field `source_security_group_ids`.\n"]
    pub fn set_source_security_group_ids(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.source_security_group_ids = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksNodeGroupRemoteAccessEl {
    type O = BlockAssignable<DataEksNodeGroupRemoteAccessEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupRemoteAccessEl {}

impl BuildDataEksNodeGroupRemoteAccessEl {
    pub fn build(self) -> DataEksNodeGroupRemoteAccessEl {
        DataEksNodeGroupRemoteAccessEl {
            ec2_ssh_key: core::default::Default::default(),
            source_security_group_ids: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupRemoteAccessElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksNodeGroupRemoteAccessElRef {
    fn new(shared: StackShared, base: String) -> DataEksNodeGroupRemoteAccessElRef {
        DataEksNodeGroupRemoteAccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupRemoteAccessElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ec2_ssh_key` after provisioning.\n"]
    pub fn ec2_ssh_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ec2_ssh_key", self.base))
    }

    #[doc = "Get a reference to the value of field `source_security_group_ids` after provisioning.\n"]
    pub fn source_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.source_security_group_ids", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEksNodeGroupResourcesElAutoscalingGroupsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataEksNodeGroupResourcesElAutoscalingGroupsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksNodeGroupResourcesElAutoscalingGroupsEl {
    type O = BlockAssignable<DataEksNodeGroupResourcesElAutoscalingGroupsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupResourcesElAutoscalingGroupsEl {}

impl BuildDataEksNodeGroupResourcesElAutoscalingGroupsEl {
    pub fn build(self) -> DataEksNodeGroupResourcesElAutoscalingGroupsEl {
        DataEksNodeGroupResourcesElAutoscalingGroupsEl {
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
    fn new(shared: StackShared, base: String) -> DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
        DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupResourcesElAutoscalingGroupsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEksNodeGroupResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscaling_groups: Option<ListField<DataEksNodeGroupResourcesElAutoscalingGroupsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_access_security_group_id: Option<PrimField<String>>,
}

impl DataEksNodeGroupResourcesEl {
    #[doc = "Set the field `autoscaling_groups`.\n"]
    pub fn set_autoscaling_groups(
        mut self,
        v: impl Into<ListField<DataEksNodeGroupResourcesElAutoscalingGroupsEl>>,
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

impl ToListMappable for DataEksNodeGroupResourcesEl {
    type O = BlockAssignable<DataEksNodeGroupResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupResourcesEl {}

impl BuildDataEksNodeGroupResourcesEl {
    pub fn build(self) -> DataEksNodeGroupResourcesEl {
        DataEksNodeGroupResourcesEl {
            autoscaling_groups: core::default::Default::default(),
            remote_access_security_group_id: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksNodeGroupResourcesElRef {
    fn new(shared: StackShared, base: String) -> DataEksNodeGroupResourcesElRef {
        DataEksNodeGroupResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> ListRef<DataEksNodeGroupResourcesElAutoscalingGroupsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.autoscaling_groups", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `remote_access_security_group_id` after provisioning.\n"]
    pub fn remote_access_security_group_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.remote_access_security_group_id", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataEksNodeGroupScalingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_size: Option<PrimField<f64>>,
}

impl DataEksNodeGroupScalingConfigEl {
    #[doc = "Set the field `desired_size`.\n"]
    pub fn set_desired_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.desired_size = Some(v.into());
        self
    }

    #[doc = "Set the field `max_size`.\n"]
    pub fn set_max_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_size = Some(v.into());
        self
    }

    #[doc = "Set the field `min_size`.\n"]
    pub fn set_min_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_size = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksNodeGroupScalingConfigEl {
    type O = BlockAssignable<DataEksNodeGroupScalingConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupScalingConfigEl {}

impl BuildDataEksNodeGroupScalingConfigEl {
    pub fn build(self) -> DataEksNodeGroupScalingConfigEl {
        DataEksNodeGroupScalingConfigEl {
            desired_size: core::default::Default::default(),
            max_size: core::default::Default::default(),
            min_size: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupScalingConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksNodeGroupScalingConfigElRef {
    fn new(shared: StackShared, base: String) -> DataEksNodeGroupScalingConfigElRef {
        DataEksNodeGroupScalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupScalingConfigElRef {
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
pub struct DataEksNodeGroupTaintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataEksNodeGroupTaintsEl {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.effect = Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataEksNodeGroupTaintsEl {
    type O = BlockAssignable<DataEksNodeGroupTaintsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEksNodeGroupTaintsEl {}

impl BuildDataEksNodeGroupTaintsEl {
    pub fn build(self) -> DataEksNodeGroupTaintsEl {
        DataEksNodeGroupTaintsEl {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataEksNodeGroupTaintsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEksNodeGroupTaintsElRef {
    fn new(shared: StackShared, base: String) -> DataEksNodeGroupTaintsElRef {
        DataEksNodeGroupTaintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEksNodeGroupTaintsElRef {
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
