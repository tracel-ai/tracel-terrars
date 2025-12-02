use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct KendraDataSourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    index_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<Vec<KendraDataSourceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_document_enrichment_configuration:
        Option<Vec<KendraDataSourceCustomDocumentEnrichmentConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KendraDataSourceTimeoutsEl>,
    dynamic: KendraDataSourceDynamic,
}
struct KendraDataSource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KendraDataSourceData>,
}
#[derive(Clone)]
pub struct KendraDataSource(Rc<KendraDataSource_>);
impl KendraDataSource {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language_code = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl Into<BlockAssignable<KendraDataSourceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom_document_enrichment_configuration`.\n"]
    pub fn set_custom_document_enrichment_configuration(
        self,
        v: impl Into<BlockAssignable<KendraDataSourceCustomDocumentEnrichmentConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .custom_document_enrichment_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .custom_document_enrichment_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KendraDataSourceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<KendraDataSourceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_document_enrichment_configuration` after provisioning.\n"]
    pub fn custom_document_enrichment_configuration(
        &self,
    ) -> ListRef<KendraDataSourceCustomDocumentEnrichmentConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.custom_document_enrichment_configuration",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraDataSourceTimeoutsElRef {
        KendraDataSourceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for KendraDataSource {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for KendraDataSource {}
impl ToListMappable for KendraDataSource {
    type O = ListRef<KendraDataSourceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for KendraDataSource_ {
    fn extract_resource_type(&self) -> String {
        "aws_kendra_data_source".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildKendraDataSource {
    pub tf_id: String,
    #[doc = ""]
    pub index_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}
impl BuildKendraDataSource {
    pub fn build(self, stack: &mut Stack) -> KendraDataSource {
        let out = KendraDataSource(Rc::new(KendraDataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KendraDataSourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                index_id: self.index_id,
                language_code: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: core::default::Default::default(),
                schedule: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                configuration: core::default::Default::default(),
                custom_document_enrichment_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct KendraDataSourceRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl KendraDataSourceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_source_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.error_message", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.index_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language_code", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> ListRef<KendraDataSourceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_document_enrichment_configuration` after provisioning.\n"]
    pub fn custom_document_enrichment_configuration(
        &self,
    ) -> ListRef<KendraDataSourceCustomDocumentEnrichmentConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.custom_document_enrichment_configuration",
                self.extract_ref()
            ),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraDataSourceTimeoutsElRef {
        KendraDataSourceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_path: Option<PrimField<String>>,
}
impl KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
    #[doc = "Set the field `key_path`.\n"]
    pub fn set_key_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_path = Some(v.into());
        self
    }
}
impl ToListMappable
    for KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl
{
    type O = BlockAssignable<
        KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
}
impl BuildKendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
    pub fn build(
        self,
    ) -> KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
        KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl {
            key_path: core::default::Default::default(),
        }
    }
}
pub struct KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
        KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `key_path` after provisioning.\n"]
    pub fn key_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_path", self.base))
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_prefix: Option<PrimField<String>>,
}
impl KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
    #[doc = "Set the field `s3_prefix`.\n"]
    pub fn set_s3_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.s3_prefix = Some(v.into());
        self
    }
}
impl ToListMappable
    for KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl
{
    type O = BlockAssignable<
        KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
}
impl BuildKendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
    pub fn build(
        self,
    ) -> KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
        KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl {
            s3_prefix: core::default::Default::default(),
        }
    }
}
pub struct KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
        KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.base))
    }
}
#[derive(Serialize, Default)]
struct KendraDataSourceConfigurationElS3ConfigurationElDynamic {
    access_control_list_configuration: Option<
        DynamicBlock<
            KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl,
        >,
    >,
    documents_metadata_configuration: Option<
        DynamicBlock<
            KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElS3ConfigurationEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_patterns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion_patterns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion_prefixes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_list_configuration: Option<
        Vec<KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents_metadata_configuration: Option<
        Vec<KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl>,
    >,
    dynamic: KendraDataSourceConfigurationElS3ConfigurationElDynamic,
}
impl KendraDataSourceConfigurationElS3ConfigurationEl {
    #[doc = "Set the field `exclusion_patterns`.\n"]
    pub fn set_exclusion_patterns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclusion_patterns = Some(v.into());
        self
    }
    #[doc = "Set the field `inclusion_patterns`.\n"]
    pub fn set_inclusion_patterns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inclusion_patterns = Some(v.into());
        self
    }
    #[doc = "Set the field `inclusion_prefixes`.\n"]
    pub fn set_inclusion_prefixes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inclusion_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `access_control_list_configuration`.\n"]
    pub fn set_access_control_list_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_control_list_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_list_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `documents_metadata_configuration`.\n"]
    pub fn set_documents_metadata_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.documents_metadata_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.documents_metadata_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for KendraDataSourceConfigurationElS3ConfigurationEl {
    type O = BlockAssignable<KendraDataSourceConfigurationElS3ConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElS3ConfigurationEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
}
impl BuildKendraDataSourceConfigurationElS3ConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElS3ConfigurationEl {
        KendraDataSourceConfigurationElS3ConfigurationEl {
            bucket_name: self.bucket_name,
            exclusion_patterns: core::default::Default::default(),
            inclusion_patterns: core::default::Default::default(),
            inclusion_prefixes: core::default::Default::default(),
            access_control_list_configuration: core::default::Default::default(),
            documents_metadata_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceConfigurationElS3ConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceConfigurationElS3ConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElS3ConfigurationElRef {
        KendraDataSourceConfigurationElS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }
    #[doc = "Get a reference to the value of field `exclusion_patterns` after provisioning.\n"]
    pub fn exclusion_patterns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.exclusion_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inclusion_patterns` after provisioning.\n"]
    pub fn inclusion_patterns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.inclusion_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `inclusion_prefixes` after provisioning.\n"]
    pub fn inclusion_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.inclusion_prefixes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_control_list_configuration` after provisioning.\n"]
    pub fn access_control_list_configuration(
        &self,
    ) -> ListRef<KendraDataSourceConfigurationElS3ConfigurationElAccessControlListConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_control_list_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `documents_metadata_configuration` after provisioning.\n"]
    pub fn documents_metadata_configuration(
        &self,
    ) -> ListRef<KendraDataSourceConfigurationElS3ConfigurationElDocumentsMetadataConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.documents_metadata_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElTemplateConfigurationEl {
    template: PrimField<String>,
}
impl KendraDataSourceConfigurationElTemplateConfigurationEl {}
impl ToListMappable for KendraDataSourceConfigurationElTemplateConfigurationEl {
    type O = BlockAssignable<KendraDataSourceConfigurationElTemplateConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElTemplateConfigurationEl {
    #[doc = ""]
    pub template: PrimField<String>,
}
impl BuildKendraDataSourceConfigurationElTemplateConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElTemplateConfigurationEl {
        KendraDataSourceConfigurationElTemplateConfigurationEl {
            template: self.template,
        }
    }
}
pub struct KendraDataSourceConfigurationElTemplateConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceConfigurationElTemplateConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElTemplateConfigurationElRef {
        KendraDataSourceConfigurationElTemplateConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElTemplateConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.template", self.base))
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl
{
    credentials: PrimField<String>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl { }
impl ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl { type O = BlockAssignable < KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl
{
    #[doc = ""]
    pub credentials: PrimField<String>,
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl { pub fn build (self) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl { KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl { credentials : self . credentials , host : self . host , port : self . port , } } }
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef { fn new (shared : StackShared , base : String) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef { KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef { shared : shared , base : base . to_string () , } } }
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `credentials` after provisioning.\n"] pub fn credentials (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.credentials" , self . base)) } # [doc = "Get a reference to the value of field `host` after provisioning.\n"] pub fn host (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.host" , self . base)) } # [doc = "Get a reference to the value of field `port` after provisioning.\n"] pub fn port (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.port" , self . base)) } }
#[derive(Serialize, Default)]
struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElDynamic { basic_authentication : Option < DynamicBlock < KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl >> , }
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl { # [serde (skip_serializing_if = "Option::is_none")] basic_authentication : Option < Vec < KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl > > , dynamic : KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElDynamic , }
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
    #[doc = "Set the field `basic_authentication`.\n"]
    pub fn set_basic_authentication(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElBasicAuthenticationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.basic_authentication = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_authentication = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl
{
    type O = BlockAssignable<
        KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl
{}
impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
    pub fn build(
        self,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl {
            basic_authentication: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef
    {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<PrimField<String>>,
    host: PrimField<String>,
    port: PrimField<f64>,
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.credentials = Some(v.into());
        self
    }
}
impl ToListMappable
    for KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl
{
    type O = BlockAssignable<
        KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
    #[doc = ""]
    pub host: PrimField<String>,
    #[doc = ""]
    pub port: PrimField<f64>,
}
impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
    pub fn build(
        self,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl {
            credentials: core::default::Default::default(),
            host: self.host,
            port: self.port,
        }
    }
}
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.credentials", self.base))
    }
    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
    seed_urls: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_crawler_mode: Option<PrimField<String>>,
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
    #[doc = "Set the field `web_crawler_mode`.\n"]
    pub fn set_web_crawler_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.web_crawler_mode = Some(v.into());
        self
    }
}
impl ToListMappable
    for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl
{
    type O = BlockAssignable<
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl
{
    #[doc = ""]
    pub seed_urls: SetField<PrimField<String>>,
}
impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
    pub fn build(
        self,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl {
            seed_urls: self.seed_urls,
            web_crawler_mode: core::default::Default::default(),
        }
    }
}
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef
    {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `seed_urls` after provisioning.\n"]
    pub fn seed_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.seed_urls", self.base))
    }
    #[doc = "Get a reference to the value of field `web_crawler_mode` after provisioning.\n"]
    pub fn web_crawler_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.web_crawler_mode", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
    site_maps: SetField<PrimField<String>>,
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {}
impl ToListMappable
    for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl
{
    type O = BlockAssignable<
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl
{
    #[doc = ""]
    pub site_maps: SetField<PrimField<String>>,
}
impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
    pub fn build(
        self,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl {
            site_maps: self.site_maps,
        }
    }
}
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef
    {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `site_maps` after provisioning.\n"]
    pub fn site_maps(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.site_maps", self.base))
    }
}
#[derive(Serialize, Default)]
struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElDynamic {
    seed_url_configuration: Option<
        DynamicBlock<
            KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl,
        >,
    >,
    site_maps_configuration: Option<
        DynamicBlock<
            KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    seed_url_configuration: Option<
        Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    site_maps_configuration: Option<
        Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl>,
    >,
    dynamic: KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElDynamic,
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
    #[doc = "Set the field `seed_url_configuration`.\n"]
    pub fn set_seed_url_configuration(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.seed_url_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.seed_url_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `site_maps_configuration`.\n"]
    pub fn set_site_maps_configuration(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.site_maps_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.site_maps_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
    type O = BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {}
impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
    pub fn build(self) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl {
            seed_url_configuration: core::default::Default::default(),
            site_maps_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `seed_url_configuration` after provisioning.\n"]
    pub fn seed_url_configuration(
        &self,
    ) -> ListRef<
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSeedUrlConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.seed_url_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `site_maps_configuration` after provisioning.\n"]
    pub fn site_maps_configuration(
        &self,
    ) -> ListRef<
        KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElSiteMapsConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.site_maps_configuration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct KendraDataSourceConfigurationElWebCrawlerConfigurationElDynamic {
    authentication_configuration: Option<
        DynamicBlock<
            KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl,
        >,
    >,
    proxy_configuration: Option<
        DynamicBlock<KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl>,
    >,
    urls: Option<DynamicBlock<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl>>,
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawl_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_content_size_per_page_in_mega_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_links_per_page: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_urls_per_minute_crawl_rate: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_exclusion_patterns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_inclusion_patterns: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_configuration: Option<
        Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_configuration:
        Option<Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    urls: Option<Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl>>,
    dynamic: KendraDataSourceConfigurationElWebCrawlerConfigurationElDynamic,
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
    #[doc = "Set the field `crawl_depth`.\n"]
    pub fn set_crawl_depth(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.crawl_depth = Some(v.into());
        self
    }
    #[doc = "Set the field `max_content_size_per_page_in_mega_bytes`.\n"]
    pub fn set_max_content_size_per_page_in_mega_bytes(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.max_content_size_per_page_in_mega_bytes = Some(v.into());
        self
    }
    #[doc = "Set the field `max_links_per_page`.\n"]
    pub fn set_max_links_per_page(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_links_per_page = Some(v.into());
        self
    }
    #[doc = "Set the field `max_urls_per_minute_crawl_rate`.\n"]
    pub fn set_max_urls_per_minute_crawl_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_urls_per_minute_crawl_rate = Some(v.into());
        self
    }
    #[doc = "Set the field `url_exclusion_patterns`.\n"]
    pub fn set_url_exclusion_patterns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.url_exclusion_patterns = Some(v.into());
        self
    }
    #[doc = "Set the field `url_inclusion_patterns`.\n"]
    pub fn set_url_inclusion_patterns(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.url_inclusion_patterns = Some(v.into());
        self
    }
    #[doc = "Set the field `authentication_configuration`.\n"]
    pub fn set_authentication_configuration(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authentication_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proxy_configuration`.\n"]
    pub fn set_proxy_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.proxy_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.proxy_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `urls`.\n"]
    pub fn set_urls(
        mut self,
        v: impl Into<BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.urls = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.urls = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
    type O = BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationElWebCrawlerConfigurationEl {}
impl BuildKendraDataSourceConfigurationElWebCrawlerConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
        KendraDataSourceConfigurationElWebCrawlerConfigurationEl {
            crawl_depth: core::default::Default::default(),
            max_content_size_per_page_in_mega_bytes: core::default::Default::default(),
            max_links_per_page: core::default::Default::default(),
            max_urls_per_minute_crawl_rate: core::default::Default::default(),
            url_exclusion_patterns: core::default::Default::default(),
            url_inclusion_patterns: core::default::Default::default(),
            authentication_configuration: core::default::Default::default(),
            proxy_configuration: core::default::Default::default(),
            urls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
        KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElWebCrawlerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `crawl_depth` after provisioning.\n"]
    pub fn crawl_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.crawl_depth", self.base))
    }
    #[doc = "Get a reference to the value of field `max_content_size_per_page_in_mega_bytes` after provisioning.\n"]
    pub fn max_content_size_per_page_in_mega_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_content_size_per_page_in_mega_bytes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_links_per_page` after provisioning.\n"]
    pub fn max_links_per_page(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_links_per_page", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `max_urls_per_minute_crawl_rate` after provisioning.\n"]
    pub fn max_urls_per_minute_crawl_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_urls_per_minute_crawl_rate", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url_exclusion_patterns` after provisioning.\n"]
    pub fn url_exclusion_patterns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.url_exclusion_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `url_inclusion_patterns` after provisioning.\n"]
    pub fn url_inclusion_patterns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.url_inclusion_patterns", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authentication_configuration` after provisioning.\n"]
    pub fn authentication_configuration(
        &self,
    ) -> ListRef<
        KendraDataSourceConfigurationElWebCrawlerConfigurationElAuthenticationConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authentication_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(
        &self,
    ) -> ListRef<KendraDataSourceConfigurationElWebCrawlerConfigurationElProxyConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `urls` after provisioning.\n"]
    pub fn urls(
        &self,
    ) -> ListRef<KendraDataSourceConfigurationElWebCrawlerConfigurationElUrlsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.urls", self.base))
    }
}
#[derive(Serialize, Default)]
struct KendraDataSourceConfigurationElDynamic {
    s3_configuration: Option<DynamicBlock<KendraDataSourceConfigurationElS3ConfigurationEl>>,
    template_configuration:
        Option<DynamicBlock<KendraDataSourceConfigurationElTemplateConfigurationEl>>,
    web_crawler_configuration:
        Option<DynamicBlock<KendraDataSourceConfigurationElWebCrawlerConfigurationEl>>,
}
#[derive(Serialize)]
pub struct KendraDataSourceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_configuration: Option<Vec<KendraDataSourceConfigurationElS3ConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template_configuration: Option<Vec<KendraDataSourceConfigurationElTemplateConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_crawler_configuration:
        Option<Vec<KendraDataSourceConfigurationElWebCrawlerConfigurationEl>>,
    dynamic: KendraDataSourceConfigurationElDynamic,
}
impl KendraDataSourceConfigurationEl {
    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl Into<BlockAssignable<KendraDataSourceConfigurationElS3ConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `template_configuration`.\n"]
    pub fn set_template_configuration(
        mut self,
        v: impl Into<BlockAssignable<KendraDataSourceConfigurationElTemplateConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.template_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.template_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `web_crawler_configuration`.\n"]
    pub fn set_web_crawler_configuration(
        mut self,
        v: impl Into<BlockAssignable<KendraDataSourceConfigurationElWebCrawlerConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.web_crawler_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.web_crawler_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for KendraDataSourceConfigurationEl {
    type O = BlockAssignable<KendraDataSourceConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceConfigurationEl {}
impl BuildKendraDataSourceConfigurationEl {
    pub fn build(self) -> KendraDataSourceConfigurationEl {
        KendraDataSourceConfigurationEl {
            s3_configuration: core::default::Default::default(),
            template_configuration: core::default::Default::default(),
            web_crawler_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KendraDataSourceConfigurationElRef {
        KendraDataSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(&self) -> ListRef<KendraDataSourceConfigurationElS3ConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `template_configuration` after provisioning.\n"]
    pub fn template_configuration(
        &self,
    ) -> ListRef<KendraDataSourceConfigurationElTemplateConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.template_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `web_crawler_configuration` after provisioning.\n"]
    pub fn web_crawler_configuration(
        &self,
    ) -> ListRef<KendraDataSourceConfigurationElWebCrawlerConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_crawler_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    date_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_list_value: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl { # [doc = "Set the field `date_value`.\n"] pub fn set_date_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . date_value = Some (v . into ()) ; self } # [doc = "Set the field `long_value`.\n"] pub fn set_long_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . long_value = Some (v . into ()) ; self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < SetField < PrimField < String > > >) -> Self { self . string_list_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\n"] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } }
impl ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl { type O = BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl
{}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl { pub fn build (self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl { KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl { date_value : core :: default :: Default :: default () , long_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , } } }
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef { fn new (shared : StackShared , base : String) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef { KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef { shared : shared , base : base . to_string () , } } }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `date_value` after provisioning.\n"] pub fn date_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.date_value" , self . base)) } # [doc = "Get a reference to the value of field `long_value` after provisioning.\n"] pub fn long_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.long_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\n"] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } }
#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElDynamic { condition_on_value : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl >> , }
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl { condition_document_attribute_key : PrimField < String > , operator : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] condition_on_value : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl > > , dynamic : KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElDynamic , }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
    #[doc = "Set the field `condition_on_value`.\n"]
    pub fn set_condition_on_value(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition_on_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition_on_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl
{
    type O = BlockAssignable<
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl
{
    #[doc = ""]
    pub condition_document_attribute_key: PrimField<String>,
    #[doc = ""]
    pub operator: PrimField<String>,
}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl
    {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl {
            condition_document_attribute_key: self.condition_document_attribute_key,
            operator: self.operator,
            condition_on_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef
    {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef { shared : shared , base : base . to_string () , }
    }
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `condition_document_attribute_key` after provisioning.\n"]
    pub fn condition_document_attribute_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.condition_document_attribute_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
    #[doc = "Get a reference to the value of field `condition_on_value` after provisioning.\n"]    pub fn condition_on_value (& self) -> ListRef < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElConditionOnValueElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.condition_on_value", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    date_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_list_value: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl { # [doc = "Set the field `date_value`.\n"] pub fn set_date_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . date_value = Some (v . into ()) ; self } # [doc = "Set the field `long_value`.\n"] pub fn set_long_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . long_value = Some (v . into ()) ; self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < SetField < PrimField < String > > >) -> Self { self . string_list_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\n"] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } }
impl ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl { type O = BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl
{}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl { pub fn build (self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl { KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl { date_value : core :: default :: Default :: default () , long_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , } } }
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef { fn new (shared : StackShared , base : String) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef { KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef { shared : shared , base : base . to_string () , } } }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `date_value` after provisioning.\n"] pub fn date_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.date_value" , self . base)) } # [doc = "Get a reference to the value of field `long_value` after provisioning.\n"] pub fn long_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.long_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\n"] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } }
#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElDynamic { target_document_attribute_value : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl >> , }
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl { # [serde (skip_serializing_if = "Option::is_none")] target_document_attribute_key : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] target_document_attribute_value_deletion : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] target_document_attribute_value : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl > > , dynamic : KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElDynamic , }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
    #[doc = "Set the field `target_document_attribute_key`.\n"]
    pub fn set_target_document_attribute_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_document_attribute_key = Some(v.into());
        self
    }
    #[doc = "Set the field `target_document_attribute_value_deletion`.\n"]
    pub fn set_target_document_attribute_value_deletion(
        mut self,
        v: impl Into<PrimField<bool>>,
    ) -> Self {
        self.target_document_attribute_value_deletion = Some(v.into());
        self
    }
    #[doc = "Set the field `target_document_attribute_value`.\n"]
    pub fn set_target_document_attribute_value(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target_document_attribute_value = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target_document_attribute_value = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl
{
    type O = BlockAssignable<
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl
{}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl {
            target_document_attribute_key: core::default::Default::default(),
            target_document_attribute_value_deletion: core::default::Default::default(),
            target_document_attribute_value: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef
    {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `target_document_attribute_key` after provisioning.\n"]
    pub fn target_document_attribute_key(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_document_attribute_key", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_document_attribute_value_deletion` after provisioning.\n"]
    pub fn target_document_attribute_value_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_document_attribute_value_deletion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_document_attribute_value` after provisioning.\n"]    pub fn target_document_attribute_value (& self) -> ListRef < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElTargetDocumentAttributeValueElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.target_document_attribute_value", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElDynamic { condition : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl >> , target : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl >> , }
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl { # [serde (skip_serializing_if = "Option::is_none")] document_content_deletion : Option < PrimField < bool > > , # [serde (skip_serializing_if = "Option::is_none")] condition : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl > > , # [serde (skip_serializing_if = "Option::is_none")] target : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl > > , dynamic : KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElDynamic , }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
    #[doc = "Set the field `document_content_deletion`.\n"]
    pub fn set_document_content_deletion(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.document_content_deletion = Some(v.into());
        self
    }
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl
{
    type O = BlockAssignable<
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl {
            document_content_deletion: core::default::Default::default(),
            condition: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `document_content_deletion` after provisioning.\n"]
    pub fn document_content_deletion(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.document_content_deletion", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(
        &self,
    ) -> ListRef<
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElConditionElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(
        &self,
    ) -> ListRef<
        KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsElTargetElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    date_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_list_value: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { # [doc = "Set the field `date_value`.\n"] pub fn set_date_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . date_value = Some (v . into ()) ; self } # [doc = "Set the field `long_value`.\n"] pub fn set_long_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . long_value = Some (v . into ()) ; self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < SetField < PrimField < String > > >) -> Self { self . string_list_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\n"] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } }
impl ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { type O = BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl
{}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { pub fn build (self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { date_value : core :: default :: Default :: default () , long_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , } } }
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef { fn new (shared : StackShared , base : String) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef { KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef { shared : shared , base : base . to_string () , } } }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `date_value` after provisioning.\n"] pub fn date_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.date_value" , self . base)) } # [doc = "Get a reference to the value of field `long_value` after provisioning.\n"] pub fn long_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.long_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\n"] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } }
#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElDynamic { condition_on_value : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl >> , }
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl { condition_document_attribute_key : PrimField < String > , operator : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] condition_on_value : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl > > , dynamic : KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElDynamic , }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl { # [doc = "Set the field `condition_on_value`.\n"] pub fn set_condition_on_value (mut self , v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . condition_on_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . condition_on_value = Some (d) ; } } self } }
impl ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl { type O = BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl
{
    #[doc = ""]
    pub condition_document_attribute_key: PrimField<String>,
    #[doc = ""]
    pub operator: PrimField<String>,
}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl { pub fn build (self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl { KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl { condition_document_attribute_key : self . condition_document_attribute_key , operator : self . operator , condition_on_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef { fn new (shared : StackShared , base : String) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef { KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef { shared : shared , base : base . to_string () , } } }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `condition_document_attribute_key` after provisioning.\n"] pub fn condition_document_attribute_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.condition_document_attribute_key" , self . base)) } # [doc = "Get a reference to the value of field `operator` after provisioning.\n"] pub fn operator (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.operator" , self . base)) } # [doc = "Get a reference to the value of field `condition_on_value` after provisioning.\n"] pub fn condition_on_value (& self) -> ListRef < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.condition_on_value" , self . base)) } }
#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElDynamic { invocation_condition : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl >> , }
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl { lambda_arn : PrimField < String > , s3_bucket : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] invocation_condition : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl > > , dynamic : KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElDynamic , }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
    #[doc = "Set the field `invocation_condition`.\n"]
    pub fn set_invocation_condition(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.invocation_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.invocation_condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl
{
    type O = BlockAssignable<
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl
{
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
    #[doc = ""]
    pub s3_bucket: PrimField<String>,
}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl
    {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl {
            lambda_arn: self.lambda_arn,
            s3_bucket: self.s3_bucket,
            invocation_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef
    {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef { shared : shared , base : base . to_string () , }
    }
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `invocation_condition` after provisioning.\n"]    pub fn invocation_condition (& self) -> ListRef < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElInvocationConditionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.invocation_condition", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    date_value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_list_value: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_value: Option<PrimField<String>>,
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { # [doc = "Set the field `date_value`.\n"] pub fn set_date_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . date_value = Some (v . into ()) ; self } # [doc = "Set the field `long_value`.\n"] pub fn set_long_value (mut self , v : impl Into < PrimField < f64 > >) -> Self { self . long_value = Some (v . into ()) ; self } # [doc = "Set the field `string_list_value`.\n"] pub fn set_string_list_value (mut self , v : impl Into < SetField < PrimField < String > > >) -> Self { self . string_list_value = Some (v . into ()) ; self } # [doc = "Set the field `string_value`.\n"] pub fn set_string_value (mut self , v : impl Into < PrimField < String > >) -> Self { self . string_value = Some (v . into ()) ; self } }
impl ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { type O = BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl
{}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { pub fn build (self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl { date_value : core :: default :: Default :: default () , long_value : core :: default :: Default :: default () , string_list_value : core :: default :: Default :: default () , string_value : core :: default :: Default :: default () , } } }
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef { fn new (shared : StackShared , base : String) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef { KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef { shared : shared , base : base . to_string () , } } }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `date_value` after provisioning.\n"] pub fn date_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.date_value" , self . base)) } # [doc = "Get a reference to the value of field `long_value` after provisioning.\n"] pub fn long_value (& self) -> PrimExpr < f64 > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.long_value" , self . base)) } # [doc = "Get a reference to the value of field `string_list_value` after provisioning.\n"] pub fn string_list_value (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.string_list_value" , self . base)) } # [doc = "Get a reference to the value of field `string_value` after provisioning.\n"] pub fn string_value (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.string_value" , self . base)) } }
#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElDynamic { condition_on_value : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl >> , }
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl { condition_document_attribute_key : PrimField < String > , operator : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] condition_on_value : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl > > , dynamic : KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElDynamic , }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl { # [doc = "Set the field `condition_on_value`.\n"] pub fn set_condition_on_value (mut self , v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . condition_on_value = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . condition_on_value = Some (d) ; } } self } }
impl ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl { type O = BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl
{
    #[doc = ""]
    pub condition_document_attribute_key: PrimField<String>,
    #[doc = ""]
    pub operator: PrimField<String>,
}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl { pub fn build (self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl { KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl { condition_document_attribute_key : self . condition_document_attribute_key , operator : self . operator , condition_on_value : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef { fn new (shared : StackShared , base : String) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef { KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef { shared : shared , base : base . to_string () , } } }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `condition_document_attribute_key` after provisioning.\n"] pub fn condition_document_attribute_key (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.condition_document_attribute_key" , self . base)) } # [doc = "Get a reference to the value of field `operator` after provisioning.\n"] pub fn operator (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.operator" , self . base)) } # [doc = "Get a reference to the value of field `condition_on_value` after provisioning.\n"] pub fn condition_on_value (& self) -> ListRef < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElConditionOnValueElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.condition_on_value" , self . base)) } }
#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElDynamic { invocation_condition : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl >> , }
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl { lambda_arn : PrimField < String > , s3_bucket : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] invocation_condition : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl > > , dynamic : KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElDynamic , }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
    #[doc = "Set the field `invocation_condition`.\n"]
    pub fn set_invocation_condition(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.invocation_condition = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.invocation_condition = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl
{
    type O = BlockAssignable<
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl
{
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
    #[doc = ""]
    pub s3_bucket: PrimField<String>,
}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
    pub fn build(
        self,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl
    {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl {
            lambda_arn: self.lambda_arn,
            s3_bucket: self.s3_bucket,
            invocation_condition: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef
    {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `invocation_condition` after provisioning.\n"]    pub fn invocation_condition (& self) -> ListRef < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElInvocationConditionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.invocation_condition", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct KendraDataSourceCustomDocumentEnrichmentConfigurationElDynamic { inline_configurations : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl >> , post_extraction_hook_configuration : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl >> , pre_extraction_hook_configuration : Option < DynamicBlock < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl >> , }
#[derive(Serialize)]
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationEl { # [serde (skip_serializing_if = "Option::is_none")] role_arn : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] inline_configurations : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl > > , # [serde (skip_serializing_if = "Option::is_none")] post_extraction_hook_configuration : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl > > , # [serde (skip_serializing_if = "Option::is_none")] pre_extraction_hook_configuration : Option < Vec < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl > > , dynamic : KendraDataSourceCustomDocumentEnrichmentConfigurationElDynamic , }
impl KendraDataSourceCustomDocumentEnrichmentConfigurationEl {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `inline_configurations`.\n"]
    pub fn set_inline_configurations(
        mut self,
        v: impl Into<
            BlockAssignable<
                KendraDataSourceCustomDocumentEnrichmentConfigurationElInlineConfigurationsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.inline_configurations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.inline_configurations = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `post_extraction_hook_configuration`.\n"]
    pub fn set_post_extraction_hook_configuration(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.post_extraction_hook_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.post_extraction_hook_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `pre_extraction_hook_configuration`.\n"]
    pub fn set_pre_extraction_hook_configuration(
        mut self,
        v : impl Into < BlockAssignable < KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pre_extraction_hook_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_extraction_hook_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for KendraDataSourceCustomDocumentEnrichmentConfigurationEl {
    type O = BlockAssignable<KendraDataSourceCustomDocumentEnrichmentConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceCustomDocumentEnrichmentConfigurationEl {}
impl BuildKendraDataSourceCustomDocumentEnrichmentConfigurationEl {
    pub fn build(self) -> KendraDataSourceCustomDocumentEnrichmentConfigurationEl {
        KendraDataSourceCustomDocumentEnrichmentConfigurationEl {
            role_arn: core::default::Default::default(),
            inline_configurations: core::default::Default::default(),
            post_extraction_hook_configuration: core::default::Default::default(),
            pre_extraction_hook_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
        KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceCustomDocumentEnrichmentConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
    #[doc = "Get a reference to the value of field `post_extraction_hook_configuration` after provisioning.\n"]
    pub fn post_extraction_hook_configuration(
        &self,
    ) -> ListRef<
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPostExtractionHookConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.post_extraction_hook_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pre_extraction_hook_configuration` after provisioning.\n"]
    pub fn pre_extraction_hook_configuration(
        &self,
    ) -> ListRef<
        KendraDataSourceCustomDocumentEnrichmentConfigurationElPreExtractionHookConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.pre_extraction_hook_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct KendraDataSourceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl KendraDataSourceTimeoutsEl {
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
impl ToListMappable for KendraDataSourceTimeoutsEl {
    type O = BlockAssignable<KendraDataSourceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildKendraDataSourceTimeoutsEl {}
impl BuildKendraDataSourceTimeoutsEl {
    pub fn build(self) -> KendraDataSourceTimeoutsEl {
        KendraDataSourceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct KendraDataSourceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for KendraDataSourceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KendraDataSourceTimeoutsElRef {
        KendraDataSourceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl KendraDataSourceTimeoutsElRef {
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
struct KendraDataSourceDynamic {
    configuration: Option<DynamicBlock<KendraDataSourceConfigurationEl>>,
    custom_document_enrichment_configuration:
        Option<DynamicBlock<KendraDataSourceCustomDocumentEnrichmentConfigurationEl>>,
}
