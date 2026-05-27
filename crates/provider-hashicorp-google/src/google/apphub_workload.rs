use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApphubWorkloadData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    discovered_workload: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    workload_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<ApphubWorkloadAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApphubWorkloadTimeoutsEl>,
    dynamic: ApphubWorkloadDynamic,
}
struct ApphubWorkload_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApphubWorkloadData>,
}
#[derive(Clone)]
pub struct ApphubWorkload(Rc<ApphubWorkload_>);
impl ApphubWorkload {
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
    #[doc = "Set the field `description`.\nUser-defined description of a Workload."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser-defined name for the Workload."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(self, v: impl Into<BlockAssignable<ApphubWorkloadAttributesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attributes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attributes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApphubWorkloadTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nPart of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-defined description of a Workload."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovered_workload` after provisioning.\nImmutable. The resource name of the original discovered workload."]
    pub fn discovered_workload(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovered_workload", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-defined name for the Workload."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the Workload. Format:\"projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}\""]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Workload state. Possible values:  STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (UUID) for the 'Workload' in the UUID4 format."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_id` after provisioning.\nThe Workload identifier."]
    pub fn workload_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_properties` after provisioning.\nProperties of an underlying compute resource represented by the Workload."]
    pub fn workload_properties(&self) -> ListRef<ApphubWorkloadWorkloadPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_reference` after provisioning.\nReference of an underlying compute resource represented by the Workload."]
    pub fn workload_reference(&self) -> ListRef<ApphubWorkloadWorkloadReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<ApphubWorkloadAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApphubWorkloadTimeoutsElRef {
        ApphubWorkloadTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApphubWorkload {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApphubWorkload {}
impl ToListMappable for ApphubWorkload {
    type O = ListRef<ApphubWorkloadRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApphubWorkload_ {
    fn extract_resource_type(&self) -> String {
        "google_apphub_workload".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApphubWorkload {
    pub tf_id: String,
    #[doc = "Part of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub application_id: PrimField<String>,
    #[doc = "Immutable. The resource name of the original discovered workload."]
    pub discovered_workload: PrimField<String>,
    #[doc = "Part of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub location: PrimField<String>,
    #[doc = "The Workload identifier."]
    pub workload_id: PrimField<String>,
}
impl BuildApphubWorkload {
    pub fn build(self, stack: &mut Stack) -> ApphubWorkload {
        let out = ApphubWorkload(Rc::new(ApphubWorkload_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApphubWorkloadData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_id: self.application_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                discovered_workload: self.discovered_workload,
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                workload_id: self.workload_id,
                attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApphubWorkloadRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApphubWorkloadRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nPart of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-defined description of a Workload."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovered_workload` after provisioning.\nImmutable. The resource name of the original discovered workload."]
    pub fn discovered_workload(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovered_workload", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-defined name for the Workload."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the Workload. Format:\"projects/{host-project-id}/locations/{location}/applications/{application-id}/workloads/{workload-id}\""]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Workload state. Possible values:  STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (UUID) for the 'Workload' in the UUID4 format."]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_id` after provisioning.\nThe Workload identifier."]
    pub fn workload_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_properties` after provisioning.\nProperties of an underlying compute resource represented by the Workload."]
    pub fn workload_properties(&self) -> ListRef<ApphubWorkloadWorkloadPropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `workload_reference` after provisioning.\nReference of an underlying compute resource represented by the Workload."]
    pub fn workload_reference(&self) -> ListRef<ApphubWorkloadWorkloadReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.workload_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<ApphubWorkloadAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApphubWorkloadTimeoutsElRef {
        ApphubWorkloadTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_metadata_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_struct: Option<PrimField<String>>,
}
impl ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl {
    #[doc = "Set the field `extended_metadata_schema`.\n"]
    pub fn set_extended_metadata_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.extended_metadata_schema = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_struct`.\n"]
    pub fn set_metadata_struct(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metadata_struct = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl {
    type O = BlockAssignable<ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl {}
impl BuildApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl {
    pub fn build(self) -> ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl {
        ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl {
            extended_metadata_schema: core::default::Default::default(),
            metadata_struct: core::default::Default::default(),
        }
    }
}
pub struct ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueElRef {
        ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `extended_metadata_schema` after provisioning.\n"]
    pub fn extended_metadata_schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.extended_metadata_schema", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_struct` after provisioning.\n"]
    pub fn metadata_struct(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metadata_struct", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadWorkloadPropertiesElExtendedMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListField<ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl>>,
}
impl ApphubWorkloadWorkloadPropertiesElExtendedMetadataEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl Into<ListField<ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueEl>>,
    ) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubWorkloadWorkloadPropertiesElExtendedMetadataEl {
    type O = BlockAssignable<ApphubWorkloadWorkloadPropertiesElExtendedMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadWorkloadPropertiesElExtendedMetadataEl {}
impl BuildApphubWorkloadWorkloadPropertiesElExtendedMetadataEl {
    pub fn build(self) -> ApphubWorkloadWorkloadPropertiesElExtendedMetadataEl {
        ApphubWorkloadWorkloadPropertiesElExtendedMetadataEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApphubWorkloadWorkloadPropertiesElExtendedMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadWorkloadPropertiesElExtendedMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApphubWorkloadWorkloadPropertiesElExtendedMetadataElRef {
        ApphubWorkloadWorkloadPropertiesElExtendedMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadWorkloadPropertiesElExtendedMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> ListRef<ApphubWorkloadWorkloadPropertiesElExtendedMetadataElValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadWorkloadPropertiesElFunctionalTypeEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl ApphubWorkloadWorkloadPropertiesElFunctionalTypeEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubWorkloadWorkloadPropertiesElFunctionalTypeEl {
    type O = BlockAssignable<ApphubWorkloadWorkloadPropertiesElFunctionalTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadWorkloadPropertiesElFunctionalTypeEl {}
impl BuildApphubWorkloadWorkloadPropertiesElFunctionalTypeEl {
    pub fn build(self) -> ApphubWorkloadWorkloadPropertiesElFunctionalTypeEl {
        ApphubWorkloadWorkloadPropertiesElFunctionalTypeEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct ApphubWorkloadWorkloadPropertiesElFunctionalTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadWorkloadPropertiesElFunctionalTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApphubWorkloadWorkloadPropertiesElFunctionalTypeElRef {
        ApphubWorkloadWorkloadPropertiesElFunctionalTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadWorkloadPropertiesElFunctionalTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadWorkloadPropertiesElIdentityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<PrimField<String>>,
}
impl ApphubWorkloadWorkloadPropertiesElIdentityEl {
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubWorkloadWorkloadPropertiesElIdentityEl {
    type O = BlockAssignable<ApphubWorkloadWorkloadPropertiesElIdentityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadWorkloadPropertiesElIdentityEl {}
impl BuildApphubWorkloadWorkloadPropertiesElIdentityEl {
    pub fn build(self) -> ApphubWorkloadWorkloadPropertiesElIdentityEl {
        ApphubWorkloadWorkloadPropertiesElIdentityEl {
            principal: core::default::Default::default(),
        }
    }
}
pub struct ApphubWorkloadWorkloadPropertiesElIdentityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadWorkloadPropertiesElIdentityElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadWorkloadPropertiesElIdentityElRef {
        ApphubWorkloadWorkloadPropertiesElIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadWorkloadPropertiesElIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadWorkloadPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_metadata: Option<ListField<ApphubWorkloadWorkloadPropertiesElExtendedMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    functional_type: Option<ListField<ApphubWorkloadWorkloadPropertiesElFunctionalTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity: Option<ListField<ApphubWorkloadWorkloadPropertiesElIdentityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl ApphubWorkloadWorkloadPropertiesEl {
    #[doc = "Set the field `extended_metadata`.\n"]
    pub fn set_extended_metadata(
        mut self,
        v: impl Into<ListField<ApphubWorkloadWorkloadPropertiesElExtendedMetadataEl>>,
    ) -> Self {
        self.extended_metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `functional_type`.\n"]
    pub fn set_functional_type(
        mut self,
        v: impl Into<ListField<ApphubWorkloadWorkloadPropertiesElFunctionalTypeEl>>,
    ) -> Self {
        self.functional_type = Some(v.into());
        self
    }
    #[doc = "Set the field `gcp_project`.\n"]
    pub fn set_gcp_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gcp_project = Some(v.into());
        self
    }
    #[doc = "Set the field `identity`.\n"]
    pub fn set_identity(
        mut self,
        v: impl Into<ListField<ApphubWorkloadWorkloadPropertiesElIdentityEl>>,
    ) -> Self {
        self.identity = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubWorkloadWorkloadPropertiesEl {
    type O = BlockAssignable<ApphubWorkloadWorkloadPropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadWorkloadPropertiesEl {}
impl BuildApphubWorkloadWorkloadPropertiesEl {
    pub fn build(self) -> ApphubWorkloadWorkloadPropertiesEl {
        ApphubWorkloadWorkloadPropertiesEl {
            extended_metadata: core::default::Default::default(),
            functional_type: core::default::Default::default(),
            gcp_project: core::default::Default::default(),
            identity: core::default::Default::default(),
            location: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct ApphubWorkloadWorkloadPropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadWorkloadPropertiesElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadWorkloadPropertiesElRef {
        ApphubWorkloadWorkloadPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadWorkloadPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `extended_metadata` after provisioning.\n"]
    pub fn extended_metadata(
        &self,
    ) -> ListRef<ApphubWorkloadWorkloadPropertiesElExtendedMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extended_metadata", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `functional_type` after provisioning.\n"]
    pub fn functional_type(
        &self,
    ) -> ListRef<ApphubWorkloadWorkloadPropertiesElFunctionalTypeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.functional_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `gcp_project` after provisioning.\n"]
    pub fn gcp_project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gcp_project", self.base))
    }
    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> ListRef<ApphubWorkloadWorkloadPropertiesElIdentityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadWorkloadReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl ApphubWorkloadWorkloadReferenceEl {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubWorkloadWorkloadReferenceEl {
    type O = BlockAssignable<ApphubWorkloadWorkloadReferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadWorkloadReferenceEl {}
impl BuildApphubWorkloadWorkloadReferenceEl {
    pub fn build(self) -> ApphubWorkloadWorkloadReferenceEl {
        ApphubWorkloadWorkloadReferenceEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct ApphubWorkloadWorkloadReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadWorkloadReferenceElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadWorkloadReferenceElRef {
        ApphubWorkloadWorkloadReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadWorkloadReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadAttributesElBusinessOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    email: PrimField<String>,
}
impl ApphubWorkloadAttributesElBusinessOwnersEl {
    #[doc = "Set the field `display_name`.\nContact's name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubWorkloadAttributesElBusinessOwnersEl {
    type O = BlockAssignable<ApphubWorkloadAttributesElBusinessOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadAttributesElBusinessOwnersEl {
    #[doc = "Email address of the contacts."]
    pub email: PrimField<String>,
}
impl BuildApphubWorkloadAttributesElBusinessOwnersEl {
    pub fn build(self) -> ApphubWorkloadAttributesElBusinessOwnersEl {
        ApphubWorkloadAttributesElBusinessOwnersEl {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}
pub struct ApphubWorkloadAttributesElBusinessOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadAttributesElBusinessOwnersElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadAttributesElBusinessOwnersElRef {
        ApphubWorkloadAttributesElBusinessOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadAttributesElBusinessOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nContact's name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nEmail address of the contacts."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadAttributesElCriticalityEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ApphubWorkloadAttributesElCriticalityEl {}
impl ToListMappable for ApphubWorkloadAttributesElCriticalityEl {
    type O = BlockAssignable<ApphubWorkloadAttributesElCriticalityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadAttributesElCriticalityEl {
    #[doc = "Criticality type. Possible values: [\"MISSION_CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub type_: PrimField<String>,
}
impl BuildApphubWorkloadAttributesElCriticalityEl {
    pub fn build(self) -> ApphubWorkloadAttributesElCriticalityEl {
        ApphubWorkloadAttributesElCriticalityEl { type_: self.type_ }
    }
}
pub struct ApphubWorkloadAttributesElCriticalityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadAttributesElCriticalityElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadAttributesElCriticalityElRef {
        ApphubWorkloadAttributesElCriticalityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadAttributesElCriticalityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nCriticality type. Possible values: [\"MISSION_CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadAttributesElDeveloperOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    email: PrimField<String>,
}
impl ApphubWorkloadAttributesElDeveloperOwnersEl {
    #[doc = "Set the field `display_name`.\nContact's name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubWorkloadAttributesElDeveloperOwnersEl {
    type O = BlockAssignable<ApphubWorkloadAttributesElDeveloperOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadAttributesElDeveloperOwnersEl {
    #[doc = "Email address of the contacts."]
    pub email: PrimField<String>,
}
impl BuildApphubWorkloadAttributesElDeveloperOwnersEl {
    pub fn build(self) -> ApphubWorkloadAttributesElDeveloperOwnersEl {
        ApphubWorkloadAttributesElDeveloperOwnersEl {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}
pub struct ApphubWorkloadAttributesElDeveloperOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadAttributesElDeveloperOwnersElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadAttributesElDeveloperOwnersElRef {
        ApphubWorkloadAttributesElDeveloperOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadAttributesElDeveloperOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nContact's name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nEmail address of the contacts."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadAttributesElEnvironmentEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ApphubWorkloadAttributesElEnvironmentEl {}
impl ToListMappable for ApphubWorkloadAttributesElEnvironmentEl {
    type O = BlockAssignable<ApphubWorkloadAttributesElEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadAttributesElEnvironmentEl {
    #[doc = "Environment type. Possible values: [\"PRODUCTION\", \"STAGING\", \"TEST\", \"DEVELOPMENT\"]"]
    pub type_: PrimField<String>,
}
impl BuildApphubWorkloadAttributesElEnvironmentEl {
    pub fn build(self) -> ApphubWorkloadAttributesElEnvironmentEl {
        ApphubWorkloadAttributesElEnvironmentEl { type_: self.type_ }
    }
}
pub struct ApphubWorkloadAttributesElEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadAttributesElEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadAttributesElEnvironmentElRef {
        ApphubWorkloadAttributesElEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadAttributesElEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nEnvironment type. Possible values: [\"PRODUCTION\", \"STAGING\", \"TEST\", \"DEVELOPMENT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadAttributesElOperatorOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    email: PrimField<String>,
}
impl ApphubWorkloadAttributesElOperatorOwnersEl {
    #[doc = "Set the field `display_name`.\nContact's name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubWorkloadAttributesElOperatorOwnersEl {
    type O = BlockAssignable<ApphubWorkloadAttributesElOperatorOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadAttributesElOperatorOwnersEl {
    #[doc = "Email address of the contacts."]
    pub email: PrimField<String>,
}
impl BuildApphubWorkloadAttributesElOperatorOwnersEl {
    pub fn build(self) -> ApphubWorkloadAttributesElOperatorOwnersEl {
        ApphubWorkloadAttributesElOperatorOwnersEl {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}
pub struct ApphubWorkloadAttributesElOperatorOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadAttributesElOperatorOwnersElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadAttributesElOperatorOwnersElRef {
        ApphubWorkloadAttributesElOperatorOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadAttributesElOperatorOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nContact's name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nEmail address of the contacts."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApphubWorkloadAttributesElDynamic {
    business_owners: Option<DynamicBlock<ApphubWorkloadAttributesElBusinessOwnersEl>>,
    criticality: Option<DynamicBlock<ApphubWorkloadAttributesElCriticalityEl>>,
    developer_owners: Option<DynamicBlock<ApphubWorkloadAttributesElDeveloperOwnersEl>>,
    environment: Option<DynamicBlock<ApphubWorkloadAttributesElEnvironmentEl>>,
    operator_owners: Option<DynamicBlock<ApphubWorkloadAttributesElOperatorOwnersEl>>,
}
#[derive(Serialize)]
pub struct ApphubWorkloadAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    business_owners: Option<Vec<ApphubWorkloadAttributesElBusinessOwnersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    criticality: Option<Vec<ApphubWorkloadAttributesElCriticalityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_owners: Option<Vec<ApphubWorkloadAttributesElDeveloperOwnersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<ApphubWorkloadAttributesElEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_owners: Option<Vec<ApphubWorkloadAttributesElOperatorOwnersEl>>,
    dynamic: ApphubWorkloadAttributesElDynamic,
}
impl ApphubWorkloadAttributesEl {
    #[doc = "Set the field `business_owners`.\n"]
    pub fn set_business_owners(
        mut self,
        v: impl Into<BlockAssignable<ApphubWorkloadAttributesElBusinessOwnersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.business_owners = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.business_owners = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(
        mut self,
        v: impl Into<BlockAssignable<ApphubWorkloadAttributesElCriticalityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.criticality = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.criticality = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `developer_owners`.\n"]
    pub fn set_developer_owners(
        mut self,
        v: impl Into<BlockAssignable<ApphubWorkloadAttributesElDeveloperOwnersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.developer_owners = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.developer_owners = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl Into<BlockAssignable<ApphubWorkloadAttributesElEnvironmentEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.environment = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `operator_owners`.\n"]
    pub fn set_operator_owners(
        mut self,
        v: impl Into<BlockAssignable<ApphubWorkloadAttributesElOperatorOwnersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operator_owners = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operator_owners = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApphubWorkloadAttributesEl {
    type O = BlockAssignable<ApphubWorkloadAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadAttributesEl {}
impl BuildApphubWorkloadAttributesEl {
    pub fn build(self) -> ApphubWorkloadAttributesEl {
        ApphubWorkloadAttributesEl {
            business_owners: core::default::Default::default(),
            criticality: core::default::Default::default(),
            developer_owners: core::default::Default::default(),
            environment: core::default::Default::default(),
            operator_owners: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApphubWorkloadAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadAttributesElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadAttributesElRef {
        ApphubWorkloadAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `business_owners` after provisioning.\n"]
    pub fn business_owners(&self) -> ListRef<ApphubWorkloadAttributesElBusinessOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.business_owners", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `criticality` after provisioning.\n"]
    pub fn criticality(&self) -> ListRef<ApphubWorkloadAttributesElCriticalityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.criticality", self.base))
    }
    #[doc = "Get a reference to the value of field `developer_owners` after provisioning.\n"]
    pub fn developer_owners(&self) -> ListRef<ApphubWorkloadAttributesElDeveloperOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_owners", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<ApphubWorkloadAttributesElEnvironmentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `operator_owners` after provisioning.\n"]
    pub fn operator_owners(&self) -> ListRef<ApphubWorkloadAttributesElOperatorOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operator_owners", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApphubWorkloadTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApphubWorkloadTimeoutsEl {
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
impl ToListMappable for ApphubWorkloadTimeoutsEl {
    type O = BlockAssignable<ApphubWorkloadTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubWorkloadTimeoutsEl {}
impl BuildApphubWorkloadTimeoutsEl {
    pub fn build(self) -> ApphubWorkloadTimeoutsEl {
        ApphubWorkloadTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApphubWorkloadTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubWorkloadTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApphubWorkloadTimeoutsElRef {
        ApphubWorkloadTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubWorkloadTimeoutsElRef {
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
struct ApphubWorkloadDynamic {
    attributes: Option<DynamicBlock<ApphubWorkloadAttributesEl>>,
}
