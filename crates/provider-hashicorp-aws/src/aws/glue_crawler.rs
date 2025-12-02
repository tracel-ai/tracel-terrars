use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GlueCrawlerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classifiers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_configuration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    table_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_target: Option<Vec<GlueCrawlerCatalogTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delta_target: Option<Vec<GlueCrawlerDeltaTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dynamodb_target: Option<Vec<GlueCrawlerDynamodbTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hudi_target: Option<Vec<GlueCrawlerHudiTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iceberg_target: Option<Vec<GlueCrawlerIcebergTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jdbc_target: Option<Vec<GlueCrawlerJdbcTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lake_formation_configuration: Option<Vec<GlueCrawlerLakeFormationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lineage_configuration: Option<Vec<GlueCrawlerLineageConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mongodb_target: Option<Vec<GlueCrawlerMongodbTargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recrawl_policy: Option<Vec<GlueCrawlerRecrawlPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_target: Option<Vec<GlueCrawlerS3TargetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_change_policy: Option<Vec<GlueCrawlerSchemaChangePolicyEl>>,
    dynamic: GlueCrawlerDynamic,
}
struct GlueCrawler_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueCrawlerData>,
}
#[derive(Clone)]
pub struct GlueCrawler(Rc<GlueCrawler_>);
impl GlueCrawler {
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
    #[doc = "Set the field `classifiers`.\n"]
    pub fn set_classifiers(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().classifiers = Some(v.into());
        self
    }
    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().configuration = Some(v.into());
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schedule = Some(v.into());
        self
    }
    #[doc = "Set the field `security_configuration`.\n"]
    pub fn set_security_configuration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_configuration = Some(v.into());
        self
    }
    #[doc = "Set the field `table_prefix`.\n"]
    pub fn set_table_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().table_prefix = Some(v.into());
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
    #[doc = "Set the field `catalog_target`.\n"]
    pub fn set_catalog_target(
        self,
        v: impl Into<BlockAssignable<GlueCrawlerCatalogTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().catalog_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.catalog_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `delta_target`.\n"]
    pub fn set_delta_target(self, v: impl Into<BlockAssignable<GlueCrawlerDeltaTargetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delta_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.delta_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `dynamodb_target`.\n"]
    pub fn set_dynamodb_target(
        self,
        v: impl Into<BlockAssignable<GlueCrawlerDynamodbTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamodb_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamodb_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `hudi_target`.\n"]
    pub fn set_hudi_target(self, v: impl Into<BlockAssignable<GlueCrawlerHudiTargetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hudi_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hudi_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `iceberg_target`.\n"]
    pub fn set_iceberg_target(
        self,
        v: impl Into<BlockAssignable<GlueCrawlerIcebergTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iceberg_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iceberg_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `jdbc_target`.\n"]
    pub fn set_jdbc_target(self, v: impl Into<BlockAssignable<GlueCrawlerJdbcTargetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().jdbc_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.jdbc_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lake_formation_configuration`.\n"]
    pub fn set_lake_formation_configuration(
        self,
        v: impl Into<BlockAssignable<GlueCrawlerLakeFormationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lake_formation_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .lake_formation_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `lineage_configuration`.\n"]
    pub fn set_lineage_configuration(
        self,
        v: impl Into<BlockAssignable<GlueCrawlerLineageConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lineage_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lineage_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mongodb_target`.\n"]
    pub fn set_mongodb_target(
        self,
        v: impl Into<BlockAssignable<GlueCrawlerMongodbTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mongodb_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mongodb_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `recrawl_policy`.\n"]
    pub fn set_recrawl_policy(
        self,
        v: impl Into<BlockAssignable<GlueCrawlerRecrawlPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recrawl_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recrawl_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_target`.\n"]
    pub fn set_s3_target(self, v: impl Into<BlockAssignable<GlueCrawlerS3TargetEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_target = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_target = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `schema_change_policy`.\n"]
    pub fn set_schema_change_policy(
        self,
        v: impl Into<BlockAssignable<GlueCrawlerSchemaChangePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema_change_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema_change_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `classifiers` after provisioning.\n"]
    pub fn classifiers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.classifiers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_prefix` after provisioning.\n"]
    pub fn table_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_prefix", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `catalog_target` after provisioning.\n"]
    pub fn catalog_target(&self) -> ListRef<GlueCrawlerCatalogTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.catalog_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delta_target` after provisioning.\n"]
    pub fn delta_target(&self) -> ListRef<GlueCrawlerDeltaTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delta_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dynamodb_target` after provisioning.\n"]
    pub fn dynamodb_target(&self) -> ListRef<GlueCrawlerDynamodbTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dynamodb_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hudi_target` after provisioning.\n"]
    pub fn hudi_target(&self) -> ListRef<GlueCrawlerHudiTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hudi_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iceberg_target` after provisioning.\n"]
    pub fn iceberg_target(&self) -> ListRef<GlueCrawlerIcebergTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iceberg_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `jdbc_target` after provisioning.\n"]
    pub fn jdbc_target(&self) -> ListRef<GlueCrawlerJdbcTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jdbc_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lake_formation_configuration` after provisioning.\n"]
    pub fn lake_formation_configuration(
        &self,
    ) -> ListRef<GlueCrawlerLakeFormationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lake_formation_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lineage_configuration` after provisioning.\n"]
    pub fn lineage_configuration(&self) -> ListRef<GlueCrawlerLineageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lineage_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mongodb_target` after provisioning.\n"]
    pub fn mongodb_target(&self) -> ListRef<GlueCrawlerMongodbTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mongodb_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recrawl_policy` after provisioning.\n"]
    pub fn recrawl_policy(&self) -> ListRef<GlueCrawlerRecrawlPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recrawl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_target` after provisioning.\n"]
    pub fn s3_target(&self) -> ListRef<GlueCrawlerS3TargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_change_policy` after provisioning.\n"]
    pub fn schema_change_policy(&self) -> ListRef<GlueCrawlerSchemaChangePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_change_policy", self.extract_ref()),
        )
    }
}
impl Referable for GlueCrawler {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GlueCrawler {}
impl ToListMappable for GlueCrawler {
    type O = ListRef<GlueCrawlerRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GlueCrawler_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_crawler".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGlueCrawler {
    pub tf_id: String,
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role: PrimField<String>,
}
impl BuildGlueCrawler {
    pub fn build(self, stack: &mut Stack) -> GlueCrawler {
        let out = GlueCrawler(Rc::new(GlueCrawler_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueCrawlerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                classifiers: core::default::Default::default(),
                configuration: core::default::Default::default(),
                database_name: self.database_name,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                role: self.role,
                schedule: core::default::Default::default(),
                security_configuration: core::default::Default::default(),
                table_prefix: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                catalog_target: core::default::Default::default(),
                delta_target: core::default::Default::default(),
                dynamodb_target: core::default::Default::default(),
                hudi_target: core::default::Default::default(),
                iceberg_target: core::default::Default::default(),
                jdbc_target: core::default::Default::default(),
                lake_formation_configuration: core::default::Default::default(),
                lineage_configuration: core::default::Default::default(),
                mongodb_target: core::default::Default::default(),
                recrawl_policy: core::default::Default::default(),
                s3_target: core::default::Default::default(),
                schema_change_policy: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GlueCrawlerRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GlueCrawlerRef {
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
    #[doc = "Get a reference to the value of field `classifiers` after provisioning.\n"]
    pub fn classifiers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.classifiers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.role", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schedule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `table_prefix` after provisioning.\n"]
    pub fn table_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.table_prefix", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `catalog_target` after provisioning.\n"]
    pub fn catalog_target(&self) -> ListRef<GlueCrawlerCatalogTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.catalog_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delta_target` after provisioning.\n"]
    pub fn delta_target(&self) -> ListRef<GlueCrawlerDeltaTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.delta_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dynamodb_target` after provisioning.\n"]
    pub fn dynamodb_target(&self) -> ListRef<GlueCrawlerDynamodbTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dynamodb_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `hudi_target` after provisioning.\n"]
    pub fn hudi_target(&self) -> ListRef<GlueCrawlerHudiTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.hudi_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `iceberg_target` after provisioning.\n"]
    pub fn iceberg_target(&self) -> ListRef<GlueCrawlerIcebergTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.iceberg_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `jdbc_target` after provisioning.\n"]
    pub fn jdbc_target(&self) -> ListRef<GlueCrawlerJdbcTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.jdbc_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lake_formation_configuration` after provisioning.\n"]
    pub fn lake_formation_configuration(
        &self,
    ) -> ListRef<GlueCrawlerLakeFormationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lake_formation_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `lineage_configuration` after provisioning.\n"]
    pub fn lineage_configuration(&self) -> ListRef<GlueCrawlerLineageConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.lineage_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mongodb_target` after provisioning.\n"]
    pub fn mongodb_target(&self) -> ListRef<GlueCrawlerMongodbTargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mongodb_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `recrawl_policy` after provisioning.\n"]
    pub fn recrawl_policy(&self) -> ListRef<GlueCrawlerRecrawlPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.recrawl_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `s3_target` after provisioning.\n"]
    pub fn s3_target(&self) -> ListRef<GlueCrawlerS3TargetElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.s3_target", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `schema_change_policy` after provisioning.\n"]
    pub fn schema_change_policy(&self) -> ListRef<GlueCrawlerSchemaChangePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema_change_policy", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerCatalogTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_name: Option<PrimField<String>>,
    database_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dlq_event_queue_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_queue_arn: Option<PrimField<String>>,
    tables: ListField<PrimField<String>>,
}
impl GlueCrawlerCatalogTargetEl {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_name = Some(v.into());
        self
    }
    #[doc = "Set the field `dlq_event_queue_arn`.\n"]
    pub fn set_dlq_event_queue_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dlq_event_queue_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `event_queue_arn`.\n"]
    pub fn set_event_queue_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_queue_arn = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerCatalogTargetEl {
    type O = BlockAssignable<GlueCrawlerCatalogTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerCatalogTargetEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub tables: ListField<PrimField<String>>,
}
impl BuildGlueCrawlerCatalogTargetEl {
    pub fn build(self) -> GlueCrawlerCatalogTargetEl {
        GlueCrawlerCatalogTargetEl {
            connection_name: core::default::Default::default(),
            database_name: self.database_name,
            dlq_event_queue_arn: core::default::Default::default(),
            event_queue_arn: core::default::Default::default(),
            tables: self.tables,
        }
    }
}
pub struct GlueCrawlerCatalogTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerCatalogTargetElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerCatalogTargetElRef {
        GlueCrawlerCatalogTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerCatalogTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `database_name` after provisioning.\n"]
    pub fn database_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.database_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dlq_event_queue_arn` after provisioning.\n"]
    pub fn dlq_event_queue_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dlq_event_queue_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `event_queue_arn` after provisioning.\n"]
    pub fn event_queue_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_queue_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerDeltaTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create_native_delta_table: Option<PrimField<bool>>,
    delta_tables: SetField<PrimField<String>>,
    write_manifest: PrimField<bool>,
}
impl GlueCrawlerDeltaTargetEl {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_name = Some(v.into());
        self
    }
    #[doc = "Set the field `create_native_delta_table`.\n"]
    pub fn set_create_native_delta_table(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.create_native_delta_table = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerDeltaTargetEl {
    type O = BlockAssignable<GlueCrawlerDeltaTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerDeltaTargetEl {
    #[doc = ""]
    pub delta_tables: SetField<PrimField<String>>,
    #[doc = ""]
    pub write_manifest: PrimField<bool>,
}
impl BuildGlueCrawlerDeltaTargetEl {
    pub fn build(self) -> GlueCrawlerDeltaTargetEl {
        GlueCrawlerDeltaTargetEl {
            connection_name: core::default::Default::default(),
            create_native_delta_table: core::default::Default::default(),
            delta_tables: self.delta_tables,
            write_manifest: self.write_manifest,
        }
    }
}
pub struct GlueCrawlerDeltaTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerDeltaTargetElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerDeltaTargetElRef {
        GlueCrawlerDeltaTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerDeltaTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `create_native_delta_table` after provisioning.\n"]
    pub fn create_native_delta_table(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_native_delta_table", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `delta_tables` after provisioning.\n"]
    pub fn delta_tables(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.delta_tables", self.base))
    }
    #[doc = "Get a reference to the value of field `write_manifest` after provisioning.\n"]
    pub fn write_manifest(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.write_manifest", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerDynamodbTargetEl {
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_all: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_rate: Option<PrimField<f64>>,
}
impl GlueCrawlerDynamodbTargetEl {
    #[doc = "Set the field `scan_all`.\n"]
    pub fn set_scan_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.scan_all = Some(v.into());
        self
    }
    #[doc = "Set the field `scan_rate`.\n"]
    pub fn set_scan_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scan_rate = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerDynamodbTargetEl {
    type O = BlockAssignable<GlueCrawlerDynamodbTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerDynamodbTargetEl {
    #[doc = ""]
    pub path: PrimField<String>,
}
impl BuildGlueCrawlerDynamodbTargetEl {
    pub fn build(self) -> GlueCrawlerDynamodbTargetEl {
        GlueCrawlerDynamodbTargetEl {
            path: self.path,
            scan_all: core::default::Default::default(),
            scan_rate: core::default::Default::default(),
        }
    }
}
pub struct GlueCrawlerDynamodbTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerDynamodbTargetElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerDynamodbTargetElRef {
        GlueCrawlerDynamodbTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerDynamodbTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_all` after provisioning.\n"]
    pub fn scan_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_all", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_rate` after provisioning.\n"]
    pub fn scan_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_rate", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerHudiTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusions: Option<ListField<PrimField<String>>>,
    maximum_traversal_depth: PrimField<f64>,
    paths: SetField<PrimField<String>>,
}
impl GlueCrawlerHudiTargetEl {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_name = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclusions = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerHudiTargetEl {
    type O = BlockAssignable<GlueCrawlerHudiTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerHudiTargetEl {
    #[doc = ""]
    pub maximum_traversal_depth: PrimField<f64>,
    #[doc = ""]
    pub paths: SetField<PrimField<String>>,
}
impl BuildGlueCrawlerHudiTargetEl {
    pub fn build(self) -> GlueCrawlerHudiTargetEl {
        GlueCrawlerHudiTargetEl {
            connection_name: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            maximum_traversal_depth: self.maximum_traversal_depth,
            paths: self.paths,
        }
    }
}
pub struct GlueCrawlerHudiTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerHudiTargetElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerHudiTargetElRef {
        GlueCrawlerHudiTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerHudiTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum_traversal_depth` after provisioning.\n"]
    pub fn maximum_traversal_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_traversal_depth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerIcebergTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusions: Option<ListField<PrimField<String>>>,
    maximum_traversal_depth: PrimField<f64>,
    paths: SetField<PrimField<String>>,
}
impl GlueCrawlerIcebergTargetEl {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_name = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclusions = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerIcebergTargetEl {
    type O = BlockAssignable<GlueCrawlerIcebergTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerIcebergTargetEl {
    #[doc = ""]
    pub maximum_traversal_depth: PrimField<f64>,
    #[doc = ""]
    pub paths: SetField<PrimField<String>>,
}
impl BuildGlueCrawlerIcebergTargetEl {
    pub fn build(self) -> GlueCrawlerIcebergTargetEl {
        GlueCrawlerIcebergTargetEl {
            connection_name: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            maximum_traversal_depth: self.maximum_traversal_depth,
            paths: self.paths,
        }
    }
}
pub struct GlueCrawlerIcebergTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerIcebergTargetElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerIcebergTargetElRef {
        GlueCrawlerIcebergTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerIcebergTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }
    #[doc = "Get a reference to the value of field `maximum_traversal_depth` after provisioning.\n"]
    pub fn maximum_traversal_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_traversal_depth", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `paths` after provisioning.\n"]
    pub fn paths(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.paths", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerJdbcTargetEl {
    connection_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_additional_metadata: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusions: Option<ListField<PrimField<String>>>,
    path: PrimField<String>,
}
impl GlueCrawlerJdbcTargetEl {
    #[doc = "Set the field `enable_additional_metadata`.\n"]
    pub fn set_enable_additional_metadata(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.enable_additional_metadata = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclusions = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerJdbcTargetEl {
    type O = BlockAssignable<GlueCrawlerJdbcTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerJdbcTargetEl {
    #[doc = ""]
    pub connection_name: PrimField<String>,
    #[doc = ""]
    pub path: PrimField<String>,
}
impl BuildGlueCrawlerJdbcTargetEl {
    pub fn build(self) -> GlueCrawlerJdbcTargetEl {
        GlueCrawlerJdbcTargetEl {
            connection_name: self.connection_name,
            enable_additional_metadata: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            path: self.path,
        }
    }
}
pub struct GlueCrawlerJdbcTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerJdbcTargetElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerJdbcTargetElRef {
        GlueCrawlerJdbcTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerJdbcTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_additional_metadata` after provisioning.\n"]
    pub fn enable_additional_metadata(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.enable_additional_metadata", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerLakeFormationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_lake_formation_credentials: Option<PrimField<bool>>,
}
impl GlueCrawlerLakeFormationConfigurationEl {
    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    #[doc = "Set the field `use_lake_formation_credentials`.\n"]
    pub fn set_use_lake_formation_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_lake_formation_credentials = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerLakeFormationConfigurationEl {
    type O = BlockAssignable<GlueCrawlerLakeFormationConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerLakeFormationConfigurationEl {}
impl BuildGlueCrawlerLakeFormationConfigurationEl {
    pub fn build(self) -> GlueCrawlerLakeFormationConfigurationEl {
        GlueCrawlerLakeFormationConfigurationEl {
            account_id: core::default::Default::default(),
            use_lake_formation_credentials: core::default::Default::default(),
        }
    }
}
pub struct GlueCrawlerLakeFormationConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerLakeFormationConfigurationElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerLakeFormationConfigurationElRef {
        GlueCrawlerLakeFormationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerLakeFormationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
    #[doc = "Get a reference to the value of field `use_lake_formation_credentials` after provisioning.\n"]
    pub fn use_lake_formation_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_lake_formation_credentials", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerLineageConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    crawler_lineage_settings: Option<PrimField<String>>,
}
impl GlueCrawlerLineageConfigurationEl {
    #[doc = "Set the field `crawler_lineage_settings`.\n"]
    pub fn set_crawler_lineage_settings(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.crawler_lineage_settings = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerLineageConfigurationEl {
    type O = BlockAssignable<GlueCrawlerLineageConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerLineageConfigurationEl {}
impl BuildGlueCrawlerLineageConfigurationEl {
    pub fn build(self) -> GlueCrawlerLineageConfigurationEl {
        GlueCrawlerLineageConfigurationEl {
            crawler_lineage_settings: core::default::Default::default(),
        }
    }
}
pub struct GlueCrawlerLineageConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerLineageConfigurationElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerLineageConfigurationElRef {
        GlueCrawlerLineageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerLineageConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `crawler_lineage_settings` after provisioning.\n"]
    pub fn crawler_lineage_settings(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.crawler_lineage_settings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerMongodbTargetEl {
    connection_name: PrimField<String>,
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_all: Option<PrimField<bool>>,
}
impl GlueCrawlerMongodbTargetEl {
    #[doc = "Set the field `scan_all`.\n"]
    pub fn set_scan_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.scan_all = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerMongodbTargetEl {
    type O = BlockAssignable<GlueCrawlerMongodbTargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerMongodbTargetEl {
    #[doc = ""]
    pub connection_name: PrimField<String>,
    #[doc = ""]
    pub path: PrimField<String>,
}
impl BuildGlueCrawlerMongodbTargetEl {
    pub fn build(self) -> GlueCrawlerMongodbTargetEl {
        GlueCrawlerMongodbTargetEl {
            connection_name: self.connection_name,
            path: self.path,
            scan_all: core::default::Default::default(),
        }
    }
}
pub struct GlueCrawlerMongodbTargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerMongodbTargetElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerMongodbTargetElRef {
        GlueCrawlerMongodbTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerMongodbTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `scan_all` after provisioning.\n"]
    pub fn scan_all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.scan_all", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerRecrawlPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    recrawl_behavior: Option<PrimField<String>>,
}
impl GlueCrawlerRecrawlPolicyEl {
    #[doc = "Set the field `recrawl_behavior`.\n"]
    pub fn set_recrawl_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.recrawl_behavior = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerRecrawlPolicyEl {
    type O = BlockAssignable<GlueCrawlerRecrawlPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerRecrawlPolicyEl {}
impl BuildGlueCrawlerRecrawlPolicyEl {
    pub fn build(self) -> GlueCrawlerRecrawlPolicyEl {
        GlueCrawlerRecrawlPolicyEl {
            recrawl_behavior: core::default::Default::default(),
        }
    }
}
pub struct GlueCrawlerRecrawlPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerRecrawlPolicyElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerRecrawlPolicyElRef {
        GlueCrawlerRecrawlPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerRecrawlPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `recrawl_behavior` after provisioning.\n"]
    pub fn recrawl_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.recrawl_behavior", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerS3TargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dlq_event_queue_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_queue_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclusions: Option<ListField<PrimField<String>>>,
    path: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_size: Option<PrimField<f64>>,
}
impl GlueCrawlerS3TargetEl {
    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_name = Some(v.into());
        self
    }
    #[doc = "Set the field `dlq_event_queue_arn`.\n"]
    pub fn set_dlq_event_queue_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dlq_event_queue_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `event_queue_arn`.\n"]
    pub fn set_event_queue_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.event_queue_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.exclusions = Some(v.into());
        self
    }
    #[doc = "Set the field `sample_size`.\n"]
    pub fn set_sample_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sample_size = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerS3TargetEl {
    type O = BlockAssignable<GlueCrawlerS3TargetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerS3TargetEl {
    #[doc = ""]
    pub path: PrimField<String>,
}
impl BuildGlueCrawlerS3TargetEl {
    pub fn build(self) -> GlueCrawlerS3TargetEl {
        GlueCrawlerS3TargetEl {
            connection_name: core::default::Default::default(),
            dlq_event_queue_arn: core::default::Default::default(),
            event_queue_arn: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            path: self.path,
            sample_size: core::default::Default::default(),
        }
    }
}
pub struct GlueCrawlerS3TargetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerS3TargetElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerS3TargetElRef {
        GlueCrawlerS3TargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerS3TargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection_name` after provisioning.\n"]
    pub fn connection_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `dlq_event_queue_arn` after provisioning.\n"]
    pub fn dlq_event_queue_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dlq_event_queue_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `event_queue_arn` after provisioning.\n"]
    pub fn event_queue_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.event_queue_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `sample_size` after provisioning.\n"]
    pub fn sample_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sample_size", self.base))
    }
}
#[derive(Serialize)]
pub struct GlueCrawlerSchemaChangePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update_behavior: Option<PrimField<String>>,
}
impl GlueCrawlerSchemaChangePolicyEl {
    #[doc = "Set the field `delete_behavior`.\n"]
    pub fn set_delete_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `update_behavior`.\n"]
    pub fn set_update_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update_behavior = Some(v.into());
        self
    }
}
impl ToListMappable for GlueCrawlerSchemaChangePolicyEl {
    type O = BlockAssignable<GlueCrawlerSchemaChangePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGlueCrawlerSchemaChangePolicyEl {}
impl BuildGlueCrawlerSchemaChangePolicyEl {
    pub fn build(self) -> GlueCrawlerSchemaChangePolicyEl {
        GlueCrawlerSchemaChangePolicyEl {
            delete_behavior: core::default::Default::default(),
            update_behavior: core::default::Default::default(),
        }
    }
}
pub struct GlueCrawlerSchemaChangePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GlueCrawlerSchemaChangePolicyElRef {
    fn new(shared: StackShared, base: String) -> GlueCrawlerSchemaChangePolicyElRef {
        GlueCrawlerSchemaChangePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GlueCrawlerSchemaChangePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `delete_behavior` after provisioning.\n"]
    pub fn delete_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_behavior", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `update_behavior` after provisioning.\n"]
    pub fn update_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_behavior", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct GlueCrawlerDynamic {
    catalog_target: Option<DynamicBlock<GlueCrawlerCatalogTargetEl>>,
    delta_target: Option<DynamicBlock<GlueCrawlerDeltaTargetEl>>,
    dynamodb_target: Option<DynamicBlock<GlueCrawlerDynamodbTargetEl>>,
    hudi_target: Option<DynamicBlock<GlueCrawlerHudiTargetEl>>,
    iceberg_target: Option<DynamicBlock<GlueCrawlerIcebergTargetEl>>,
    jdbc_target: Option<DynamicBlock<GlueCrawlerJdbcTargetEl>>,
    lake_formation_configuration: Option<DynamicBlock<GlueCrawlerLakeFormationConfigurationEl>>,
    lineage_configuration: Option<DynamicBlock<GlueCrawlerLineageConfigurationEl>>,
    mongodb_target: Option<DynamicBlock<GlueCrawlerMongodbTargetEl>>,
    recrawl_policy: Option<DynamicBlock<GlueCrawlerRecrawlPolicyEl>>,
    s3_target: Option<DynamicBlock<GlueCrawlerS3TargetEl>>,
    schema_change_policy: Option<DynamicBlock<GlueCrawlerSchemaChangePolicyEl>>,
}
