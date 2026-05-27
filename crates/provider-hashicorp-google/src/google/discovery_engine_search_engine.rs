use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineSearchEngineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    app_type: Option<PrimField<String>>,
    collection_id: PrimField<String>,
    data_store_ids: ListField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_analytics: Option<PrimField<bool>>,
    display_name: PrimField<String>,
    engine_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    industry_vertical: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_config: Option<Vec<DiscoveryEngineSearchEngineCommonConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    knowledge_graph_config: Option<Vec<DiscoveryEngineSearchEngineKnowledgeGraphConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_engine_config: Option<Vec<DiscoveryEngineSearchEngineSearchEngineConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineSearchEngineTimeoutsEl>,
    dynamic: DiscoveryEngineSearchEngineDynamic,
}
struct DiscoveryEngineSearchEngine_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineSearchEngineData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineSearchEngine(Rc<DiscoveryEngineSearchEngine_>);
impl DiscoveryEngineSearchEngine {
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
    #[doc = "Set the field `app_type`.\nThis is the application type this engine resource represents.\nThe supported values: 'APP_TYPE_UNSPECIFIED', 'APP_TYPE_INTRANET'."]
    pub fn set_app_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().app_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_analytics`.\nWhether to disable analytics for searches performed on this engine."]
    pub fn set_disable_analytics(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_analytics = Some(v.into());
        self
    }
    #[doc = "Set the field `features`.\nA map of the feature config for the engine to opt in or opt out of features."]
    pub fn set_features(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().features = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `industry_vertical`.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn set_industry_vertical(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().industry_vertical = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_key_name`.\nThe KMS key to be used to protect this Engine at creation time.\n\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\n\nIf this field is set and processed successfully, the Engine will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `common_config`.\n"]
    pub fn set_common_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineSearchEngineCommonConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().common_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.common_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `knowledge_graph_config`.\n"]
    pub fn set_knowledge_graph_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineSearchEngineKnowledgeGraphConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().knowledge_graph_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.knowledge_graph_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `search_engine_config`.\n"]
    pub fn set_search_engine_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineSearchEngineSearchEngineConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().search_engine_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.search_engine_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineSearchEngineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `app_type` after provisioning.\nThis is the application type this engine resource represents.\nThe supported values: 'APP_TYPE_UNSPECIFIED', 'APP_TYPE_INTRANET'."]
    pub fn app_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. For SOLUTION_TYPE_SEARCH type of engines, they can only associate with at most one data store."]
    pub fn data_store_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_analytics` after provisioning.\nWhether to disable analytics for searches performed on this engine."]
    pub fn disable_analytics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_analytics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nUnique ID to use for Search Engine App."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `features` after provisioning.\nA map of the feature config for the engine to opt in or opt out of features."]
    pub fn features(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn industry_vertical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.industry_vertical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe KMS key to be used to protect this Engine at creation time.\n\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\n\nIf this field is set and processed successfully, the Engine will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the search engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp the Engine was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> ListRef<DiscoveryEngineSearchEngineCommonConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.common_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_graph_config` after provisioning.\n"]
    pub fn knowledge_graph_config(
        &self,
    ) -> ListRef<DiscoveryEngineSearchEngineKnowledgeGraphConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_graph_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `search_engine_config` after provisioning.\n"]
    pub fn search_engine_config(
        &self,
    ) -> ListRef<DiscoveryEngineSearchEngineSearchEngineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.search_engine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineSearchEngineTimeoutsElRef {
        DiscoveryEngineSearchEngineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineSearchEngine {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineSearchEngine {}
impl ToListMappable for DiscoveryEngineSearchEngine {
    type O = ListRef<DiscoveryEngineSearchEngineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineSearchEngine_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_search_engine".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineSearchEngine {
    pub tf_id: String,
    #[doc = "The collection ID."]
    pub collection_id: PrimField<String>,
    #[doc = "The data stores associated with this engine. For SOLUTION_TYPE_SEARCH type of engines, they can only associate with at most one data store."]
    pub data_store_ids: ListField<PrimField<String>>,
    #[doc = "Required. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub display_name: PrimField<String>,
    #[doc = "Unique ID to use for Search Engine App."]
    pub engine_id: PrimField<String>,
    #[doc = "Location."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineSearchEngine {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineSearchEngine {
        let out = DiscoveryEngineSearchEngine(Rc::new(DiscoveryEngineSearchEngine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineSearchEngineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                app_type: core::default::Default::default(),
                collection_id: self.collection_id,
                data_store_ids: self.data_store_ids,
                deletion_policy: core::default::Default::default(),
                disable_analytics: core::default::Default::default(),
                display_name: self.display_name,
                engine_id: self.engine_id,
                features: core::default::Default::default(),
                id: core::default::Default::default(),
                industry_vertical: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                common_config: core::default::Default::default(),
                knowledge_graph_config: core::default::Default::default(),
                search_engine_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineSearchEngineRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineSearchEngineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineSearchEngineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `app_type` after provisioning.\nThis is the application type this engine resource represents.\nThe supported values: 'APP_TYPE_UNSPECIFIED', 'APP_TYPE_INTRANET'."]
    pub fn app_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.app_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.collection_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp the Engine was created at."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_ids` after provisioning.\nThe data stores associated with this engine. For SOLUTION_TYPE_SEARCH type of engines, they can only associate with at most one data store."]
    pub fn data_store_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_store_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disable_analytics` after provisioning.\nWhether to disable analytics for searches performed on this engine."]
    pub fn disable_analytics(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_analytics", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the engine. Should be human readable. UTF-8 encoded string with limit of 1024 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nUnique ID to use for Search Engine App."]
    pub fn engine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `features` after provisioning.\nA map of the feature config for the engine to opt in or opt out of features."]
    pub fn features(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.features", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the engine registers. The restriction of the Engine industry vertical is based on DataStore: If unspecified, default to GENERIC. Vertical on Engine has to match vertical of the DataStore liniked to the engine. Default value: \"GENERIC\" Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn industry_vertical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.industry_vertical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nThe KMS key to be used to protect this Engine at creation time.\n\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\n\nIf this field is set and processed successfully, the Engine will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the search engine. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
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
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTimestamp the Engine was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `common_config` after provisioning.\n"]
    pub fn common_config(&self) -> ListRef<DiscoveryEngineSearchEngineCommonConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.common_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `knowledge_graph_config` after provisioning.\n"]
    pub fn knowledge_graph_config(
        &self,
    ) -> ListRef<DiscoveryEngineSearchEngineKnowledgeGraphConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.knowledge_graph_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `search_engine_config` after provisioning.\n"]
    pub fn search_engine_config(
        &self,
    ) -> ListRef<DiscoveryEngineSearchEngineSearchEngineConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.search_engine_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineSearchEngineTimeoutsElRef {
        DiscoveryEngineSearchEngineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngineCommonConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    company_name: Option<PrimField<String>>,
}
impl DiscoveryEngineSearchEngineCommonConfigEl {
    #[doc = "Set the field `company_name`.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features.cd"]
    pub fn set_company_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.company_name = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineSearchEngineCommonConfigEl {
    type O = BlockAssignable<DiscoveryEngineSearchEngineCommonConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineSearchEngineCommonConfigEl {}
impl BuildDiscoveryEngineSearchEngineCommonConfigEl {
    pub fn build(self) -> DiscoveryEngineSearchEngineCommonConfigEl {
        DiscoveryEngineSearchEngineCommonConfigEl {
            company_name: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineSearchEngineCommonConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineSearchEngineCommonConfigElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineSearchEngineCommonConfigElRef {
        DiscoveryEngineSearchEngineCommonConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineSearchEngineCommonConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `company_name` after provisioning.\nThe name of the company, business or entity that is associated with the engine. Setting this may help improve LLM related features.cd"]
    pub fn company_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.company_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_private_kg_auto_complete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_private_kg_enrichment: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_private_kg_query_ui_chips: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_private_kg_query_understanding: Option<PrimField<bool>>,
}
impl DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl {
    #[doc = "Set the field `disable_private_kg_auto_complete`.\nWhether to disable the private KG auto complete for the engine."]
    pub fn set_disable_private_kg_auto_complete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_private_kg_auto_complete = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_private_kg_enrichment`.\nWhether to disable the private KG enrichment for the engine."]
    pub fn set_disable_private_kg_enrichment(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_private_kg_enrichment = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_private_kg_query_ui_chips`.\nWhether to disable the private KG for query UI chips."]
    pub fn set_disable_private_kg_query_ui_chips(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_private_kg_query_ui_chips = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_private_kg_query_understanding`.\nWhether to disable the private KG query understanding for the engine."]
    pub fn set_disable_private_kg_query_understanding(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.disable_private_kg_query_understanding = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl {
    type O = BlockAssignable<DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl {}
impl BuildDiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl {
    pub fn build(self) -> DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl {
        DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl {
            disable_private_kg_auto_complete: core::default::Default::default(),
            disable_private_kg_enrichment: core::default::Default::default(),
            disable_private_kg_query_ui_chips: core::default::Default::default(),
            disable_private_kg_query_understanding: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigElRef {
        DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_private_kg_auto_complete` after provisioning.\nWhether to disable the private KG auto complete for the engine."]
    pub fn disable_private_kg_auto_complete(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_private_kg_auto_complete", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_private_kg_enrichment` after provisioning.\nWhether to disable the private KG enrichment for the engine."]
    pub fn disable_private_kg_enrichment(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_private_kg_enrichment", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_private_kg_query_ui_chips` after provisioning.\nWhether to disable the private KG for query UI chips."]
    pub fn disable_private_kg_query_ui_chips(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_private_kg_query_ui_chips", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_private_kg_query_understanding` after provisioning.\nWhether to disable the private KG query understanding for the engine."]
    pub fn disable_private_kg_query_understanding(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_private_kg_query_understanding", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineSearchEngineKnowledgeGraphConfigElDynamic {
    feature_config:
        Option<DynamicBlock<DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl>>,
}
#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngineKnowledgeGraphConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_knowledge_graph_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_cloud_knowledge_graph: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_knowledge_graph: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feature_config: Option<Vec<DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl>>,
    dynamic: DiscoveryEngineSearchEngineKnowledgeGraphConfigElDynamic,
}
impl DiscoveryEngineSearchEngineKnowledgeGraphConfigEl {
    #[doc = "Set the field `cloud_knowledge_graph_types`.\nSpecify entity types to support."]
    pub fn set_cloud_knowledge_graph_types(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.cloud_knowledge_graph_types = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_cloud_knowledge_graph`.\nWhether to enable the Cloud Knowledge Graph for the engine."]
    pub fn set_enable_cloud_knowledge_graph(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_cloud_knowledge_graph = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_private_knowledge_graph`.\nWhether to enable the Private Knowledge Graph for the engine."]
    pub fn set_enable_private_knowledge_graph(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_knowledge_graph = Some(v.into());
        self
    }
    #[doc = "Set the field `feature_config`.\n"]
    pub fn set_feature_config(
        mut self,
        v: impl Into<BlockAssignable<DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.feature_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.feature_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineSearchEngineKnowledgeGraphConfigEl {
    type O = BlockAssignable<DiscoveryEngineSearchEngineKnowledgeGraphConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineSearchEngineKnowledgeGraphConfigEl {}
impl BuildDiscoveryEngineSearchEngineKnowledgeGraphConfigEl {
    pub fn build(self) -> DiscoveryEngineSearchEngineKnowledgeGraphConfigEl {
        DiscoveryEngineSearchEngineKnowledgeGraphConfigEl {
            cloud_knowledge_graph_types: core::default::Default::default(),
            enable_cloud_knowledge_graph: core::default::Default::default(),
            enable_private_knowledge_graph: core::default::Default::default(),
            feature_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineSearchEngineKnowledgeGraphConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineSearchEngineKnowledgeGraphConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineSearchEngineKnowledgeGraphConfigElRef {
        DiscoveryEngineSearchEngineKnowledgeGraphConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineSearchEngineKnowledgeGraphConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cloud_knowledge_graph_types` after provisioning.\nSpecify entity types to support."]
    pub fn cloud_knowledge_graph_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cloud_knowledge_graph_types", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cloud_knowledge_graph` after provisioning.\nWhether to enable the Cloud Knowledge Graph for the engine."]
    pub fn enable_cloud_knowledge_graph(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cloud_knowledge_graph", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_private_knowledge_graph` after provisioning.\nWhether to enable the Private Knowledge Graph for the engine."]
    pub fn enable_private_knowledge_graph(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_knowledge_graph", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `feature_config` after provisioning.\n"]
    pub fn feature_config(
        &self,
    ) -> ListRef<DiscoveryEngineSearchEngineKnowledgeGraphConfigElFeatureConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.feature_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngineSearchEngineConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    required_subscription_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_add_ons: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_tier: Option<PrimField<String>>,
}
impl DiscoveryEngineSearchEngineSearchEngineConfigEl {
    #[doc = "Set the field `required_subscription_tier`.\nThe required subscription tier of this engine.\n\nThey cannot be modified after engine creation. If the required subscription tier is search, user with higher license tier like assist can still access the standalone app associated with this engine. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_ENTERPRISE_EMERGING\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\", \"SUBSCRIPTION_TIER_EDU_EMERGING\", \"SUBSCRIPTION_TIER_EDU_PRO_EMERGING\", \"SUBSCRIPTION_TIER_FRONTLINE_STARTER\"]"]
    pub fn set_required_subscription_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.required_subscription_tier = Some(v.into());
        self
    }
    #[doc = "Set the field `search_add_ons`.\nThe add-on that this search engine enables. Possible values: [\"SEARCH_ADD_ON_LLM\"]"]
    pub fn set_search_add_ons(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.search_add_ons = Some(v.into());
        self
    }
    #[doc = "Set the field `search_tier`.\nThe search feature tier of this engine. Defaults to SearchTier.SEARCH_TIER_STANDARD if not specified. Default value: \"SEARCH_TIER_STANDARD\" Possible values: [\"SEARCH_TIER_STANDARD\", \"SEARCH_TIER_ENTERPRISE\"]"]
    pub fn set_search_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.search_tier = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineSearchEngineSearchEngineConfigEl {
    type O = BlockAssignable<DiscoveryEngineSearchEngineSearchEngineConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineSearchEngineSearchEngineConfigEl {}
impl BuildDiscoveryEngineSearchEngineSearchEngineConfigEl {
    pub fn build(self) -> DiscoveryEngineSearchEngineSearchEngineConfigEl {
        DiscoveryEngineSearchEngineSearchEngineConfigEl {
            required_subscription_tier: core::default::Default::default(),
            search_add_ons: core::default::Default::default(),
            search_tier: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineSearchEngineSearchEngineConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineSearchEngineSearchEngineConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineSearchEngineSearchEngineConfigElRef {
        DiscoveryEngineSearchEngineSearchEngineConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineSearchEngineSearchEngineConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `required_subscription_tier` after provisioning.\nThe required subscription tier of this engine.\n\nThey cannot be modified after engine creation. If the required subscription tier is search, user with higher license tier like assist can still access the standalone app associated with this engine. Possible values: [\"SUBSCRIPTION_TIER_UNSPECIFIED\", \"SUBSCRIPTION_TIER_SEARCH\", \"SUBSCRIPTION_TIER_SEARCH_AND_ASSISTANT\", \"SUBSCRIPTION_TIER_FRONTLINE_WORKER\", \"SUBSCRIPTION_TIER_AGENTSPACE_STARTER\", \"SUBSCRIPTION_TIER_AGENTSPACE_BUSINESS\", \"SUBSCRIPTION_TIER_ENTERPRISE\", \"SUBSCRIPTION_TIER_ENTERPRISE_EMERGING\", \"SUBSCRIPTION_TIER_EDU\", \"SUBSCRIPTION_TIER_EDU_PRO\", \"SUBSCRIPTION_TIER_EDU_EMERGING\", \"SUBSCRIPTION_TIER_EDU_PRO_EMERGING\", \"SUBSCRIPTION_TIER_FRONTLINE_STARTER\"]"]
    pub fn required_subscription_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.required_subscription_tier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `search_add_ons` after provisioning.\nThe add-on that this search engine enables. Possible values: [\"SEARCH_ADD_ON_LLM\"]"]
    pub fn search_add_ons(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.search_add_ons", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `search_tier` after provisioning.\nThe search feature tier of this engine. Defaults to SearchTier.SEARCH_TIER_STANDARD if not specified. Default value: \"SEARCH_TIER_STANDARD\" Possible values: [\"SEARCH_TIER_STANDARD\", \"SEARCH_TIER_ENTERPRISE\"]"]
    pub fn search_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.search_tier", self.base))
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineSearchEngineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineSearchEngineTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineSearchEngineTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineSearchEngineTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineSearchEngineTimeoutsEl {}
impl BuildDiscoveryEngineSearchEngineTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineSearchEngineTimeoutsEl {
        DiscoveryEngineSearchEngineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineSearchEngineTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineSearchEngineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineSearchEngineTimeoutsElRef {
        DiscoveryEngineSearchEngineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineSearchEngineTimeoutsElRef {
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
struct DiscoveryEngineSearchEngineDynamic {
    common_config: Option<DynamicBlock<DiscoveryEngineSearchEngineCommonConfigEl>>,
    knowledge_graph_config: Option<DynamicBlock<DiscoveryEngineSearchEngineKnowledgeGraphConfigEl>>,
    search_engine_config: Option<DynamicBlock<DiscoveryEngineSearchEngineSearchEngineConfigEl>>,
}
