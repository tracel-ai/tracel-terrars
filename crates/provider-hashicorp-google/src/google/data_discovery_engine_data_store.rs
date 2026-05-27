use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataDiscoveryEngineDataStoreData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_store_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataDiscoveryEngineDataStore_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDiscoveryEngineDataStoreData>,
}
#[derive(Clone)]
pub struct DataDiscoveryEngineDataStore(Rc<DataDiscoveryEngineDataStore_>);
impl DataDiscoveryEngineDataStore {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `data_store_id`.\nThe unique id of the data store."]
    pub fn set_data_store_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_store_id = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nThe display name of the data store. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `advanced_site_search_config` after provisioning.\nConfiguration data for advance site search."]
    pub fn advanced_site_search_config(
        &self,
    ) -> ListRef<DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_site_search_config", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `document_processing_config` after provisioning.\nConfiguration for Document understanding and enrichment."]
    pub fn document_processing_config(
        &self,
    ) -> ListRef<DataDiscoveryEngineDataStoreDocumentProcessingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.document_processing_config", self.extract_ref()),
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
}
impl Referable for DataDiscoveryEngineDataStore {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataDiscoveryEngineDataStore {}
impl ToListMappable for DataDiscoveryEngineDataStore {
    type O = ListRef<DataDiscoveryEngineDataStoreRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataDiscoveryEngineDataStore_ {
    fn extract_datasource_type(&self) -> String {
        "google_discovery_engine_data_store".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataDiscoveryEngineDataStore {
    pub tf_id: String,
}
impl BuildDataDiscoveryEngineDataStore {
    pub fn build(self, stack: &mut Stack) -> DataDiscoveryEngineDataStore {
        let out = DataDiscoveryEngineDataStore(Rc::new(DataDiscoveryEngineDataStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDiscoveryEngineDataStoreData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                data_store_id: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataDiscoveryEngineDataStoreRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataDiscoveryEngineDataStoreRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `advanced_site_search_config` after provisioning.\nConfiguration data for advance site search."]
    pub fn advanced_site_search_config(
        &self,
    ) -> ListRef<DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_site_search_config", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `document_processing_config` after provisioning.\nConfiguration for Document understanding and enrichment."]
    pub fn document_processing_config(
        &self,
    ) -> ListRef<DataDiscoveryEngineDataStoreDocumentProcessingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.document_processing_config", self.extract_ref()),
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
}
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_automatic_refresh: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_initial_index: Option<PrimField<bool>>,
}
impl DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
    #[doc = "Set the field `disable_automatic_refresh`.\n"]
    pub fn set_disable_automatic_refresh(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_automatic_refresh = Some(v.into());
        self
    }
    #[doc = "Set the field `disable_initial_index`.\n"]
    pub fn set_disable_initial_index(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.disable_initial_index = Some(v.into());
        self
    }
}
impl ToListMappable for DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
    type O = BlockAssignable<DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {}
impl BuildDataDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
    pub fn build(self) -> DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
        DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigEl {
            disable_automatic_refresh: core::default::Default::default(),
            disable_initial_index: core::default::Default::default(),
        }
    }
}
pub struct DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
        DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDiscoveryEngineDataStoreAdvancedSiteSearchConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `disable_automatic_refresh` after provisioning.\n"]
    pub fn disable_automatic_refresh(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_automatic_refresh", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `disable_initial_index` after provisioning.\n"]
    pub fn disable_initial_index(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_initial_index", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    chunk_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_ancestor_headings: Option<PrimField<bool>>,
}
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl { # [doc = "Set the field `chunk_size`.\n"] pub fn set_chunk_size (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . chunk_size = Some (v . into ()) ; self } # [doc = "Set the field `include_ancestor_headings`.\n"] pub fn set_include_ancestor_headings (mut self , v : impl Into < PrimField < bool > >) -> Self { self . include_ancestor_headings = Some (v . into ()) ; self } }
impl ToListMappable for DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl { type O = BlockAssignable < DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl
{}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl { pub fn build (self) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl { DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl { chunk_size : core :: default :: Default :: default () , include_ancestor_headings : core :: default :: Default :: default () , } } }
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef { fn new (shared : StackShared , base : String) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef { DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `chunk_size` after provisioning.\n"] pub fn chunk_size (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.chunk_size" , self . base)) } # [doc = "Get a reference to the value of field `include_ancestor_headings` after provisioning.\n"] pub fn include_ancestor_headings (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.include_ancestor_headings" , self . base)) } }
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl { # [serde (skip_serializing_if = "Option::is_none")] layout_based_chunking_config : Option < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl > > , }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
    #[doc = "Set the field `layout_based_chunking_config`.\n"]
    pub fn set_layout_based_chunking_config(
        mut self,
        v : impl Into < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigEl > >,
    ) -> Self {
        self.layout_based_chunking_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
    type O =
        BlockAssignable<DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
    pub fn build(self) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
        DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl {
            layout_based_chunking_config: core::default::Default::default(),
        }
    }
}
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
        DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `layout_based_chunking_config` after provisioning.\n"]    pub fn layout_based_chunking_config (& self) -> ListRef < DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElLayoutBasedChunkingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.layout_based_chunking_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl
{}
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl { }
impl ToListMappable for DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl { type O = BlockAssignable < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl
{}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl { pub fn build (self) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl { DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl { } } }
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef { fn new (shared : StackShared , base : String) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef { DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl
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
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl { # [doc = "Set the field `enable_image_annotation`.\n"] pub fn set_enable_image_annotation (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_image_annotation = Some (v . into ()) ; self } # [doc = "Set the field `enable_table_annotation`.\n"] pub fn set_enable_table_annotation (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_table_annotation = Some (v . into ()) ; self } # [doc = "Set the field `exclude_html_classes`.\n"] pub fn set_exclude_html_classes (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . exclude_html_classes = Some (v . into ()) ; self } # [doc = "Set the field `exclude_html_elements`.\n"] pub fn set_exclude_html_elements (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . exclude_html_elements = Some (v . into ()) ; self } # [doc = "Set the field `exclude_html_ids`.\n"] pub fn set_exclude_html_ids (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . exclude_html_ids = Some (v . into ()) ; self } # [doc = "Set the field `structured_content_types`.\n"] pub fn set_structured_content_types (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . structured_content_types = Some (v . into ()) ; self } }
impl ToListMappable for DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl { type O = BlockAssignable < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl
{}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl { pub fn build (self) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl { DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl { enable_image_annotation : core :: default :: Default :: default () , enable_table_annotation : core :: default :: Default :: default () , exclude_html_classes : core :: default :: Default :: default () , exclude_html_elements : core :: default :: Default :: default () , exclude_html_ids : core :: default :: Default :: default () , structured_content_types : core :: default :: Default :: default () , } } }
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef { fn new (shared : StackShared , base : String) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef { DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enable_image_annotation` after provisioning.\n"] pub fn enable_image_annotation (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_image_annotation" , self . base)) } # [doc = "Get a reference to the value of field `enable_table_annotation` after provisioning.\n"] pub fn enable_table_annotation (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_table_annotation" , self . base)) } # [doc = "Get a reference to the value of field `exclude_html_classes` after provisioning.\n"] pub fn exclude_html_classes (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclude_html_classes" , self . base)) } # [doc = "Get a reference to the value of field `exclude_html_elements` after provisioning.\n"] pub fn exclude_html_elements (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclude_html_elements" , self . base)) } # [doc = "Get a reference to the value of field `exclude_html_ids` after provisioning.\n"] pub fn exclude_html_ids (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclude_html_ids" , self . base)) } # [doc = "Get a reference to the value of field `structured_content_types` after provisioning.\n"] pub fn structured_content_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.structured_content_types" , self . base)) } }
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    use_native_text: Option<PrimField<bool>>,
}
impl
    DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl
{
    #[doc = "Set the field `use_native_text`.\n"]
    pub fn set_use_native_text(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_native_text = Some(v.into());
        self
    }
}
impl ToListMappable for DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl { type O = BlockAssignable < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl
{}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl { pub fn build (self) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl { DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl { use_native_text : core :: default :: Default :: default () , } } }
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef { fn new (shared : StackShared , base : String) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef { DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `use_native_text` after provisioning.\n"] pub fn use_native_text (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.use_native_text" , self . base)) } }
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl { # [serde (skip_serializing_if = "Option::is_none")] digital_parsing_config : Option < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] layout_parsing_config : Option < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] ocr_parsing_config : Option < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl > > , }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {
    #[doc = "Set the field `digital_parsing_config`.\n"]
    pub fn set_digital_parsing_config(
        mut self,
        v : impl Into < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigEl > >,
    ) -> Self {
        self.digital_parsing_config = Some(v.into());
        self
    }
    #[doc = "Set the field `layout_parsing_config`.\n"]
    pub fn set_layout_parsing_config(
        mut self,
        v : impl Into < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigEl > >,
    ) -> Self {
        self.layout_parsing_config = Some(v.into());
        self
    }
    #[doc = "Set the field `ocr_parsing_config`.\n"]
    pub fn set_ocr_parsing_config(
        mut self,
        v : impl Into < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigEl > >,
    ) -> Self {
        self.ocr_parsing_config = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl
{
    type O = BlockAssignable<
        DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {
    pub fn build(
        self,
    ) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {
        DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl {
            digital_parsing_config: core::default::Default::default(),
            layout_parsing_config: core::default::Default::default(),
            ocr_parsing_config: core::default::Default::default(),
        }
    }
}
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
        DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `digital_parsing_config` after provisioning.\n"]    pub fn digital_parsing_config (& self) -> ListRef < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElDigitalParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.digital_parsing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `layout_parsing_config` after provisioning.\n"]    pub fn layout_parsing_config (& self) -> ListRef < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElLayoutParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.layout_parsing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ocr_parsing_config` after provisioning.\n"]    pub fn ocr_parsing_config (& self) -> ListRef < DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElOcrParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ocr_parsing_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl
{}
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl { }
impl ToListMappable for DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl { type O = BlockAssignable < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl
{}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl { pub fn build (self) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl { DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl { } } }
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef { fn new (shared : StackShared , base : String) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef { DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } }
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl
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
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl { # [doc = "Set the field `enable_image_annotation`.\n"] pub fn set_enable_image_annotation (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_image_annotation = Some (v . into ()) ; self } # [doc = "Set the field `enable_table_annotation`.\n"] pub fn set_enable_table_annotation (mut self , v : impl Into < PrimField < bool > >) -> Self { self . enable_table_annotation = Some (v . into ()) ; self } # [doc = "Set the field `exclude_html_classes`.\n"] pub fn set_exclude_html_classes (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . exclude_html_classes = Some (v . into ()) ; self } # [doc = "Set the field `exclude_html_elements`.\n"] pub fn set_exclude_html_elements (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . exclude_html_elements = Some (v . into ()) ; self } # [doc = "Set the field `exclude_html_ids`.\n"] pub fn set_exclude_html_ids (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . exclude_html_ids = Some (v . into ()) ; self } # [doc = "Set the field `structured_content_types`.\n"] pub fn set_structured_content_types (mut self , v : impl Into < ListField < PrimField < String > > >) -> Self { self . structured_content_types = Some (v . into ()) ; self } }
impl ToListMappable for DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl { type O = BlockAssignable < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl
{}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl { pub fn build (self) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl { DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl { enable_image_annotation : core :: default :: Default :: default () , enable_table_annotation : core :: default :: Default :: default () , exclude_html_classes : core :: default :: Default :: default () , exclude_html_elements : core :: default :: Default :: default () , exclude_html_ids : core :: default :: Default :: default () , structured_content_types : core :: default :: Default :: default () , } } }
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef { fn new (shared : StackShared , base : String) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef { DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `enable_image_annotation` after provisioning.\n"] pub fn enable_image_annotation (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_image_annotation" , self . base)) } # [doc = "Get a reference to the value of field `enable_table_annotation` after provisioning.\n"] pub fn enable_table_annotation (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.enable_table_annotation" , self . base)) } # [doc = "Get a reference to the value of field `exclude_html_classes` after provisioning.\n"] pub fn exclude_html_classes (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclude_html_classes" , self . base)) } # [doc = "Get a reference to the value of field `exclude_html_elements` after provisioning.\n"] pub fn exclude_html_elements (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclude_html_elements" , self . base)) } # [doc = "Get a reference to the value of field `exclude_html_ids` after provisioning.\n"] pub fn exclude_html_ids (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.exclude_html_ids" , self . base)) } # [doc = "Get a reference to the value of field `structured_content_types` after provisioning.\n"] pub fn structured_content_types (& self) -> ListRef < PrimExpr < String > > { ListRef :: new (self . shared () . clone () , format ! ("{}.structured_content_types" , self . base)) } }
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    use_native_text: Option<PrimField<bool>>,
}
impl
    DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl
{
    #[doc = "Set the field `use_native_text`.\n"]
    pub fn set_use_native_text(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_native_text = Some(v.into());
        self
    }
}
impl ToListMappable for DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl { type O = BlockAssignable < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl
{}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl { pub fn build (self) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl { DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl { use_native_text : core :: default :: Default :: default () , } } }
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef { fn new (shared : StackShared , base : String) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef { DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef { shared : shared , base : base . to_string () , } } }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `use_native_text` after provisioning.\n"] pub fn use_native_text (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.use_native_text" , self . base)) } }
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl { # [serde (skip_serializing_if = "Option::is_none")] digital_parsing_config : Option < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] file_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] layout_parsing_config : Option < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl > > , # [serde (skip_serializing_if = "Option::is_none")] ocr_parsing_config : Option < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl > > , }
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
    #[doc = "Set the field `digital_parsing_config`.\n"]
    pub fn set_digital_parsing_config(
        mut self,
        v : impl Into < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigEl > >,
    ) -> Self {
        self.digital_parsing_config = Some(v.into());
        self
    }
    #[doc = "Set the field `file_type`.\n"]
    pub fn set_file_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_type = Some(v.into());
        self
    }
    #[doc = "Set the field `layout_parsing_config`.\n"]
    pub fn set_layout_parsing_config(
        mut self,
        v : impl Into < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigEl > >,
    ) -> Self {
        self.layout_parsing_config = Some(v.into());
        self
    }
    #[doc = "Set the field `ocr_parsing_config`.\n"]
    pub fn set_ocr_parsing_config(
        mut self,
        v : impl Into < ListField < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigEl > >,
    ) -> Self {
        self.ocr_parsing_config = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl
{
    type O = BlockAssignable<
        DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
    pub fn build(
        self,
    ) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
        DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl {
            digital_parsing_config: core::default::Default::default(),
            file_type: core::default::Default::default(),
            layout_parsing_config: core::default::Default::default(),
            ocr_parsing_config: core::default::Default::default(),
        }
    }
}
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
        DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `digital_parsing_config` after provisioning.\n"]    pub fn digital_parsing_config (& self) -> ListRef < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElDigitalParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.digital_parsing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }
    #[doc = "Get a reference to the value of field `layout_parsing_config` after provisioning.\n"]    pub fn layout_parsing_config (& self) -> ListRef < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElLayoutParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.layout_parsing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ocr_parsing_config` after provisioning.\n"]    pub fn ocr_parsing_config (& self) -> ListRef < DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElOcrParsingConfigElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.ocr_parsing_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    chunking_config:
        Option<ListField<DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_parsing_config: Option<
        ListField<DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parsing_config_overrides: Option<
        SetField<DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl>,
    >,
}
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigEl {
    #[doc = "Set the field `chunking_config`.\n"]
    pub fn set_chunking_config(
        mut self,
        v: impl Into<ListField<DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigEl>>,
    ) -> Self {
        self.chunking_config = Some(v.into());
        self
    }
    #[doc = "Set the field `default_parsing_config`.\n"]
    pub fn set_default_parsing_config(
        mut self,
        v: impl Into<
            ListField<DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigEl>,
        >,
    ) -> Self {
        self.default_parsing_config = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `parsing_config_overrides`.\n"]
    pub fn set_parsing_config_overrides(
        mut self,
        v: impl Into<
            SetField<
                DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesEl,
            >,
        >,
    ) -> Self {
        self.parsing_config_overrides = Some(v.into());
        self
    }
}
impl ToListMappable for DataDiscoveryEngineDataStoreDocumentProcessingConfigEl {
    type O = BlockAssignable<DataDiscoveryEngineDataStoreDocumentProcessingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigEl {}
impl BuildDataDiscoveryEngineDataStoreDocumentProcessingConfigEl {
    pub fn build(self) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigEl {
        DataDiscoveryEngineDataStoreDocumentProcessingConfigEl {
            chunking_config: core::default::Default::default(),
            default_parsing_config: core::default::Default::default(),
            name: core::default::Default::default(),
            parsing_config_overrides: core::default::Default::default(),
        }
    }
}
pub struct DataDiscoveryEngineDataStoreDocumentProcessingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDiscoveryEngineDataStoreDocumentProcessingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDiscoveryEngineDataStoreDocumentProcessingConfigElRef {
        DataDiscoveryEngineDataStoreDocumentProcessingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDiscoveryEngineDataStoreDocumentProcessingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `chunking_config` after provisioning.\n"]
    pub fn chunking_config(
        &self,
    ) -> ListRef<DataDiscoveryEngineDataStoreDocumentProcessingConfigElChunkingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chunking_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `default_parsing_config` after provisioning.\n"]
    pub fn default_parsing_config(
        &self,
    ) -> ListRef<DataDiscoveryEngineDataStoreDocumentProcessingConfigElDefaultParsingConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_parsing_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `parsing_config_overrides` after provisioning.\n"]
    pub fn parsing_config_overrides(
        &self,
    ) -> SetRef<DataDiscoveryEngineDataStoreDocumentProcessingConfigElParsingConfigOverridesElRef>
    {
        SetRef::new(
            self.shared().clone(),
            format!("{}.parsing_config_overrides", self.base),
        )
    }
}
