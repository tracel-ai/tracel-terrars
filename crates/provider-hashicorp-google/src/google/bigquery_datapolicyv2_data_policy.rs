use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BigqueryDatapolicyv2DataPolicyData {
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
    etag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grantees: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_masking_policy: Option<Vec<BigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryDatapolicyv2DataPolicyTimeoutsEl>,
    dynamic: BigqueryDatapolicyv2DataPolicyDynamic,
}
struct BigqueryDatapolicyv2DataPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryDatapolicyv2DataPolicyData>,
}
#[derive(Clone)]
pub struct BigqueryDatapolicyv2DataPolicy(Rc<BigqueryDatapolicyv2DataPolicy_>);
impl BigqueryDatapolicyv2DataPolicy {
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
    #[doc = "Set the field `etag`.\nThe etag for this Data Policy.\nThis field is used for UpdateDataPolicy calls. If Data Policy exists, this\nfield is required and must match the server's etag. It will also be\npopulated in the response of GetDataPolicy, CreateDataPolicy, and\nUpdateDataPolicy calls."]
    pub fn set_etag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().etag = Some(v.into());
        self
    }
    #[doc = "Set the field `grantees`.\nThe list of IAM principals that have Fine Grained Access to the underlying\ndata goverened by this data policy.\n\nUses the [IAM V2 principal\nsyntax](https://cloud.google.com/iam/docs/principal-identifiers#v2) Only\nsupports principal types users, groups, serviceaccounts, cloudidentity.\nThis field is supported in V2 Data Policy only. In case of V1 data policies\n(i.e. verion = 1 and policy_tag is set), this field is not populated."]
    pub fn set_grantees(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().grantees = Some(v.into());
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
        v: impl Into<BlockAssignable<BigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl>>,
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
    pub fn set_timeouts(self, v: impl Into<BigqueryDatapolicyv2DataPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `data_policy_id` after provisioning.\nUser-assigned (human readable) ID of the data policy that needs to be\nunique within a project. Used as {data_policy_id} in part of the resource\nname."]
    pub fn data_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_policy_type` after provisioning.\nType of data policy.\nPossible values:\nDATA_MASKING_POLICY\nRAW_DATA_ACCESS_POLICY\nCOLUMN_LEVEL_SECURITY_POLICY"]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag for this Data Policy.\nThis field is used for UpdateDataPolicy calls. If Data Policy exists, this\nfield is required and must match the server's etag. It will also be\npopulated in the response of GetDataPolicy, CreateDataPolicy, and\nUpdateDataPolicy calls."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grantees` after provisioning.\nThe list of IAM principals that have Fine Grained Access to the underlying\ndata goverened by this data policy.\n\nUses the [IAM V2 principal\nsyntax](https://cloud.google.com/iam/docs/principal-identifiers#v2) Only\nsupports principal types users, groups, serviceaccounts, cloudidentity.\nThis field is supported in V2 Data Policy only. In case of V1 data policies\n(i.e. verion = 1 and policy_tag is set), this field is not populated."]
    pub fn grantees(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grantees", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name of this data policy, in the format of\n'projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_tag` after provisioning.\nPolicy tag resource name, in the format of\n'projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policyTag_id}'.\npolicy_tag is supported only for V1 data policies."]
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
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the Data Policy resource.\nPossible values:\nV1\nV2"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_masking_policy` after provisioning.\n"]
    pub fn data_masking_policy(
        &self,
    ) -> ListRef<BigqueryDatapolicyv2DataPolicyDataMaskingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_masking_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatapolicyv2DataPolicyTimeoutsElRef {
        BigqueryDatapolicyv2DataPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BigqueryDatapolicyv2DataPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BigqueryDatapolicyv2DataPolicy {}
impl ToListMappable for BigqueryDatapolicyv2DataPolicy {
    type O = ListRef<BigqueryDatapolicyv2DataPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BigqueryDatapolicyv2DataPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_datapolicyv2_data_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBigqueryDatapolicyv2DataPolicy {
    pub tf_id: String,
    #[doc = "User-assigned (human readable) ID of the data policy that needs to be\nunique within a project. Used as {data_policy_id} in part of the resource\nname."]
    pub data_policy_id: PrimField<String>,
    #[doc = "Type of data policy.\nPossible values:\nDATA_MASKING_POLICY\nRAW_DATA_ACCESS_POLICY\nCOLUMN_LEVEL_SECURITY_POLICY"]
    pub data_policy_type: PrimField<String>,
    #[doc = "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: PrimField<String>,
}
impl BuildBigqueryDatapolicyv2DataPolicy {
    pub fn build(self, stack: &mut Stack) -> BigqueryDatapolicyv2DataPolicy {
        let out = BigqueryDatapolicyv2DataPolicy(Rc::new(BigqueryDatapolicyv2DataPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryDatapolicyv2DataPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_policy_id: self.data_policy_id,
                data_policy_type: self.data_policy_type,
                deletion_policy: core::default::Default::default(),
                etag: core::default::Default::default(),
                grantees: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
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
pub struct BigqueryDatapolicyv2DataPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatapolicyv2DataPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BigqueryDatapolicyv2DataPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `data_policy_id` after provisioning.\nUser-assigned (human readable) ID of the data policy that needs to be\nunique within a project. Used as {data_policy_id} in part of the resource\nname."]
    pub fn data_policy_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_policy_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_policy_type` after provisioning.\nType of data policy.\nPossible values:\nDATA_MASKING_POLICY\nRAW_DATA_ACCESS_POLICY\nCOLUMN_LEVEL_SECURITY_POLICY"]
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\nThe etag for this Data Policy.\nThis field is used for UpdateDataPolicy calls. If Data Policy exists, this\nfield is required and must match the server's etag. It will also be\npopulated in the response of GetDataPolicy, CreateDataPolicy, and\nUpdateDataPolicy calls."]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `grantees` after provisioning.\nThe list of IAM principals that have Fine Grained Access to the underlying\ndata goverened by this data policy.\n\nUses the [IAM V2 principal\nsyntax](https://cloud.google.com/iam/docs/principal-identifiers#v2) Only\nsupports principal types users, groups, serviceaccounts, cloudidentity.\nThis field is supported in V2 Data Policy only. In case of V1 data policies\n(i.e. verion = 1 and policy_tag is set), this field is not populated."]
    pub fn grantees(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.grantees", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name of this data policy, in the format of\n'projects/{project_number}/locations/{location_id}/dataPolicies/{data_policy_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `policy_tag` after provisioning.\nPolicy tag resource name, in the format of\n'projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{policyTag_id}'.\npolicy_tag is supported only for V1 data policies."]
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
    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the Data Policy resource.\nPossible values:\nV1\nV2"]
    pub fn version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_masking_policy` after provisioning.\n"]
    pub fn data_masking_policy(
        &self,
    ) -> ListRef<BigqueryDatapolicyv2DataPolicyDataMaskingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.data_masking_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDatapolicyv2DataPolicyTimeoutsElRef {
        BigqueryDatapolicyv2DataPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    predefined_expression: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routine: Option<PrimField<String>>,
}
impl BigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl {
    #[doc = "Set the field `predefined_expression`.\nA predefined masking expression.\nPossible values:\nSHA256\nALWAYS_NULL\nDEFAULT_MASKING_VALUE\nLAST_FOUR_CHARACTERS\nFIRST_FOUR_CHARACTERS\nEMAIL_MASK\nDATE_YEAR_MASK\nRANDOM_HASH"]
    pub fn set_predefined_expression(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.predefined_expression = Some(v.into());
        self
    }
    #[doc = "Set the field `routine`.\nThe name of the BigQuery routine that contains the custom masking\nroutine, in the format of\n'projects/{project_number}/datasets/{dataset_id}/routines/{routine_id}'."]
    pub fn set_routine(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.routine = Some(v.into());
        self
    }
}
impl ToListMappable for BigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl {
    type O = BlockAssignable<BigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl {}
impl BuildBigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl {
    pub fn build(self) -> BigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl {
        BigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl {
            predefined_expression: core::default::Default::default(),
            routine: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDatapolicyv2DataPolicyDataMaskingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatapolicyv2DataPolicyDataMaskingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BigqueryDatapolicyv2DataPolicyDataMaskingPolicyElRef {
        BigqueryDatapolicyv2DataPolicyDataMaskingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatapolicyv2DataPolicyDataMaskingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `predefined_expression` after provisioning.\nA predefined masking expression.\nPossible values:\nSHA256\nALWAYS_NULL\nDEFAULT_MASKING_VALUE\nLAST_FOUR_CHARACTERS\nFIRST_FOUR_CHARACTERS\nEMAIL_MASK\nDATE_YEAR_MASK\nRANDOM_HASH"]
    pub fn predefined_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.predefined_expression", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `routine` after provisioning.\nThe name of the BigQuery routine that contains the custom masking\nroutine, in the format of\n'projects/{project_number}/datasets/{dataset_id}/routines/{routine_id}'."]
    pub fn routine(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routine", self.base))
    }
}
#[derive(Serialize)]
pub struct BigqueryDatapolicyv2DataPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BigqueryDatapolicyv2DataPolicyTimeoutsEl {
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
impl ToListMappable for BigqueryDatapolicyv2DataPolicyTimeoutsEl {
    type O = BlockAssignable<BigqueryDatapolicyv2DataPolicyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBigqueryDatapolicyv2DataPolicyTimeoutsEl {}
impl BuildBigqueryDatapolicyv2DataPolicyTimeoutsEl {
    pub fn build(self) -> BigqueryDatapolicyv2DataPolicyTimeoutsEl {
        BigqueryDatapolicyv2DataPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BigqueryDatapolicyv2DataPolicyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BigqueryDatapolicyv2DataPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryDatapolicyv2DataPolicyTimeoutsElRef {
        BigqueryDatapolicyv2DataPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BigqueryDatapolicyv2DataPolicyTimeoutsElRef {
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
struct BigqueryDatapolicyv2DataPolicyDynamic {
    data_masking_policy: Option<DynamicBlock<BigqueryDatapolicyv2DataPolicyDataMaskingPolicyEl>>,
}
