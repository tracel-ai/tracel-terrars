use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryRoutineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_governance_type: Option<PrimField<String>>,
    dataset_id: PrimField<String>,
    definition_body: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    determinism_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    imported_libraries: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_table_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    return_type: Option<PrimField<String>>,
    routine_id: PrimField<String>,
    routine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<Vec<BigqueryRoutineArgumentsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remote_function_options: Option<Vec<BigqueryRoutineRemoteFunctionOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spark_options: Option<Vec<BigqueryRoutineSparkOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryRoutineTimeoutsEl>,
    dynamic: BigqueryRoutineDynamic,
}
struct BigqueryRoutine_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryRoutineData>,
}
#[derive(Clone)]
pub struct BigqueryRoutine(Rc<BigqueryRoutine_>);
impl BigqueryRoutine {
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
    #[doc = "Set the field `data_governance_type`.\nIf set to DATA_MASKING, the function is validated and made available as a masking function. For more information, see https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask Possible values: [\"DATA_MASKING\"]"]
    pub fn set_data_governance_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().data_governance_type = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nThe description of the routine if defined."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `determinism_level`.\nThe determinism level of the JavaScript UDF if defined. Possible values: [\"DETERMINISM_LEVEL_UNSPECIFIED\", \"DETERMINISTIC\", \"NOT_DETERMINISTIC\"]"]
    pub fn set_determinism_level(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().determinism_level = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `imported_libraries`.\nOptional. If language = \"JAVASCRIPT\", this field stores the path of the\nimported JAVASCRIPT libraries."]
    pub fn set_imported_libraries(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().imported_libraries = Some(v.into());
        self
    }
    #[doc = "Set the field `language`.\nThe language of the routine. Possible values: [\"SQL\", \"JAVASCRIPT\", \"PYTHON\", \"JAVA\", \"SCALA\"]"]
    pub fn set_language(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().language = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `return_table_type`.\nOptional. Can be set only if routineType = \"TABLE_VALUED_FUNCTION\".\n\nIf absent, the return table type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the columns in the evaluated table result will\nbe cast to match the column types specificed in return table type, at query time."]
    pub fn set_return_table_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().return_table_type = Some(v.into());
        self
    }
    #[doc = "Set the field `return_type`.\nA JSON schema for the return type. Optional if language = \"SQL\"; required otherwise.\nIf absent, the return type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the evaluated result will be cast to\nthe specified returned type at query time. ~>**NOTE**: Because this field expects a JSON\nstring, any changes to the string will create a diff, even if the JSON itself hasn't\nchanged. If the API returns a different value for the same schema, e.g. it switche\nd the order of values or replaced STRUCT field type with RECORD field type, we currently\ncannot suppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn set_return_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().return_type = Some(v.into());
        self
    }
    #[doc = "Set the field `security_mode`.\nOptional. The security mode of the routine, if defined. If not defined, the security mode is automatically determined from the routine's configuration. Possible values: [\"DEFINER\", \"INVOKER\"]"]
    pub fn set_security_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().security_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `arguments`.\n"]
    pub fn set_arguments(self, v: impl Into<BlockAssignable<BigqueryRoutineArgumentsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().arguments = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.arguments = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `remote_function_options`.\n"]
    pub fn set_remote_function_options(
        self,
        v: impl Into<BlockAssignable<BigqueryRoutineRemoteFunctionOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_function_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_function_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spark_options`.\n"]
    pub fn set_spark_options(
        self,
        v: impl Into<BlockAssignable<BigqueryRoutineSparkOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryRoutineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this routine was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_governance_type` after provisioning.\nIf set to DATA_MASKING, the function is validated and made available as a masking function. For more information, see https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask Possible values: [\"DATA_MASKING\"]"]
    pub fn data_governance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_governance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this routine"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `definition_body` after provisioning.\nThe body of the routine. For functions, this is the expression in the AS clause.\nIf language=SQL, it is the substring inside (but excluding) the parentheses."]
    pub fn definition_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.definition_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the routine if defined."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `determinism_level` after provisioning.\nThe determinism level of the JavaScript UDF if defined. Possible values: [\"DETERMINISM_LEVEL_UNSPECIFIED\", \"DETERMINISTIC\", \"NOT_DETERMINISTIC\"]"]
    pub fn determinism_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.determinism_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `imported_libraries` after provisioning.\nOptional. If language = \"JAVASCRIPT\", this field stores the path of the\nimported JAVASCRIPT libraries."]
    pub fn imported_libraries(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.imported_libraries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language` after provisioning.\nThe language of the routine. Possible values: [\"SQL\", \"JAVASCRIPT\", \"PYTHON\", \"JAVA\", \"SCALA\"]"]
    pub fn language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this routine was modified, in milliseconds since the\nepoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `return_table_type` after provisioning.\nOptional. Can be set only if routineType = \"TABLE_VALUED_FUNCTION\".\n\nIf absent, the return table type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the columns in the evaluated table result will\nbe cast to match the column types specificed in return table type, at query time."]
    pub fn return_table_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_table_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `return_type` after provisioning.\nA JSON schema for the return type. Optional if language = \"SQL\"; required otherwise.\nIf absent, the return type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the evaluated result will be cast to\nthe specified returned type at query time. ~>**NOTE**: Because this field expects a JSON\nstring, any changes to the string will create a diff, even if the JSON itself hasn't\nchanged. If the API returns a different value for the same schema, e.g. it switche\nd the order of values or replaced STRUCT field type with RECORD field type, we currently\ncannot suppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn return_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routine_id` after provisioning.\nThe ID of the the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
    pub fn routine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routine_type` after provisioning.\nThe type of routine. Possible values: [\"SCALAR_FUNCTION\", \"PROCEDURE\", \"TABLE_VALUED_FUNCTION\"]"]
    pub fn routine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_mode` after provisioning.\nOptional. The security mode of the routine, if defined. If not defined, the security mode is automatically determined from the routine's configuration. Possible values: [\"DEFINER\", \"INVOKER\"]"]
    pub fn security_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arguments` after provisioning.\n"]
    pub fn arguments(&self) -> ListRef<BigqueryRoutineArgumentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.arguments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remote_function_options` after provisioning.\n"]
    pub fn remote_function_options(&self) -> ListRef<BigqueryRoutineRemoteFunctionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remote_function_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_options` after provisioning.\n"]
    pub fn spark_options(&self) -> ListRef<BigqueryRoutineSparkOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryRoutineTimeoutsElRef {
        BigqueryRoutineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryRoutine {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryRoutine {}
impl ToListMappable for BigqueryRoutine {
    type O = ListRef<BigqueryRoutineRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryRoutine_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_routine".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryRoutine {
    pub tf_id: String,
    #[doc = "The ID of the dataset containing this routine"]
    pub dataset_id: PrimField<String>,
    #[doc = "The body of the routine. For functions, this is the expression in the AS clause.\nIf language=SQL, it is the substring inside (but excluding) the parentheses."]
    pub definition_body: PrimField<String>,
    #[doc = "The ID of the the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
    pub routine_id: PrimField<String>,
    #[doc = "The type of routine. Possible values: [\"SCALAR_FUNCTION\", \"PROCEDURE\", \"TABLE_VALUED_FUNCTION\"]"]
    pub routine_type: PrimField<String>,
}
impl BuildBigqueryRoutine {
    pub fn build(self, stack: &mut Stack) -> BigqueryRoutine {
        let out = BigqueryRoutine(Rc::new(BigqueryRoutine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryRoutineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_governance_type: core::default::Default::default(),
                dataset_id: self.dataset_id,
                definition_body: self.definition_body,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                determinism_level: core::default::Default::default(),
                id: core::default::Default::default(),
                imported_libraries: core::default::Default::default(),
                language: core::default::Default::default(),
                project: core::default::Default::default(),
                return_table_type: core::default::Default::default(),
                return_type: core::default::Default::default(),
                routine_id: self.routine_id,
                routine_type: self.routine_type,
                security_mode: core::default::Default::default(),
                arguments: core::default::Default::default(),
                remote_function_options: core::default::Default::default(),
                spark_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryRoutineRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryRoutineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryRoutineRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this routine was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_governance_type` after provisioning.\nIf set to DATA_MASKING, the function is validated and made available as a masking function. For more information, see https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask Possible values: [\"DATA_MASKING\"]"]
    pub fn data_governance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_governance_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this routine"]
    pub fn dataset_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `definition_body` after provisioning.\nThe body of the routine. For functions, this is the expression in the AS clause.\nIf language=SQL, it is the substring inside (but excluding) the parentheses."]
    pub fn definition_body(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.definition_body", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the routine if defined."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `determinism_level` after provisioning.\nThe determinism level of the JavaScript UDF if defined. Possible values: [\"DETERMINISM_LEVEL_UNSPECIFIED\", \"DETERMINISTIC\", \"NOT_DETERMINISTIC\"]"]
    pub fn determinism_level(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.determinism_level", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `imported_libraries` after provisioning.\nOptional. If language = \"JAVASCRIPT\", this field stores the path of the\nimported JAVASCRIPT libraries."]
    pub fn imported_libraries(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.imported_libraries", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `language` after provisioning.\nThe language of the routine. Possible values: [\"SQL\", \"JAVASCRIPT\", \"PYTHON\", \"JAVA\", \"SCALA\"]"]
    pub fn language(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.language", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this routine was modified, in milliseconds since the\nepoch."]
    pub fn last_modified_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_modified_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `return_table_type` after provisioning.\nOptional. Can be set only if routineType = \"TABLE_VALUED_FUNCTION\".\n\nIf absent, the return table type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the columns in the evaluated table result will\nbe cast to match the column types specificed in return table type, at query time."]
    pub fn return_table_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_table_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `return_type` after provisioning.\nA JSON schema for the return type. Optional if language = \"SQL\"; required otherwise.\nIf absent, the return type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the evaluated result will be cast to\nthe specified returned type at query time. ~>**NOTE**: Because this field expects a JSON\nstring, any changes to the string will create a diff, even if the JSON itself hasn't\nchanged. If the API returns a different value for the same schema, e.g. it switche\nd the order of values or replaced STRUCT field type with RECORD field type, we currently\ncannot suppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn return_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.return_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routine_id` after provisioning.\nThe ID of the the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
    pub fn routine_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routine_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `routine_type` after provisioning.\nThe type of routine. Possible values: [\"SCALAR_FUNCTION\", \"PROCEDURE\", \"TABLE_VALUED_FUNCTION\"]"]
    pub fn routine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routine_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_mode` after provisioning.\nOptional. The security mode of the routine, if defined. If not defined, the security mode is automatically determined from the routine's configuration. Possible values: [\"DEFINER\", \"INVOKER\"]"]
    pub fn security_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arguments` after provisioning.\n"]
    pub fn arguments(&self) -> ListRef<BigqueryRoutineArgumentsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.arguments", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `remote_function_options` after provisioning.\n"]
    pub fn remote_function_options(&self) -> ListRef<BigqueryRoutineRemoteFunctionOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remote_function_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spark_options` after provisioning.\n"]
    pub fn spark_options(&self) -> ListRef<BigqueryRoutineSparkOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spark_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryRoutineTimeoutsElRef {
        BigqueryRoutineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryRoutineArgumentsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    argument_kind: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}
impl BigqueryRoutineArgumentsEl {
    #[doc = "Set the field `argument_kind`.\nDefaults to FIXED_TYPE. Default value: \"FIXED_TYPE\" Possible values: [\"FIXED_TYPE\", \"ANY_TYPE\"]"]
    pub fn set_argument_kind(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.argument_kind = Some(v.into());
        self
    }
    #[doc = "Set the field `data_type`.\nA JSON schema for the data type. Required unless argumentKind = ANY_TYPE.\n~>**NOTE**: Because this field expects a JSON string, any changes to the string\nwill create a diff, even if the JSON itself hasn't changed. If the API returns\na different value for the same schema, e.g. it switched the order of values\nor replaced STRUCT field type with RECORD field type, we currently cannot\nsuppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn set_data_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data_type = Some(v.into());
        self
    }
    #[doc = "Set the field `mode`.\nSpecifies whether the argument is input or output. Can be set for procedures only. Possible values: [\"IN\", \"OUT\", \"INOUT\"]"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
    #[doc = "Set the field `name`.\nThe name of this argument. Can be absent for function return argument."]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryRoutineArgumentsEl {
    type O = BlockAssignable<BigqueryRoutineArgumentsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryRoutineArgumentsEl {}
impl BuildBigqueryRoutineArgumentsEl {
    pub fn build(self) -> BigqueryRoutineArgumentsEl {
        BigqueryRoutineArgumentsEl {
            argument_kind: core::default::Default::default(),
            data_type: core::default::Default::default(),
            mode: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}
pub struct BigqueryRoutineArgumentsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryRoutineArgumentsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryRoutineArgumentsElRef {
        BigqueryRoutineArgumentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryRoutineArgumentsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `argument_kind` after provisioning.\nDefaults to FIXED_TYPE. Default value: \"FIXED_TYPE\" Possible values: [\"FIXED_TYPE\", \"ANY_TYPE\"]"]
    pub fn argument_kind(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.argument_kind", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `data_type` after provisioning.\nA JSON schema for the data type. Required unless argumentKind = ANY_TYPE.\n~>**NOTE**: Because this field expects a JSON string, any changes to the string\nwill create a diff, even if the JSON itself hasn't changed. If the API returns\na different value for the same schema, e.g. it switched the order of values\nor replaced STRUCT field type with RECORD field type, we currently cannot\nsuppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn data_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }
    #[doc = "Get a reference to the value of field `mode` after provisioning.\nSpecifies whether the argument is input or output. Can be set for procedures only. Possible values: [\"IN\", \"OUT\", \"INOUT\"]"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this argument. Can be absent for function return argument."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryRoutineRemoteFunctionOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    connection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_batching_rows: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined_context: Option<RecField<PrimField<String>>>,
}
impl BigqueryRoutineRemoteFunctionOptionsEl {
    #[doc = "Set the field `connection`.\nFully qualified name of the user-provided connection object which holds\nthe authentication information to send requests to the remote service.\nFormat: \"projects/{projectId}/locations/{locationId}/connections/{connectionId}\""]
    pub fn set_connection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint`.\nEndpoint of the user-provided remote service, e.g.\n'https://us-east1-my_gcf_project.cloudfunctions.net/remote_add'"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }
    #[doc = "Set the field `max_batching_rows`.\nMax number of rows in each batch sent to the remote service. If absent or if 0,\nBigQuery dynamically decides the number of rows in a batch."]
    pub fn set_max_batching_rows(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_batching_rows = Some(v.into());
        self
    }
    #[doc = "Set the field `user_defined_context`.\nUser-defined context as a set of key/value pairs, which will be sent as function\ninvocation context together with batched arguments in the requests to the remote\nservice. The total number of bytes of keys and values must be less than 8KB.\n\nAn object containing a list of \"key\": value pairs. Example:\n'{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn set_user_defined_context(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.user_defined_context = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryRoutineRemoteFunctionOptionsEl {
    type O = BlockAssignable<BigqueryRoutineRemoteFunctionOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryRoutineRemoteFunctionOptionsEl {}
