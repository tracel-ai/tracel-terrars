use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApihubApiHubInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_hub_instance_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<ApihubApiHubInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApihubApiHubInstanceTimeoutsEl>,
    dynamic: ApihubApiHubInstanceDynamic,
}
struct ApihubApiHubInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApihubApiHubInstanceData>,
}
#[derive(Clone)]
pub struct ApihubApiHubInstance(Rc<ApihubApiHubInstance_>);
impl ApihubApiHubInstance {
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
    #[doc = "Set the field `api_hub_instance_id`.\nOptional. Identifier to assign to the Api Hub instance. Must be unique within\nscope of the parent resource. If the field is not provided,\nsystem generated id will be used.\n\nThis value should be 4-40 characters, and valid characters\nare '/a-z[0-9]-_/'."]
    pub fn set_api_hub_instance_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_hub_instance_id = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nOptional. Description of the ApiHub instance."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nOptional. Instance labels to represent user-provided metadata.\nRefer to cloud documentation on labels for more details.\nhttps://cloud.google.com/compute/docs/labeling-resources\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl Into<BlockAssignable<ApihubApiHubInstanceConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApihubApiHubInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `api_hub_instance_id` after provisioning.\nOptional. Identifier to assign to the Api Hub instance. Must be unique within\nscope of the parent resource. If the field is not provided,\nsystem generated id will be used.\n\nThis value should be 4-40 characters, and valid characters\nare '/a-z[0-9]-_/'."]
    pub fn api_hub_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_hub_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the ApiHub instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Instance labels to represent user-provided metadata.\nRefer to cloud documentation on labels for more details.\nhttps://cloud.google.com/compute/docs/labeling-resources\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Format:\n'projects/{project}/locations/{location}/apiHubInstances/{apiHubInstance}'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the ApiHub instance.\nPossible values:\nSTATE_UNSPECIFIED\nINACTIVE\nCREATING\nACTIVE\nUPDATING\nDELETING\nFAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nOutput only. Extra information about ApiHub instance state. Currently the message\nwould be populated when state is 'FAILED'."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Last update timestamp."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ApihubApiHubInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubApiHubInstanceTimeoutsElRef {
        ApihubApiHubInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ApihubApiHubInstance {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApihubApiHubInstance {}
impl ToListMappable for ApihubApiHubInstance {
    type O = ListRef<ApihubApiHubInstanceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApihubApiHubInstance_ {
    fn extract_resource_type(&self) -> String {
        "google_apihub_api_hub_instance".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApihubApiHubInstance {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildApihubApiHubInstance {
    pub fn build(self, stack: &mut Stack) -> ApihubApiHubInstance {
        let out = ApihubApiHubInstance(Rc::new(ApihubApiHubInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApihubApiHubInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_hub_instance_id: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApihubApiHubInstanceRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubApiHubInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApihubApiHubInstanceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_hub_instance_id` after provisioning.\nOptional. Identifier to assign to the Api Hub instance. Must be unique within\nscope of the parent resource. If the field is not provided,\nsystem generated id will be used.\n\nThis value should be 4-40 characters, and valid characters\nare '/a-z[0-9]-_/'."]
    pub fn api_hub_instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.api_hub_instance_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Creation timestamp."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the ApiHub instance."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nOptional. Instance labels to represent user-provided metadata.\nRefer to cloud documentation on labels for more details.\nhttps://cloud.google.com/compute/docs/labeling-resources\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Format:\n'projects/{project}/locations/{location}/apiHubInstances/{apiHubInstance}'."]
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
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The current state of the ApiHub instance.\nPossible values:\nSTATE_UNSPECIFIED\nINACTIVE\nCREATING\nACTIVE\nUPDATING\nDELETING\nFAILED"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state_message` after provisioning.\nOutput only. Extra information about ApiHub instance state. Currently the message\nwould be populated when state is 'FAILED'."]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Last update timestamp."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ApihubApiHubInstanceConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubApiHubInstanceTimeoutsElRef {
        ApihubApiHubInstanceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubApiHubInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cmek_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_search: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertex_location: Option<PrimField<String>>,
}
impl ApihubApiHubInstanceConfigEl {
    #[doc = "Set the field `cmek_key_name`.\nOptional. The Customer Managed Encryption Key (CMEK) used for data encryption.\nThe CMEK name should follow the format of\n'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)',\nwhere the location must match the instance location.\nIf the CMEK is not provided, a GMEK will be created for the instance."]
    pub fn set_cmek_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cmek_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_search`.\nOptional. If true, the search will be disabled for the instance. The default value\nis false."]
    pub fn set_disable_search(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_search = Some(v.into());
        self
    }
    #[doc = "Set the field `encryption_type`.\nOptional. Encryption type for the region. If the encryption type is CMEK, the\ncmek_key_name must be provided. If no encryption type is provided,\nGMEK will be used.\nPossible values:\nENCRYPTION_TYPE_UNSPECIFIED\nGMEK\nCMEK"]
    pub fn set_encryption_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_type = Some(v.into());
        self
    }
    #[doc = "Set the field `vertex_location`.\nOptional. The name of the Vertex AI location where the data store is stored."]
    pub fn set_vertex_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vertex_location = Some(v.into());
        self
    }
}
impl ToListMappable for ApihubApiHubInstanceConfigEl {
    type O = BlockAssignable<ApihubApiHubInstanceConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubApiHubInstanceConfigEl {}
impl BuildApihubApiHubInstanceConfigEl {
    pub fn build(self) -> ApihubApiHubInstanceConfigEl {
        ApihubApiHubInstanceConfigEl {
            cmek_key_name: core::default::Default::default(),
            disable_search: core::default::Default::default(),
            encryption_type: core::default::Default::default(),
            vertex_location: core::default::Default::default(),
        }
    }
}
pub struct ApihubApiHubInstanceConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubApiHubInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> ApihubApiHubInstanceConfigElRef {
        ApihubApiHubInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubApiHubInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cmek_key_name` after provisioning.\nOptional. The Customer Managed Encryption Key (CMEK) used for data encryption.\nThe CMEK name should follow the format of\n'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)',\nwhere the location must match the instance location.\nIf the CMEK is not provided, a GMEK will be created for the instance."]
    pub fn cmek_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cmek_key_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_search` after provisioning.\nOptional. If true, the search will be disabled for the instance. The default value\nis false."]
    pub fn disable_search(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_search", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `encryption_type` after provisioning.\nOptional. Encryption type for the region. If the encryption type is CMEK, the\ncmek_key_name must be provided. If no encryption type is provided,\nGMEK will be used.\nPossible values:\nENCRYPTION_TYPE_UNSPECIFIED\nGMEK\nCMEK"]
    pub fn encryption_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vertex_location` after provisioning.\nOptional. The name of the Vertex AI location where the data store is stored."]
    pub fn vertex_location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vertex_location", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApihubApiHubInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ApihubApiHubInstanceTimeoutsEl {
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
impl ToListMappable for ApihubApiHubInstanceTimeoutsEl {
    type O = BlockAssignable<ApihubApiHubInstanceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApihubApiHubInstanceTimeoutsEl {}
impl BuildApihubApiHubInstanceTimeoutsEl {
    pub fn build(self) -> ApihubApiHubInstanceTimeoutsEl {
        ApihubApiHubInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ApihubApiHubInstanceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApihubApiHubInstanceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApihubApiHubInstanceTimeoutsElRef {
        ApihubApiHubInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApihubApiHubInstanceTimeoutsElRef {
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
struct ApihubApiHubInstanceDynamic {
    config: Option<DynamicBlock<ApihubApiHubInstanceConfigEl>>,
}
