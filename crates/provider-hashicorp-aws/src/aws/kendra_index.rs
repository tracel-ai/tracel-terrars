use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct KendraIndexData {
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
    edition: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_context_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_units: Option<Vec<KendraIndexCapacityUnitsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_metadata_configuration_updates:
        Option<Vec<KendraIndexDocumentMetadataConfigurationUpdatesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption_configuration:
        Option<Vec<KendraIndexServerSideEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<KendraIndexTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_group_resolution_configuration: Option<Vec<KendraIndexUserGroupResolutionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_token_configurations: Option<Vec<KendraIndexUserTokenConfigurationsEl>>,
    dynamic: KendraIndexDynamic,
}

struct KendraIndex_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<KendraIndexData>,
}

#[derive(Clone)]
pub struct KendraIndex(Rc<KendraIndex_>);

impl KendraIndex {
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

    #[doc = "Set the field `edition`.\n"]
    pub fn set_edition(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edition = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
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

    #[doc = "Set the field `user_context_policy`.\n"]
    pub fn set_user_context_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().user_context_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_units`.\n"]
    pub fn set_capacity_units(
        self,
        v: impl Into<BlockAssignable<KendraIndexCapacityUnitsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_units = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_units = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `document_metadata_configuration_updates`.\n"]
    pub fn set_document_metadata_configuration_updates(
        self,
        v: impl Into<BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .document_metadata_configuration_updates = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .document_metadata_configuration_updates = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `server_side_encryption_configuration`.\n"]
    pub fn set_server_side_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<KendraIndexServerSideEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .server_side_encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .server_side_encryption_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<KendraIndexTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `user_group_resolution_configuration`.\n"]
    pub fn set_user_group_resolution_configuration(
        self,
        v: impl Into<BlockAssignable<KendraIndexUserGroupResolutionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_group_resolution_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .user_group_resolution_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `user_token_configurations`.\n"]
    pub fn set_user_token_configurations(
        self,
        v: impl Into<BlockAssignable<KendraIndexUserTokenConfigurationsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().user_token_configurations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.user_token_configurations = Some(d);
            }
        }
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `index_statistics` after provisioning.\n"]
    pub fn index_statistics(&self) -> ListRef<KendraIndexIndexStatisticsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.index_statistics", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_context_policy` after provisioning.\n"]
    pub fn user_context_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_context_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> ListRef<KendraIndexCapacityUnitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_units", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> ListRef<KendraIndexServerSideEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.server_side_encryption_configuration",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraIndexTimeoutsElRef {
        KendraIndexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(
        &self,
    ) -> ListRef<KendraIndexUserGroupResolutionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> ListRef<KendraIndexUserTokenConfigurationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_token_configurations", self.extract_ref()),
        )
    }
}

