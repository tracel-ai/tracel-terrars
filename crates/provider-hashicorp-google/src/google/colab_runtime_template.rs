use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ColabRuntimeTemplateData {
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
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tags: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_persistent_disk_spec: Option<Vec<ColabRuntimeTemplateDataPersistentDiskSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_spec: Option<Vec<ColabRuntimeTemplateEncryptionSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    euc_config: Option<Vec<ColabRuntimeTemplateEucConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_shutdown_config: Option<Vec<ColabRuntimeTemplateIdleShutdownConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_spec: Option<Vec<ColabRuntimeTemplateMachineSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_spec: Option<Vec<ColabRuntimeTemplateNetworkSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_vm_config: Option<Vec<ColabRuntimeTemplateShieldedVmConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    software_config: Option<Vec<ColabRuntimeTemplateSoftwareConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ColabRuntimeTemplateTimeoutsEl>,
    dynamic: ColabRuntimeTemplateDynamic,
}
struct ColabRuntimeTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ColabRuntimeTemplateData>,
}
#[derive(Clone)]
pub struct ColabRuntimeTemplate(Rc<ColabRuntimeTemplate_>);
impl ColabRuntimeTemplate {
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
    #[doc = "Set the field `description`.\nThe description of the Runtime Template."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nLabels to identify and group the runtime template.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe resource name of the Runtime Template"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }
    #[doc = "Set the field `network_tags`.\nApplies the given Compute Engine tags to the runtime."]
    pub fn set_network_tags(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().network_tags = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `data_persistent_disk_spec`.\n"]
    pub fn set_data_persistent_disk_spec(
        self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateDataPersistentDiskSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_persistent_disk_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_persistent_disk_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateEncryptionSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `euc_config`.\n"]
    pub fn set_euc_config(
        self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateEucConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().euc_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.euc_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `idle_shutdown_config`.\n"]
    pub fn set_idle_shutdown_config(
        self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateIdleShutdownConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().idle_shutdown_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.idle_shutdown_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateMachineSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().machine_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.machine_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_spec`.\n"]
    pub fn set_network_spec(
        self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateNetworkSpecEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `shielded_vm_config`.\n"]
    pub fn set_shielded_vm_config(
        self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateShieldedVmConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shielded_vm_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shielded_vm_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateSoftwareConfigEl>>,
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
    pub fn set_timeouts(self, v: impl Into<ColabRuntimeTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Runtime Template."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Runtime Template."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to identify and group the runtime template.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Runtime Template"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\nApplies the given Compute Engine tags to the runtime."]
    pub fn network_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_persistent_disk_spec` after provisioning.\n"]
    pub fn data_persistent_disk_spec(
        &self,
    ) -> ListRef<ColabRuntimeTemplateDataPersistentDiskSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_persistent_disk_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<ColabRuntimeTemplateEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `euc_config` after provisioning.\n"]
    pub fn euc_config(&self) -> ListRef<ColabRuntimeTemplateEucConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.euc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `idle_shutdown_config` after provisioning.\n"]
    pub fn idle_shutdown_config(&self) -> ListRef<ColabRuntimeTemplateIdleShutdownConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.idle_shutdown_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> ListRef<ColabRuntimeTemplateMachineSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_spec` after provisioning.\n"]
    pub fn network_spec(&self) -> ListRef<ColabRuntimeTemplateNetworkSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_vm_config` after provisioning.\n"]
    pub fn shielded_vm_config(&self) -> ListRef<ColabRuntimeTemplateShieldedVmConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_vm_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> ListRef<ColabRuntimeTemplateSoftwareConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabRuntimeTemplateTimeoutsElRef {
        ColabRuntimeTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ColabRuntimeTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ColabRuntimeTemplate {}
impl ToListMappable for ColabRuntimeTemplate {
    type O = ListRef<ColabRuntimeTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ColabRuntimeTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_colab_runtime_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildColabRuntimeTemplate {
    pub tf_id: String,
    #[doc = "Required. The display name of the Runtime Template."]
    pub display_name: PrimField<String>,
    #[doc = "The location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub location: PrimField<String>,
}
impl BuildColabRuntimeTemplate {
    pub fn build(self, stack: &mut Stack) -> ColabRuntimeTemplate {
        let out = ColabRuntimeTemplate(Rc::new(ColabRuntimeTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ColabRuntimeTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: core::default::Default::default(),
                network_tags: core::default::Default::default(),
                project: core::default::Default::default(),
                data_persistent_disk_spec: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                euc_config: core::default::Default::default(),
                idle_shutdown_config: core::default::Default::default(),
                machine_spec: core::default::Default::default(),
                network_spec: core::default::Default::default(),
                shielded_vm_config: core::default::Default::default(),
                software_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ColabRuntimeTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ColabRuntimeTemplateRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Runtime Template."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Runtime Template."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels to identify and group the runtime template.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Runtime Template"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_tags` after provisioning.\nApplies the given Compute Engine tags to the runtime."]
    pub fn network_tags(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_persistent_disk_spec` after provisioning.\n"]
    pub fn data_persistent_disk_spec(
        &self,
    ) -> ListRef<ColabRuntimeTemplateDataPersistentDiskSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_persistent_disk_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> ListRef<ColabRuntimeTemplateEncryptionSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `euc_config` after provisioning.\n"]
    pub fn euc_config(&self) -> ListRef<ColabRuntimeTemplateEucConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.euc_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `idle_shutdown_config` after provisioning.\n"]
    pub fn idle_shutdown_config(&self) -> ListRef<ColabRuntimeTemplateIdleShutdownConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.idle_shutdown_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> ListRef<ColabRuntimeTemplateMachineSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_spec` after provisioning.\n"]
    pub fn network_spec(&self) -> ListRef<ColabRuntimeTemplateNetworkSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `shielded_vm_config` after provisioning.\n"]
    pub fn shielded_vm_config(&self) -> ListRef<ColabRuntimeTemplateShieldedVmConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.shielded_vm_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> ListRef<ColabRuntimeTemplateSoftwareConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.software_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabRuntimeTemplateTimeoutsElRef {
        ColabRuntimeTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateDataPersistentDiskSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
}
impl ColabRuntimeTemplateDataPersistentDiskSpecEl {
    #[doc = "Set the field `disk_size_gb`.\nThe disk size of the runtime in GB. If specified, the diskType must also be specified. The minimum size is 10GB and the maximum is 65536GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }
    #[doc = "Set the field `disk_type`.\nThe type of the persistent disk."]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateDataPersistentDiskSpecEl {
    type O = BlockAssignable<ColabRuntimeTemplateDataPersistentDiskSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateDataPersistentDiskSpecEl {}
impl BuildColabRuntimeTemplateDataPersistentDiskSpecEl {
    pub fn build(self) -> ColabRuntimeTemplateDataPersistentDiskSpecEl {
        ColabRuntimeTemplateDataPersistentDiskSpecEl {
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateDataPersistentDiskSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateDataPersistentDiskSpecElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateDataPersistentDiskSpecElRef {
        ColabRuntimeTemplateDataPersistentDiskSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateDataPersistentDiskSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe disk size of the runtime in GB. If specified, the diskType must also be specified. The minimum size is 10GB and the maximum is 65536GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }
    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nThe type of the persistent disk."]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateEncryptionSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
}
impl ColabRuntimeTemplateEncryptionSpecEl {
    #[doc = "Set the field `kms_key_name`.\nThe Cloud KMS encryption key (customer-managed encryption key) used to protect the runtime."]
    pub fn set_kms_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_name = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateEncryptionSpecEl {
    type O = BlockAssignable<ColabRuntimeTemplateEncryptionSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateEncryptionSpecEl {}
impl BuildColabRuntimeTemplateEncryptionSpecEl {
    pub fn build(self) -> ColabRuntimeTemplateEncryptionSpecEl {
        ColabRuntimeTemplateEncryptionSpecEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateEncryptionSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateEncryptionSpecElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateEncryptionSpecElRef {
        ColabRuntimeTemplateEncryptionSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateEncryptionSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS encryption key (customer-managed encryption key) used to protect the runtime."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateEucConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    euc_disabled: Option<PrimField<bool>>,
}
impl ColabRuntimeTemplateEucConfigEl {
    #[doc = "Set the field `euc_disabled`.\nDisable end user credential access for the runtime."]
    pub fn set_euc_disabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.euc_disabled = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateEucConfigEl {
    type O = BlockAssignable<ColabRuntimeTemplateEucConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateEucConfigEl {}
impl BuildColabRuntimeTemplateEucConfigEl {
    pub fn build(self) -> ColabRuntimeTemplateEucConfigEl {
        ColabRuntimeTemplateEucConfigEl {
            euc_disabled: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateEucConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateEucConfigElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateEucConfigElRef {
        ColabRuntimeTemplateEucConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateEucConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `euc_disabled` after provisioning.\nDisable end user credential access for the runtime."]
    pub fn euc_disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.euc_disabled", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateIdleShutdownConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout: Option<PrimField<String>>,
}
impl ColabRuntimeTemplateIdleShutdownConfigEl {
    #[doc = "Set the field `idle_timeout`.\nThe duration after which the runtime is automatically shut down. An input of 0s disables the idle shutdown feature, and a valid range is [10m, 24h]."]
    pub fn set_idle_timeout(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.idle_timeout = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateIdleShutdownConfigEl {
    type O = BlockAssignable<ColabRuntimeTemplateIdleShutdownConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateIdleShutdownConfigEl {}
impl BuildColabRuntimeTemplateIdleShutdownConfigEl {
    pub fn build(self) -> ColabRuntimeTemplateIdleShutdownConfigEl {
        ColabRuntimeTemplateIdleShutdownConfigEl {
            idle_timeout: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateIdleShutdownConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateIdleShutdownConfigElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateIdleShutdownConfigElRef {
        ColabRuntimeTemplateIdleShutdownConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateIdleShutdownConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\nThe duration after which the runtime is automatically shut down. An input of 0s disables the idle shutdown feature, and a valid range is [10m, 24h]."]
    pub fn idle_timeout(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateMachineSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accelerator_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_type: Option<PrimField<String>>,
}
impl ColabRuntimeTemplateMachineSpecEl {
    #[doc = "Set the field `accelerator_count`.\nThe number of accelerators used by the runtime."]
    pub fn set_accelerator_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accelerator_count = Some(v.into());
        self
    }
    #[doc = "Set the field `accelerator_type`.\nThe type of hardware accelerator used by the runtime. If specified, acceleratorCount must also be specified."]
    pub fn set_accelerator_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.accelerator_type = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_type`.\nThe Compute Engine machine type selected for the runtime."]
    pub fn set_machine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.machine_type = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateMachineSpecEl {
    type O = BlockAssignable<ColabRuntimeTemplateMachineSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateMachineSpecEl {}
impl BuildColabRuntimeTemplateMachineSpecEl {
    pub fn build(self) -> ColabRuntimeTemplateMachineSpecEl {
        ColabRuntimeTemplateMachineSpecEl {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateMachineSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateMachineSpecElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateMachineSpecElRef {
        ColabRuntimeTemplateMachineSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateMachineSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of accelerators used by the runtime."]
    pub fn accelerator_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_count", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of hardware accelerator used by the runtime. If specified, acceleratorCount must also be specified."]
    pub fn accelerator_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accelerator_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe Compute Engine machine type selected for the runtime."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateNetworkSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_internet_access: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
}
impl ColabRuntimeTemplateNetworkSpecEl {
    #[doc = "Set the field `enable_internet_access`.\nEnable public internet access for the runtime."]
    pub fn set_enable_internet_access(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_internet_access = Some(v.into());
        self
    }
    #[doc = "Set the field `network`.\nThe name of the VPC that this runtime is in."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }
    #[doc = "Set the field `subnetwork`.\nThe name of the subnetwork that this runtime is in."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateNetworkSpecEl {
    type O = BlockAssignable<ColabRuntimeTemplateNetworkSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateNetworkSpecEl {}
impl BuildColabRuntimeTemplateNetworkSpecEl {
    pub fn build(self) -> ColabRuntimeTemplateNetworkSpecEl {
        ColabRuntimeTemplateNetworkSpecEl {
            enable_internet_access: core::default::Default::default(),
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateNetworkSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateNetworkSpecElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateNetworkSpecElRef {
        ColabRuntimeTemplateNetworkSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateNetworkSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_internet_access` after provisioning.\nEnable public internet access for the runtime."]
    pub fn enable_internet_access(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_internet_access", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC that this runtime is in."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\nThe name of the subnetwork that this runtime is in."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateShieldedVmConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
}
impl ColabRuntimeTemplateShieldedVmConfigEl {
    #[doc = "Set the field `enable_secure_boot`.\nEnables secure boot for the runtime."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateShieldedVmConfigEl {
    type O = BlockAssignable<ColabRuntimeTemplateShieldedVmConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateShieldedVmConfigEl {}
impl BuildColabRuntimeTemplateShieldedVmConfigEl {
    pub fn build(self) -> ColabRuntimeTemplateShieldedVmConfigEl {
        ColabRuntimeTemplateShieldedVmConfigEl {
            enable_secure_boot: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateShieldedVmConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateShieldedVmConfigElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateShieldedVmConfigElRef {
        ColabRuntimeTemplateShieldedVmConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateShieldedVmConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\nEnables secure boot for the runtime."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_secure_boot", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateSoftwareConfigElColabImageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    release_name: Option<PrimField<String>>,
}
impl ColabRuntimeTemplateSoftwareConfigElColabImageEl {
    #[doc = "Set the field `release_name`.\nThe release name of the NotebookRuntime Colab image, e.g. \"py310\". If not specified, detault to the latest release."]
    pub fn set_release_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.release_name = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateSoftwareConfigElColabImageEl {
    type O = BlockAssignable<ColabRuntimeTemplateSoftwareConfigElColabImageEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateSoftwareConfigElColabImageEl {}
impl BuildColabRuntimeTemplateSoftwareConfigElColabImageEl {
    pub fn build(self) -> ColabRuntimeTemplateSoftwareConfigElColabImageEl {
        ColabRuntimeTemplateSoftwareConfigElColabImageEl {
            release_name: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateSoftwareConfigElColabImageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateSoftwareConfigElColabImageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ColabRuntimeTemplateSoftwareConfigElColabImageElRef {
        ColabRuntimeTemplateSoftwareConfigElColabImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateSoftwareConfigElColabImageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `release_name` after provisioning.\nThe release name of the NotebookRuntime Colab image, e.g. \"py310\". If not specified, detault to the latest release."]
    pub fn release_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_name", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateSoftwareConfigElEnvEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl ColabRuntimeTemplateSoftwareConfigElEnvEl {
    #[doc = "Set the field `name`.\nName of the environment variable. Must be a valid C identifier."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\nVariables that reference a $(VAR_NAME) are expanded using the previous defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not."]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateSoftwareConfigElEnvEl {
    type O = BlockAssignable<ColabRuntimeTemplateSoftwareConfigElEnvEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateSoftwareConfigElEnvEl {}
impl BuildColabRuntimeTemplateSoftwareConfigElEnvEl {
    pub fn build(self) -> ColabRuntimeTemplateSoftwareConfigElEnvEl {
        ColabRuntimeTemplateSoftwareConfigElEnvEl {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateSoftwareConfigElEnvElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateSoftwareConfigElEnvElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateSoftwareConfigElEnvElRef {
        ColabRuntimeTemplateSoftwareConfigElEnvElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateSoftwareConfigElEnvElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment variable. Must be a valid C identifier."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\nVariables that reference a $(VAR_NAME) are expanded using the previous defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not."]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    post_startup_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_startup_script_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_startup_script_url: Option<PrimField<String>>,
}
impl ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl {
    #[doc = "Set the field `post_startup_script`.\nPost startup script to run after runtime is started."]
    pub fn set_post_startup_script(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.post_startup_script = Some(v.into());
        self
    }
    #[doc = "Set the field `post_startup_script_behavior`.\nPost startup script behavior that defines download and execution behavior. Possible values: [\"RUN_ONCE\", \"RUN_EVERY_START\", \"DOWNLOAD_AND_RUN_EVERY_START\"]"]
    pub fn set_post_startup_script_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.post_startup_script_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `post_startup_script_url`.\nPost startup script url to download. Example: https://bucket/script.sh."]
    pub fn set_post_startup_script_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.post_startup_script_url = Some(v.into());
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl {
    type O = BlockAssignable<ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl {}
impl BuildColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl {
    pub fn build(self) -> ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl {
        ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl {
            post_startup_script: core::default::Default::default(),
            post_startup_script_behavior: core::default::Default::default(),
            post_startup_script_url: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigElRef {
        ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `post_startup_script` after provisioning.\nPost startup script to run after runtime is started."]
    pub fn post_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_startup_script", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `post_startup_script_behavior` after provisioning.\nPost startup script behavior that defines download and execution behavior. Possible values: [\"RUN_ONCE\", \"RUN_EVERY_START\", \"DOWNLOAD_AND_RUN_EVERY_START\"]"]
    pub fn post_startup_script_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_startup_script_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `post_startup_script_url` after provisioning.\nPost startup script url to download. Example: https://bucket/script.sh."]
    pub fn post_startup_script_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_startup_script_url", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ColabRuntimeTemplateSoftwareConfigElDynamic {
    colab_image: Option<DynamicBlock<ColabRuntimeTemplateSoftwareConfigElColabImageEl>>,
    env: Option<DynamicBlock<ColabRuntimeTemplateSoftwareConfigElEnvEl>>,
    post_startup_script_config:
        Option<DynamicBlock<ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl>>,
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateSoftwareConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    colab_image: Option<Vec<ColabRuntimeTemplateSoftwareConfigElColabImageEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    env: Option<Vec<ColabRuntimeTemplateSoftwareConfigElEnvEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_startup_script_config:
        Option<Vec<ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl>>,
    dynamic: ColabRuntimeTemplateSoftwareConfigElDynamic,
}
impl ColabRuntimeTemplateSoftwareConfigEl {
    #[doc = "Set the field `colab_image`.\n"]
    pub fn set_colab_image(
        mut self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateSoftwareConfigElColabImageEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.colab_image = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.colab_image = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateSoftwareConfigElEnvEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.env = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.env = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `post_startup_script_config`.\n"]
    pub fn set_post_startup_script_config(
        mut self,
        v: impl Into<BlockAssignable<ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.post_startup_script_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.post_startup_script_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ColabRuntimeTemplateSoftwareConfigEl {
    type O = BlockAssignable<ColabRuntimeTemplateSoftwareConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateSoftwareConfigEl {}
impl BuildColabRuntimeTemplateSoftwareConfigEl {
    pub fn build(self) -> ColabRuntimeTemplateSoftwareConfigEl {
        ColabRuntimeTemplateSoftwareConfigEl {
            colab_image: core::default::Default::default(),
            env: core::default::Default::default(),
            post_startup_script_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateSoftwareConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateSoftwareConfigElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateSoftwareConfigElRef {
        ColabRuntimeTemplateSoftwareConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateSoftwareConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `colab_image` after provisioning.\n"]
    pub fn colab_image(&self) -> ListRef<ColabRuntimeTemplateSoftwareConfigElColabImageElRef> {
        ListRef::new(self.shared().clone(), format!("{}.colab_image", self.base))
    }
    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> ListRef<ColabRuntimeTemplateSoftwareConfigElEnvElRef> {
        ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }
    #[doc = "Get a reference to the value of field `post_startup_script_config` after provisioning.\n"]
    pub fn post_startup_script_config(
        &self,
    ) -> ListRef<ColabRuntimeTemplateSoftwareConfigElPostStartupScriptConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.post_startup_script_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ColabRuntimeTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ColabRuntimeTemplateTimeoutsEl {
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
impl ToListMappable for ColabRuntimeTemplateTimeoutsEl {
    type O = BlockAssignable<ColabRuntimeTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildColabRuntimeTemplateTimeoutsEl {}
impl BuildColabRuntimeTemplateTimeoutsEl {
    pub fn build(self) -> ColabRuntimeTemplateTimeoutsEl {
        ColabRuntimeTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ColabRuntimeTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ColabRuntimeTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ColabRuntimeTemplateTimeoutsElRef {
        ColabRuntimeTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ColabRuntimeTemplateTimeoutsElRef {
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
#[derive(Serialize, Default)]
struct ColabRuntimeTemplateDynamic {
    data_persistent_disk_spec: Option<DynamicBlock<ColabRuntimeTemplateDataPersistentDiskSpecEl>>,
    encryption_spec: Option<DynamicBlock<ColabRuntimeTemplateEncryptionSpecEl>>,
    euc_config: Option<DynamicBlock<ColabRuntimeTemplateEucConfigEl>>,
    idle_shutdown_config: Option<DynamicBlock<ColabRuntimeTemplateIdleShutdownConfigEl>>,
    machine_spec: Option<DynamicBlock<ColabRuntimeTemplateMachineSpecEl>>,
    network_spec: Option<DynamicBlock<ColabRuntimeTemplateNetworkSpecEl>>,
    shielded_vm_config: Option<DynamicBlock<ColabRuntimeTemplateShieldedVmConfigEl>>,
    software_config: Option<DynamicBlock<ColabRuntimeTemplateSoftwareConfigEl>>,
}
