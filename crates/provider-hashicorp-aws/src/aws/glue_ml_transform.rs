use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct GlueMlTransformData {
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
    glue_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_capacity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_retries: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_workers: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_record_tables: Option<Vec<GlueMlTransformInputRecordTablesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<GlueMlTransformParametersEl>>,
    dynamic: GlueMlTransformDynamic,
}

struct GlueMlTransform_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GlueMlTransformData>,
}

#[derive(Clone)]
pub struct GlueMlTransform(Rc<GlueMlTransform_>);

impl GlueMlTransform {
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

    #[doc = "Set the field `glue_version`.\n"]
    pub fn set_glue_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().glue_version = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_capacity = Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_retries = Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_workers`.\n"]
    pub fn set_number_of_workers(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().number_of_workers = Some(v.into());
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

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = Some(v.into());
        self
    }

    #[doc = "Set the field `worker_type`.\n"]
    pub fn set_worker_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().worker_type = Some(v.into());
        self
    }

    #[doc = "Set the field `input_record_tables`.\n"]
    pub fn set_input_record_tables(
        self,
        v: impl Into<BlockAssignable<GlueMlTransformInputRecordTablesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().input_record_tables = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.input_record_tables = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl Into<BlockAssignable<GlueMlTransformParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `glue_version` after provisioning.\n"]
    pub fn glue_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.glue_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_count` after provisioning.\n"]
    pub fn label_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_retries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `number_of_workers` after provisioning.\n"]
    pub fn number_of_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_workers", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<GlueMlTransformSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `worker_type` after provisioning.\n"]
    pub fn worker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.worker_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `input_record_tables` after provisioning.\n"]
    pub fn input_record_tables(&self) -> ListRef<GlueMlTransformInputRecordTablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_record_tables", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<GlueMlTransformParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
}

