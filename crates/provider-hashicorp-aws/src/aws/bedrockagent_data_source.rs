use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct BedrockagentDataSourceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    knowledge_base_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_source_configuration: Option<Vec<BedrockagentDataSourceDataSourceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption_configuration:
        Option<Vec<BedrockagentDataSourceServerSideEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BedrockagentDataSourceTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vector_ingestion_configuration:
        Option<Vec<BedrockagentDataSourceVectorIngestionConfigurationEl>>,
    dynamic: BedrockagentDataSourceDynamic,
}

struct BedrockagentDataSource_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BedrockagentDataSourceData>,
}

#[derive(Clone)]
pub struct BedrockagentDataSource(Rc<BedrockagentDataSource_>);

impl BedrockagentDataSource {
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

    #[doc = "Set the field `data_deletion_policy`.\n"]
    pub fn set_data_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_deletion_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_configuration`.\n"]
    pub fn set_data_source_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentDataSourceDataSourceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_source_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_source_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `server_side_encryption_configuration`.\n"]
    pub fn set_server_side_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentDataSourceServerSideEncryptionConfigurationEl>>,
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
    pub fn set_timeouts(self, v: impl Into<BedrockagentDataSourceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Set the field `vector_ingestion_configuration`.\n"]
    pub fn set_vector_ingestion_configuration(
        self,
        v: impl Into<BlockAssignable<BedrockagentDataSourceVectorIngestionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vector_ingestion_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .vector_ingestion_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `data_deletion_policy` after provisioning.\n"]
    pub fn data_deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_deletion_policy", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_id` after provisioning.\n"]
    pub fn knowledge_base_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.knowledge_base_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `data_source_configuration` after provisioning.\n"]
    pub fn data_source_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceServerSideEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.server_side_encryption_configuration",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentDataSourceTimeoutsElRef {
        BedrockagentDataSourceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vector_ingestion_configuration` after provisioning.\n"]
    pub fn vector_ingestion_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceVectorIngestionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vector_ingestion_configuration", self.extract_ref()),
        )
    }
}

impl Referable for BedrockagentDataSource {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for BedrockagentDataSource {}

impl ToListMappable for BedrockagentDataSource {
    type O = ListRef<BedrockagentDataSourceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BedrockagentDataSource_ {
    fn extract_resource_type(&self) -> String {
        "aws_bedrockagent_data_source".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentDataSource {
    pub tf_id: String,
    #[doc = ""]
    pub knowledge_base_id: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildBedrockagentDataSource {
    pub fn build(self, stack: &mut Stack) -> BedrockagentDataSource {
        let out = BedrockagentDataSource(Rc::new(BedrockagentDataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BedrockagentDataSourceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                knowledge_base_id: self.knowledge_base_id,
                name: self.name,
                region: core::default::Default::default(),
                data_source_configuration: core::default::Default::default(),
                server_side_encryption_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vector_ingestion_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentDataSourceRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl BedrockagentDataSourceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_deletion_policy` after provisioning.\n"]
    pub fn data_deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_deletion_policy", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `knowledge_base_id` after provisioning.\n"]
    pub fn knowledge_base_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.knowledge_base_id", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `data_source_configuration` after provisioning.\n"]
    pub fn data_source_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_source_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `server_side_encryption_configuration` after provisioning.\n"]
    pub fn server_side_encryption_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceServerSideEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.server_side_encryption_configuration",
                self.extract_ref()
            ),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentDataSourceTimeoutsElRef {
        BedrockagentDataSourceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vector_ingestion_configuration` after provisioning.\n"]
    pub fn vector_ingestion_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceVectorIngestionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vector_ingestion_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_filters: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion_filters: Option<SetField<PrimField<String>>>,
    object_type: PrimField<String>,
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
    #[doc = "Set the field `exclusion_filters`.\n"]
    pub fn set_exclusion_filters(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclusion_filters = Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion_filters`.\n"]
    pub fn set_inclusion_filters(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inclusion_filters = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl
{
    #[doc = ""]
    pub object_type: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
            exclusion_filters: core::default::Default::default(),
            inclusion_filters: core::default::Default::default(),
            object_type: self.object_type,
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_filters` after provisioning.\n"]
    pub fn exclusion_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_filters` after provisioning.\n"]
    pub fn inclusion_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.inclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `object_type` after provisioning.\n"]
    pub fn object_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElDynamic {
    filters: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
            filters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElDynamic {
    pattern_object_filter: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern_object_filter: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    #[doc = "Set the field `pattern_object_filter`.\n"]
    pub fn set_pattern_object_filter(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pattern_object_filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pattern_object_filter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
            type_: self.type_,
            pattern_object_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern_object_filter` after provisioning.\n"]
    pub fn pattern_object_filter(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.pattern_object_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElDynamic {
    filter_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_configuration: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElDynamic,
}

impl
    BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl
{
    #[doc = "Set the field `filter_configuration`.\n"]
    pub fn set_filter_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl {
            filter_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_configuration` after provisioning.\n"]
    pub fn filter_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElFilterConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.filter_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl
{
    auth_type: PrimField<String>,
    credentials_secret_arn: PrimField<String>,
    host_type: PrimField<String>,
    host_url: PrimField<String>,
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl {}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl
{
    #[doc = ""]
    pub auth_type: PrimField<String>,
    #[doc = ""]
    pub credentials_secret_arn: PrimField<String>,
    #[doc = ""]
    pub host_type: PrimField<String>,
    #[doc = ""]
    pub host_url: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl {
            auth_type: self.auth_type,
            credentials_secret_arn: self.credentials_secret_arn,
            host_type: self.host_type,
            host_url: self.host_url,
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials_secret_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `host_type` after provisioning.\n"]
    pub fn host_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_type", self.base))
    }

    #[doc = "Get a reference to the value of field `host_url` after provisioning.\n"]
    pub fn host_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElDynamic {
    crawler_configuration: Option<
        DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl>,
    >,
    source_configuration: Option<
        DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_configuration: Option<
        Vec<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_configuration: Option<
        Vec<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl>,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.crawler_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl {
    type O =
        BlockAssignable<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl {}

impl BuildBedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl {
    pub fn build(self) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl {
            crawler_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElCrawlerConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.crawler_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElSourceConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl {
    bucket_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_owner_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion_prefixes: Option<SetField<PrimField<String>>>,
}

impl BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl {
    #[doc = "Set the field `bucket_owner_account_id`.\n"]
    pub fn set_bucket_owner_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_owner_account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion_prefixes`.\n"]
    pub fn set_inclusion_prefixes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inclusion_prefixes = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl {
    type O = BlockAssignable<BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl {
    #[doc = ""]
    pub bucket_arn: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl {
    pub fn build(self) -> BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl {
            bucket_arn: self.bucket_arn,
            bucket_owner_account_id: core::default::Default::default(),
            inclusion_prefixes: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_arn` after provisioning.\n"]
    pub fn bucket_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_owner_account_id` after provisioning.\n"]
    pub fn bucket_owner_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_owner_account_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `inclusion_prefixes` after provisioning.\n"]
    pub fn inclusion_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.inclusion_prefixes", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_filters: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion_filters: Option<SetField<PrimField<String>>>,
    object_type: PrimField<String>,
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
    #[doc = "Set the field `exclusion_filters`.\n"]
    pub fn set_exclusion_filters(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclusion_filters = Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion_filters`.\n"]
    pub fn set_inclusion_filters(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inclusion_filters = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl
{
    #[doc = ""]
    pub object_type: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
            exclusion_filters: core::default::Default::default(),
            inclusion_filters: core::default::Default::default(),
            object_type: self.object_type,
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_filters` after provisioning.\n"]
    pub fn exclusion_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_filters` after provisioning.\n"]
    pub fn inclusion_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.inclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `object_type` after provisioning.\n"]
    pub fn object_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElDynamic {
    filters: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
            filters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElDynamic {
    pattern_object_filter: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern_object_filter: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    #[doc = "Set the field `pattern_object_filter`.\n"]
    pub fn set_pattern_object_filter(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pattern_object_filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pattern_object_filter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl {
            type_: self.type_,
            pattern_object_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern_object_filter` after provisioning.\n"]
    pub fn pattern_object_filter(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.pattern_object_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElDynamic {
    filter_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_configuration: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElDynamic,
}

impl
    BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl
{
    #[doc = "Set the field `filter_configuration`.\n"]
    pub fn set_filter_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl {
            filter_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_configuration` after provisioning.\n"]
    pub fn filter_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElFilterConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.filter_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl
{
    auth_type: PrimField<String>,
    credentials_secret_arn: PrimField<String>,
    host_url: PrimField<String>,
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl {}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl
{
    #[doc = ""]
    pub auth_type: PrimField<String>,
    #[doc = ""]
    pub credentials_secret_arn: PrimField<String>,
    #[doc = ""]
    pub host_url: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl {
            auth_type: self.auth_type,
            credentials_secret_arn: self.credentials_secret_arn,
            host_url: self.host_url,
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials_secret_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `host_url` after provisioning.\n"]
    pub fn host_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElDynamic {
    crawler_configuration: Option<
        DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl>,
    >,
    source_configuration: Option<
        DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_configuration: Option<
        Vec<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_configuration: Option<
        Vec<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl>,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.crawler_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl {
    type O =
        BlockAssignable<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl {}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl {
    pub fn build(self) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl {
            crawler_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElCrawlerConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.crawler_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElSourceConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_filters: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion_filters: Option<SetField<PrimField<String>>>,
    object_type: PrimField<String>,
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
    #[doc = "Set the field `exclusion_filters`.\n"]
    pub fn set_exclusion_filters(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclusion_filters = Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion_filters`.\n"]
    pub fn set_inclusion_filters(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inclusion_filters = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl
{
    #[doc = ""]
    pub object_type: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl {
            exclusion_filters: core::default::Default::default(),
            inclusion_filters: core::default::Default::default(),
            object_type: self.object_type,
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_filters` after provisioning.\n"]
    pub fn exclusion_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.exclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusion_filters` after provisioning.\n"]
    pub fn inclusion_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.inclusion_filters", self.base))
    }

    #[doc = "Get a reference to the value of field `object_type` after provisioning.\n"]
    pub fn object_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElDynamic {
    filters: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filters = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filters = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl {
            filters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElFiltersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.filters", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElDynamic {
    pattern_object_filter: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern_object_filter: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    #[doc = "Set the field `pattern_object_filter`.\n"]
    pub fn set_pattern_object_filter(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.pattern_object_filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.pattern_object_filter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl
{
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl {
            type_: self.type_,
            pattern_object_filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern_object_filter` after provisioning.\n"]
    pub fn pattern_object_filter(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElPatternObjectFilterElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.pattern_object_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElDynamic {
    filter_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_configuration: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElDynamic,
}

impl
    BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl
{
    #[doc = "Set the field `filter_configuration`.\n"]
    pub fn set_filter_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl {
            filter_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_configuration` after provisioning.\n"]
    pub fn filter_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElFilterConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.filter_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl
{
    auth_type: PrimField<String>,
    credentials_secret_arn: PrimField<String>,
    domain: PrimField<String>,
    host_type: PrimField<String>,
    site_urls: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tenant_id: Option<PrimField<String>>,
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl {
    #[doc = "Set the field `tenant_id`.\n"]
    pub fn set_tenant_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tenant_id = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl
{
    #[doc = ""]
    pub auth_type: PrimField<String>,
    #[doc = ""]
    pub credentials_secret_arn: PrimField<String>,
    #[doc = ""]
    pub domain: PrimField<String>,
    #[doc = ""]
    pub host_type: PrimField<String>,
    #[doc = ""]
    pub site_urls: SetField<PrimField<String>>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl {
            auth_type: self.auth_type,
            credentials_secret_arn: self.credentials_secret_arn,
            domain: self.domain,
            host_type: self.host_type,
            site_urls: self.site_urls,
            tenant_id: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl
    BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_type` after provisioning.\n"]
    pub fn auth_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `credentials_secret_arn` after provisioning.\n"]
    pub fn credentials_secret_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.credentials_secret_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `host_type` after provisioning.\n"]
    pub fn host_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host_type", self.base))
    }

    #[doc = "Get a reference to the value of field `site_urls` after provisioning.\n"]
    pub fn site_urls(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.site_urls", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElDynamic {
    crawler_configuration: Option<
        DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl>,
    >,
    source_configuration: Option<
        DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_configuration: Option<
        Vec<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_configuration: Option<
        Vec<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl>,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.crawler_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl {
    type O =
        BlockAssignable<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl {}

impl BuildBedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl {
    pub fn build(self) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl {
            crawler_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElCrawlerConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.crawler_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElSourceConfigurationElRef>{
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    max_pages: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit: Option<PrimField<f64>>,
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl {
    #[doc = "Set the field `max_pages`.\n"]
    pub fn set_max_pages(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_pages = Some(v.into());
        self
    }

    #[doc = "Set the field `rate_limit`.\n"]
    pub fn set_rate_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.rate_limit = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl {
            max_pages: core::default::Default::default(),
            rate_limit: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsElRef {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_pages` after provisioning.\n"]
    pub fn max_pages(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_pages", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit` after provisioning.\n"]
    pub fn rate_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rate_limit", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElDynamic {
    crawler_limits: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusion_filters: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion_filters: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_agent: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_limits: Option<
        Vec<BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl>,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl {
    #[doc = "Set the field `exclusion_filters`.\n"]
    pub fn set_exclusion_filters(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.exclusion_filters = Some(v.into());
        self
    }

    #[doc = "Set the field `inclusion_filters`.\n"]
    pub fn set_inclusion_filters(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.inclusion_filters = Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.scope = Some(v.into());
        self
    }

    #[doc = "Set the field `user_agent`.\n"]
    pub fn set_user_agent(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.user_agent = Some(v.into());
        self
    }

    #[doc = "Set the field `crawler_limits`.\n"]
    pub fn set_crawler_limits(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.crawler_limits = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_limits = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl
    {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl {
            exclusion_filters: core::default::Default::default(),
            inclusion_filters: core::default::Default::default(),
            scope: core::default::Default::default(),
            user_agent: core::default::Default::default(),
            crawler_limits: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElRef
    {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_filters` after provisioning.\n"]
    pub fn exclusion_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.exclusion_filters", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `inclusion_filters` after provisioning.\n"]
    pub fn inclusion_filters(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.inclusion_filters", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `user_agent` after provisioning.\n"]
    pub fn user_agent(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `crawler_limits` after provisioning.\n"]
    pub fn crawler_limits(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElCrawlerLimitsElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.crawler_limits", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl {
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl {
            url: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsElRef {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElDynamic {
    seed_urls: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    seed_urls: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl,
        >,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl {
    #[doc = "Set the field `seed_urls`.\n"]
    pub fn set_seed_urls(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.seed_urls = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.seed_urls = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl {
            seed_urls: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `seed_urls` after provisioning.\n"]
    pub fn seed_urls(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElSeedUrlsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.seed_urls", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElDynamic {
    url_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    url_configuration: Option<
        Vec<BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl>,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl {
    #[doc = "Set the field `url_configuration`.\n"]
    pub fn set_url_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.url_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.url_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl
{}

impl BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl
    {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl {
            url_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElRef
    {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url_configuration` after provisioning.\n"]
    pub fn url_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElUrlConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.url_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElDynamic {
    crawler_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl,
        >,
    >,
    source_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_configuration: Option<
        Vec<
            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_configuration: Option<
        Vec<BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl>,
    >,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl {
    #[doc = "Set the field `crawler_configuration`.\n"]
    pub fn set_crawler_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.crawler_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.crawler_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl {
    type O = BlockAssignable<BedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl {}

impl BuildBedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl {
    pub fn build(self) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl {
            crawler_configuration: core::default::Default::default(),
            source_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_configuration` after provisioning.\n"]
    pub fn crawler_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElCrawlerConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.crawler_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElSourceConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDataSourceConfigurationElDynamic {
    confluence_configuration: Option<
        DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl>,
    >,
    s3_configuration:
        Option<DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl>>,
    salesforce_configuration: Option<
        DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl>,
    >,
    share_point_configuration: Option<
        DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl>,
    >,
    web_configuration:
        Option<DynamicBlock<BedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl>>,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceDataSourceConfigurationEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confluence_configuration:
        Option<Vec<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_configuration: Option<Vec<BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salesforce_configuration:
        Option<Vec<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share_point_configuration:
        Option<Vec<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_configuration:
        Option<Vec<BedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl>>,
    dynamic: BedrockagentDataSourceDataSourceConfigurationElDynamic,
}

impl BedrockagentDataSourceDataSourceConfigurationEl {
    #[doc = "Set the field `confluence_configuration`.\n"]
    pub fn set_confluence_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.confluence_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.confluence_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `s3_configuration`.\n"]
    pub fn set_s3_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationEl>>,
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

    #[doc = "Set the field `salesforce_configuration`.\n"]
    pub fn set_salesforce_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.salesforce_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `share_point_configuration`.\n"]
    pub fn set_share_point_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.share_point_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.share_point_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `web_configuration`.\n"]
    pub fn set_web_configuration(
        mut self,
        v: impl Into<BlockAssignable<BedrockagentDataSourceDataSourceConfigurationElWebConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.web_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.web_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceDataSourceConfigurationEl {
    type O = BlockAssignable<BedrockagentDataSourceDataSourceConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceDataSourceConfigurationEl {
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildBedrockagentDataSourceDataSourceConfigurationEl {
    pub fn build(self) -> BedrockagentDataSourceDataSourceConfigurationEl {
        BedrockagentDataSourceDataSourceConfigurationEl {
            type_: self.type_,
            confluence_configuration: core::default::Default::default(),
            s3_configuration: core::default::Default::default(),
            salesforce_configuration: core::default::Default::default(),
            share_point_configuration: core::default::Default::default(),
            web_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceDataSourceConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceDataSourceConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceDataSourceConfigurationElRef {
        BedrockagentDataSourceDataSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceDataSourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `confluence_configuration` after provisioning.\n"]
    pub fn confluence_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElConfluenceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.confluence_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3_configuration` after provisioning.\n"]
    pub fn s3_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElS3ConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `salesforce_configuration` after provisioning.\n"]
    pub fn salesforce_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElSalesforceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.salesforce_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `share_point_configuration` after provisioning.\n"]
    pub fn share_point_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElSharePointConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.share_point_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `web_configuration` after provisioning.\n"]
    pub fn web_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceDataSourceConfigurationElWebConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceServerSideEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}

impl BedrockagentDataSourceServerSideEncryptionConfigurationEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentDataSourceServerSideEncryptionConfigurationEl {
    type O = BlockAssignable<BedrockagentDataSourceServerSideEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceServerSideEncryptionConfigurationEl {}

impl BuildBedrockagentDataSourceServerSideEncryptionConfigurationEl {
    pub fn build(self) -> BedrockagentDataSourceServerSideEncryptionConfigurationEl {
        BedrockagentDataSourceServerSideEncryptionConfigurationEl {
            kms_key_arn: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceServerSideEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceServerSideEncryptionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceServerSideEncryptionConfigurationElRef {
        BedrockagentDataSourceServerSideEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceServerSideEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl BedrockagentDataSourceTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for BedrockagentDataSourceTimeoutsEl {
    type O = BlockAssignable<BedrockagentDataSourceTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceTimeoutsEl {}

impl BuildBedrockagentDataSourceTimeoutsEl {
    pub fn build(self) -> BedrockagentDataSourceTimeoutsEl {
        BedrockagentDataSourceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BedrockagentDataSourceTimeoutsElRef {
        BedrockagentDataSourceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl
{
    max_tokens: PrimField<f64>,
    overlap_percentage: PrimField<f64>,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl { }

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl
{
    #[doc = ""]
    pub max_tokens: PrimField<f64>,
    #[doc = ""]
    pub overlap_percentage: PrimField<f64>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl {
            max_tokens: self.max_tokens,
            overlap_percentage: self.overlap_percentage,
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `overlap_percentage` after provisioning.\n"]
    pub fn overlap_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.overlap_percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl
{
    max_tokens: PrimField<f64>,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl {

}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl
{
    #[doc = ""]
    pub max_tokens: PrimField<f64>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl {
            max_tokens: self.max_tokens,
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElDynamic {
    level_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl {
    overlap_tokens: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level_configuration: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl,
        >,
    >,
    dynamic: BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElDynamic,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl {
    #[doc = "Set the field `level_configuration`.\n"]
    pub fn set_level_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.level_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.level_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl
{
    #[doc = ""]
    pub overlap_tokens: PrimField<f64>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl {
            overlap_tokens: self.overlap_tokens,
            level_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `overlap_tokens` after provisioning.\n"]
    pub fn overlap_tokens(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.overlap_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `level_configuration` after provisioning.\n"]
    pub fn level_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElLevelConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.level_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl
{
    breakpoint_percentile_threshold: PrimField<f64>,
    buffer_size: PrimField<f64>,
    max_token: PrimField<f64>,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl { }

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl
{
    #[doc = ""]
    pub breakpoint_percentile_threshold: PrimField<f64>,
    #[doc = ""]
    pub buffer_size: PrimField<f64>,
    #[doc = ""]
    pub max_token: PrimField<f64>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl {
            breakpoint_percentile_threshold: self.breakpoint_percentile_threshold,
            buffer_size: self.buffer_size,
            max_token: self.max_token,
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `breakpoint_percentile_threshold` after provisioning.\n"]
    pub fn breakpoint_percentile_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.breakpoint_percentile_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_size` after provisioning.\n"]
    pub fn buffer_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_size", self.base))
    }

    #[doc = "Get a reference to the value of field `max_token` after provisioning.\n"]
    pub fn max_token(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_token", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElDynamic {
    fixed_size_chunking_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl,
        >,
    >,
    hierarchical_chunking_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl,
        >,
    >,
    semantic_chunking_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl {
    chunking_strategy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_size_chunking_configuration: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    hierarchical_chunking_configuration: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    semantic_chunking_configuration: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl,
        >,
    >,
    dynamic: BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElDynamic,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl {
    #[doc = "Set the field `fixed_size_chunking_configuration`.\n"]
    pub fn set_fixed_size_chunking_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fixed_size_chunking_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_size_chunking_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `hierarchical_chunking_configuration`.\n"]
    pub fn set_hierarchical_chunking_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hierarchical_chunking_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hierarchical_chunking_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `semantic_chunking_configuration`.\n"]
    pub fn set_semantic_chunking_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.semantic_chunking_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.semantic_chunking_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl {
    #[doc = ""]
    pub chunking_strategy: PrimField<String>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl {
            chunking_strategy: self.chunking_strategy,
            fixed_size_chunking_configuration: core::default::Default::default(),
            hierarchical_chunking_configuration: core::default::Default::default(),
            semantic_chunking_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chunking_strategy` after provisioning.\n"]
    pub fn chunking_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.chunking_strategy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `fixed_size_chunking_configuration` after provisioning.\n"]
    pub fn fixed_size_chunking_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElFixedSizeChunkingConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.fixed_size_chunking_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `hierarchical_chunking_configuration` after provisioning.\n"]
    pub fn hierarchical_chunking_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElHierarchicalChunkingConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.hierarchical_chunking_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `semantic_chunking_configuration` after provisioning.\n"]
    pub fn semantic_chunking_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElSemanticChunkingConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.semantic_chunking_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl
{
    uri: PrimField<String>,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl {

}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl
{
    #[doc = ""]
    pub uri: PrimField<String>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl {
            uri: self.uri,
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElDynamic {
    s3_location: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_location: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl,
        >,
    >,
    dynamic: BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElDynamic,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl {
    #[doc = "Set the field `s3_location`.\n"]
    pub fn set_s3_location(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_location = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_location = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl
{}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl {
            s3_location: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_location` after provisioning.\n"]
    pub fn s3_location(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElS3LocationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.s3_location", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl
{
    lambda_arn: PrimField<String>,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl {

}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl
{
    #[doc = ""]
    pub lambda_arn: PrimField<String>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl {
            lambda_arn: self.lambda_arn,
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElDynamic {
    transformation_lambda_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    transformation_lambda_configuration: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl,
        >,
    >,
    dynamic: BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElDynamic,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl {
    #[doc = "Set the field `transformation_lambda_configuration`.\n"]
    pub fn set_transformation_lambda_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transformation_lambda_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_lambda_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl
{}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl {
            transformation_lambda_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `transformation_lambda_configuration` after provisioning.\n"]
    pub fn transformation_lambda_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElTransformationLambdaConfigurationElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.transformation_lambda_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElDynamic {
    transformation_function: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl {
    step_to_apply: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transformation_function: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl,
        >,
    >,
    dynamic: BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElDynamic,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl {
    #[doc = "Set the field `transformation_function`.\n"]
    pub fn set_transformation_function(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transformation_function = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation_function = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl
{
    #[doc = ""]
    pub step_to_apply: PrimField<String>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl {
            step_to_apply: self.step_to_apply,
            transformation_function: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `step_to_apply` after provisioning.\n"]
    pub fn step_to_apply(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.step_to_apply", self.base))
    }

    #[doc = "Get a reference to the value of field `transformation_function` after provisioning.\n"]
    pub fn transformation_function(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElTransformationFunctionElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.transformation_function", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElDynamic {
    intermediate_storage: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl,
        >,
    >,
    transformation: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    intermediate_storage: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    transformation: Option<
        Vec<BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl>,
    >,
    dynamic: BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElDynamic,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl {
    #[doc = "Set the field `intermediate_storage`.\n"]
    pub fn set_intermediate_storage(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.intermediate_storage = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.intermediate_storage = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `transformation`.\n"]
    pub fn set_transformation(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.transformation = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.transformation = Some(d);
            }
        }
        self
    }
}

impl ToListMappable
    for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl
{
    type O = BlockAssignable<
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl
{}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl
    {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl {
            intermediate_storage: core::default::Default::default(),
            transformation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElRef
    {
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `intermediate_storage` after provisioning.\n"]
    pub fn intermediate_storage(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElIntermediateStorageElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.intermediate_storage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `transformation` after provisioning.\n"]
    pub fn transformation(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElTransformationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.transformation", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl
{
    parsing_prompt_string: PrimField<String>,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl {

}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl
{
    #[doc = ""]
    pub parsing_prompt_string: PrimField<String>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl {
        BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl {
            parsing_prompt_string: self.parsing_prompt_string,
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parsing_prompt_string` after provisioning.\n"]
    pub fn parsing_prompt_string(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.parsing_prompt_string", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElDynamic {
    parsing_prompt: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl {
    model_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parsing_prompt: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl,
        >,
    >,
    dynamic: BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElDynamic,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl {
    #[doc = "Set the field `parsing_prompt`.\n"]
    pub fn set_parsing_prompt(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parsing_prompt = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parsing_prompt = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl {
    type O =
        BlockAssignable<
            BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl
{
    #[doc = ""]
    pub model_arn: PrimField<String>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl {
            model_arn: self.model_arn,
            parsing_prompt: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model_arn` after provisioning.\n"]
    pub fn model_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.model_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `parsing_prompt` after provisioning.\n"]
    pub fn parsing_prompt(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElParsingPromptElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.parsing_prompt", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElDynamic {
    bedrock_foundation_model_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl {
    parsing_strategy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bedrock_foundation_model_configuration: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl,
        >,
    >,
    dynamic: BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElDynamic,
}

impl BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl {
    #[doc = "Set the field `bedrock_foundation_model_configuration`.\n"]
    pub fn set_bedrock_foundation_model_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bedrock_foundation_model_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_foundation_model_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl {
    type O =
        BlockAssignable<BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl {
    #[doc = ""]
    pub parsing_strategy: PrimField<String>,
}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl {
    pub fn build(
        self,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl {
        BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl {
            parsing_strategy: self.parsing_strategy,
            bedrock_foundation_model_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parsing_strategy` after provisioning.\n"]
    pub fn parsing_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parsing_strategy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `bedrock_foundation_model_configuration` after provisioning.\n"]
    pub fn bedrock_foundation_model_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElBedrockFoundationModelConfigurationElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.bedrock_foundation_model_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceVectorIngestionConfigurationElDynamic {
    chunking_configuration: Option<
        DynamicBlock<BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl>,
    >,
    custom_transformation_configuration: Option<
        DynamicBlock<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl,
        >,
    >,
    parsing_configuration: Option<
        DynamicBlock<BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentDataSourceVectorIngestionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    chunking_configuration:
        Option<Vec<BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_transformation_configuration: Option<
        Vec<
            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    parsing_configuration:
        Option<Vec<BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl>>,
    dynamic: BedrockagentDataSourceVectorIngestionConfigurationElDynamic,
}

impl BedrockagentDataSourceVectorIngestionConfigurationEl {
    #[doc = "Set the field `chunking_configuration`.\n"]
    pub fn set_chunking_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.chunking_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.chunking_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `custom_transformation_configuration`.\n"]
    pub fn set_custom_transformation_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.custom_transformation_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_transformation_configuration = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `parsing_configuration`.\n"]
    pub fn set_parsing_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.parsing_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.parsing_configuration = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for BedrockagentDataSourceVectorIngestionConfigurationEl {
    type O = BlockAssignable<BedrockagentDataSourceVectorIngestionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentDataSourceVectorIngestionConfigurationEl {}

impl BuildBedrockagentDataSourceVectorIngestionConfigurationEl {
    pub fn build(self) -> BedrockagentDataSourceVectorIngestionConfigurationEl {
        BedrockagentDataSourceVectorIngestionConfigurationEl {
            chunking_configuration: core::default::Default::default(),
            custom_transformation_configuration: core::default::Default::default(),
            parsing_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BedrockagentDataSourceVectorIngestionConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BedrockagentDataSourceVectorIngestionConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BedrockagentDataSourceVectorIngestionConfigurationElRef {
        BedrockagentDataSourceVectorIngestionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentDataSourceVectorIngestionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chunking_configuration` after provisioning.\n"]
    pub fn chunking_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceVectorIngestionConfigurationElChunkingConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.chunking_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `custom_transformation_configuration` after provisioning.\n"]
    pub fn custom_transformation_configuration(
        &self,
    ) -> ListRef<
        BedrockagentDataSourceVectorIngestionConfigurationElCustomTransformationConfigurationElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_transformation_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `parsing_configuration` after provisioning.\n"]
    pub fn parsing_configuration(
        &self,
    ) -> ListRef<BedrockagentDataSourceVectorIngestionConfigurationElParsingConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parsing_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentDataSourceDynamic {
    data_source_configuration:
        Option<DynamicBlock<BedrockagentDataSourceDataSourceConfigurationEl>>,
    server_side_encryption_configuration:
        Option<DynamicBlock<BedrockagentDataSourceServerSideEncryptionConfigurationEl>>,
    vector_ingestion_configuration:
        Option<DynamicBlock<BedrockagentDataSourceVectorIngestionConfigurationEl>>,
}