impl BuildBigqueryRoutineRemoteFunctionOptionsEl {
    pub fn build(self) -> BigqueryRoutineRemoteFunctionOptionsEl {
        BigqueryRoutineRemoteFunctionOptionsEl {
            connection: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            max_batching_rows: core::default::Default::default(),
            user_defined_context: core::default::Default::default(),
        }
    }
}
pub struct BigqueryRoutineRemoteFunctionOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryRoutineRemoteFunctionOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryRoutineRemoteFunctionOptionsElRef {
        BigqueryRoutineRemoteFunctionOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryRoutineRemoteFunctionOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connection` after provisioning.\nFully qualified name of the user-provided connection object which holds\nthe authentication information to send requests to the remote service.\nFormat: \"projects/{projectId}/locations/{locationId}/connections/{connectionId}\""]
    pub fn connection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection", self.base))
    }
    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nEndpoint of the user-provided remote service, e.g.\n'https://us-east1-my_gcf_project.cloudfunctions.net/remote_add'"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
    #[doc = "Get a reference to the value of field `max_batching_rows` after provisioning.\nMax number of rows in each batch sent to the remote service. If absent or if 0,\nBigQuery dynamically decides the number of rows in a batch."]
    pub fn max_batching_rows(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_batching_rows", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `user_defined_context` after provisioning.\nUser-defined context as a set of key/value pairs, which will be sent as function\ninvocation context together with batched arguments in the requests to the remote\nservice. The total number of bytes of keys and values must be less than 8KB.\n\nAn object containing a list of \"key\": value pairs. Example:\n'{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn user_defined_context(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.user_defined_context", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryRoutineSparkOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    archive_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jar_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_class: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    main_file_uri: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    py_file_uris: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_version: Option<PrimField<String>>,
}
impl BigqueryRoutineSparkOptionsEl {
    #[doc = "Set the field `archive_uris`.\nArchive files to be extracted into the working directory of each executor. For more information about Apache Spark, see Apache Spark."]
    pub fn set_archive_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.archive_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `connection`.\nFully qualified name of the user-provided Spark connection object.\nFormat: \"projects/{projectId}/locations/{locationId}/connections/{connectionId}\""]
    pub fn set_connection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection = Some(v.into());
        self
    }
    #[doc = "Set the field `container_image`.\nCustom container image for the runtime environment."]
    pub fn set_container_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.container_image = Some(v.into());
        self
    }
    #[doc = "Set the field `file_uris`.\nFiles to be placed in the working directory of each executor. For more information about Apache Spark, see Apache Spark."]
    pub fn set_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `jar_uris`.\nJARs to include on the driver and executor CLASSPATH. For more information about Apache Spark, see Apache Spark."]
    pub fn set_jar_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.jar_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `main_class`.\nThe fully qualified name of a class in jarUris, for example, com.example.wordcount.\nExactly one of mainClass and main_jar_uri field should be set for Java/Scala language type."]
    pub fn set_main_class(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_class = Some(v.into());
        self
    }
    #[doc = "Set the field `main_file_uri`.\nThe main file/jar URI of the Spark application.\nExactly one of the definitionBody field and the mainFileUri field must be set for Python.\nExactly one of mainClass and mainFileUri field should be set for Java/Scala language type."]
    pub fn set_main_file_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.main_file_uri = Some(v.into());
        self
    }
    #[doc = "Set the field `properties`.\nConfiguration properties as a set of key/value pairs, which will be passed on to the Spark application.\nFor more information, see Apache Spark and the procedure option list.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_properties(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.properties = Some(v.into());
        self
    }
    #[doc = "Set the field `py_file_uris`.\nPython files to be placed on the PYTHONPATH for PySpark application. Supported file types: .py, .egg, and .zip. For more information about Apache Spark, see Apache Spark."]
    pub fn set_py_file_uris(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.py_file_uris = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime_version`.\nRuntime version. If not specified, the default runtime version is used."]
    pub fn set_runtime_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.runtime_version = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryRoutineSparkOptionsEl {
    type O = BlockAssignable<BigqueryRoutineSparkOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryRoutineSparkOptionsEl {}
impl BuildBigqueryRoutineSparkOptionsEl {
    pub fn build(self) -> BigqueryRoutineSparkOptionsEl {
        BigqueryRoutineSparkOptionsEl {
            archive_uris: core::default::Default::default(),
            connection: core::default::Default::default(),
            container_image: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_file_uri: core::default::Default::default(),
            properties: core::default::Default::default(),
            py_file_uris: core::default::Default::default(),
            runtime_version: core::default::Default::default(),
        }
    }
}
pub struct BigqueryRoutineSparkOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryRoutineSparkOptionsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryRoutineSparkOptionsElRef {
        BigqueryRoutineSparkOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryRoutineSparkOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `archive_uris` after provisioning.\nArchive files to be extracted into the working directory of each executor. For more information about Apache Spark, see Apache Spark."]
    pub fn archive_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `connection` after provisioning.\nFully qualified name of the user-provided Spark connection object.\nFormat: \"projects/{projectId}/locations/{locationId}/connections/{connectionId}\""]
    pub fn connection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection", self.base))
    }
    #[doc = "Get a reference to the value of field `container_image` after provisioning.\nCustom container image for the runtime environment."]
    pub fn container_image(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_image", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `file_uris` after provisioning.\nFiles to be placed in the working directory of each executor. For more information about Apache Spark, see Apache Spark."]
    pub fn file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `jar_uris` after provisioning.\nJARs to include on the driver and executor CLASSPATH. For more information about Apache Spark, see Apache Spark."]
    pub fn jar_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.jar_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `main_class` after provisioning.\nThe fully qualified name of a class in jarUris, for example, com.example.wordcount.\nExactly one of mainClass and main_jar_uri field should be set for Java/Scala language type."]
    pub fn main_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }
    #[doc = "Get a reference to the value of field `main_file_uri` after provisioning.\nThe main file/jar URI of the Spark application.\nExactly one of the definitionBody field and the mainFileUri field must be set for Python.\nExactly one of mainClass and mainFileUri field should be set for Java/Scala language type."]
    pub fn main_file_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.main_file_uri", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `properties` after provisioning.\nConfiguration properties as a set of key/value pairs, which will be passed on to the Spark application.\nFor more information, see Apache Spark and the procedure option list.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn properties(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
    #[doc = "Get a reference to the value of field `py_file_uris` after provisioning.\nPython files to be placed on the PYTHONPATH for PySpark application. Supported file types: .py, .egg, and .zip. For more information about Apache Spark, see Apache Spark."]
    pub fn py_file_uris(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.py_file_uris", self.base))
    }
    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\nRuntime version. If not specified, the default runtime version is used."]
    pub fn runtime_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.runtime_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryRoutineTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryRoutineTimeoutsEl {
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
impl ToListMappable for BigqueryRoutineTimeoutsEl {
    type O = BlockAssignable<BigqueryRoutineTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryRoutineTimeoutsEl {}
impl BuildBigqueryRoutineTimeoutsEl {
    pub fn build(self) -> BigqueryRoutineTimeoutsEl {
        BigqueryRoutineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryRoutineTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryRoutineTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryRoutineTimeoutsElRef {
        BigqueryRoutineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryRoutineTimeoutsElRef {
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
struct BigqueryRoutineDynamic {
    arguments: Option<DynamicBlock<BigqueryRoutineArgumentsEl>>,
    remote_function_options: Option<DynamicBlock<BigqueryRoutineRemoteFunctionOptionsEl>>,
    spark_options: Option<DynamicBlock<BigqueryRoutineSparkOptionsEl>>,
}
