use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryDatapolicyDataPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    data_policy_id: PrimField<String>,
    data_policy_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    policy_tag: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_masking_policy: Option<Vec<BigqueryDatapolicyDataPolicyDataMaskingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryDatapolicyDataPolicyTimeoutsEl>,
    dynamic: BigqueryDatapolicyDataPolicyDynamic,
}
struct BigqueryDatapolicyDataPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryDatapolicyDataPolicyData>,
}
#[derive(Clone)]
pub struct BigqueryDatapolicyDataPolicy(Rc<BigqueryDatapolicyDataPolicy_>);
impl BigqueryDatapolicyDataPolicy {
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
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `data_masking_policy`.\n"]
    pub fn set_data_masking_policy(
        self,
        v: impl Into<BlockAssignable<BigqueryDatapolicyDataPolicyDataMaskingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_masking_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_masking_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryDatapolicyDataPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `data_policy_id` after provisioning.\nUser-assigned (human readable) ID of the data policy that needs to be unique within a project. Used as {dataPolicyId} in part of the resource name."]
    pub fn data_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_policy_type` after provisioning.\nThe enrollment level of the service. Possible values: [\"COLUMN_LEVEL_SECURITY_POLICY\", \"DATA_MASKING_POLICY\"]"]
    pub fn data_policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_policy_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the data policy."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of this data policy, in the format of projects/{project_number}/locations/{locationId}/dataPolicies/{dataPolicyId}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_tag` after provisioning.\nPolicy tag resource name, in the format of projects/{project_number}/locations/{locationId}/taxonomies/{taxonomyId}/policyTags/{policyTag_id}."]
    pub fn policy_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_tag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_masking_policy` after provisioning.\n"]
    pub fn data_masking_policy(
        &self,
    ) -> ListRef<BigqueryDatapolicyDataPolicyDataMaskingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_masking_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatapolicyDataPolicyTimeoutsElRef {
        BigqueryDatapolicyDataPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryDatapolicyDataPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryDatapolicyDataPolicy {}
impl ToListMappable for BigqueryDatapolicyDataPolicy {
    type O = ListRef<BigqueryDatapolicyDataPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryDatapolicyDataPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_datapolicy_data_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryDatapolicyDataPolicy {
    pub tf_id: String,
    #[doc = "User-assigned (human readable) ID of the data policy that needs to be unique within a project. Used as {dataPolicyId} in part of the resource name."]
    pub data_policy_id: PrimField<String>,
    #[doc = "The enrollment level of the service. Possible values: [\"COLUMN_LEVEL_SECURITY_POLICY\", \"DATA_MASKING_POLICY\"]"]
    pub data_policy_type: PrimField<String>,
    #[doc = "The name of the location of the data policy."]
    pub location: PrimField<String>,
    #[doc = "Policy tag resource name, in the format of projects/{project_number}/locations/{locationId}/taxonomies/{taxonomyId}/policyTags/{policyTag_id}."]
    pub policy_tag: PrimField<String>,
}
impl BuildBigqueryDatapolicyDataPolicy {
    pub fn build(self, stack: &mut Stack) -> BigqueryDatapolicyDataPolicy {
        let out = BigqueryDatapolicyDataPolicy(Rc::new(BigqueryDatapolicyDataPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryDatapolicyDataPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_policy_id: self.data_policy_id,
                data_policy_type: self.data_policy_type,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                policy_tag: self.policy_tag,
                project: core::default::Default::default(),
                data_masking_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BigqueryDatapolicyDataPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatapolicyDataPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryDatapolicyDataPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_policy_id` after provisioning.\nUser-assigned (human readable) ID of the data policy that needs to be unique within a project. Used as {dataPolicyId} in part of the resource name."]
    pub fn data_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_policy_type` after provisioning.\nThe enrollment level of the service. Possible values: [\"COLUMN_LEVEL_SECURITY_POLICY\", \"DATA_MASKING_POLICY\"]"]
    pub fn data_policy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_policy_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the data policy."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name of this data policy, in the format of projects/{project_number}/locations/{locationId}/dataPolicies/{dataPolicyId}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_tag` after provisioning.\nPolicy tag resource name, in the format of projects/{project_number}/locations/{locationId}/taxonomies/{taxonomyId}/policyTags/{policyTag_id}."]
    pub fn policy_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.policy_tag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_masking_policy` after provisioning.\n"]
    pub fn data_masking_policy(
        &self,
    ) -> ListRef<BigqueryDatapolicyDataPolicyDataMaskingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_masking_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatapolicyDataPolicyTimeoutsElRef {
        BigqueryDatapolicyDataPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryDatapolicyDataPolicyDataMaskingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routine: Option<PrimField<String>>,
}
impl BigqueryDatapolicyDataPolicyDataMaskingPolicyEl {
    #[doc = "Set the field `predefined_expression`.\nThe available masking rules. Learn more here: https://cloud.google.com/bigquery/docs/column-data-masking-intro#masking_options. Possible values: [\"SHA256\", \"ALWAYS_NULL\", \"DEFAULT_MASKING_VALUE\", \"LAST_FOUR_CHARACTERS\", \"FIRST_FOUR_CHARACTERS\", \"EMAIL_MASK\", \"DATE_YEAR_MASK\"]"]
    pub fn set_predefined_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.predefined_expression = Some(v.into());
        self
    }
    #[doc = "Set the field `routine`.\nThe name of the BigQuery routine that contains the custom masking routine, in the format of projects/{projectNumber}/datasets/{dataset_id}/routines/{routine_id}."]
    pub fn set_routine(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.routine = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryDatapolicyDataPolicyDataMaskingPolicyEl {
    type O = BlockAssignable<BigqueryDatapolicyDataPolicyDataMaskingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatapolicyDataPolicyDataMaskingPolicyEl {}
impl BuildBigqueryDatapolicyDataPolicyDataMaskingPolicyEl {
    pub fn build(self) -> BigqueryDatapolicyDataPolicyDataMaskingPolicyEl {
        BigqueryDatapolicyDataPolicyDataMaskingPolicyEl {
            predefined_expression: core::default::Default::default(),
            routine: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDatapolicyDataPolicyDataMaskingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatapolicyDataPolicyDataMaskingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryDatapolicyDataPolicyDataMaskingPolicyElRef {
        BigqueryDatapolicyDataPolicyDataMaskingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatapolicyDataPolicyDataMaskingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `predefined_expression` after provisioning.\nThe available masking rules. Learn more here: https://cloud.google.com/bigquery/docs/column-data-masking-intro#masking_options. Possible values: [\"SHA256\", \"ALWAYS_NULL\", \"DEFAULT_MASKING_VALUE\", \"LAST_FOUR_CHARACTERS\", \"FIRST_FOUR_CHARACTERS\", \"EMAIL_MASK\", \"DATE_YEAR_MASK\"]"]
    pub fn predefined_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.predefined_expression", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `routine` after provisioning.\nThe name of the BigQuery routine that contains the custom masking routine, in the format of projects/{projectNumber}/datasets/{dataset_id}/routines/{routine_id}."]
    pub fn routine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routine", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatapolicyDataPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryDatapolicyDataPolicyTimeoutsEl {
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
impl ToListMappable for BigqueryDatapolicyDataPolicyTimeoutsEl {
    type O = BlockAssignable<BigqueryDatapolicyDataPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatapolicyDataPolicyTimeoutsEl {}
impl BuildBigqueryDatapolicyDataPolicyTimeoutsEl {
    pub fn build(self) -> BigqueryDatapolicyDataPolicyTimeoutsEl {
        BigqueryDatapolicyDataPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDatapolicyDataPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatapolicyDataPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatapolicyDataPolicyTimeoutsElRef {
        BigqueryDatapolicyDataPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatapolicyDataPolicyTimeoutsElRef {
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
struct BigqueryDatapolicyDataPolicyDynamic {
    data_masking_policy: Option<DynamicBlock<BigqueryDatapolicyDataPolicyDataMaskingPolicyEl>>,
}
