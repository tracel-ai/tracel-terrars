use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ModelArmorTemplateData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    template_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_config: Option<Vec<ModelArmorTemplateFilterConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_metadata: Option<Vec<ModelArmorTemplateTemplateMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ModelArmorTemplateTimeoutsEl>,
    dynamic: ModelArmorTemplateDynamic,
}
struct ModelArmorTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ModelArmorTemplateData>,
}
#[derive(Clone)]
pub struct ModelArmorTemplate(Rc<ModelArmorTemplate_>);
impl ModelArmorTemplate {
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
    #[doc = "Set the field `labels`.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_config`.\n"]
    pub fn set_filter_config(
        self,
        v: impl Into<BlockAssignable<ModelArmorTemplateFilterConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `template_metadata`.\n"]
    pub fn set_template_metadata(
        self,
        v: impl Into<BlockAssignable<ModelArmorTemplateTemplateMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().template_metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.template_metadata = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ModelArmorTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp"]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
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
    #[doc = "Get a reference to the value of field `template_id` after provisioning.\nId of the requesting object\nIf auto-generating Id server-side, remove this field and\ntemplate_id from the method_signature of Create RPC"]
    pub fn template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter_config` after provisioning.\n"]
    pub fn filter_config(&self) -> ListRef<ModelArmorTemplateFilterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_metadata` after provisioning.\n"]
    pub fn template_metadata(&self) -> ListRef<ModelArmorTemplateTemplateMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ModelArmorTemplateTimeoutsElRef {
        ModelArmorTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ModelArmorTemplate {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ModelArmorTemplate {}
impl ToListMappable for ModelArmorTemplate {
    type O = ListRef<ModelArmorTemplateRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ModelArmorTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_model_armor_template".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildModelArmorTemplate {
    pub tf_id: String,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
    #[doc = "Id of the requesting object\nIf auto-generating Id server-side, remove this field and\ntemplate_id from the method_signature of Create RPC"]
    pub template_id: PrimField<String>,
}
impl BuildModelArmorTemplate {
    pub fn build(self, stack: &mut Stack) -> ModelArmorTemplate {
        let out = ModelArmorTemplate(Rc::new(ModelArmorTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ModelArmorTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                template_id: self.template_id,
                filter_config: core::default::Default::default(),
                template_metadata: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ModelArmorTemplateRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ModelArmorTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreate time stamp"]
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
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
    #[doc = "Get a reference to the value of field `template_id` after provisioning.\nId of the requesting object\nIf auto-generating Id server-side, remove this field and\ntemplate_id from the method_signature of Create RPC"]
    pub fn template_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.template_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nUpdate time stamp"]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter_config` after provisioning.\n"]
    pub fn filter_config(&self) -> ListRef<ModelArmorTemplateFilterConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filter_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `template_metadata` after provisioning.\n"]
    pub fn template_metadata(&self) -> ListRef<ModelArmorTemplateTemplateMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template_metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ModelArmorTemplateTimeoutsElRef {
        ModelArmorTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_enforcement: Option<PrimField<String>>,
}
impl ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl {
    #[doc = "Set the field `filter_enforcement`.\nTells whether the Malicious URI filter is enabled or disabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_enforcement = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl {
    type O = BlockAssignable<ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl {}
impl BuildModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl {
    pub fn build(self) -> ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl {
        ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl {
            filter_enforcement: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsElRef {
        ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether the Malicious URI filter is enabled or disabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_enforcement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_enforcement: Option<PrimField<String>>,
}
impl ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl {
    #[doc = "Set the field `confidence_level`.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn set_confidence_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidence_level = Some(v.into());
        self
    }
    #[doc = "Set the field `filter_enforcement`.\nTells whether Prompt injection and Jailbreak filter is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_enforcement = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl {
    type O = BlockAssignable<ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl {}
impl BuildModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl {
    pub fn build(self) -> ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl {
        ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl {
            confidence_level: core::default::Default::default(),
            filter_enforcement: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsElRef {
        ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidence_level` after provisioning.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn confidence_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidence_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether Prompt injection and Jailbreak filter is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_enforcement", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence_level: Option<PrimField<String>>,
    filter_type: PrimField<String>,
}
impl ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl {
    #[doc = "Set the field `confidence_level`.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn set_confidence_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.confidence_level = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl {
    type O = BlockAssignable<ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl {
    #[doc = "Possible values:\nSEXUALLY_EXPLICIT\nHATE_SPEECH\nHARASSMENT\nDANGEROUS"]
    pub filter_type: PrimField<String>,
}
impl BuildModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl {
    pub fn build(self) -> ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl {
        ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl {
            confidence_level: core::default::Default::default(),
            filter_type: self.filter_type,
        }
    }
}
pub struct ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersElRef {
        ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `confidence_level` after provisioning.\nPossible values:\nLOW_AND_ABOVE\nMEDIUM_AND_ABOVE\nHIGH"]
    pub fn confidence_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.confidence_level", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `filter_type` after provisioning.\nPossible values:\nSEXUALLY_EXPLICIT\nHATE_SPEECH\nHARASSMENT\nDANGEROUS"]
    pub fn filter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct ModelArmorTemplateFilterConfigElRaiSettingsElDynamic {
    rai_filters: Option<DynamicBlock<ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl>>,
}
#[derive(Serialize)]
pub struct ModelArmorTemplateFilterConfigElRaiSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rai_filters: Option<Vec<ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl>>,
    dynamic: ModelArmorTemplateFilterConfigElRaiSettingsElDynamic,
}
impl ModelArmorTemplateFilterConfigElRaiSettingsEl {
    #[doc = "Set the field `rai_filters`.\n"]
    pub fn set_rai_filters(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rai_filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rai_filters = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ModelArmorTemplateFilterConfigElRaiSettingsEl {
    type O = BlockAssignable<ModelArmorTemplateFilterConfigElRaiSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateFilterConfigElRaiSettingsEl {}
impl BuildModelArmorTemplateFilterConfigElRaiSettingsEl {
    pub fn build(self) -> ModelArmorTemplateFilterConfigElRaiSettingsEl {
        ModelArmorTemplateFilterConfigElRaiSettingsEl {
            rai_filters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ModelArmorTemplateFilterConfigElRaiSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateFilterConfigElRaiSettingsElRef {
    fn new(shared: StackShared, base: String) -> ModelArmorTemplateFilterConfigElRaiSettingsElRef {
        ModelArmorTemplateFilterConfigElRaiSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateFilterConfigElRaiSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `rai_filters` after provisioning.\n"]
    pub fn rai_filters(
        &self,
    ) -> ListRef<ModelArmorTemplateFilterConfigElRaiSettingsElRaiFiltersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rai_filters", self.base))
    }
}
#[derive(Serialize)]
pub struct ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    deidentify_template: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inspect_template: Option<PrimField<String>>,
}
impl ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl {
    #[doc = "Set the field `deidentify_template`.\nOptional Sensitive Data Protection Deidentify template resource name.\nIf provided then DeidentifyContent action is performed during Sanitization\nusing this template and inspect template. The De-identified data will\nbe returned in SdpDeidentifyResult.\nNote that all info-types present in the deidentify template must be present\nin inspect template.\ne.g.\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn set_deidentify_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.deidentify_template = Some(v.into());
        self
    }
    #[doc = "Set the field `inspect_template`.\nSensitive Data Protection inspect template resource name\nIf only inspect template is provided (de-identify template not provided),\nthen Sensitive Data Protection InspectContent action is performed during\nSanitization. All Sensitive Data Protection findings identified during\ninspection will be returned as SdpFinding in SdpInsepctionResult.\ne.g:-\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn set_inspect_template(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.inspect_template = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl {
    type O = BlockAssignable<ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl {}
impl BuildModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl {
    pub fn build(self) -> ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl {
        ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl {
            deidentify_template: core::default::Default::default(),
            inspect_template: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigElRef {
        ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deidentify_template` after provisioning.\nOptional Sensitive Data Protection Deidentify template resource name.\nIf provided then DeidentifyContent action is performed during Sanitization\nusing this template and inspect template. The De-identified data will\nbe returned in SdpDeidentifyResult.\nNote that all info-types present in the deidentify template must be present\nin inspect template.\ne.g.\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn deidentify_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deidentify_template", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inspect_template` after provisioning.\nSensitive Data Protection inspect template resource name\nIf only inspect template is provided (de-identify template not provided),\nthen Sensitive Data Protection InspectContent action is performed during\nSanitization. All Sensitive Data Protection findings identified during\ninspection will be returned as SdpFinding in SdpInsepctionResult.\ne.g:-\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn inspect_template(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.inspect_template", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_enforcement: Option<PrimField<String>>,
}
impl ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl {
    #[doc = "Set the field `filter_enforcement`.\nTells whether the Sensitive Data Protection basic config is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn set_filter_enforcement(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_enforcement = Some(v.into());
        self
    }
}
impl ToListMappable for ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl {
    type O = BlockAssignable<ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl {}
impl BuildModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl {
    pub fn build(self) -> ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl {
        ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl {
            filter_enforcement: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigElRef {
        ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filter_enforcement` after provisioning.\nTells whether the Sensitive Data Protection basic config is enabled or\ndisabled.\nPossible values:\nENABLED\nDISABLED"]
    pub fn filter_enforcement(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter_enforcement", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ModelArmorTemplateFilterConfigElSdpSettingsElDynamic {
    advanced_config:
        Option<DynamicBlock<ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl>>,
    basic_config: Option<DynamicBlock<ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl>>,
}
#[derive(Serialize)]
pub struct ModelArmorTemplateFilterConfigElSdpSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_config: Option<Vec<ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic_config: Option<Vec<ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl>>,
    dynamic: ModelArmorTemplateFilterConfigElSdpSettingsElDynamic,
}
impl ModelArmorTemplateFilterConfigElSdpSettingsEl {
    #[doc = "Set the field `advanced_config`.\n"]
    pub fn set_advanced_config(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.advanced_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `basic_config`.\n"]
    pub fn set_basic_config(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.basic_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ModelArmorTemplateFilterConfigElSdpSettingsEl {
    type O = BlockAssignable<ModelArmorTemplateFilterConfigElSdpSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateFilterConfigElSdpSettingsEl {}
impl BuildModelArmorTemplateFilterConfigElSdpSettingsEl {
    pub fn build(self) -> ModelArmorTemplateFilterConfigElSdpSettingsEl {
        ModelArmorTemplateFilterConfigElSdpSettingsEl {
            advanced_config: core::default::Default::default(),
            basic_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ModelArmorTemplateFilterConfigElSdpSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateFilterConfigElSdpSettingsElRef {
    fn new(shared: StackShared, base: String) -> ModelArmorTemplateFilterConfigElSdpSettingsElRef {
        ModelArmorTemplateFilterConfigElSdpSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateFilterConfigElSdpSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `advanced_config` after provisioning.\n"]
    pub fn advanced_config(
        &self,
    ) -> ListRef<ModelArmorTemplateFilterConfigElSdpSettingsElAdvancedConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.advanced_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `basic_config` after provisioning.\n"]
    pub fn basic_config(
        &self,
    ) -> ListRef<ModelArmorTemplateFilterConfigElSdpSettingsElBasicConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.basic_config", self.base))
    }
}
#[derive(Serialize, Default)]
struct ModelArmorTemplateFilterConfigElDynamic {
    malicious_uri_filter_settings:
        Option<DynamicBlock<ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl>>,
    pi_and_jailbreak_filter_settings:
        Option<DynamicBlock<ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl>>,
    rai_settings: Option<DynamicBlock<ModelArmorTemplateFilterConfigElRaiSettingsEl>>,
    sdp_settings: Option<DynamicBlock<ModelArmorTemplateFilterConfigElSdpSettingsEl>>,
}
#[derive(Serialize)]
pub struct ModelArmorTemplateFilterConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    malicious_uri_filter_settings:
        Option<Vec<ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pi_and_jailbreak_filter_settings:
        Option<Vec<ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rai_settings: Option<Vec<ModelArmorTemplateFilterConfigElRaiSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sdp_settings: Option<Vec<ModelArmorTemplateFilterConfigElSdpSettingsEl>>,
    dynamic: ModelArmorTemplateFilterConfigElDynamic,
}
impl ModelArmorTemplateFilterConfigEl {
    #[doc = "Set the field `malicious_uri_filter_settings`.\n"]
    pub fn set_malicious_uri_filter_settings(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.malicious_uri_filter_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.malicious_uri_filter_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pi_and_jailbreak_filter_settings`.\n"]
    pub fn set_pi_and_jailbreak_filter_settings(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pi_and_jailbreak_filter_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pi_and_jailbreak_filter_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `rai_settings`.\n"]
    pub fn set_rai_settings(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorTemplateFilterConfigElRaiSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rai_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rai_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sdp_settings`.\n"]
    pub fn set_sdp_settings(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorTemplateFilterConfigElSdpSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sdp_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sdp_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ModelArmorTemplateFilterConfigEl {
    type O = BlockAssignable<ModelArmorTemplateFilterConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateFilterConfigEl {}
impl BuildModelArmorTemplateFilterConfigEl {
    pub fn build(self) -> ModelArmorTemplateFilterConfigEl {
        ModelArmorTemplateFilterConfigEl {
            malicious_uri_filter_settings: core::default::Default::default(),
            pi_and_jailbreak_filter_settings: core::default::Default::default(),
            rai_settings: core::default::Default::default(),
            sdp_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ModelArmorTemplateFilterConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateFilterConfigElRef {
    fn new(shared: StackShared, base: String) -> ModelArmorTemplateFilterConfigElRef {
        ModelArmorTemplateFilterConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateFilterConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `malicious_uri_filter_settings` after provisioning.\n"]
    pub fn malicious_uri_filter_settings(
        &self,
    ) -> ListRef<ModelArmorTemplateFilterConfigElMaliciousUriFilterSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.malicious_uri_filter_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pi_and_jailbreak_filter_settings` after provisioning.\n"]
    pub fn pi_and_jailbreak_filter_settings(
        &self,
    ) -> ListRef<ModelArmorTemplateFilterConfigElPiAndJailbreakFilterSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pi_and_jailbreak_filter_settings", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `rai_settings` after provisioning.\n"]
    pub fn rai_settings(&self) -> ListRef<ModelArmorTemplateFilterConfigElRaiSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rai_settings", self.base))
    }
    #[doc = "Get a reference to the value of field `sdp_settings` after provisioning.\n"]
    pub fn sdp_settings(&self) -> ListRef<ModelArmorTemplateFilterConfigElSdpSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sdp_settings", self.base))
    }
}
#[derive(Serialize)]
pub struct ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl {
    enable_multi_language_detection: PrimField<bool>,
}
impl ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl {}
impl ToListMappable for ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl {
    type O = BlockAssignable<ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl {
    #[doc = "If true, multi language detection will be enabled."]
    pub enable_multi_language_detection: PrimField<bool>,
}
impl BuildModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl {
    pub fn build(self) -> ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl {
        ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl {
            enable_multi_language_detection: self.enable_multi_language_detection,
        }
    }
}
pub struct ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionElRef {
        ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable_multi_language_detection` after provisioning.\nIf true, multi language detection will be enabled."]
    pub fn enable_multi_language_detection(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_multi_language_detection", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ModelArmorTemplateTemplateMetadataElDynamic {
    multi_language_detection:
        Option<DynamicBlock<ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl>>,
}
#[derive(Serialize)]
pub struct ModelArmorTemplateTemplateMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_llm_response_safety_error_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_llm_response_safety_error_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_prompt_safety_error_code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_prompt_safety_error_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforcement_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_partial_invocation_failures: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_sanitize_operations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_template_operations: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multi_language_detection:
        Option<Vec<ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl>>,
    dynamic: ModelArmorTemplateTemplateMetadataElDynamic,
}
impl ModelArmorTemplateTemplateMetadataEl {
    #[doc = "Set the field `custom_llm_response_safety_error_code`.\nIndicates the custom error code set by the user to be returned to the end\nuser if the LLM response trips Model Armor filters."]
    pub fn set_custom_llm_response_safety_error_code(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.custom_llm_response_safety_error_code = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_llm_response_safety_error_message`.\nIndicates the custom error message set by the user to be returned to the\nend user if the LLM response trips Model Armor filters."]
    pub fn set_custom_llm_response_safety_error_message(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.custom_llm_response_safety_error_message = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_prompt_safety_error_code`.\nIndicates the custom error code set by the user to be returned to the end\nuser by the service extension if the prompt trips Model Armor filters."]
    pub fn set_custom_prompt_safety_error_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_prompt_safety_error_code = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_prompt_safety_error_message`.\nIndicates the custom error message set by the user to be returned to the\nend user if the prompt trips Model Armor filters."]
    pub fn set_custom_prompt_safety_error_message(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.custom_prompt_safety_error_message = Some(v.into());
        self
    }
    #[doc = "Set the field `enforcement_type`.\nPossible values:\nINSPECT_ONLY\nINSPECT_AND_BLOCK"]
    pub fn set_enforcement_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforcement_type = Some(v.into());
        self
    }
    #[doc = "Set the field `ignore_partial_invocation_failures`.\nIf true, partial detector failures should be ignored."]
    pub fn set_ignore_partial_invocation_failures(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.ignore_partial_invocation_failures = Some(v.into());
        self
    }
    #[doc = "Set the field `log_sanitize_operations`.\nIf true, log sanitize operations."]
    pub fn set_log_sanitize_operations(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.log_sanitize_operations = Some(v.into());
        self
    }
    #[doc = "Set the field `log_template_operations`.\nIf true, log template crud operations."]
    pub fn set_log_template_operations(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.log_template_operations = Some(v.into());
        self
    }
    #[doc = "Set the field `multi_language_detection`.\n"]
    pub fn set_multi_language_detection(
        mut self,
        v: impl Into<BlockAssignable<ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.multi_language_detection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_language_detection = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ModelArmorTemplateTemplateMetadataEl {
    type O = BlockAssignable<ModelArmorTemplateTemplateMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateTemplateMetadataEl {}
impl BuildModelArmorTemplateTemplateMetadataEl {
    pub fn build(self) -> ModelArmorTemplateTemplateMetadataEl {
        ModelArmorTemplateTemplateMetadataEl {
            custom_llm_response_safety_error_code: core::default::Default::default(),
            custom_llm_response_safety_error_message: core::default::Default::default(),
            custom_prompt_safety_error_code: core::default::Default::default(),
            custom_prompt_safety_error_message: core::default::Default::default(),
            enforcement_type: core::default::Default::default(),
            ignore_partial_invocation_failures: core::default::Default::default(),
            log_sanitize_operations: core::default::Default::default(),
            log_template_operations: core::default::Default::default(),
            multi_language_detection: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ModelArmorTemplateTemplateMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateTemplateMetadataElRef {
    fn new(shared: StackShared, base: String) -> ModelArmorTemplateTemplateMetadataElRef {
        ModelArmorTemplateTemplateMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateTemplateMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `custom_llm_response_safety_error_code` after provisioning.\nIndicates the custom error code set by the user to be returned to the end\nuser if the LLM response trips Model Armor filters."]
    pub fn custom_llm_response_safety_error_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_llm_response_safety_error_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_llm_response_safety_error_message` after provisioning.\nIndicates the custom error message set by the user to be returned to the\nend user if the LLM response trips Model Armor filters."]
    pub fn custom_llm_response_safety_error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_llm_response_safety_error_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_prompt_safety_error_code` after provisioning.\nIndicates the custom error code set by the user to be returned to the end\nuser by the service extension if the prompt trips Model Armor filters."]
    pub fn custom_prompt_safety_error_code(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_prompt_safety_error_code", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `custom_prompt_safety_error_message` after provisioning.\nIndicates the custom error message set by the user to be returned to the\nend user if the prompt trips Model Armor filters."]
    pub fn custom_prompt_safety_error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_prompt_safety_error_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enforcement_type` after provisioning.\nPossible values:\nINSPECT_ONLY\nINSPECT_AND_BLOCK"]
    pub fn enforcement_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforcement_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ignore_partial_invocation_failures` after provisioning.\nIf true, partial detector failures should be ignored."]
    pub fn ignore_partial_invocation_failures(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ignore_partial_invocation_failures", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_sanitize_operations` after provisioning.\nIf true, log sanitize operations."]
    pub fn log_sanitize_operations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_sanitize_operations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_template_operations` after provisioning.\nIf true, log template crud operations."]
    pub fn log_template_operations(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_template_operations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `multi_language_detection` after provisioning.\n"]
    pub fn multi_language_detection(
        &self,
    ) -> ListRef<ModelArmorTemplateTemplateMetadataElMultiLanguageDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.multi_language_detection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ModelArmorTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ModelArmorTemplateTimeoutsEl {
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
impl ToListMappable for ModelArmorTemplateTimeoutsEl {
    type O = BlockAssignable<ModelArmorTemplateTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildModelArmorTemplateTimeoutsEl {}
impl BuildModelArmorTemplateTimeoutsEl {
    pub fn build(self) -> ModelArmorTemplateTimeoutsEl {
        ModelArmorTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ModelArmorTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ModelArmorTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ModelArmorTemplateTimeoutsElRef {
        ModelArmorTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ModelArmorTemplateTimeoutsElRef {
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
struct ModelArmorTemplateDynamic {
    filter_config: Option<DynamicBlock<ModelArmorTemplateFilterConfigEl>>,
    template_metadata: Option<DynamicBlock<ModelArmorTemplateTemplateMetadataEl>>,
}
