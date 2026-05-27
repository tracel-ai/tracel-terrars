use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApphubServiceData {
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
    discovered_service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    service_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<ApphubServiceAttributesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApphubServiceTimeoutsEl>,
    dynamic: ApphubServiceDynamic,
}
struct ApphubService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApphubServiceData>,
}
#[derive(Clone)]
pub struct ApphubService(Rc<ApphubService_>);
impl ApphubService {
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
    #[doc = "Set the field `description`.\nUser-defined description of a Service."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser-defined name for the Service."]
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
    pub fn set_attributes(self, v: impl Into<BlockAssignable<ApphubServiceAttributesEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<ApphubServiceTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-defined description of a Service."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovered_service` after provisioning.\nImmutable. The resource name of the original discovered service."]
    pub fn discovered_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovered_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-defined name for the Service."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of a Service. Format:\n\"projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}\""]
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
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe Service identifier."]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_properties` after provisioning.\nProperties of an underlying cloud resource that can comprise a Service."]
    pub fn service_properties(&self) -> ListRef<ApphubServiceServicePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_reference` after provisioning.\nReference to an underlying networking resource that can comprise a Service."]
    pub fn service_reference(&self) -> ListRef<ApphubServiceServiceReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Service state. Possible values: STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (UUID) for the 'Service' in the UUID4\nformat."]
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
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<ApphubServiceAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApphubServiceTimeoutsElRef {
        ApphubServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApphubService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApphubService {}
impl ToListMappable for ApphubService {
    type O = ListRef<ApphubServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApphubService_ {
    fn extract_resource_type(&self) -> String {
        "google_apphub_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApphubService {
    pub tf_id: String,
    #[doc = "Part of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub application_id: PrimField<String>,
    #[doc = "Immutable. The resource name of the original discovered service."]
    pub discovered_service: PrimField<String>,
    #[doc = "Part of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub location: PrimField<String>,
    #[doc = "The Service identifier."]
    pub service_id: PrimField<String>,
}
impl BuildApphubService {
    pub fn build(self, stack: &mut Stack) -> ApphubService {
        let out = ApphubService(Rc::new(ApphubService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApphubServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                application_id: self.application_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                discovered_service: self.discovered_service,
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                service_id: self.service_id,
                attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApphubServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApphubServiceRef {
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nUser-defined description of a Service."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `discovered_service` after provisioning.\nImmutable. The resource name of the original discovered service."]
    pub fn discovered_service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.discovered_service", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-defined name for the Service."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of a Service. Format:\n\"projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}\""]
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
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe Service identifier."]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_properties` after provisioning.\nProperties of an underlying cloud resource that can comprise a Service."]
    pub fn service_properties(&self) -> ListRef<ApphubServiceServicePropertiesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_properties", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_reference` after provisioning.\nReference to an underlying networking resource that can comprise a Service."]
    pub fn service_reference(&self) -> ListRef<ApphubServiceServiceReferenceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_reference", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. Service state. Possible values: STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (UUID) for the 'Service' in the UUID4\nformat."]
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
    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> ListRef<ApphubServiceAttributesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.attributes", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApphubServiceTimeoutsElRef {
        ApphubServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApphubServiceServicePropertiesElExtendedMetadataElValueEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_metadata_schema: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_struct: Option<PrimField<String>>,
}
impl ApphubServiceServicePropertiesElExtendedMetadataElValueEl {
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
impl ToListMappable for ApphubServiceServicePropertiesElExtendedMetadataElValueEl {
    type O = BlockAssignable<ApphubServiceServicePropertiesElExtendedMetadataElValueEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceServicePropertiesElExtendedMetadataElValueEl {}
impl BuildApphubServiceServicePropertiesElExtendedMetadataElValueEl {
    pub fn build(self) -> ApphubServiceServicePropertiesElExtendedMetadataElValueEl {
        ApphubServiceServicePropertiesElExtendedMetadataElValueEl {
            extended_metadata_schema: core::default::Default::default(),
            metadata_struct: core::default::Default::default(),
        }
    }
}
pub struct ApphubServiceServicePropertiesElExtendedMetadataElValueElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceServicePropertiesElExtendedMetadataElValueElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApphubServiceServicePropertiesElExtendedMetadataElValueElRef {
        ApphubServiceServicePropertiesElExtendedMetadataElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceServicePropertiesElExtendedMetadataElValueElRef {
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
pub struct ApphubServiceServicePropertiesElExtendedMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<ListField<ApphubServiceServicePropertiesElExtendedMetadataElValueEl>>,
}
impl ApphubServiceServicePropertiesElExtendedMetadataEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl Into<ListField<ApphubServiceServicePropertiesElExtendedMetadataElValueEl>>,
    ) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubServiceServicePropertiesElExtendedMetadataEl {
    type O = BlockAssignable<ApphubServiceServicePropertiesElExtendedMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceServicePropertiesElExtendedMetadataEl {}
impl BuildApphubServiceServicePropertiesElExtendedMetadataEl {
    pub fn build(self) -> ApphubServiceServicePropertiesElExtendedMetadataEl {
        ApphubServiceServicePropertiesElExtendedMetadataEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct ApphubServiceServicePropertiesElExtendedMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceServicePropertiesElExtendedMetadataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApphubServiceServicePropertiesElExtendedMetadataElRef {
        ApphubServiceServicePropertiesElExtendedMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceServicePropertiesElExtendedMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> ListRef<ApphubServiceServicePropertiesElExtendedMetadataElValueElRef> {
        ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceServicePropertiesElFunctionalTypeEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl ApphubServiceServicePropertiesElFunctionalTypeEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubServiceServicePropertiesElFunctionalTypeEl {
    type O = BlockAssignable<ApphubServiceServicePropertiesElFunctionalTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceServicePropertiesElFunctionalTypeEl {}
impl BuildApphubServiceServicePropertiesElFunctionalTypeEl {
    pub fn build(self) -> ApphubServiceServicePropertiesElFunctionalTypeEl {
        ApphubServiceServicePropertiesElFunctionalTypeEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct ApphubServiceServicePropertiesElFunctionalTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceServicePropertiesElFunctionalTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApphubServiceServicePropertiesElFunctionalTypeElRef {
        ApphubServiceServicePropertiesElFunctionalTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceServicePropertiesElFunctionalTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceServicePropertiesElIdentityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    principal: Option<PrimField<String>>,
}
impl ApphubServiceServicePropertiesElIdentityEl {
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.principal = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubServiceServicePropertiesElIdentityEl {
    type O = BlockAssignable<ApphubServiceServicePropertiesElIdentityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceServicePropertiesElIdentityEl {}
impl BuildApphubServiceServicePropertiesElIdentityEl {
    pub fn build(self) -> ApphubServiceServicePropertiesElIdentityEl {
        ApphubServiceServicePropertiesElIdentityEl {
            principal: core::default::Default::default(),
        }
    }
}
pub struct ApphubServiceServicePropertiesElIdentityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceServicePropertiesElIdentityElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceServicePropertiesElIdentityElRef {
        ApphubServiceServicePropertiesElIdentityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceServicePropertiesElIdentityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceServicePropertiesElRegistrationTypeEl {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl ApphubServiceServicePropertiesElRegistrationTypeEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubServiceServicePropertiesElRegistrationTypeEl {
    type O = BlockAssignable<ApphubServiceServicePropertiesElRegistrationTypeEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceServicePropertiesElRegistrationTypeEl {}
impl BuildApphubServiceServicePropertiesElRegistrationTypeEl {
    pub fn build(self) -> ApphubServiceServicePropertiesElRegistrationTypeEl {
        ApphubServiceServicePropertiesElRegistrationTypeEl {
            type_: core::default::Default::default(),
        }
    }
}
pub struct ApphubServiceServicePropertiesElRegistrationTypeElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceServicePropertiesElRegistrationTypeElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApphubServiceServicePropertiesElRegistrationTypeElRef {
        ApphubServiceServicePropertiesElRegistrationTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceServicePropertiesElRegistrationTypeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceServicePropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    extended_metadata: Option<ListField<ApphubServiceServicePropertiesElExtendedMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    functional_type: Option<ListField<ApphubServiceServicePropertiesElFunctionalTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gcp_project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity: Option<ListField<ApphubServiceServicePropertiesElIdentityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registration_type: Option<ListField<ApphubServiceServicePropertiesElRegistrationTypeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}
impl ApphubServiceServicePropertiesEl {
    #[doc = "Set the field `extended_metadata`.\n"]
    pub fn set_extended_metadata(
        mut self,
        v: impl Into<ListField<ApphubServiceServicePropertiesElExtendedMetadataEl>>,
    ) -> Self {
        self.extended_metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `functional_type`.\n"]
    pub fn set_functional_type(
        mut self,
        v: impl Into<ListField<ApphubServiceServicePropertiesElFunctionalTypeEl>>,
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
        v: impl Into<ListField<ApphubServiceServicePropertiesElIdentityEl>>,
    ) -> Self {
        self.identity = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `registration_type`.\n"]
    pub fn set_registration_type(
        mut self,
        v: impl Into<ListField<ApphubServiceServicePropertiesElRegistrationTypeEl>>,
    ) -> Self {
        self.registration_type = Some(v.into());
        self
    }
    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.zone = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubServiceServicePropertiesEl {
    type O = BlockAssignable<ApphubServiceServicePropertiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceServicePropertiesEl {}
impl BuildApphubServiceServicePropertiesEl {
    pub fn build(self) -> ApphubServiceServicePropertiesEl {
        ApphubServiceServicePropertiesEl {
            extended_metadata: core::default::Default::default(),
            functional_type: core::default::Default::default(),
            gcp_project: core::default::Default::default(),
            identity: core::default::Default::default(),
            location: core::default::Default::default(),
            registration_type: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}
pub struct ApphubServiceServicePropertiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceServicePropertiesElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceServicePropertiesElRef {
        ApphubServiceServicePropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceServicePropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `extended_metadata` after provisioning.\n"]
    pub fn extended_metadata(
        &self,
    ) -> ListRef<ApphubServiceServicePropertiesElExtendedMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.extended_metadata", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `functional_type` after provisioning.\n"]
    pub fn functional_type(&self) -> ListRef<ApphubServiceServicePropertiesElFunctionalTypeElRef> {
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
    pub fn identity(&self) -> ListRef<ApphubServiceServicePropertiesElIdentityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.identity", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `registration_type` after provisioning.\n"]
    pub fn registration_type(
        &self,
    ) -> ListRef<ApphubServiceServicePropertiesElRegistrationTypeElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.registration_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceServiceReferenceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<PrimField<String>>,
}
impl ApphubServiceServiceReferenceEl {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uri = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubServiceServiceReferenceEl {
    type O = BlockAssignable<ApphubServiceServiceReferenceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceServiceReferenceEl {}
impl BuildApphubServiceServiceReferenceEl {
    pub fn build(self) -> ApphubServiceServiceReferenceEl {
        ApphubServiceServiceReferenceEl {
            uri: core::default::Default::default(),
        }
    }
}
pub struct ApphubServiceServiceReferenceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceServiceReferenceElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceServiceReferenceElRef {
        ApphubServiceServiceReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceServiceReferenceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceAttributesElBusinessOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    email: PrimField<String>,
}
impl ApphubServiceAttributesElBusinessOwnersEl {
    #[doc = "Set the field `display_name`.\nContact's name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubServiceAttributesElBusinessOwnersEl {
    type O = BlockAssignable<ApphubServiceAttributesElBusinessOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceAttributesElBusinessOwnersEl {
    #[doc = "Required. Email address of the contacts."]
    pub email: PrimField<String>,
}
impl BuildApphubServiceAttributesElBusinessOwnersEl {
    pub fn build(self) -> ApphubServiceAttributesElBusinessOwnersEl {
        ApphubServiceAttributesElBusinessOwnersEl {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}
pub struct ApphubServiceAttributesElBusinessOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceAttributesElBusinessOwnersElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceAttributesElBusinessOwnersElRef {
        ApphubServiceAttributesElBusinessOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceAttributesElBusinessOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nContact's name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contacts."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceAttributesElCriticalityEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ApphubServiceAttributesElCriticalityEl {}
impl ToListMappable for ApphubServiceAttributesElCriticalityEl {
    type O = BlockAssignable<ApphubServiceAttributesElCriticalityEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceAttributesElCriticalityEl {
    #[doc = "Criticality type. Possible values: [\"MISSION_CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub type_: PrimField<String>,
}
impl BuildApphubServiceAttributesElCriticalityEl {
    pub fn build(self) -> ApphubServiceAttributesElCriticalityEl {
        ApphubServiceAttributesElCriticalityEl { type_: self.type_ }
    }
}
pub struct ApphubServiceAttributesElCriticalityElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceAttributesElCriticalityElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceAttributesElCriticalityElRef {
        ApphubServiceAttributesElCriticalityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceAttributesElCriticalityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nCriticality type. Possible values: [\"MISSION_CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceAttributesElDeveloperOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    email: PrimField<String>,
}
impl ApphubServiceAttributesElDeveloperOwnersEl {
    #[doc = "Set the field `display_name`.\nContact's name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubServiceAttributesElDeveloperOwnersEl {
    type O = BlockAssignable<ApphubServiceAttributesElDeveloperOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceAttributesElDeveloperOwnersEl {
    #[doc = "Required. Email address of the contacts."]
    pub email: PrimField<String>,
}
impl BuildApphubServiceAttributesElDeveloperOwnersEl {
    pub fn build(self) -> ApphubServiceAttributesElDeveloperOwnersEl {
        ApphubServiceAttributesElDeveloperOwnersEl {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}
pub struct ApphubServiceAttributesElDeveloperOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceAttributesElDeveloperOwnersElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceAttributesElDeveloperOwnersElRef {
        ApphubServiceAttributesElDeveloperOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceAttributesElDeveloperOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nContact's name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contacts."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceAttributesElEnvironmentEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}
impl ApphubServiceAttributesElEnvironmentEl {}
impl ToListMappable for ApphubServiceAttributesElEnvironmentEl {
    type O = BlockAssignable<ApphubServiceAttributesElEnvironmentEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceAttributesElEnvironmentEl {
    #[doc = "Environment type. Possible values: [\"PRODUCTION\", \"STAGING\", \"TEST\", \"DEVELOPMENT\"]"]
    pub type_: PrimField<String>,
}
impl BuildApphubServiceAttributesElEnvironmentEl {
    pub fn build(self) -> ApphubServiceAttributesElEnvironmentEl {
        ApphubServiceAttributesElEnvironmentEl { type_: self.type_ }
    }
}
pub struct ApphubServiceAttributesElEnvironmentElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceAttributesElEnvironmentElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceAttributesElEnvironmentElRef {
        ApphubServiceAttributesElEnvironmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceAttributesElEnvironmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\nEnvironment type. Possible values: [\"PRODUCTION\", \"STAGING\", \"TEST\", \"DEVELOPMENT\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct ApphubServiceAttributesElOperatorOwnersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    email: PrimField<String>,
}
impl ApphubServiceAttributesElOperatorOwnersEl {
    #[doc = "Set the field `display_name`.\nContact's name."]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
}
impl ToListMappable for ApphubServiceAttributesElOperatorOwnersEl {
    type O = BlockAssignable<ApphubServiceAttributesElOperatorOwnersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceAttributesElOperatorOwnersEl {
    #[doc = "Required. Email address of the contacts."]
    pub email: PrimField<String>,
}
impl BuildApphubServiceAttributesElOperatorOwnersEl {
    pub fn build(self) -> ApphubServiceAttributesElOperatorOwnersEl {
        ApphubServiceAttributesElOperatorOwnersEl {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}
pub struct ApphubServiceAttributesElOperatorOwnersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceAttributesElOperatorOwnersElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceAttributesElOperatorOwnersElRef {
        ApphubServiceAttributesElOperatorOwnersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceAttributesElOperatorOwnersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nContact's name."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contacts."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApphubServiceAttributesElDynamic {
    business_owners: Option<DynamicBlock<ApphubServiceAttributesElBusinessOwnersEl>>,
    criticality: Option<DynamicBlock<ApphubServiceAttributesElCriticalityEl>>,
    developer_owners: Option<DynamicBlock<ApphubServiceAttributesElDeveloperOwnersEl>>,
    environment: Option<DynamicBlock<ApphubServiceAttributesElEnvironmentEl>>,
    operator_owners: Option<DynamicBlock<ApphubServiceAttributesElOperatorOwnersEl>>,
}
#[derive(Serialize)]
pub struct ApphubServiceAttributesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    business_owners: Option<Vec<ApphubServiceAttributesElBusinessOwnersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    criticality: Option<Vec<ApphubServiceAttributesElCriticalityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    developer_owners: Option<Vec<ApphubServiceAttributesElDeveloperOwnersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<ApphubServiceAttributesElEnvironmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_owners: Option<Vec<ApphubServiceAttributesElOperatorOwnersEl>>,
    dynamic: ApphubServiceAttributesElDynamic,
}
impl ApphubServiceAttributesEl {
    #[doc = "Set the field `business_owners`.\n"]
    pub fn set_business_owners(
        mut self,
        v: impl Into<BlockAssignable<ApphubServiceAttributesElBusinessOwnersEl>>,
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
        v: impl Into<BlockAssignable<ApphubServiceAttributesElCriticalityEl>>,
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
        v: impl Into<BlockAssignable<ApphubServiceAttributesElDeveloperOwnersEl>>,
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
        v: impl Into<BlockAssignable<ApphubServiceAttributesElEnvironmentEl>>,
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
        v: impl Into<BlockAssignable<ApphubServiceAttributesElOperatorOwnersEl>>,
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
impl ToListMappable for ApphubServiceAttributesEl {
    type O = BlockAssignable<ApphubServiceAttributesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceAttributesEl {}
impl BuildApphubServiceAttributesEl {
    pub fn build(self) -> ApphubServiceAttributesEl {
        ApphubServiceAttributesEl {
            business_owners: core::default::Default::default(),
            criticality: core::default::Default::default(),
            developer_owners: core::default::Default::default(),
            environment: core::default::Default::default(),
            operator_owners: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApphubServiceAttributesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceAttributesElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceAttributesElRef {
        ApphubServiceAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceAttributesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `business_owners` after provisioning.\n"]
    pub fn business_owners(&self) -> ListRef<ApphubServiceAttributesElBusinessOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.business_owners", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `criticality` after provisioning.\n"]
    pub fn criticality(&self) -> ListRef<ApphubServiceAttributesElCriticalityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.criticality", self.base))
    }
    #[doc = "Get a reference to the value of field `developer_owners` after provisioning.\n"]
    pub fn developer_owners(&self) -> ListRef<ApphubServiceAttributesElDeveloperOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.developer_owners", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> ListRef<ApphubServiceAttributesElEnvironmentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `operator_owners` after provisioning.\n"]
    pub fn operator_owners(&self) -> ListRef<ApphubServiceAttributesElOperatorOwnersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operator_owners", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApphubServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApphubServiceTimeoutsEl {
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
impl ToListMappable for ApphubServiceTimeoutsEl {
    type O = BlockAssignable<ApphubServiceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApphubServiceTimeoutsEl {}
impl BuildApphubServiceTimeoutsEl {
    pub fn build(self) -> ApphubServiceTimeoutsEl {
        ApphubServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApphubServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApphubServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApphubServiceTimeoutsElRef {
        ApphubServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApphubServiceTimeoutsElRef {
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
struct ApphubServiceDynamic {
    attributes: Option<DynamicBlock<ApphubServiceAttributesEl>>,
}