impl Referable for GlueMlTransform {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for GlueMlTransform {}

impl ToListMappable for GlueMlTransform {
    type O = ListRef<GlueMlTransformRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GlueMlTransform_ {
    fn extract_resource_type(&self) -> String {
        "aws_glue_ml_transform".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueMlTransform {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub role_arn: PrimField<String>,
}

impl BuildGlueMlTransform {
    pub fn build(self, stack: &mut Stack) -> GlueMlTransform {
        let out = GlueMlTransform(Rc::new(GlueMlTransform_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GlueMlTransformData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                glue_version: core::default::Default::default(),
                id: core::default::Default::default(),
                max_capacity: core::default::Default::default(),
                max_retries: core::default::Default::default(),
                name: self.name,
                number_of_workers: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeout: core::default::Default::default(),
                worker_type: core::default::Default::default(),
                input_record_tables: core::default::Default::default(),
                parameters: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueMlTransformRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueMlTransformRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl GlueMlTransformRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `glue_version` after provisioning.\n"]
    pub fn glue_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.glue_version", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `label_count` after provisioning.\n"]
    pub fn label_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.label_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_retries", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `number_of_workers` after provisioning.\n"]
    pub fn number_of_workers(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_workers", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> ListRef<GlueMlTransformSchemaElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `worker_type` after provisioning.\n"]
    pub fn worker_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.worker_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `input_record_tables` after provisioning.\n"]
    pub fn input_record_tables(&self) -> ListRef<GlueMlTransformInputRecordTablesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.input_record_tables", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> ListRef<GlueMlTransformParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.parameters", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct GlueMlTransformSchemaEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl GlueMlTransformSchemaEl {
    #[doc = "Set the field `data_type`.\n"]
    pub fn set_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_type = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for GlueMlTransformSchemaEl {
    type O = BlockAssignable<GlueMlTransformSchemaEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueMlTransformSchemaEl {}

impl BuildGlueMlTransformSchemaEl {
    pub fn build(self) -> GlueMlTransformSchemaEl {
        GlueMlTransformSchemaEl {
            data_type: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct GlueMlTransformSchemaElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueMlTransformSchemaElRef {
    fn new(shared: StackShared, base: String) -> GlueMlTransformSchemaElRef {
        GlueMlTransformSchemaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueMlTransformSchemaElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_type` after provisioning.\n"]
    pub fn data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueMlTransformInputRecordTablesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    catalog_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_name: Option<PrimField<String>>,
    database_name: PrimField<String>,
    table_name: PrimField<String>,
}

impl GlueMlTransformInputRecordTablesEl {
    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.catalog_id = Some(v.into());
        self
    }

    #[doc = "Set the field `connection_name`.\n"]
    pub fn set_connection_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_name = Some(v.into());
        self
    }
}

impl ToListMappable for GlueMlTransformInputRecordTablesEl {
    type O = BlockAssignable<GlueMlTransformInputRecordTablesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueMlTransformInputRecordTablesEl {
    #[doc = ""]
    pub database_name: PrimField<String>,
    #[doc = ""]
    pub table_name: PrimField<String>,
}

impl BuildGlueMlTransformInputRecordTablesEl {
    pub fn build(self) -> GlueMlTransformInputRecordTablesEl {
        GlueMlTransformInputRecordTablesEl {
            catalog_id: core::default::Default::default(),
            connection_name: core::default::Default::default(),
            database_name: self.database_name,
            table_name: self.table_name,
        }
    }
}

pub struct GlueMlTransformInputRecordTablesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueMlTransformInputRecordTablesElRef {
    fn new(shared: StackShared, base: String) -> GlueMlTransformInputRecordTablesElRef {
        GlueMlTransformInputRecordTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueMlTransformInputRecordTablesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.base))
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

    #[doc = "Get a reference to the value of field `table_name` after provisioning.\n"]
    pub fn table_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table_name", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueMlTransformParametersElFindMatchesParametersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    accuracy_cost_trade_off: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_provided_labels: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    precision_recall_trade_off: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_key_column_name: Option<PrimField<String>>,
}

impl GlueMlTransformParametersElFindMatchesParametersEl {
    #[doc = "Set the field `accuracy_cost_trade_off`.\n"]
    pub fn set_accuracy_cost_trade_off(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.accuracy_cost_trade_off = Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_provided_labels`.\n"]
    pub fn set_enforce_provided_labels(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enforce_provided_labels = Some(v.into());
        self
    }

    #[doc = "Set the field `precision_recall_trade_off`.\n"]
    pub fn set_precision_recall_trade_off(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.precision_recall_trade_off = Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key_column_name`.\n"]
    pub fn set_primary_key_column_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_key_column_name = Some(v.into());
        self
    }
}

impl ToListMappable for GlueMlTransformParametersElFindMatchesParametersEl {
    type O = BlockAssignable<GlueMlTransformParametersElFindMatchesParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueMlTransformParametersElFindMatchesParametersEl {}

impl BuildGlueMlTransformParametersElFindMatchesParametersEl {
    pub fn build(self) -> GlueMlTransformParametersElFindMatchesParametersEl {
        GlueMlTransformParametersElFindMatchesParametersEl {
            accuracy_cost_trade_off: core::default::Default::default(),
            enforce_provided_labels: core::default::Default::default(),
            precision_recall_trade_off: core::default::Default::default(),
            primary_key_column_name: core::default::Default::default(),
        }
    }
}

pub struct GlueMlTransformParametersElFindMatchesParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueMlTransformParametersElFindMatchesParametersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GlueMlTransformParametersElFindMatchesParametersElRef {
        GlueMlTransformParametersElFindMatchesParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueMlTransformParametersElFindMatchesParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accuracy_cost_trade_off` after provisioning.\n"]
    pub fn accuracy_cost_trade_off(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.accuracy_cost_trade_off", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enforce_provided_labels` after provisioning.\n"]
    pub fn enforce_provided_labels(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enforce_provided_labels", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `precision_recall_trade_off` after provisioning.\n"]
    pub fn precision_recall_trade_off(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.precision_recall_trade_off", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `primary_key_column_name` after provisioning.\n"]
    pub fn primary_key_column_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.primary_key_column_name", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct GlueMlTransformParametersElDynamic {
    find_matches_parameters:
        Option<DynamicBlock<GlueMlTransformParametersElFindMatchesParametersEl>>,
}

#[derive(Serialize)]
pub struct GlueMlTransformParametersEl {
    transform_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    find_matches_parameters: Option<Vec<GlueMlTransformParametersElFindMatchesParametersEl>>,
    dynamic: GlueMlTransformParametersElDynamic,
}

impl GlueMlTransformParametersEl {
    #[doc = "Set the field `find_matches_parameters`.\n"]
    pub fn set_find_matches_parameters(
        mut self,
        v: impl Into<BlockAssignable<GlueMlTransformParametersElFindMatchesParametersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.find_matches_parameters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.find_matches_parameters = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for GlueMlTransformParametersEl {
    type O = BlockAssignable<GlueMlTransformParametersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueMlTransformParametersEl {
    #[doc = ""]
    pub transform_type: PrimField<String>,
}

impl BuildGlueMlTransformParametersEl {
    pub fn build(self) -> GlueMlTransformParametersEl {
        GlueMlTransformParametersEl {
            transform_type: self.transform_type,
            find_matches_parameters: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct GlueMlTransformParametersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for GlueMlTransformParametersElRef {
    fn new(shared: StackShared, base: String) -> GlueMlTransformParametersElRef {
        GlueMlTransformParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueMlTransformParametersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `transform_type` after provisioning.\n"]
    pub fn transform_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.transform_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `find_matches_parameters` after provisioning.\n"]
    pub fn find_matches_parameters(
        &self,
    ) -> ListRef<GlueMlTransformParametersElFindMatchesParametersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.find_matches_parameters", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct GlueMlTransformDynamic {
    input_record_tables: Option<DynamicBlock<GlueMlTransformInputRecordTablesEl>>,
    parameters: Option<DynamicBlock<GlueMlTransformParametersEl>>,
}
