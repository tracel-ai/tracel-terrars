use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DiscoveryEngineDataStoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_config: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_advanced_site_search: Option<PrimField<bool>>,
    data_store_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    industry_vertical: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_default_schema_creation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    solution_types: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_site_search_config: Option<Vec<DiscoveryEngineDataStoreAdvancedSiteSearchConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_processing_config: Option<Vec<DiscoveryEngineDataStoreDocumentProcessingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DiscoveryEngineDataStoreTimeoutsEl>,
    dynamic: DiscoveryEngineDataStoreDynamic,
}
struct DiscoveryEngineDataStore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DiscoveryEngineDataStoreData>,
}
#[derive(Clone)]
pub struct DiscoveryEngineDataStore(Rc<DiscoveryEngineDataStore_>);
impl DiscoveryEngineDataStore {
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
    #[doc = "Set the field `content_config`.\nThe content config of the data store. Possible values: [\"NO_CONTENT\", \"CONTENT_REQUIRED\", \"PUBLIC_WEBSITE\"]"]
    pub fn set_content_config(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_config = Some(v.into());
        self
    }
    #[doc = "Set the field `create_advanced_site_search`.\nIf true, an advanced data store for site search will be created. If the\ndata store is not configured as site search (GENERIC vertical and\nPUBLIC_WEBSITE contentConfig), this flag will be ignored."]
    pub fn set_create_advanced_site_search(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_advanced_site_search = Some(v.into());
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
    #[doc = "Set the field `kms_key_name`.\nKMS key resource name which will be used to encrypt resources:\n'/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'\nThe KMS key to be used to protect this DataStore at creation time. Must be\nset for requests that need to comply with CMEK Org Policy protections.\nIf this field is set and processed successfully, the DataStore will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `skip_default_schema_creation`.\nA boolean flag indicating whether to skip the default schema creation for\nthe data store. Only enable this flag if you are certain that the default\nschema is incompatible with your use case.\nIf set to true, you must manually create a schema for the data store\nbefore any documents can be ingested.\nThis flag cannot be specified if 'data_store.starting_schema' is\nspecified."]
    pub fn set_skip_default_schema_creation(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_default_schema_creation = Some(v.into());
        self
    }
    #[doc = "Set the field `solution_types`.\nThe solutions that the data store enrolls. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn set_solution_types(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().solution_types = Some(v.into());
        self
    }
    #[doc = "Set the field `advanced_site_search_config`.\n"]
    pub fn set_advanced_site_search_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineDataStoreAdvancedSiteSearchConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_site_search_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_site_search_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `document_processing_config`.\n"]
    pub fn set_document_processing_config(
        self,
        v: impl Into<BlockAssignable<DiscoveryEngineDataStoreDocumentProcessingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().document_processing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.document_processing_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DiscoveryEngineDataStoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `content_config` after provisioning.\nThe content config of the data store. Possible values: [\"NO_CONTENT\", \"CONTENT_REQUIRED\", \"PUBLIC_WEBSITE\"]"]
    pub fn content_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_advanced_site_search` after provisioning.\nIf true, an advanced data store for site search will be created. If the\ndata store is not configured as site search (GENERIC vertical and\nPUBLIC_WEBSITE contentConfig), this flag will be ignored."]
    pub fn create_advanced_site_search(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_advanced_site_search", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the DataStore was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_id` after provisioning.\nThe unique id of the data store."]
    pub fn data_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_schema_id` after provisioning.\nThe id of the default Schema associated with this data store."]
    pub fn default_schema_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_schema_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the data store. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
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
    #[doc = "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the data store registers. Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn industry_vertical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.industry_vertical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key resource name which will be used to encrypt resources:\n'/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'\nThe KMS key to be used to protect this DataStore at creation time. Must be\nset for requests that need to comply with CMEK Org Policy protections.\nIf this field is set and processed successfully, the DataStore will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the data store. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
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
    #[doc = "Get a reference to the value of field `skip_default_schema_creation` after provisioning.\nA boolean flag indicating whether to skip the default schema creation for\nthe data store. Only enable this flag if you are certain that the default\nschema is incompatible with your use case.\nIf set to true, you must manually create a schema for the data store\nbefore any documents can be ingested.\nThis flag cannot be specified if 'data_store.starting_schema' is\nspecified."]
    pub fn skip_default_schema_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_default_schema_creation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `solution_types` after provisioning.\nThe solutions that the data store enrolls. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn solution_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.solution_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_site_search_config` after provisioning.\n"]
    pub fn advanced_site_search_config(
        &self,
    ) -> ListRef<DiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_site_search_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `document_processing_config` after provisioning.\n"]
    pub fn document_processing_config(
        &self,
    ) -> ListRef<DiscoveryEngineDataStoreDocumentProcessingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.document_processing_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineDataStoreTimeoutsElRef {
        DiscoveryEngineDataStoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DiscoveryEngineDataStore {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DiscoveryEngineDataStore {}
impl ToListMappable for DiscoveryEngineDataStore {
    type O = ListRef<DiscoveryEngineDataStoreRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DiscoveryEngineDataStore_ {
    fn extract_resource_type(&self) -> String {
        "google_discovery_engine_data_store".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDiscoveryEngineDataStore {
    pub tf_id: String,
    #[doc = "The unique id of the data store."]
    pub data_store_id: PrimField<String>,
    #[doc = "The display name of the data store. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub display_name: PrimField<String>,
    #[doc = "The industry vertical that the data store registers. Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub industry_vertical: PrimField<String>,
    #[doc = "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: PrimField<String>,
}
impl BuildDiscoveryEngineDataStore {
    pub fn build(self, stack: &mut Stack) -> DiscoveryEngineDataStore {
        let out = DiscoveryEngineDataStore(Rc::new(DiscoveryEngineDataStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DiscoveryEngineDataStoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                content_config: core::default::Default::default(),
                create_advanced_site_search: core::default::Default::default(),
                data_store_id: self.data_store_id,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                industry_vertical: self.industry_vertical,
                kms_key_name: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                skip_default_schema_creation: core::default::Default::default(),
                solution_types: core::default::Default::default(),
                advanced_site_search_config: core::default::Default::default(),
                document_processing_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DiscoveryEngineDataStoreRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DiscoveryEngineDataStoreRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_config` after provisioning.\nThe content config of the data store. Possible values: [\"NO_CONTENT\", \"CONTENT_REQUIRED\", \"PUBLIC_WEBSITE\"]"]
    pub fn content_config(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_advanced_site_search` after provisioning.\nIf true, an advanced data store for site search will be created. If the\ndata store is not configured as site search (GENERIC vertical and\nPUBLIC_WEBSITE contentConfig), this flag will be ignored."]
    pub fn create_advanced_site_search(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_advanced_site_search", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the DataStore was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_store_id` after provisioning.\nThe unique id of the data store."]
    pub fn data_store_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_store_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `default_schema_id` after provisioning.\nThe id of the default Schema associated with this data store."]
    pub fn default_schema_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_schema_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the data store. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
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
    #[doc = "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the data store registers. Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn industry_vertical(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.industry_vertical", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key resource name which will be used to encrypt resources:\n'/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'\nThe KMS key to be used to protect this DataStore at creation time. Must be\nset for requests that need to comply with CMEK Org Policy protections.\nIf this field is set and processed successfully, the DataStore will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.kms_key_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the data store. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
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
    #[doc = "Get a reference to the value of field `skip_default_schema_creation` after provisioning.\nA boolean flag indicating whether to skip the default schema creation for\nthe data store. Only enable this flag if you are certain that the default\nschema is incompatible with your use case.\nIf set to true, you must manually create a schema for the data store\nbefore any documents can be ingested.\nThis flag cannot be specified if 'data_store.starting_schema' is\nspecified."]
    pub fn skip_default_schema_creation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.skip_default_schema_creation", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `solution_types` after provisioning.\nThe solutions that the data store enrolls. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn solution_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.solution_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `advanced_site_search_config` after provisioning.\n"]
    pub fn advanced_site_search_config(
        &self,
    ) -> ListRef<DiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_site_search_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `document_processing_config` after provisioning.\n"]
    pub fn document_processing_config(
        &self,
    ) -> ListRef<DiscoveryEngineDataStoreDocumentProcessingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.document_processing_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineDataStoreTimeoutsElRef {
        DiscoveryEngineDataStoreTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_automatic_refresh: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_initial_index: Option<PrimField<bool>>,
}
impl DiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
    #[doc = "Set the field `disable_automatic_refresh`.\nIf set true, automatic refresh is disabled for the DataStore."]
    pub fn set_disable_automatic_refresh(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_automatic_refresh = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_initial_index`.\nIf set true, initial indexing is disabled for the DataStore."]
    pub fn set_disable_initial_index(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_initial_index = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
    type O = BlockAssignable<DiscoveryEngineDataStoreAdvancedSiteSearchConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {}
impl BuildDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
    pub fn build(self) -> DiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
        DiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
            disable_automatic_refresh: core::default::Default::default(),
            disable_initial_index: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
        DiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_automatic_refresh` after provisioning.\nIf set true, automatic refresh is disabled for the DataStore."]
    pub fn disable_automatic_refresh(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_automatic_refresh", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_initial_index` after provisioning.\nIf set true, initial indexing is disabled for the DataStore."]
    pub fn disable_initial_index(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_initial_index", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_ancestor_headings: Option<PrimField<bool>>,
}
impl DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl {
    #[doc = "Set the field `chunk_size`.\nThe token size limit for each chunk.\nSupported values: 100-500 (inclusive). Default value: 500."]
    pub fn set_chunk_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.chunk_size = Some(v.into());
        self
    }
    #[doc = "Set the field `include_ancestor_headings`.\nWhether to include appending different levels of headings to chunks from the middle of the document to prevent context loss.\nDefault value: False."]
    pub fn set_include_ancestor_headings(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_ancestor_headings = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl { type O = BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl
{}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl { pub fn build (self) -> DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl { DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl { chunk_size : core :: default :: Default :: default () , include_ancestor_headings : core :: default :: Default :: default () , } } }
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef { DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef { shared : shared , base : base . to_string () , } } }
impl
    DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `chunk_size` after provisioning.\nThe token size limit for each chunk.\nSupported values: 100-500 (inclusive). Default value: 500."]
    pub fn chunk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.chunk_size", self.base))
    }
    #[doc = "Get a reference to the value of field `include_ancestor_headings` after provisioning.\nWhether to include appending different levels of headings to chunks from the middle of the document to prevent context loss.\nDefault value: False."]
    pub fn include_ancestor_headings(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_ancestor_headings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElDynamic { layout_based_chunking_config : Option < DynamicBlock < DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl >> , }
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl { # [serde (skip_serializing_if = "Option::is_none")] layout_based_chunking_config : Option < Vec < DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl > > , dynamic : DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElDynamic , }
impl DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
    #[doc = "Set the field `layout_based_chunking_config`.\n"]
    pub fn set_layout_based_chunking_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.layout_based_chunking_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.layout_based_chunking_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
    type O = BlockAssignable<DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
    pub fn build(self) -> DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
        DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
            layout_based_chunking_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
        DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `layout_based_chunking_config` after provisioning.\n"]    pub fn layout_based_chunking_config (& self) -> ListRef < DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.layout_based_chunking_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl
{}
impl
    DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl
{
}
impl ToListMappable for DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl { type O = BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl
{}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl { pub fn build (self) -> DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl { DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl { } } }
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef { DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_image_annotation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_table_annotation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_html_classes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_html_elements: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_html_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    structured_content_types: Option<ListField<PrimField<String>>>,
}
impl DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl {
    #[doc = "Set the field `enable_image_annotation`.\nIf true, the LLM based annotation is added to the image during parsing."]
    pub fn set_enable_image_annotation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_image_annotation = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_table_annotation`.\nIf true, the LLM based annotation is added to the table during parsing."]
    pub fn set_enable_table_annotation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_table_annotation = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_html_classes`.\nList of HTML classes to exclude from the parsed content."]
    pub fn set_exclude_html_classes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_html_classes = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_html_elements`.\nList of HTML elements to exclude from the parsed content."]
    pub fn set_exclude_html_elements(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_html_elements = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_html_ids`.\nList of HTML ids to exclude from the parsed content."]
    pub fn set_exclude_html_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_html_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `structured_content_types`.\nContains the required structure types to extract from the document. Supported values: 'shareholder-structure'."]
    pub fn set_structured_content_types(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.structured_content_types = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl { type O = BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl
{}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl { pub fn build (self) -> DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl { DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl { enable_image_annotation : core :: default :: Default :: default () , enable_table_annotation : core :: default :: Default :: default () , exclude_html_classes : core :: default :: Default :: default () , exclude_html_elements : core :: default :: Default :: default () , exclude_html_ids : core :: default :: Default :: default () , structured_content_types : core :: default :: Default :: default () , } } }
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef { DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl
    DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_image_annotation` after provisioning.\nIf true, the LLM based annotation is added to the image during parsing."]
    pub fn enable_image_annotation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_image_annotation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_table_annotation` after provisioning.\nIf true, the LLM based annotation is added to the table during parsing."]
    pub fn enable_table_annotation(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_table_annotation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_html_classes` after provisioning.\nList of HTML classes to exclude from the parsed content."]
    pub fn exclude_html_classes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_html_classes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_html_elements` after provisioning.\nList of HTML elements to exclude from the parsed content."]
    pub fn exclude_html_elements(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_html_elements", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclude_html_ids` after provisioning.\nList of HTML ids to exclude from the parsed content."]
    pub fn exclude_html_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.exclude_html_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `structured_content_types` after provisioning.\nContains the required structure types to extract from the document. Supported values: 'shareholder-structure'."]
    pub fn structured_content_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.structured_content_types", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    use_native_text: Option<PrimField<bool>>,
}
impl DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl {
    #[doc = "Set the field `use_native_text`.\nIf true, will use native text instead of OCR text on pages containing native text."]
    pub fn set_use_native_text(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_native_text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl
{
    type O = BlockAssignable<
        DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl
{}
impl
    BuildDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl
{
    pub fn build(
        self,
    ) -> DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl
    {
        DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl {
            use_native_text: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef { DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `use_native_text` after provisioning.\nIf true, will use native text instead of OCR text on pages containing native text."]
    pub fn use_native_text(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_native_text", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDynamic { digital_parsing_config : Option < DynamicBlock < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl >> , layout_parsing_config : Option < DynamicBlock < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl >> , ocr_parsing_config : Option < DynamicBlock < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl >> , }
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl { # [serde (skip_serializing_if = "Option::is_none")] digital_parsing_config : Option < Vec < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] layout_parsing_config : Option < Vec < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] ocr_parsing_config : Option < Vec < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl > > , dynamic : DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDynamic , }
impl DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {
    #[doc = "Set the field `digital_parsing_config`.\n"]
    pub fn set_digital_parsing_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.digital_parsing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.digital_parsing_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `layout_parsing_config`.\n"]
    pub fn set_layout_parsing_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.layout_parsing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.layout_parsing_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ocr_parsing_config`.\n"]
    pub fn set_ocr_parsing_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ocr_parsing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ocr_parsing_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {
    type O =
        BlockAssignable<DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {
    pub fn build(self) -> DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {
        DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {
            digital_parsing_config: core::default::Default::default(),
            layout_parsing_config: core::default::Default::default(),
            ocr_parsing_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
        DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `digital_parsing_config` after provisioning.\n"]    pub fn digital_parsing_config (& self) -> ListRef < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.digital_parsing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `layout_parsing_config` after provisioning.\n"]    pub fn layout_parsing_config (& self) -> ListRef < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.layout_parsing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ocr_parsing_config` after provisioning.\n"]    pub fn ocr_parsing_config (& self) -> ListRef < DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ocr_parsing_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl
{}
impl
    DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl
{
}
impl ToListMappable for DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl { type O = BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl
{}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl { pub fn build (self) -> DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl { DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl { } } }
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef { DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_image_annotation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_table_annotation: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_html_classes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_html_elements: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_html_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    structured_content_types: Option<ListField<PrimField<String>>>,
}
impl
    DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl
{
    #[doc = "Set the field `enable_image_annotation`.\nIf true, the LLM based annotation is added to the image during parsing."]
    pub fn set_enable_image_annotation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_image_annotation = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_table_annotation`.\nIf true, the LLM based annotation is added to the table during parsing."]
    pub fn set_enable_table_annotation(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_table_annotation = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_html_classes`.\nList of HTML classes to exclude from the parsed content."]
    pub fn set_exclude_html_classes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_html_classes = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_html_elements`.\nList of HTML elements to exclude from the parsed content."]
    pub fn set_exclude_html_elements(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_html_elements = Some(v.into());
        self
    }
    #[doc = "Set the field `exclude_html_ids`.\nList of HTML ids to exclude from the parsed content."]
    pub fn set_exclude_html_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclude_html_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `structured_content_types`.\nContains the required structure types to extract from the document. Supported values: 'shareholder-structure'."]
    pub fn set_structured_content_types(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.structured_content_types = Some(v.into());
        self
    }
}
impl ToListMappable for DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl { type O = BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl
{}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl { pub fn build (self) -> DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl { DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl { enable_image_annotation : core :: default :: Default :: default () , enable_table_annotation : core :: default :: Default :: default () , exclude_html_classes : core :: default :: Default :: default () , exclude_html_elements : core :: default :: Default :: default () , exclude_html_ids : core :: default :: Default :: default () , structured_content_types : core :: default :: Default :: default () , } } }
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef { DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enable_image_annotation` after provisioning.\nIf true, the LLM based annotation is added to the image during parsing."] pub fn enable_image_annotation (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_image_annotation" , self . base)) } # [doc = "Get a reference to the value of field `enable_table_annotation` after provisioning.\nIf true, the LLM based annotation is added to the table during parsing."] pub fn enable_table_annotation (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_table_annotation" , self . base)) } # [doc = "Get a reference to the value of field `exclude_html_classes` after provisioning.\nList of HTML classes to exclude from the parsed content."] pub fn exclude_html_classes (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclude_html_classes" , self . base)) } # [doc = "Get a reference to the value of field `exclude_html_elements` after provisioning.\nList of HTML elements to exclude from the parsed content."] pub fn exclude_html_elements (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclude_html_elements" , self . base)) } # [doc = "Get a reference to the value of field `exclude_html_ids` after provisioning.\nList of HTML ids to exclude from the parsed content."] pub fn exclude_html_ids (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclude_html_ids" , self . base)) } # [doc = "Get a reference to the value of field `structured_content_types` after provisioning.\nContains the required structure types to extract from the document. Supported values: 'shareholder-structure'."] pub fn structured_content_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.structured_content_types" , self . base)) } }
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    use_native_text: Option<PrimField<bool>>,
}
impl DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl {
    #[doc = "Set the field `use_native_text`.\nIf true, will use native text instead of OCR text on pages containing native text."]
    pub fn set_use_native_text(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_native_text = Some(v.into());
        self
    }
}
impl ToListMappable
    for DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl
{
    type O = BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl
{}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl { pub fn build (self) -> DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl { DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl { use_native_text : core :: default :: Default :: default () , } } }
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef { fn new (shared : StackShared , base : String) -> DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef { DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl
    DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `use_native_text` after provisioning.\nIf true, will use native text instead of OCR text on pages containing native text."]
    pub fn use_native_text(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_native_text", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDynamic { digital_parsing_config : Option < DynamicBlock < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl >> , layout_parsing_config : Option < DynamicBlock < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl >> , ocr_parsing_config : Option < DynamicBlock < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl >> , }
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl { file_type : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] digital_parsing_config : Option < Vec < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] layout_parsing_config : Option < Vec < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] ocr_parsing_config : Option < Vec < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl > > , dynamic : DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDynamic , }
impl DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
    #[doc = "Set the field `digital_parsing_config`.\n"]
    pub fn set_digital_parsing_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.digital_parsing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.digital_parsing_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `layout_parsing_config`.\n"]
    pub fn set_layout_parsing_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.layout_parsing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.layout_parsing_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ocr_parsing_config`.\n"]
    pub fn set_ocr_parsing_config(
        mut self,
        v : impl Into < BlockAssignable < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ocr_parsing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ocr_parsing_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
    type O =
        BlockAssignable<DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
    #[doc = ""]
    pub file_type: PrimField<String>,
}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
    pub fn build(
        self,
    ) -> DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
        DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
            file_type: self.file_type,
            digital_parsing_config: core::default::Default::default(),
            layout_parsing_config: core::default::Default::default(),
            ocr_parsing_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
        DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
    #[doc = "Get a reference to the value of field `digital_parsing_config` after provisioning.\n"]    pub fn digital_parsing_config (& self) -> ListRef < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.digital_parsing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `layout_parsing_config` after provisioning.\n"]    pub fn layout_parsing_config (& self) -> ListRef < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.layout_parsing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ocr_parsing_config` after provisioning.\n"]    pub fn ocr_parsing_config (& self) -> ListRef < DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ocr_parsing_config", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DiscoveryEngineDataStoreDocumentProcessingConfigElDynamic {
    chunking_config:
        Option<DynamicBlock<DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl>>,
    default_parsing_config: Option<
        DynamicBlock<DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl>,
    >,
    parsing_config_overrides: Option<
        DynamicBlock<DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl>,
    >,
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    chunking_config:
        Option<Vec<DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_parsing_config:
        Option<Vec<DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parsing_config_overrides:
        Option<Vec<DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl>>,
    dynamic: DiscoveryEngineDataStoreDocumentProcessingConfigElDynamic,
}
impl DiscoveryEngineDataStoreDocumentProcessingConfigEl {
    #[doc = "Set the field `chunking_config`.\n"]
    pub fn set_chunking_config(
        mut self,
        v: impl Into<
            BlockAssignable<DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.chunking_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.chunking_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_parsing_config`.\n"]
    pub fn set_default_parsing_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_parsing_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_parsing_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `parsing_config_overrides`.\n"]
    pub fn set_parsing_config_overrides(
        mut self,
        v: impl Into<
            BlockAssignable<
                DiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parsing_config_overrides = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parsing_config_overrides = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DiscoveryEngineDataStoreDocumentProcessingConfigEl {
    type O = BlockAssignable<DiscoveryEngineDataStoreDocumentProcessingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataStoreDocumentProcessingConfigEl {}
impl BuildDiscoveryEngineDataStoreDocumentProcessingConfigEl {
    pub fn build(self) -> DiscoveryEngineDataStoreDocumentProcessingConfigEl {
        DiscoveryEngineDataStoreDocumentProcessingConfigEl {
            chunking_config: core::default::Default::default(),
            default_parsing_config: core::default::Default::default(),
            parsing_config_overrides: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataStoreDocumentProcessingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreDocumentProcessingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DiscoveryEngineDataStoreDocumentProcessingConfigElRef {
        DiscoveryEngineDataStoreDocumentProcessingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataStoreDocumentProcessingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the Document Processing Config. Format:\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/documentProcessingConfig'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `chunking_config` after provisioning.\n"]
    pub fn chunking_config(
        &self,
    ) -> ListRef<DiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chunking_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_parsing_config` after provisioning.\n"]
    pub fn default_parsing_config(
        &self,
    ) -> ListRef<DiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_parsing_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DiscoveryEngineDataStoreTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DiscoveryEngineDataStoreTimeoutsEl {
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
impl ToListMappable for DiscoveryEngineDataStoreTimeoutsEl {
    type O = BlockAssignable<DiscoveryEngineDataStoreTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDiscoveryEngineDataStoreTimeoutsEl {}
impl BuildDiscoveryEngineDataStoreTimeoutsEl {
    pub fn build(self) -> DiscoveryEngineDataStoreTimeoutsEl {
        DiscoveryEngineDataStoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DiscoveryEngineDataStoreTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DiscoveryEngineDataStoreTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DiscoveryEngineDataStoreTimeoutsElRef {
        DiscoveryEngineDataStoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DiscoveryEngineDataStoreTimeoutsElRef {
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
struct DiscoveryEngineDataStoreDynamic {
    advanced_site_search_config:
        Option<DynamicBlock<DiscoveryEngineDataStoreAdvancedSiteSearchConfigEl>>,
    document_processing_config:
        Option<DynamicBlock<DiscoveryEngineDataStoreDocumentProcessingConfigEl>>,
}