impl Referable for KendraIndex {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for KendraIndex {}

impl ToListMappable for KendraIndex {
    type O = ListRef<KendraIndexRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for KendraIndex_ {
    fn extract_resource_type(&self) -> String {
        "aws_kendra_index".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKendraIndex {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildKendraIndex {
    pub fn build(self, stack: &mut Stack) -> KendraIndex {
        let out = KendraIndex(Rc::new(KendraIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(KendraIndexData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                edition: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                user_context_policy: core::default::Default::default(),
                capacity_units: core::default::Default::default(),
                document_metadata_configuration_updates: core::default::Default::default(),
                server_side_encryption_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                user_group_resolution_configuration: core::default::Default::default(),
                user_token_configurations: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KendraIndexRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl KendraIndexRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `edition` after provisioning.\n"]
    pub fn edition(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edition", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `index_statistics` after provisioning.\n"]
    pub fn index_statistics(&self) -> ListRef<KendraIndexIndexStatisticsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.index_statistics", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_context_policy` after provisioning.\n"]
    pub fn user_context_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_context_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_units` after provisioning.\n"]
    pub fn capacity_units(&self) -> ListRef<KendraIndexCapacityUnitsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_units", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> ListRef<KendraIndexServerSideEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.server_side_encryption_configuration",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KendraIndexTimeoutsElRef {
        KendraIndexTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_configuration` after provisioning.\n"]
    pub fn user_group_resolution_configuration(
        &self,
    ) -> ListRef<KendraIndexUserGroupResolutionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `user_token_configurations` after provisioning.\n"]
    pub fn user_token_configurations(&self) -> ListRef<KendraIndexUserTokenConfigurationsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.user_token_configurations", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct KendraIndexIndexStatisticsElFaqStatisticsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed_question_answers_count: Option<PrimField<f64>>,
}

impl KendraIndexIndexStatisticsElFaqStatisticsEl {
    #[doc = "Set the field `indexed_question_answers_count`.\n"]
    pub fn set_indexed_question_answers_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.indexed_question_answers_count = Some(v.into());
        self
    }
}

impl ToListMappable for KendraIndexIndexStatisticsElFaqStatisticsEl {
    type O = BlockAssignable<KendraIndexIndexStatisticsElFaqStatisticsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexIndexStatisticsElFaqStatisticsEl {}

impl BuildKendraIndexIndexStatisticsElFaqStatisticsEl {
    pub fn build(self) -> KendraIndexIndexStatisticsElFaqStatisticsEl {
        KendraIndexIndexStatisticsElFaqStatisticsEl {
            indexed_question_answers_count: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexIndexStatisticsElFaqStatisticsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexIndexStatisticsElFaqStatisticsElRef {
    fn new(shared: StackShared, base: String) -> KendraIndexIndexStatisticsElFaqStatisticsElRef {
        KendraIndexIndexStatisticsElFaqStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexIndexStatisticsElFaqStatisticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_question_answers_count` after provisioning.\n"]
    pub fn indexed_question_answers_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.indexed_question_answers_count", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed_text_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed_text_documents_count: Option<PrimField<f64>>,
}

impl KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    #[doc = "Set the field `indexed_text_bytes`.\n"]
    pub fn set_indexed_text_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.indexed_text_bytes = Some(v.into());
        self
    }

    #[doc = "Set the field `indexed_text_documents_count`.\n"]
    pub fn set_indexed_text_documents_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.indexed_text_documents_count = Some(v.into());
        self
    }
}

impl ToListMappable for KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    type O = BlockAssignable<KendraIndexIndexStatisticsElTextDocumentStatisticsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexIndexStatisticsElTextDocumentStatisticsEl {}

impl BuildKendraIndexIndexStatisticsElTextDocumentStatisticsEl {
    pub fn build(self) -> KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
        KendraIndexIndexStatisticsElTextDocumentStatisticsEl {
            indexed_text_bytes: core::default::Default::default(),
            indexed_text_documents_count: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
        KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexIndexStatisticsElTextDocumentStatisticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `indexed_text_bytes` after provisioning.\n"]
    pub fn indexed_text_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.indexed_text_bytes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `indexed_text_documents_count` after provisioning.\n"]
    pub fn indexed_text_documents_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.indexed_text_documents_count", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KendraIndexIndexStatisticsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    faq_statistics: Option<ListField<KendraIndexIndexStatisticsElFaqStatisticsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_document_statistics:
        Option<ListField<KendraIndexIndexStatisticsElTextDocumentStatisticsEl>>,
}

impl KendraIndexIndexStatisticsEl {
    #[doc = "Set the field `faq_statistics`.\n"]
    pub fn set_faq_statistics(
        mut self,
        v: impl Into<ListField<KendraIndexIndexStatisticsElFaqStatisticsEl>>,
    ) -> Self {
        self.faq_statistics = Some(v.into());
        self
    }

    #[doc = "Set the field `text_document_statistics`.\n"]
    pub fn set_text_document_statistics(
        mut self,
        v: impl Into<ListField<KendraIndexIndexStatisticsElTextDocumentStatisticsEl>>,
    ) -> Self {
        self.text_document_statistics = Some(v.into());
        self
    }
}

impl ToListMappable for KendraIndexIndexStatisticsEl {
    type O = BlockAssignable<KendraIndexIndexStatisticsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexIndexStatisticsEl {}

impl BuildKendraIndexIndexStatisticsEl {
    pub fn build(self) -> KendraIndexIndexStatisticsEl {
        KendraIndexIndexStatisticsEl {
            faq_statistics: core::default::Default::default(),
            text_document_statistics: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexIndexStatisticsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexIndexStatisticsElRef {
    fn new(shared: StackShared, base: String) -> KendraIndexIndexStatisticsElRef {
        KendraIndexIndexStatisticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexIndexStatisticsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `faq_statistics` after provisioning.\n"]
    pub fn faq_statistics(&self) -> ListRef<KendraIndexIndexStatisticsElFaqStatisticsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.faq_statistics", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `text_document_statistics` after provisioning.\n"]
    pub fn text_document_statistics(
        &self,
    ) -> ListRef<KendraIndexIndexStatisticsElTextDocumentStatisticsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.text_document_statistics", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KendraIndexCapacityUnitsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_capacity_units: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_capacity_units: Option<PrimField<f64>>,
}

impl KendraIndexCapacityUnitsEl {
    #[doc = "Set the field `query_capacity_units`.\n"]
    pub fn set_query_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.query_capacity_units = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_capacity_units`.\n"]
    pub fn set_storage_capacity_units(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.storage_capacity_units = Some(v.into());
        self
    }
}

impl ToListMappable for KendraIndexCapacityUnitsEl {
    type O = BlockAssignable<KendraIndexCapacityUnitsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexCapacityUnitsEl {}

impl BuildKendraIndexCapacityUnitsEl {
    pub fn build(self) -> KendraIndexCapacityUnitsEl {
        KendraIndexCapacityUnitsEl {
            query_capacity_units: core::default::Default::default(),
            storage_capacity_units: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexCapacityUnitsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexCapacityUnitsElRef {
    fn new(shared: StackShared, base: String) -> KendraIndexCapacityUnitsElRef {
        KendraIndexCapacityUnitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexCapacityUnitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_capacity_units` after provisioning.\n"]
    pub fn query_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_capacity_units", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `storage_capacity_units` after provisioning.\n"]
    pub fn storage_capacity_units(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_capacity_units", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    freshness: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    importance: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rank_order: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values_importance_map: Option<RecField<PrimField<f64>>>,
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    #[doc = "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.duration = Some(v.into());
        self
    }

    #[doc = "Set the field `freshness`.\n"]
    pub fn set_freshness(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.freshness = Some(v.into());
        self
    }

    #[doc = "Set the field `importance`.\n"]
    pub fn set_importance(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.importance = Some(v.into());
        self
    }

    #[doc = "Set the field `rank_order`.\n"]
    pub fn set_rank_order(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rank_order = Some(v.into());
        self
    }

    #[doc = "Set the field `values_importance_map`.\n"]
    pub fn set_values_importance_map(mut self, v: impl Into<RecField<PrimField<f64>>>) -> Self {
        self.values_importance_map = Some(v.into());
        self
    }
}

impl ToListMappable for KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    type O = BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {}

impl BuildKendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
    pub fn build(self) -> KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
        KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl {
            duration: core::default::Default::default(),
            freshness: core::default::Default::default(),
            importance: core::default::Default::default(),
            rank_order: core::default::Default::default(),
            values_importance_map: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
        KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `freshness` after provisioning.\n"]
    pub fn freshness(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.freshness", self.base))
    }

    #[doc = "Get a reference to the value of field `importance` after provisioning.\n"]
    pub fn importance(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.importance", self.base))
    }

    #[doc = "Get a reference to the value of field `rank_order` after provisioning.\n"]
    pub fn rank_order(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rank_order", self.base))
    }

    #[doc = "Get a reference to the value of field `values_importance_map` after provisioning.\n"]
    pub fn values_importance_map(&self) -> RecRef<PrimExpr<f64>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.values_importance_map", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    displayable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facetable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    searchable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sortable: Option<PrimField<bool>>,
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    #[doc = "Set the field `displayable`.\n"]
    pub fn set_displayable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.displayable = Some(v.into());
        self
    }

    #[doc = "Set the field `facetable`.\n"]
    pub fn set_facetable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.facetable = Some(v.into());
        self
    }

    #[doc = "Set the field `searchable`.\n"]
    pub fn set_searchable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.searchable = Some(v.into());
        self
    }

    #[doc = "Set the field `sortable`.\n"]
    pub fn set_sortable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.sortable = Some(v.into());
        self
    }
}

impl ToListMappable for KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    type O = BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {}

impl BuildKendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
    pub fn build(self) -> KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
        KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl {
            displayable: core::default::Default::default(),
            facetable: core::default::Default::default(),
            searchable: core::default::Default::default(),
            sortable: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
        KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `displayable` after provisioning.\n"]
    pub fn displayable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.displayable", self.base))
    }

    #[doc = "Get a reference to the value of field `facetable` after provisioning.\n"]
    pub fn facetable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.facetable", self.base))
    }

    #[doc = "Get a reference to the value of field `searchable` after provisioning.\n"]
    pub fn searchable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.searchable", self.base))
    }

    #[doc = "Get a reference to the value of field `sortable` after provisioning.\n"]
    pub fn sortable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.sortable", self.base))
    }
}

#[derive(Serialize, Default)]
struct KendraIndexDocumentMetadataConfigurationUpdatesElDynamic {
    relevance: Option<DynamicBlock<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    search: Option<DynamicBlock<KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
}

#[derive(Serialize)]
pub struct KendraIndexDocumentMetadataConfigurationUpdatesEl {
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relevance: Option<Vec<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<Vec<KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
    dynamic: KendraIndexDocumentMetadataConfigurationUpdatesElDynamic,
}

impl KendraIndexDocumentMetadataConfigurationUpdatesEl {
    #[doc = "Set the field `relevance`.\n"]
    pub fn set_relevance(
        mut self,
        v: impl Into<BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.relevance = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.relevance = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `search`.\n"]
    pub fn set_search(
        mut self,
        v: impl Into<BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesElSearchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.search = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.search = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KendraIndexDocumentMetadataConfigurationUpdatesEl {
    type O = BlockAssignable<KendraIndexDocumentMetadataConfigurationUpdatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexDocumentMetadataConfigurationUpdatesEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildKendraIndexDocumentMetadataConfigurationUpdatesEl {
    pub fn build(self) -> KendraIndexDocumentMetadataConfigurationUpdatesEl {
        KendraIndexDocumentMetadataConfigurationUpdatesEl {
            name: self.name,
            type_: self.type_,
            relevance: core::default::Default::default(),
            search: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KendraIndexDocumentMetadataConfigurationUpdatesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexDocumentMetadataConfigurationUpdatesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraIndexDocumentMetadataConfigurationUpdatesElRef {
        KendraIndexDocumentMetadataConfigurationUpdatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexDocumentMetadataConfigurationUpdatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `relevance` after provisioning.\n"]
    pub fn relevance(
        &self,
    ) -> ListRef<KendraIndexDocumentMetadataConfigurationUpdatesElRelevanceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.relevance", self.base))
    }

    #[doc = "Get a reference to the value of field `search` after provisioning.\n"]
    pub fn search(&self) -> ListRef<KendraIndexDocumentMetadataConfigurationUpdatesElSearchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.search", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexServerSideEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
}

impl KendraIndexServerSideEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }
}

impl ToListMappable for KendraIndexServerSideEncryptionConfigurationEl {
    type O = BlockAssignable<KendraIndexServerSideEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexServerSideEncryptionConfigurationEl {}

impl BuildKendraIndexServerSideEncryptionConfigurationEl {
    pub fn build(self) -> KendraIndexServerSideEncryptionConfigurationEl {
        KendraIndexServerSideEncryptionConfigurationEl {
            kms_key_id: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexServerSideEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexServerSideEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KendraIndexServerSideEncryptionConfigurationElRef {
        KendraIndexServerSideEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexServerSideEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct KendraIndexTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl KendraIndexTimeoutsEl {
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

impl ToListMappable for KendraIndexTimeoutsEl {
    type O = BlockAssignable<KendraIndexTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexTimeoutsEl {}

impl BuildKendraIndexTimeoutsEl {
    pub fn build(self) -> KendraIndexTimeoutsEl {
        KendraIndexTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> KendraIndexTimeoutsElRef {
        KendraIndexTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexTimeoutsElRef {
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

#[derive(Serialize)]
pub struct KendraIndexUserGroupResolutionConfigurationEl {
    user_group_resolution_mode: PrimField<String>,
}

impl KendraIndexUserGroupResolutionConfigurationEl {}

impl ToListMappable for KendraIndexUserGroupResolutionConfigurationEl {
    type O = BlockAssignable<KendraIndexUserGroupResolutionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexUserGroupResolutionConfigurationEl {
    #[doc = ""]
    pub user_group_resolution_mode: PrimField<String>,
}

impl BuildKendraIndexUserGroupResolutionConfigurationEl {
    pub fn build(self) -> KendraIndexUserGroupResolutionConfigurationEl {
        KendraIndexUserGroupResolutionConfigurationEl {
            user_group_resolution_mode: self.user_group_resolution_mode,
        }
    }
}

pub struct KendraIndexUserGroupResolutionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexUserGroupResolutionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> KendraIndexUserGroupResolutionConfigurationElRef {
        KendraIndexUserGroupResolutionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexUserGroupResolutionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `user_group_resolution_mode` after provisioning.\n"]
    pub fn user_group_resolution_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_group_resolution_mode", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    group_attribute_field: PrimField<String>,
    user_name_attribute_field: PrimField<String>,
}

impl KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {}

impl ToListMappable for KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    type O = BlockAssignable<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    #[doc = ""]
    pub group_attribute_field: PrimField<String>,
    #[doc = ""]
    pub user_name_attribute_field: PrimField<String>,
}

impl BuildKendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
    pub fn build(self) -> KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
        KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl {
            group_attribute_field: self.group_attribute_field,
            user_name_attribute_field: self.user_name_attribute_field,
        }
    }
}

pub struct KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
        KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_attribute_field` after provisioning.\n"]
    pub fn group_attribute_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_attribute_field", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_name_attribute_field` after provisioning.\n"]
    pub fn user_name_attribute_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_name_attribute_field", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    claim_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_attribute_field: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuer: Option<PrimField<String>>,
    key_location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secrets_manager_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_name_attribute_field: Option<PrimField<String>>,
}

impl KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[doc = "Set the field `claim_regex`.\n"]
    pub fn set_claim_regex(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.claim_regex = Some(v.into());
        self
    }

    #[doc = "Set the field `group_attribute_field`.\n"]
    pub fn set_group_attribute_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.group_attribute_field = Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\n"]
    pub fn set_issuer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.issuer = Some(v.into());
        self
    }

    #[doc = "Set the field `secrets_manager_arn`.\n"]
    pub fn set_secrets_manager_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secrets_manager_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }

    #[doc = "Set the field `user_name_attribute_field`.\n"]
    pub fn set_user_name_attribute_field(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_name_attribute_field = Some(v.into());
        self
    }
}

impl ToListMappable for KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    type O = BlockAssignable<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    #[doc = ""]
    pub key_location: PrimField<String>,
}

impl BuildKendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
    pub fn build(self) -> KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
        KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl {
            claim_regex: core::default::Default::default(),
            group_attribute_field: core::default::Default::default(),
            issuer: core::default::Default::default(),
            key_location: self.key_location,
            secrets_manager_arn: core::default::Default::default(),
            url: core::default::Default::default(),
            user_name_attribute_field: core::default::Default::default(),
        }
    }
}

pub struct KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
        KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_regex` after provisioning.\n"]
    pub fn claim_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.claim_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `group_attribute_field` after provisioning.\n"]
    pub fn group_attribute_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.group_attribute_field", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\n"]
    pub fn issuer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `key_location` after provisioning.\n"]
    pub fn key_location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_location", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_manager_arn` after provisioning.\n"]
    pub fn secrets_manager_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.secrets_manager_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `user_name_attribute_field` after provisioning.\n"]
    pub fn user_name_attribute_field(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_name_attribute_field", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct KendraIndexUserTokenConfigurationsElDynamic {
    json_token_type_configuration:
        Option<DynamicBlock<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>>,
    jwt_token_type_configuration:
        Option<DynamicBlock<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
}

#[derive(Serialize)]
pub struct KendraIndexUserTokenConfigurationsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_token_type_configuration:
        Option<Vec<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jwt_token_type_configuration:
        Option<Vec<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
    dynamic: KendraIndexUserTokenConfigurationsElDynamic,
}

impl KendraIndexUserTokenConfigurationsEl {
    #[doc = "Set the field `json_token_type_configuration`.\n"]
    pub fn set_json_token_type_configuration(
        mut self,
        v: impl Into<BlockAssignable<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_token_type_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_token_type_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `jwt_token_type_configuration`.\n"]
    pub fn set_jwt_token_type_configuration(
        mut self,
        v: impl Into<BlockAssignable<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.jwt_token_type_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt_token_type_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for KendraIndexUserTokenConfigurationsEl {
    type O = BlockAssignable<KendraIndexUserTokenConfigurationsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKendraIndexUserTokenConfigurationsEl {}

impl BuildKendraIndexUserTokenConfigurationsEl {
    pub fn build(self) -> KendraIndexUserTokenConfigurationsEl {
        KendraIndexUserTokenConfigurationsEl {
            json_token_type_configuration: core::default::Default::default(),
            jwt_token_type_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct KendraIndexUserTokenConfigurationsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for KendraIndexUserTokenConfigurationsElRef {
    fn new(shared: StackShared, base: String) -> KendraIndexUserTokenConfigurationsElRef {
        KendraIndexUserTokenConfigurationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KendraIndexUserTokenConfigurationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `json_token_type_configuration` after provisioning.\n"]
    pub fn json_token_type_configuration(
        &self,
    ) -> ListRef<KendraIndexUserTokenConfigurationsElJsonTokenTypeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.json_token_type_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `jwt_token_type_configuration` after provisioning.\n"]
    pub fn jwt_token_type_configuration(
        &self,
    ) -> ListRef<KendraIndexUserTokenConfigurationsElJwtTokenTypeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jwt_token_type_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct KendraIndexDynamic {
    capacity_units: Option<DynamicBlock<KendraIndexCapacityUnitsEl>>,
    document_metadata_configuration_updates:
        Option<DynamicBlock<KendraIndexDocumentMetadataConfigurationUpdatesEl>>,
    server_side_encryption_configuration:
        Option<DynamicBlock<KendraIndexServerSideEncryptionConfigurationEl>>,
    user_group_resolution_configuration:
        Option<DynamicBlock<KendraIndexUserGroupResolutionConfigurationEl>>,
    user_token_configurations: Option<DynamicBlock<KendraIndexUserTokenConfigurationsEl>>,
}
