use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct NotebooksRuntimeData {
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
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_config: Option<Vec<NotebooksRuntimeAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software_config: Option<Vec<NotebooksRuntimeSoftwareConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<NotebooksRuntimeTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_machine: Option<Vec<NotebooksRuntimeVirtualMachineEl>>,
    dynamic: NotebooksRuntimeDynamic,
}
struct NotebooksRuntime_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NotebooksRuntimeData>,
}
#[derive(Clone)]
pub struct NotebooksRuntime(Rc<NotebooksRuntime_>);
impl NotebooksRuntime {
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
    #[doc = "Set the field `labels`.\nThe labels to associate with this runtime. Label **keys** must\ncontain 1 to 63 characters, and must conform to [RFC 1035]\n(https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be\nempty, but, if present, must contain 1 to 63 characters, and must\nconform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No\nmore than 32 labels can be associated with a cluster.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        self,
        v: impl Into<BlockAssignable<NotebooksRuntimeAccessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        self,
        v: impl Into<BlockAssignable<NotebooksRuntimeSoftwareConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().software_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.software_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<NotebooksRuntimeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_machine`.\n"]
    pub fn set_virtual_machine(
        self,
        v: impl Into<BlockAssignable<NotebooksRuntimeVirtualMachineEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().virtual_machine = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.virtual_machine = Some(d);
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_state` after provisioning.\nThe health state of this runtime. For a list of possible output\nvalues, see 'https://cloud.google.com/vertex-ai/docs/workbench/\nreference/rest/v1/projects.locations.runtimes#healthstate'."]
    pub fn health_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this runtime. Label **keys** must\ncontain 1 to 63 characters, and must conform to [RFC 1035]\n(https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be\nempty, but, if present, must contain 1 to 63 characters, and must\nconform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No\nmore than 32 labels can be associated with a cluster.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nA reference to the zone where the machine resides."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metrics` after provisioning.\nContains Runtime daemon metrics such as Service status and JupyterLab\nstatus"]
    pub fn metrics(&self) -> ListRef<NotebooksRuntimeMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name specified for the Notebook runtime."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of this runtime."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<NotebooksRuntimeAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> ListRef<NotebooksRuntimeSoftwareConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NotebooksRuntimeTimeoutsElRef {
        NotebooksRuntimeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_machine` after provisioning.\n"]
    pub fn virtual_machine(&self) -> ListRef<NotebooksRuntimeVirtualMachineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_machine", self.extract_ref()),
        )
    }
}
impl Referable for NotebooksRuntime {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for NotebooksRuntime {}
impl ToListMappable for NotebooksRuntime {
    type O = ListRef<NotebooksRuntimeRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for NotebooksRuntime_ {
    fn extract_resource_type(&self) -> String {
        "google_notebooks_runtime".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildNotebooksRuntime {
    pub tf_id: String,
    #[doc = "A reference to the zone where the machine resides."]
    pub location: PrimField<String>,
    #[doc = "The name specified for the Notebook runtime."]
    pub name: PrimField<String>,
}
impl BuildNotebooksRuntime {
    pub fn build(self, stack: &mut Stack) -> NotebooksRuntime {
        let out = NotebooksRuntime(Rc::new(NotebooksRuntime_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NotebooksRuntimeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                access_config: core::default::Default::default(),
                software_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                virtual_machine: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct NotebooksRuntimeRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl NotebooksRuntimeRef {
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
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_state` after provisioning.\nThe health state of this runtime. For a list of possible output\nvalues, see 'https://cloud.google.com/vertex-ai/docs/workbench/\nreference/rest/v1/projects.locations.runtimes#healthstate'."]
    pub fn health_state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this runtime. Label **keys** must\ncontain 1 to 63 characters, and must conform to [RFC 1035]\n(https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be\nempty, but, if present, must contain 1 to 63 characters, and must\nconform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No\nmore than 32 labels can be associated with a cluster.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nA reference to the zone where the machine resides."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metrics` after provisioning.\nContains Runtime daemon metrics such as Service status and JupyterLab\nstatus"]
    pub fn metrics(&self) -> ListRef<NotebooksRuntimeMetricsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metrics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name specified for the Notebook runtime."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of this runtime."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<NotebooksRuntimeAccessConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> ListRef<NotebooksRuntimeSoftwareConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NotebooksRuntimeTimeoutsElRef {
        NotebooksRuntimeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_machine` after provisioning.\n"]
    pub fn virtual_machine(&self) -> ListRef<NotebooksRuntimeVirtualMachineElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_machine", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct NotebooksRuntimeMetricsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    system_metrics: Option<RecField<PrimField<String>>>,
}
impl NotebooksRuntimeMetricsEl {
    #[doc = "Set the field `system_metrics`.\n"]
    pub fn set_system_metrics(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.system_metrics = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksRuntimeMetricsEl {
    type O = BlockAssignable<NotebooksRuntimeMetricsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeMetricsEl {}
impl BuildNotebooksRuntimeMetricsEl {
    pub fn build(self) -> NotebooksRuntimeMetricsEl {
        NotebooksRuntimeMetricsEl {
            system_metrics: core::default::Default::default(),
        }
    }
}
pub struct NotebooksRuntimeMetricsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeMetricsElRef {
    fn new(shared: StackShared, base: String) -> NotebooksRuntimeMetricsElRef {
        NotebooksRuntimeMetricsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeMetricsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `system_metrics` after provisioning.\n"]
    pub fn system_metrics(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.system_metrics", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NotebooksRuntimeAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_owner: Option<PrimField<String>>,
}
impl NotebooksRuntimeAccessConfigEl {
    #[doc = "Set the field `access_type`.\nThe type of access mode this instance. For valid values, see\n'https://cloud.google.com/vertex-ai/docs/workbench/reference/\nrest/v1/projects.locations.runtimes#RuntimeAccessType'."]
    pub fn set_access_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_type = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime_owner`.\nThe owner of this runtime after creation. Format: 'alias@example.com'.\nCurrently supports one owner only."]
    pub fn set_runtime_owner(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runtime_owner = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksRuntimeAccessConfigEl {
    type O = BlockAssignable<NotebooksRuntimeAccessConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeAccessConfigEl {}
impl BuildNotebooksRuntimeAccessConfigEl {
    pub fn build(self) -> NotebooksRuntimeAccessConfigEl {
        NotebooksRuntimeAccessConfigEl {
            access_type: core::default::Default::default(),
            runtime_owner: core::default::Default::default(),
        }
    }
}
pub struct NotebooksRuntimeAccessConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeAccessConfigElRef {
    fn new(shared: StackShared, base: String) -> NotebooksRuntimeAccessConfigElRef {
        NotebooksRuntimeAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_type` after provisioning.\nThe type of access mode this instance. For valid values, see\n'https://cloud.google.com/vertex-ai/docs/workbench/reference/\nrest/v1/projects.locations.runtimes#RuntimeAccessType'."]
    pub fn access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.base))
    }
    #[doc = "Get a reference to the value of field `proxy_uri` after provisioning.\nThe proxy endpoint that is used to access the runtime."]
    pub fn proxy_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proxy_uri", self.base))
    }
    #[doc = "Get a reference to the value of field `runtime_owner` after provisioning.\nThe owner of this runtime after creation. Format: 'alias@example.com'.\nCurrently supports one owner only."]
    pub fn runtime_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_owner", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NotebooksRuntimeSoftwareConfigElKernelsEl {
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl NotebooksRuntimeSoftwareConfigElKernelsEl {
    #[doc = "Set the field `tag`.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksRuntimeSoftwareConfigElKernelsEl {
    type O = BlockAssignable<NotebooksRuntimeSoftwareConfigElKernelsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeSoftwareConfigElKernelsEl {
    #[doc = "The path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub repository: PrimField<String>,
}
impl BuildNotebooksRuntimeSoftwareConfigElKernelsEl {
    pub fn build(self) -> NotebooksRuntimeSoftwareConfigElKernelsEl {
        NotebooksRuntimeSoftwareConfigElKernelsEl {
            repository: self.repository,
            tag: core::default::Default::default(),
        }
    }
}
pub struct NotebooksRuntimeSoftwareConfigElKernelsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeSoftwareConfigElKernelsElRef {
    fn new(shared: StackShared, base: String) -> NotebooksRuntimeSoftwareConfigElKernelsElRef {
        NotebooksRuntimeSoftwareConfigElKernelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeSoftwareConfigElKernelsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize, Default)]
struct NotebooksRuntimeSoftwareConfigElDynamic {
    kernels: Option<DynamicBlock<NotebooksRuntimeSoftwareConfigElKernelsEl>>,
}
#[derive(Serialize)]
pub struct NotebooksRuntimeSoftwareConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_gpu_driver_path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_health_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_shutdown: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_shutdown_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    install_gpu_driver: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notebook_upgrade_schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_startup_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_startup_script_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kernels: Option<Vec<NotebooksRuntimeSoftwareConfigElKernelsEl>>,
    dynamic: NotebooksRuntimeSoftwareConfigElDynamic,
}
impl NotebooksRuntimeSoftwareConfigEl {
    #[doc = "Set the field `custom_gpu_driver_path`.\nSpecify a custom Cloud Storage path where the GPU driver is stored.\nIf not specified, we'll automatically choose from official GPU drivers."]
    pub fn set_custom_gpu_driver_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.custom_gpu_driver_path = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_health_monitoring`.\nVerifies core internal services are running. Default: True."]
    pub fn set_enable_health_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_health_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_shutdown`.\nRuntime will automatically shutdown after idle_shutdown_time.\nDefault: True"]
    pub fn set_idle_shutdown(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.idle_shutdown = Some(v.into());
        self
    }
    #[doc = "Set the field `idle_shutdown_timeout`.\nTime in minutes to wait before shuting down runtime.\nDefault: 180 minutes"]
    pub fn set_idle_shutdown_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_shutdown_timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `install_gpu_driver`.\nInstall Nvidia Driver automatically."]
    pub fn set_install_gpu_driver(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.install_gpu_driver = Some(v.into());
        self
    }
    #[doc = "Set the field `notebook_upgrade_schedule`.\nCron expression in UTC timezone for schedule instance auto upgrade.\nPlease follow the [cron format](https://en.wikipedia.org/wiki/Cron)."]
    pub fn set_notebook_upgrade_schedule(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.notebook_upgrade_schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `post_startup_script`.\nPath to a Bash script that automatically runs after a notebook instance\nfully boots up. The path must be a URL or\nCloud Storage path (gs://path-to-file/file-name)."]
    pub fn set_post_startup_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.post_startup_script = Some(v.into());
        self
    }
    #[doc = "Set the field `post_startup_script_behavior`.\nBehavior for the post startup script. Possible values: [\"POST_STARTUP_SCRIPT_BEHAVIOR_UNSPECIFIED\", \"RUN_EVERY_START\", \"DOWNLOAD_AND_RUN_EVERY_START\"]"]
    pub fn set_post_startup_script_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.post_startup_script_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `kernels`.\n"]
    pub fn set_kernels(
        mut self,
        v: impl Into<BlockAssignable<NotebooksRuntimeSoftwareConfigElKernelsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kernels = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kernels = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NotebooksRuntimeSoftwareConfigEl {
    type O = BlockAssignable<NotebooksRuntimeSoftwareConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeSoftwareConfigEl {}
impl BuildNotebooksRuntimeSoftwareConfigEl {
    pub fn build(self) -> NotebooksRuntimeSoftwareConfigEl {
        NotebooksRuntimeSoftwareConfigEl {
            custom_gpu_driver_path: core::default::Default::default(),
            enable_health_monitoring: core::default::Default::default(),
            idle_shutdown: core::default::Default::default(),
            idle_shutdown_timeout: core::default::Default::default(),
            install_gpu_driver: core::default::Default::default(),
            notebook_upgrade_schedule: core::default::Default::default(),
            post_startup_script: core::default::Default::default(),
            post_startup_script_behavior: core::default::Default::default(),
            kernels: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NotebooksRuntimeSoftwareConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeSoftwareConfigElRef {
    fn new(shared: StackShared, base: String) -> NotebooksRuntimeSoftwareConfigElRef {
        NotebooksRuntimeSoftwareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeSoftwareConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_gpu_driver_path` after provisioning.\nSpecify a custom Cloud Storage path where the GPU driver is stored.\nIf not specified, we'll automatically choose from official GPU drivers."]
    pub fn custom_gpu_driver_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_gpu_driver_path", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_health_monitoring` after provisioning.\nVerifies core internal services are running. Default: True."]
    pub fn enable_health_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_health_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `idle_shutdown` after provisioning.\nRuntime will automatically shutdown after idle_shutdown_time.\nDefault: True"]
    pub fn idle_shutdown(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_shutdown", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `idle_shutdown_timeout` after provisioning.\nTime in minutes to wait before shuting down runtime.\nDefault: 180 minutes"]
    pub fn idle_shutdown_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.idle_shutdown_timeout", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `install_gpu_driver` after provisioning.\nInstall Nvidia Driver automatically."]
    pub fn install_gpu_driver(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.install_gpu_driver", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `notebook_upgrade_schedule` after provisioning.\nCron expression in UTC timezone for schedule instance auto upgrade.\nPlease follow the [cron format](https://en.wikipedia.org/wiki/Cron)."]
    pub fn notebook_upgrade_schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.notebook_upgrade_schedule", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `post_startup_script` after provisioning.\nPath to a Bash script that automatically runs after a notebook instance\nfully boots up. The path must be a URL or\nCloud Storage path (gs://path-to-file/file-name)."]
    pub fn post_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_startup_script", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `post_startup_script_behavior` after provisioning.\nBehavior for the post startup script. Possible values: [\"POST_STARTUP_SCRIPT_BEHAVIOR_UNSPECIFIED\", \"RUN_EVERY_START\", \"DOWNLOAD_AND_RUN_EVERY_START\"]"]
    pub fn post_startup_script_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_startup_script_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `upgradeable` after provisioning.\nBool indicating whether an newer image is available in an image family."]
    pub fn upgradeable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.upgradeable", self.base))
    }
    #[doc = "Get a reference to the value of field `kernels` after provisioning.\n"]
    pub fn kernels(&self) -> ListRef<NotebooksRuntimeSoftwareConfigElKernelsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kernels", self.base))
    }
}
#[derive(Serialize)]
pub struct NotebooksRuntimeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl NotebooksRuntimeTimeoutsEl {
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
impl ToListMappable for NotebooksRuntimeTimeoutsEl {
    type O = BlockAssignable<NotebooksRuntimeTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeTimeoutsEl {}
impl BuildNotebooksRuntimeTimeoutsEl {
    pub fn build(self) -> NotebooksRuntimeTimeoutsEl {
        NotebooksRuntimeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct NotebooksRuntimeTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> NotebooksRuntimeTimeoutsElRef {
        NotebooksRuntimeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeTimeoutsElRef {
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
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    core_count: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl {
    #[doc = "Set the field `core_count`.\nCount of cores of this accelerator."]
    pub fn set_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.core_count = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nAccelerator model. For valid values, see\n'https://cloud.google.com/vertex-ai/docs/workbench/reference/\nrest/v1/projects.locations.runtimes#AcceleratorType'"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl {
    type O =
        BlockAssignable<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl {}
impl BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl {
    pub fn build(
        self,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl {
            core_count: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigElRef {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `core_count` after provisioning.\nCount of cores of this accelerator."]
    pub fn core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.core_count", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nAccelerator model. For valid values, see\n'https://cloud.google.com/vertex-ai/docs/workbench/reference/\nrest/v1/projects.locations.runtimes#AcceleratorType'"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl {
    repository: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<PrimField<String>>,
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl {
    #[doc = "Set the field `tag`.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn set_tag(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tag = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl {
    type O =
        BlockAssignable<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl {
    #[doc = "The path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub repository: PrimField<String>,
}
impl BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl {
    pub fn build(self) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl {
            repository: self.repository,
            tag: core::default::Default::default(),
        }
    }
}
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesElRef {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository` after provisioning.\nThe path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub fn repository(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
    #[doc = "Get a reference to the value of field `tag` after provisioning.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}
#[derive(Serialize)]
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl {
    #[doc = "Set the field `description`.\nProvide this property when creating the disk."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_name`.\nSpecifies the disk name. If not specified, the default is\nto use the name of the instance. If the disk with the\ninstance name exists already in the given zone/region, a\nnew name will be automatically generated."]
    pub fn set_disk_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_name = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_size_gb`.\nSpecifies the size of the disk in base-2 GB. If not\nspecified, the disk will be the same size as the image\n(usually 10GB). If specified, the size must be equal to\nor larger than 10GB. Default 100 GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nThe type of the boot disk attached to this runtime,\ndefaults to standard persistent disk. For valid values,\nsee 'https://cloud.google.com/vertex-ai/docs/workbench/\nreference/rest/v1/projects.locations.runtimes#disktype'"]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to apply to this disk. These can be later modified\nby the disks.setLabels method. This field is only\napplicable for persistent disks."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
}
impl ToListMappable
    for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl
{
    type O = BlockAssignable<
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl
{}
impl BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl {
    pub fn build(
        self,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl {
            description: core::default::Default::default(),
            disk_name: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            labels: core::default::Default::default(),
        }
    }
}
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsElRef {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nProvide this property when creating the disk."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_name` after provisioning.\nSpecifies the disk name. If not specified, the default is\nto use the name of the instance. If the disk with the\ninstance name exists already in the given zone/region, a\nnew name will be automatically generated."]
    pub fn disk_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_name", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nSpecifies the size of the disk in base-2 GB. If not\nspecified, the disk will be the same size as the image\n(usually 10GB). If specified, the size must be equal to\nor larger than 10GB. Default 100 GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nThe type of the boot disk attached to this runtime,\ndefaults to standard persistent disk. For valid values,\nsee 'https://cloud.google.com/vertex-ai/docs/workbench/\nreference/rest/v1/projects.locations.runtimes#disktype'"]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to apply to this disk. These can be later modified\nby the disks.setLabels method. This field is only\napplicable for persistent disks."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
}
#[derive(Serialize, Default)]
struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElDynamic {
    initialize_params: Option<
        DynamicBlock<
            NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initialize_params: Option<
        Vec<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl>,
    >,
    dynamic: NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElDynamic,
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl {
    #[doc = "Set the field `interface`.\n\"Specifies the disk interface to use for attaching this disk,\nwhich is either SCSI or NVME. The default is SCSI. Persistent\ndisks must always use SCSI and the request will fail if you attempt\nto attach a persistent disk in any other format than SCSI. Local SSDs\ncan use either NVME or SCSI. For performance characteristics of SCSI\nover NVMe, see Local SSD performance. Valid values: * NVME * SCSI\"."]
    pub fn set_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nThe mode in which to attach this disk, either READ_WRITE\nor READ_ONLY. If not specified, the default is to attach\nthe disk in READ_WRITE mode."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `source`.\nSpecifies a valid partial or full URL to an existing\nPersistent Disk resource."]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\nSpecifies the type of the disk, either SCRATCH or PERSISTENT.\nIf not specified, the default is PERSISTENT."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    #[doc = "Set the field `initialize_params`.\n"]
    pub fn set_initialize_params(
        mut self,
        v: impl Into<
            BlockAssignable<
                NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.initialize_params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.initialize_params = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl {
    type O = BlockAssignable<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl {}
impl BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl {
    pub fn build(self) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl {
            interface: core::default::Default::default(),
            mode: core::default::Default::default(),
            source: core::default::Default::default(),
            type_: core::default::Default::default(),
            initialize_params: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElRef {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_delete` after provisioning.\nOptional. Specifies whether the disk will be auto-deleted\nwhen the instance is deleted (but not when the disk is\ndetached from the instance)."]
    pub fn auto_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }
    #[doc = "Get a reference to the value of field `boot` after provisioning.\nOptional. Indicates that this is a boot disk. The virtual\nmachine will use the first partition of the disk for its\nroot filesystem."]
    pub fn boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot", self.base))
    }
    #[doc = "Get a reference to the value of field `device_name` after provisioning.\nOptional. Specifies a unique device name of your choice\nthat is reflected into the /dev/disk/by-id/google-* tree\nof a Linux operating system running within the instance.\nThis name can be used to reference the device for mounting,\nresizing, and so on, from within the instance.\nIf not specified, the server chooses a default device name\nto apply to this disk, in the form persistent-disk-x, where\nx is a number assigned by Google Compute Engine. This field\nis only applicable for persistent disks."]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
    #[doc = "Get a reference to the value of field `guest_os_features` after provisioning.\nIndicates a list of features to enable on the guest operating\nsystem. Applicable only for bootable images. To see a list of\navailable features, read 'https://cloud.google.com/compute/docs/\nimages/create-delete-deprecate-private-images#guest-os-features'\noptions. ''"]
    pub fn guest_os_features(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.guest_os_features", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `index` after provisioning.\nOutput only. A zero-based index to this disk, where 0 is\nreserved for the boot disk. If you have many disks attached\nto an instance, each disk would have a unique index number."]
    pub fn index(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.index", self.base))
    }
    #[doc = "Get a reference to the value of field `interface` after provisioning.\n\"Specifies the disk interface to use for attaching this disk,\nwhich is either SCSI or NVME. The default is SCSI. Persistent\ndisks must always use SCSI and the request will fail if you attempt\nto attach a persistent disk in any other format than SCSI. Local SSDs\ncan use either NVME or SCSI. For performance characteristics of SCSI\nover NVMe, see Local SSD performance. Valid values: * NVME * SCSI\"."]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }
    #[doc = "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always compute#attachedDisk for attached\ndisks."]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.base))
    }
    #[doc = "Get a reference to the value of field `licenses` after provisioning.\nOutput only. Any valid publicly visible licenses."]
    pub fn licenses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.licenses", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nThe mode in which to attach this disk, either READ_WRITE\nor READ_ONLY. If not specified, the default is to attach\nthe disk in READ_WRITE mode."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\nSpecifies a valid partial or full URL to an existing\nPersistent Disk resource."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nSpecifies the type of the disk, either SCRATCH or PERSISTENT.\nIf not specified, the default is PERSISTENT."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `initialize_params` after provisioning.\n"]
    pub fn initialize_params(
        &self,
    ) -> ListRef<
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElInitializeParamsElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.initialize_params", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key: Option<PrimField<String>>,
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl {
    #[doc = "Set the field `kms_key`.\nThe Cloud KMS resource identifier of the customer-managed\nencryption key used to protect a resource, such as a disks.\nIt has the following format:\n'projects/{PROJECT_ID}/locations/{REGION}/keyRings/\n{KEY_RING_NAME}/cryptoKeys/{KEY_NAME}'"]
    pub fn set_kms_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key = Some(v.into());
        self
    }
}
impl ToListMappable for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl {
    type O =
        BlockAssignable<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl {}
impl BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl {
    pub fn build(self) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl {
            kms_key: core::default::Default::default(),
        }
    }
}
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigElRef {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS resource identifier of the customer-managed\nencryption key used to protect a resource, such as a disks.\nIt has the following format:\n'projects/{PROJECT_ID}/locations/{REGION}/keyRings/\n{KEY_RING_NAME}/cryptoKeys/{KEY_NAME}'"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}
#[derive(Serialize)]
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\nDefines whether the instance has integrity monitoring enabled.\nEnables monitoring and attestation of the boot integrity of\nthe instance. The attestation is performed against the\nintegrity policy baseline. This baseline is initially derived\nfrom the implicitly trusted boot image when the instance is\ncreated. Enabled by default."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_secure_boot`.\nDefines whether the instance has Secure Boot enabled.Secure\nBoot helps ensure that the system only runs authentic software\nby verifying the digital signature of all boot components, and\nhalting the boot process if signature verification fails.\nDisabled by default."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_vtpm`.\nDefines whether the instance has the vTPM enabled. Enabled by\ndefault."]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}
impl ToListMappable
    for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl
{
    type O = BlockAssignable<
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl {}
impl BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl {
    pub fn build(
        self,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigElRef {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nDefines whether the instance has integrity monitoring enabled.\nEnables monitoring and attestation of the boot integrity of\nthe instance. The attestation is performed against the\nintegrity policy baseline. This baseline is initially derived\nfrom the implicitly trusted boot image when the instance is\ncreated. Enabled by default."]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_integrity_monitoring", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\nDefines whether the instance has Secure Boot enabled.Secure\nBoot helps ensure that the system only runs authentic software\nby verifying the digital signature of all boot components, and\nhalting the boot process if signature verification fails.\nDisabled by default."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\nDefines whether the instance has the vTPM enabled. Enabled by\ndefault."]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
#[derive(Serialize, Default)]
struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDynamic {
    accelerator_config: Option<
        DynamicBlock<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl>,
    >,
    container_images: Option<
        DynamicBlock<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl>,
    >,
    data_disk:
        Option<DynamicBlock<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl>>,
    encryption_config: Option<
        DynamicBlock<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl>,
    >,
    shielded_instance_config: Option<
        DynamicBlock<
            NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ip_only: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    machine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nic_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reserved_ip_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_config:
        Option<Vec<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_images:
        Option<Vec<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_disk: Option<Vec<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_config:
        Option<Vec<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config:
        Option<Vec<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl>>,
    dynamic: NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDynamic,
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigEl {
    #[doc = "Set the field `internal_ip_only`.\nIf true, runtime will only have internal IP addresses. By default,\nruntimes are not restricted to internal IP addresses, and will\nhave ephemeral external IP addresses assigned to each vm. This\n'internal_ip_only' restriction can only be enabled for subnetwork\nenabled networks, and all dependencies must be configured to be\naccessible without external IP addresses."]
    pub fn set_internal_ip_only(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.internal_ip_only = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nThe labels to associate with this runtime. Label **keys** must\ncontain 1 to 63 characters, and must conform to [RFC 1035]\n(https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be\nempty, but, if present, must contain 1 to 63 characters, and must\nconform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No\nmore than 32 labels can be associated with a cluster."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata`.\nThe Compute Engine metadata entries to add to virtual machine.\n(see [Project and instance metadata](https://cloud.google.com\n/compute/docs/storing-retrieving-metadata#project_and_instance\n_metadata))."]
    pub fn set_metadata(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe Compute Engine network to be used for machine communications.\nCannot be specified with subnetwork. If neither 'network' nor\n'subnet' is specified, the \"default\" network of the project is\nused, if it exists. A full URL or partial URI. Examples:\n  * 'https://www.googleapis.com/compute/v1/projects/[project_id]/\n  regions/global/default'\n  * 'projects/[project_id]/regions/global/default'\nRuntimes are managed resources inside Google Infrastructure.\nRuntimes support the following network configurations:\n  * Google Managed Network (Network & subnet are empty)\n  * Consumer Project VPC (network & subnet are required). Requires\n  configuring Private Service Access.\n  * Shared VPC (network & subnet are required). Requires\n  configuring Private Service Access."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `nic_type`.\nThe type of vNIC to be used on this interface. This may be gVNIC\nor VirtioNet. Possible values: [\"UNSPECIFIED_NIC_TYPE\", \"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn set_nic_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nic_type = Some(v.into());
        self
    }
    #[doc = "Set the field `reserved_ip_range`.\nReserved IP Range name is used for VPC Peering. The\nsubnetwork allocation will use the range *name* if it's assigned."]
    pub fn set_reserved_ip_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reserved_ip_range = Some(v.into());
        self
    }
    #[doc = "Set the field `subnet`.\nThe Compute Engine subnetwork to be used for machine\ncommunications. Cannot be specified with network. A full URL or\npartial URI are valid. Examples:\n  * 'https://www.googleapis.com/compute/v1/projects/[project_id]/\n  regions/us-east1/subnetworks/sub0'\n  * 'projects/[project_id]/regions/us-east1/subnetworks/sub0'"]
    pub fn set_subnet(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\nThe Compute Engine tags to add to runtime (see [Tagging instances]\n(https://cloud.google.com/compute/docs/\nlabel-or-tag-resources#tags))."]
    pub fn set_tags(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_config`.\n"]
    pub fn set_accelerator_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.accelerator_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `container_images`.\n"]
    pub fn set_container_images(
        mut self,
        v: impl Into<
            BlockAssignable<
                NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.container_images = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.container_images = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `data_disk`.\n"]
    pub fn set_data_disk(
        mut self,
        v: impl Into<BlockAssignable<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.data_disk = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.data_disk = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.shielded_instance_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NotebooksRuntimeVirtualMachineElVirtualMachineConfigEl {
    type O = BlockAssignable<NotebooksRuntimeVirtualMachineElVirtualMachineConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigEl {
    #[doc = "The Compute Engine machine type used for runtimes."]
    pub machine_type: PrimField<String>,
}
impl BuildNotebooksRuntimeVirtualMachineElVirtualMachineConfigEl {
    pub fn build(self) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigEl {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigEl {
            internal_ip_only: core::default::Default::default(),
            labels: core::default::Default::default(),
            machine_type: self.machine_type,
            metadata: core::default::Default::default(),
            network: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            reserved_ip_range: core::default::Default::default(),
            subnet: core::default::Default::default(),
            tags: core::default::Default::default(),
            accelerator_config: core::default::Default::default(),
            container_images: core::default::Default::default(),
            data_disk: core::default::Default::default(),
            encryption_config: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NotebooksRuntimeVirtualMachineElVirtualMachineConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeVirtualMachineElVirtualMachineConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> NotebooksRuntimeVirtualMachineElVirtualMachineConfigElRef {
        NotebooksRuntimeVirtualMachineElVirtualMachineConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeVirtualMachineElVirtualMachineConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `guest_attributes` after provisioning.\nThe Compute Engine guest attributes. (see [Project and instance\nguest attributes](https://cloud.google.com/compute/docs/\nstoring-retrieving-metadata#guest_attributes))."]
    pub fn guest_attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.guest_attributes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `internal_ip_only` after provisioning.\nIf true, runtime will only have internal IP addresses. By default,\nruntimes are not restricted to internal IP addresses, and will\nhave ephemeral external IP addresses assigned to each vm. This\n'internal_ip_only' restriction can only be enabled for subnetwork\nenabled networks, and all dependencies must be configured to be\naccessible without external IP addresses."]
    pub fn internal_ip_only(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.internal_ip_only", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nThe labels to associate with this runtime. Label **keys** must\ncontain 1 to 63 characters, and must conform to [RFC 1035]\n(https://www.ietf.org/rfc/rfc1035.txt). Label **values** may be\nempty, but, if present, must contain 1 to 63 characters, and must\nconform to [RFC 1035](https://www.ietf.org/rfc/rfc1035.txt). No\nmore than 32 labels can be associated with a cluster."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe Compute Engine machine type used for runtimes."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\nThe Compute Engine metadata entries to add to virtual machine.\n(see [Project and instance metadata](https://cloud.google.com\n/compute/docs/storing-retrieving-metadata#project_and_instance\n_metadata))."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe Compute Engine network to be used for machine communications.\nCannot be specified with subnetwork. If neither 'network' nor\n'subnet' is specified, the \"default\" network of the project is\nused, if it exists. A full URL or partial URI. Examples:\n  * 'https://www.googleapis.com/compute/v1/projects/[project_id]/\n  regions/global/default'\n  * 'projects/[project_id]/regions/global/default'\nRuntimes are managed resources inside Google Infrastructure.\nRuntimes support the following network configurations:\n  * Google Managed Network (Network & subnet are empty)\n  * Consumer Project VPC (network & subnet are required). Requires\n  configuring Private Service Access.\n  * Shared VPC (network & subnet are required). Requires\n  configuring Private Service Access."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `nic_type` after provisioning.\nThe type of vNIC to be used on this interface. This may be gVNIC\nor VirtioNet. Possible values: [\"UNSPECIFIED_NIC_TYPE\", \"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn nic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }
    #[doc = "Get a reference to the value of field `reserved_ip_range` after provisioning.\nReserved IP Range name is used for VPC Peering. The\nsubnetwork allocation will use the range *name* if it's assigned."]
    pub fn reserved_ip_range(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reserved_ip_range", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `subnet` after provisioning.\nThe Compute Engine subnetwork to be used for machine\ncommunications. Cannot be specified with network. A full URL or\npartial URI are valid. Examples:\n  * 'https://www.googleapis.com/compute/v1/projects/[project_id]/\n  regions/us-east1/subnetworks/sub0'\n  * 'projects/[project_id]/regions/us-east1/subnetworks/sub0'"]
    pub fn subnet(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\nThe Compute Engine tags to add to runtime (see [Tagging instances]\n(https://cloud.google.com/compute/docs/\nlabel-or-tag-resources#tags))."]
    pub fn tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe zone where the virtual machine is located."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
    #[doc = "Get a reference to the value of field `accelerator_config` after provisioning.\n"]
    pub fn accelerator_config(
        &self,
    ) -> ListRef<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElAcceleratorConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.accelerator_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_images` after provisioning.\n"]
    pub fn container_images(
        &self,
    ) -> ListRef<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElContainerImagesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.container_images", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_disk` after provisioning.\n"]
    pub fn data_disk(
        &self,
    ) -> ListRef<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElDataDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_disk", self.base))
    }
    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(
        &self,
    ) -> ListRef<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElEncryptionConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(
        &self,
    ) -> ListRef<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElShieldedInstanceConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_instance_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NotebooksRuntimeVirtualMachineElDynamic {
    virtual_machine_config:
        Option<DynamicBlock<NotebooksRuntimeVirtualMachineElVirtualMachineConfigEl>>,
}
#[derive(Serialize)]
pub struct NotebooksRuntimeVirtualMachineEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_machine_config: Option<Vec<NotebooksRuntimeVirtualMachineElVirtualMachineConfigEl>>,
    dynamic: NotebooksRuntimeVirtualMachineElDynamic,
}
impl NotebooksRuntimeVirtualMachineEl {
    #[doc = "Set the field `virtual_machine_config`.\n"]
    pub fn set_virtual_machine_config(
        mut self,
        v: impl Into<BlockAssignable<NotebooksRuntimeVirtualMachineElVirtualMachineConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.virtual_machine_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_machine_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for NotebooksRuntimeVirtualMachineEl {
    type O = BlockAssignable<NotebooksRuntimeVirtualMachineEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildNotebooksRuntimeVirtualMachineEl {}
impl BuildNotebooksRuntimeVirtualMachineEl {
    pub fn build(self) -> NotebooksRuntimeVirtualMachineEl {
        NotebooksRuntimeVirtualMachineEl {
            virtual_machine_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct NotebooksRuntimeVirtualMachineElRef {
    shared: StackShared,
    base: String,
}
impl Ref for NotebooksRuntimeVirtualMachineElRef {
    fn new(shared: StackShared, base: String) -> NotebooksRuntimeVirtualMachineElRef {
        NotebooksRuntimeVirtualMachineElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl NotebooksRuntimeVirtualMachineElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe unique identifier of the Managed Compute Engine instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_name` after provisioning.\nThe user-friendly name of the Managed Compute Engine instance."]
    pub fn instance_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_machine_config` after provisioning.\n"]
    pub fn virtual_machine_config(
        &self,
    ) -> ListRef<NotebooksRuntimeVirtualMachineElVirtualMachineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_machine_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct NotebooksRuntimeDynamic {
    access_config: Option<DynamicBlock<NotebooksRuntimeAccessConfigEl>>,
    software_config: Option<DynamicBlock<NotebooksRuntimeSoftwareConfigEl>>,
    virtual_machine: Option<DynamicBlock<NotebooksRuntimeVirtualMachineEl>>,
}
