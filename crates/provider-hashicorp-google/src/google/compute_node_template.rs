use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeNodeTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_overcommit_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinity_labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerators: Option<Vec<ComputeNodeTemplateAcceleratorsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disks: Option<Vec<ComputeNodeTemplateDisksEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type_flexibility: Option<Vec<ComputeNodeTemplateNodeTypeFlexibilityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_binding: Option<Vec<ComputeNodeTemplateServerBindingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeNodeTemplateTimeoutsEl>,
    dynamic: ComputeNodeTemplateDynamic,
}
struct ComputeNodeTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeNodeTemplateData>,
}
#[derive(Clone)]
pub struct ComputeNodeTemplate(Rc<ComputeNodeTemplate_>);
impl ComputeNodeTemplate {
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
    #[doc = "Set the field `cpu_overcommit_type`.\nCPU overcommit. Default value: \"NONE\" Possible values: [\"ENABLED\", \"NONE\"]"]
    pub fn set_cpu_overcommit_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cpu_overcommit_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional textual description of the resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `node_affinity_labels`.\nLabels to use for node affinity, which will be used in\ninstance scheduling."]
    pub fn set_node_affinity_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().node_affinity_labels = Some(v.into());
        self
    }
    #[doc = "Set the field `node_type`.\nNode type to use for nodes group that are created from this template.\nOnly one of nodeTypeFlexibility and nodeType can be specified."]
    pub fn set_node_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().node_type = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where nodes using the node template will be created.\nIf it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        self,
        v: impl Into<BlockAssignable<ComputeNodeTemplateAcceleratorsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().accelerators = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.accelerators = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `disks`.\n"]
    pub fn set_disks(self, v: impl Into<BlockAssignable<ComputeNodeTemplateDisksEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disks = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disks = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `node_type_flexibility`.\n"]
    pub fn set_node_type_flexibility(
        self,
        v: impl Into<BlockAssignable<ComputeNodeTemplateNodeTypeFlexibilityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_type_flexibility = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_type_flexibility = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `server_binding`.\n"]
    pub fn set_server_binding(
        self,
        v: impl Into<BlockAssignable<ComputeNodeTemplateServerBindingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_binding = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_binding = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeNodeTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cpu_overcommit_type` after provisioning.\nCPU overcommit. Default value: \"NONE\" Possible values: [\"ENABLED\", \"NONE\"]"]
    pub fn cpu_overcommit_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_overcommit_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_affinity_labels` after provisioning.\nLabels to use for node affinity, which will be used in\ninstance scheduling."]
    pub fn node_affinity_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.node_affinity_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nNode type to use for nodes group that are created from this template.\nOnly one of nodeTypeFlexibility and nodeType can be specified."]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where nodes using the node template will be created.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> ListRef<ComputeNodeTemplateAcceleratorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disks` after provisioning.\n"]
    pub fn disks(&self) -> ListRef<ComputeNodeTemplateDisksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type_flexibility` after provisioning.\n"]
    pub fn node_type_flexibility(&self) -> ListRef<ComputeNodeTemplateNodeTypeFlexibilityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_type_flexibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_binding` after provisioning.\n"]
    pub fn server_binding(&self) -> ListRef<ComputeNodeTemplateServerBindingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_binding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeTemplateTimeoutsElRef {
        ComputeNodeTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeNodeTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeNodeTemplate {}
impl ToListMappable for ComputeNodeTemplate {
    type O = ListRef<ComputeNodeTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeNodeTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_node_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeNodeTemplate {
    pub tf_id: String,
    #[doc = "Name of the resource."]
    pub name: PrimField<String>,
}
impl BuildComputeNodeTemplate {
    pub fn build(self, stack: &mut Stack) -> ComputeNodeTemplate {
        let out = ComputeNodeTemplate(Rc::new(ComputeNodeTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeNodeTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cpu_overcommit_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                node_affinity_labels: core::default::Default::default(),
                node_type: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                accelerators: core::default::Default::default(),
                disks: core::default::Default::default(),
                node_type_flexibility: core::default::Default::default(),
                server_binding: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeNodeTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeNodeTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu_overcommit_type` after provisioning.\nCPU overcommit. Default value: \"NONE\" Possible values: [\"ENABLED\", \"NONE\"]"]
    pub fn cpu_overcommit_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_overcommit_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_affinity_labels` after provisioning.\nLabels to use for node affinity, which will be used in\ninstance scheduling."]
    pub fn node_affinity_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.node_affinity_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type` after provisioning.\nNode type to use for nodes group that are created from this template.\nOnly one of nodeTypeFlexibility and nodeType can be specified."]
    pub fn node_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.node_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where nodes using the node template will be created.\nIf it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> ListRef<ComputeNodeTemplateAcceleratorsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerators", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disks` after provisioning.\n"]
    pub fn disks(&self) -> ListRef<ComputeNodeTemplateDisksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.disks", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `node_type_flexibility` after provisioning.\n"]
    pub fn node_type_flexibility(&self) -> ListRef<ComputeNodeTemplateNodeTypeFlexibilityElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.node_type_flexibility", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_binding` after provisioning.\n"]
    pub fn server_binding(&self) -> ListRef<ComputeNodeTemplateServerBindingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_binding", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNodeTemplateTimeoutsElRef {
        ComputeNodeTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeNodeTemplateAcceleratorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
}
impl ComputeNodeTemplateAcceleratorsEl {
    #[doc = "Set the field `accelerator_count`.\nThe number of the guest accelerator cards exposed to this\nnode template."]
    pub fn set_accelerator_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accelerator_count = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_type`.\nFull or partial URL of the accelerator type resource to expose\nto this node template."]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeNodeTemplateAcceleratorsEl {
    type O = BlockAssignable<ComputeNodeTemplateAcceleratorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeTemplateAcceleratorsEl {}
impl BuildComputeNodeTemplateAcceleratorsEl {
    pub fn build(self) -> ComputeNodeTemplateAcceleratorsEl {
        ComputeNodeTemplateAcceleratorsEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeNodeTemplateAcceleratorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeTemplateAcceleratorsElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeTemplateAcceleratorsElRef {
        ComputeNodeTemplateAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeTemplateAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of the guest accelerator cards exposed to this\nnode template."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nFull or partial URL of the accelerator type resource to expose\nto this node template."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeNodeTemplateDisksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
}
impl ComputeNodeTemplateDisksEl {
    #[doc = "Set the field `disk_count`.\nSpecifies the number of such disks."]
    pub fn set_disk_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_count = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\nSpecifies the size of the disk in base-2 GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nSpecifies the desired disk type on the node. This disk type must be a local storage type (e.g.: local-ssd). Note that for nodeTemplates, this should be the name of the disk type and not its URL."]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeNodeTemplateDisksEl {
    type O = BlockAssignable<ComputeNodeTemplateDisksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeTemplateDisksEl {}
impl BuildComputeNodeTemplateDisksEl {
    pub fn build(self) -> ComputeNodeTemplateDisksEl {
        ComputeNodeTemplateDisksEl {
            disk_count: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
        }
    }
}
pub struct ComputeNodeTemplateDisksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeTemplateDisksElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeTemplateDisksElRef {
        ComputeNodeTemplateDisksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeTemplateDisksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_count` after provisioning.\nSpecifies the number of such disks."]
    pub fn disk_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_count", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nSpecifies the size of the disk in base-2 GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nSpecifies the desired disk type on the node. This disk type must be a local storage type (e.g.: local-ssd). Note that for nodeTemplates, this should be the name of the disk type and not its URL."]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeNodeTemplateNodeTypeFlexibilityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpus: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<String>>,
}
impl ComputeNodeTemplateNodeTypeFlexibilityEl {
    #[doc = "Set the field `cpus`.\nNumber of virtual CPUs to use."]
    pub fn set_cpus(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpus = Some(v.into());
        self
    }
    #[doc = "Set the field `memory`.\nPhysical memory available to the node, defined in MB."]
    pub fn set_memory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeNodeTemplateNodeTypeFlexibilityEl {
    type O = BlockAssignable<ComputeNodeTemplateNodeTypeFlexibilityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeTemplateNodeTypeFlexibilityEl {}
impl BuildComputeNodeTemplateNodeTypeFlexibilityEl {
    pub fn build(self) -> ComputeNodeTemplateNodeTypeFlexibilityEl {
        ComputeNodeTemplateNodeTypeFlexibilityEl {
            cpus: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}
pub struct ComputeNodeTemplateNodeTypeFlexibilityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeTemplateNodeTypeFlexibilityElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeTemplateNodeTypeFlexibilityElRef {
        ComputeNodeTemplateNodeTypeFlexibilityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeTemplateNodeTypeFlexibilityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpus` after provisioning.\nNumber of virtual CPUs to use."]
    pub fn cpus(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpus", self.base))
    }
    #[doc = "Get a reference to the value of field `local_ssd` after provisioning.\nUse local SSD"]
    pub fn local_ssd(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.local_ssd", self.base))
    }
    #[doc = "Get a reference to the value of field `memory` after provisioning.\nPhysical memory available to the node, defined in MB."]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeNodeTemplateServerBindingEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ComputeNodeTemplateServerBindingEl {}
impl ToListMappable for ComputeNodeTemplateServerBindingEl {
    type O = BlockAssignable<ComputeNodeTemplateServerBindingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeTemplateServerBindingEl {
    #[doc = "Type of server binding policy. If 'RESTART_NODE_ON_ANY_SERVER',\nnodes using this template will restart on any physical server\nfollowing a maintenance event.\n\nIf 'RESTART_NODE_ON_MINIMAL_SERVER', nodes using this template\nwill restart on the same physical server following a maintenance\nevent, instead of being live migrated to or restarted on a new\nphysical server. This option may be useful if you are using\nsoftware licenses tied to the underlying server characteristics\nsuch as physical sockets or cores, to avoid the need for\nadditional licenses when maintenance occurs. However, VMs on such\nnodes will experience outages while maintenance is applied. Possible values: [\"RESTART_NODE_ON_ANY_SERVER\", \"RESTART_NODE_ON_MINIMAL_SERVERS\"]"]
    pub type_: PrimField<String>,
}
impl BuildComputeNodeTemplateServerBindingEl {
    pub fn build(self) -> ComputeNodeTemplateServerBindingEl {
        ComputeNodeTemplateServerBindingEl { type_: self.type_ }
    }
}
pub struct ComputeNodeTemplateServerBindingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeTemplateServerBindingElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeTemplateServerBindingElRef {
        ComputeNodeTemplateServerBindingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeTemplateServerBindingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of server binding policy. If 'RESTART_NODE_ON_ANY_SERVER',\nnodes using this template will restart on any physical server\nfollowing a maintenance event.\n\nIf 'RESTART_NODE_ON_MINIMAL_SERVER', nodes using this template\nwill restart on the same physical server following a maintenance\nevent, instead of being live migrated to or restarted on a new\nphysical server. This option may be useful if you are using\nsoftware licenses tied to the underlying server characteristics\nsuch as physical sockets or cores, to avoid the need for\nadditional licenses when maintenance occurs. However, VMs on such\nnodes will experience outages while maintenance is applied. Possible values: [\"RESTART_NODE_ON_ANY_SERVER\", \"RESTART_NODE_ON_MINIMAL_SERVERS\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeNodeTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ComputeNodeTemplateTimeoutsEl {
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
}
impl ToListMappable for ComputeNodeTemplateTimeoutsEl {
    type O = BlockAssignable<ComputeNodeTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeNodeTemplateTimeoutsEl {}
impl BuildComputeNodeTemplateTimeoutsEl {
    pub fn build(self) -> ComputeNodeTemplateTimeoutsEl {
        ComputeNodeTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ComputeNodeTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeNodeTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeNodeTemplateTimeoutsElRef {
        ComputeNodeTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeNodeTemplateTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct ComputeNodeTemplateDynamic {
    accelerators: Option<DynamicBlock<ComputeNodeTemplateAcceleratorsEl>>,
    disks: Option<DynamicBlock<ComputeNodeTemplateDisksEl>>,
    node_type_flexibility: Option<DynamicBlock<ComputeNodeTemplateNodeTypeFlexibilityEl>>,
    server_binding: Option<DynamicBlock<ComputeNodeTemplateServerBindingEl>>,
}
