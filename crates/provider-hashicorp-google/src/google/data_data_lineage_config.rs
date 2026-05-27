use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataDataLineageConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    parent: PrimField<String>,
}
struct DataDataLineageConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDataLineageConfigData>,
}
#[derive(Clone)]
pub struct DataDataLineageConfig(Rc<DataDataLineageConfig_>);
impl DataDataLineageConfig {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed for optimistic concurrency control when patching config."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingestion` after provisioning.\nDefines how Lineage should be ingested for this resource."]
    pub fn ingestion(&self) -> ListRef<DataDataLineageConfigIngestionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the data lineage configuration for integration."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the config.\nFormat: organizations/{organization_id}/locations/{location}/config,\nfolders/{folder_id}/locations/{location}/config,\nprojects/{project_id}/locations/{location}/config,\nor projects/{project_number}/locations/{location}/config."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nParent scope for the config.\nFormat: projects/{project-id|project-number} or folders/{folder-number} or organizations/{organization-number}."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
}
impl Referable for DataDataLineageConfig {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataDataLineageConfig {}
impl ToListMappable for DataDataLineageConfig {
    type O = ListRef<DataDataLineageConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataDataLineageConfig_ {
    fn extract_datasource_type(&self) -> String {
        "google_data_lineage_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataDataLineageConfig {
    pub tf_id: String,
    #[doc = "The region of the data lineage configuration for integration."]
    pub location: PrimField<String>,
    #[doc = "Parent scope for the config.\nFormat: projects/{project-id|project-number} or folders/{folder-number} or organizations/{organization-number}."]
    pub parent: PrimField<String>,
}
impl BuildDataDataLineageConfig {
    pub fn build(self, stack: &mut Stack) -> DataDataLineageConfig {
        let out = DataDataLineageConfig(Rc::new(DataDataLineageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDataLineageConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataDataLineageConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataLineageConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataDataLineageConfigRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nUsed for optimistic concurrency control when patching config."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ingestion` after provisioning.\nDefines how Lineage should be ingested for this resource."]
    pub fn ingestion(&self) -> ListRef<DataDataLineageConfigIngestionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe region of the data lineage configuration for integration."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the config.\nFormat: organizations/{organization_id}/locations/{location}/config,\nfolders/{folder_id}/locations/{location}/config,\nprojects/{project_id}/locations/{location}/config,\nor projects/{project_number}/locations/{location}/config."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nParent scope for the config.\nFormat: projects/{project-id|project-number} or folders/{folder-number} or organizations/{organization-number}."]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataDataLineageConfigIngestionElRuleElIntegrationSelectorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    integration: Option<PrimField<String>>,
}
impl DataDataLineageConfigIngestionElRuleElIntegrationSelectorEl {
    #[doc = "Set the field `integration`.\n"]
    pub fn set_integration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.integration = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataLineageConfigIngestionElRuleElIntegrationSelectorEl {
    type O = BlockAssignable<DataDataLineageConfigIngestionElRuleElIntegrationSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataLineageConfigIngestionElRuleElIntegrationSelectorEl {}
impl BuildDataDataLineageConfigIngestionElRuleElIntegrationSelectorEl {
    pub fn build(self) -> DataDataLineageConfigIngestionElRuleElIntegrationSelectorEl {
        DataDataLineageConfigIngestionElRuleElIntegrationSelectorEl {
            integration: core::default::Default::default(),
        }
    }
}
pub struct DataDataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
        DataDataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `integration` after provisioning.\n"]
    pub fn integration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.integration", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDataLineageConfigIngestionElRuleElLineageEnablementEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}
impl DataDataLineageConfigIngestionElRuleElLineageEnablementEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataLineageConfigIngestionElRuleElLineageEnablementEl {
    type O = BlockAssignable<DataDataLineageConfigIngestionElRuleElLineageEnablementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataLineageConfigIngestionElRuleElLineageEnablementEl {}
impl BuildDataDataLineageConfigIngestionElRuleElLineageEnablementEl {
    pub fn build(self) -> DataDataLineageConfigIngestionElRuleElLineageEnablementEl {
        DataDataLineageConfigIngestionElRuleElLineageEnablementEl {
            enabled: core::default::Default::default(),
        }
    }
}
pub struct DataDataLineageConfigIngestionElRuleElLineageEnablementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataLineageConfigIngestionElRuleElLineageEnablementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataDataLineageConfigIngestionElRuleElLineageEnablementElRef {
        DataDataLineageConfigIngestionElRuleElLineageEnablementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataLineageConfigIngestionElRuleElLineageEnablementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize)]
pub struct DataDataLineageConfigIngestionElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_selector:
        Option<ListField<DataDataLineageConfigIngestionElRuleElIntegrationSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lineage_enablement:
        Option<ListField<DataDataLineageConfigIngestionElRuleElLineageEnablementEl>>,
}
impl DataDataLineageConfigIngestionElRuleEl {
    #[doc = "Set the field `integration_selector`.\n"]
    pub fn set_integration_selector(
        mut self,
        v: impl Into<ListField<DataDataLineageConfigIngestionElRuleElIntegrationSelectorEl>>,
    ) -> Self {
        self.integration_selector = Some(v.into());
        self
    }
    #[doc = "Set the field `lineage_enablement`.\n"]
    pub fn set_lineage_enablement(
        mut self,
        v: impl Into<ListField<DataDataLineageConfigIngestionElRuleElLineageEnablementEl>>,
    ) -> Self {
        self.lineage_enablement = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataLineageConfigIngestionElRuleEl {
    type O = BlockAssignable<DataDataLineageConfigIngestionElRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataLineageConfigIngestionElRuleEl {}
impl BuildDataDataLineageConfigIngestionElRuleEl {
    pub fn build(self) -> DataDataLineageConfigIngestionElRuleEl {
        DataDataLineageConfigIngestionElRuleEl {
            integration_selector: core::default::Default::default(),
            lineage_enablement: core::default::Default::default(),
        }
    }
}
pub struct DataDataLineageConfigIngestionElRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataLineageConfigIngestionElRuleElRef {
    fn new(shared: StackShared, base: String) -> DataDataLineageConfigIngestionElRuleElRef {
        DataDataLineageConfigIngestionElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataLineageConfigIngestionElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `integration_selector` after provisioning.\n"]
    pub fn integration_selector(
        &self,
    ) -> ListRef<DataDataLineageConfigIngestionElRuleElIntegrationSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.integration_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lineage_enablement` after provisioning.\n"]
    pub fn lineage_enablement(
        &self,
    ) -> ListRef<DataDataLineageConfigIngestionElRuleElLineageEnablementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lineage_enablement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataDataLineageConfigIngestionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<ListField<DataDataLineageConfigIngestionElRuleEl>>,
}
impl DataDataLineageConfigIngestionEl {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v: impl Into<ListField<DataDataLineageConfigIngestionElRuleEl>>,
    ) -> Self {
        self.rule = Some(v.into());
        self
    }
}
impl ToListMappable for DataDataLineageConfigIngestionEl {
    type O = BlockAssignable<DataDataLineageConfigIngestionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataDataLineageConfigIngestionEl {}
impl BuildDataDataLineageConfigIngestionEl {
    pub fn build(self) -> DataDataLineageConfigIngestionEl {
        DataDataLineageConfigIngestionEl {
            rule: core::default::Default::default(),
        }
    }
}
pub struct DataDataLineageConfigIngestionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataDataLineageConfigIngestionElRef {
    fn new(shared: StackShared, base: String) -> DataDataLineageConfigIngestionElRef {
        DataDataLineageConfigIngestionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataDataLineageConfigIngestionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<DataDataLineageConfigIngestionElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}
