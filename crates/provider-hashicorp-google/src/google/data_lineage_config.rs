use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataLineageConfigData {
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
    location: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingestion: Option<Vec<DataLineageConfigIngestionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataLineageConfigTimeoutsEl>,
    dynamic: DataLineageConfigDynamic,
}
struct DataLineageConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLineageConfigData>,
}
#[derive(Clone)]
pub struct DataLineageConfig(Rc<DataLineageConfig_>);
impl DataLineageConfig {
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
    #[doc = "Set the field `ingestion`.\n"]
    pub fn set_ingestion(
        self,
        v: impl Into<BlockAssignable<DataLineageConfigIngestionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingestion = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingestion = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataLineageConfigTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `ingestion` after provisioning.\n"]
    pub fn ingestion(&self) -> ListRef<DataLineageConfigIngestionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLineageConfigTimeoutsElRef {
        DataLineageConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataLineageConfig {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataLineageConfig {}
impl ToListMappable for DataLineageConfig {
    type O = ListRef<DataLineageConfigRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataLineageConfig_ {
    fn extract_resource_type(&self) -> String {
        "google_data_lineage_config".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataLineageConfig {
    pub tf_id: String,
    #[doc = "The region of the data lineage configuration for integration."]
    pub location: PrimField<String>,
    #[doc = "Parent scope for the config.\nFormat: projects/{project-id|project-number} or folders/{folder-number} or organizations/{organization-number}."]
    pub parent: PrimField<String>,
}
impl BuildDataLineageConfig {
    pub fn build(self, stack: &mut Stack) -> DataLineageConfig {
        let out = DataLineageConfig(Rc::new(DataLineageConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLineageConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                ingestion: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataLineageConfigRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLineageConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataLineageConfigRef {
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
    #[doc = "Get a reference to the value of field `ingestion` after provisioning.\n"]
    pub fn ingestion(&self) -> ListRef<DataLineageConfigIngestionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLineageConfigTimeoutsElRef {
        DataLineageConfigTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataLineageConfigIngestionElRuleElIntegrationSelectorEl {
    integration: PrimField<String>,
}
impl DataLineageConfigIngestionElRuleElIntegrationSelectorEl {}
impl ToListMappable for DataLineageConfigIngestionElRuleElIntegrationSelectorEl {
    type O = BlockAssignable<DataLineageConfigIngestionElRuleElIntegrationSelectorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLineageConfigIngestionElRuleElIntegrationSelectorEl {
    #[doc = "Integration to which the rule applies. Possible values: [\"DATAPROC\", \"LOOKER_CORE\"]"]
    pub integration: PrimField<String>,
}
impl BuildDataLineageConfigIngestionElRuleElIntegrationSelectorEl {
    pub fn build(self) -> DataLineageConfigIngestionElRuleElIntegrationSelectorEl {
        DataLineageConfigIngestionElRuleElIntegrationSelectorEl {
            integration: self.integration,
        }
    }
}
pub struct DataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
        DataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLineageConfigIngestionElRuleElIntegrationSelectorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `integration` after provisioning.\nIntegration to which the rule applies. Possible values: [\"DATAPROC\", \"LOOKER_CORE\"]"]
    pub fn integration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.integration", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLineageConfigIngestionElRuleElLineageEnablementEl {
    enabled: PrimField<bool>,
}
impl DataLineageConfigIngestionElRuleElLineageEnablementEl {}
impl ToListMappable for DataLineageConfigIngestionElRuleElLineageEnablementEl {
    type O = BlockAssignable<DataLineageConfigIngestionElRuleElLineageEnablementEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLineageConfigIngestionElRuleElLineageEnablementEl {
    #[doc = "Whether ingestion of lineage should be enabled."]
    pub enabled: PrimField<bool>,
}
impl BuildDataLineageConfigIngestionElRuleElLineageEnablementEl {
    pub fn build(self) -> DataLineageConfigIngestionElRuleElLineageEnablementEl {
        DataLineageConfigIngestionElRuleElLineageEnablementEl {
            enabled: self.enabled,
        }
    }
}
pub struct DataLineageConfigIngestionElRuleElLineageEnablementElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLineageConfigIngestionElRuleElLineageEnablementElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataLineageConfigIngestionElRuleElLineageEnablementElRef {
        DataLineageConfigIngestionElRuleElLineageEnablementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLineageConfigIngestionElRuleElLineageEnablementElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether ingestion of lineage should be enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataLineageConfigIngestionElRuleElDynamic {
    integration_selector:
        Option<DynamicBlock<DataLineageConfigIngestionElRuleElIntegrationSelectorEl>>,
    lineage_enablement: Option<DynamicBlock<DataLineageConfigIngestionElRuleElLineageEnablementEl>>,
}
#[derive(Serialize)]
pub struct DataLineageConfigIngestionElRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    integration_selector: Option<Vec<DataLineageConfigIngestionElRuleElIntegrationSelectorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lineage_enablement: Option<Vec<DataLineageConfigIngestionElRuleElLineageEnablementEl>>,
    dynamic: DataLineageConfigIngestionElRuleElDynamic,
}
impl DataLineageConfigIngestionElRuleEl {
    #[doc = "Set the field `integration_selector`.\n"]
    pub fn set_integration_selector(
        mut self,
        v: impl Into<BlockAssignable<DataLineageConfigIngestionElRuleElIntegrationSelectorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.integration_selector = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.integration_selector = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lineage_enablement`.\n"]
    pub fn set_lineage_enablement(
        mut self,
        v: impl Into<BlockAssignable<DataLineageConfigIngestionElRuleElLineageEnablementEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.lineage_enablement = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.lineage_enablement = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLineageConfigIngestionElRuleEl {
    type O = BlockAssignable<DataLineageConfigIngestionElRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLineageConfigIngestionElRuleEl {}
impl BuildDataLineageConfigIngestionElRuleEl {
    pub fn build(self) -> DataLineageConfigIngestionElRuleEl {
        DataLineageConfigIngestionElRuleEl {
            integration_selector: core::default::Default::default(),
            lineage_enablement: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLineageConfigIngestionElRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLineageConfigIngestionElRuleElRef {
    fn new(shared: StackShared, base: String) -> DataLineageConfigIngestionElRuleElRef {
        DataLineageConfigIngestionElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLineageConfigIngestionElRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `integration_selector` after provisioning.\n"]
    pub fn integration_selector(
        &self,
    ) -> ListRef<DataLineageConfigIngestionElRuleElIntegrationSelectorElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.integration_selector", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `lineage_enablement` after provisioning.\n"]
    pub fn lineage_enablement(
        &self,
    ) -> ListRef<DataLineageConfigIngestionElRuleElLineageEnablementElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lineage_enablement", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataLineageConfigIngestionElDynamic {
    rule: Option<DynamicBlock<DataLineageConfigIngestionElRuleEl>>,
}
#[derive(Serialize)]
pub struct DataLineageConfigIngestionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<DataLineageConfigIngestionElRuleEl>>,
    dynamic: DataLineageConfigIngestionElDynamic,
}
impl DataLineageConfigIngestionEl {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v: impl Into<BlockAssignable<DataLineageConfigIngestionElRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataLineageConfigIngestionEl {
    type O = BlockAssignable<DataLineageConfigIngestionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLineageConfigIngestionEl {}
impl BuildDataLineageConfigIngestionEl {
    pub fn build(self) -> DataLineageConfigIngestionEl {
        DataLineageConfigIngestionEl {
            rule: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataLineageConfigIngestionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLineageConfigIngestionElRef {
    fn new(shared: StackShared, base: String) -> DataLineageConfigIngestionElRef {
        DataLineageConfigIngestionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLineageConfigIngestionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<DataLineageConfigIngestionElRuleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}
#[derive(Serialize)]
pub struct DataLineageConfigTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl DataLineageConfigTimeoutsEl {
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
impl ToListMappable for DataLineageConfigTimeoutsEl {
    type O = BlockAssignable<DataLineageConfigTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataLineageConfigTimeoutsEl {}
impl BuildDataLineageConfigTimeoutsEl {
    pub fn build(self) -> DataLineageConfigTimeoutsEl {
        DataLineageConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct DataLineageConfigTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataLineageConfigTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataLineageConfigTimeoutsElRef {
        DataLineageConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataLineageConfigTimeoutsElRef {
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
struct DataLineageConfigDynamic {
    ingestion: Option<DynamicBlock<DataLineageConfigIngestionEl>>,
}
