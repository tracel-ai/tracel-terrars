use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct SccFolderSccBigQueryExportData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    big_query_export_id: PrimField<String>,
    dataset: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    description: PrimField<String>,
    filter: PrimField<String>,
    folder: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<SccFolderSccBigQueryExportTimeoutsEl>,
}
struct SccFolderSccBigQueryExport_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SccFolderSccBigQueryExportData>,
}
#[derive(Clone)]
pub struct SccFolderSccBigQueryExport(Rc<SccFolderSccBigQueryExport_>);
impl SccFolderSccBigQueryExport {
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<SccFolderSccBigQueryExportTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `big_query_export_id` after provisioning.\nThis must be unique within the organization."]
    pub fn big_query_export_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.big_query_export_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the BigQuery export was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nThe dataset to write findings' updates to.\nIts format is \"projects/[projectId]/datasets/[bigquery_dataset_id]\".\nBigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_)."]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the export (max of 1024 characters)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nExpression that defines the filter to apply across create/update\nevents of findings. The\nexpression is a list of zero or more restrictions combined via\nlogical operators AND and OR. Parentheses are supported, and OR\nhas higher precedence than AND.\n\nRestrictions have the form <field> <operator> <value> and may have\na - character in front of them to indicate negation. The fields\nmap to those defined in the corresponding resource.\n\nThe supported operators are:\n\n* = for all value types.\n* >, <, >=, <= for integer values.\n* :, meaning substring matching, for strings.\n\nThe supported value types are:\n\n* string literals in quotes.\n* integer literals without quotes.\n* boolean literals true and false without quotes.\n\nSee\n[Filtering notifications](https://cloud.google.com/security-command-center/docs/how-to-api-filter-notifications)\nfor information on how to write a filter."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe folder where Cloud Security Command Center Big Query Export\nConfig lives in."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `most_recent_editor` after provisioning.\nEmail address of the user who last edited the BigQuery export."]
    pub fn most_recent_editor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_recent_editor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of this export, in the format\n'projects/{{project}}/bigQueryExports/{{big_query_export_id}}'.\nThis field is provided in responses, and is ignored when provided in create requests."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\nThe service account that needs permission to create table and upload data to the BigQuery dataset."]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which the BigQuery export was updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccFolderSccBigQueryExportTimeoutsElRef {
        SccFolderSccBigQueryExportTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for SccFolderSccBigQueryExport {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for SccFolderSccBigQueryExport {}
impl ToListMappable for SccFolderSccBigQueryExport {
    type O = ListRef<SccFolderSccBigQueryExportRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for SccFolderSccBigQueryExport_ {
    fn extract_resource_type(&self) -> String {
        "google_scc_folder_scc_big_query_export".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildSccFolderSccBigQueryExport {
    pub tf_id: String,
    #[doc = "This must be unique within the organization."]
    pub big_query_export_id: PrimField<String>,
    #[doc = "The dataset to write findings' updates to.\nIts format is \"projects/[projectId]/datasets/[bigquery_dataset_id]\".\nBigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_)."]
    pub dataset: PrimField<String>,
    #[doc = "The description of the export (max of 1024 characters)."]
    pub description: PrimField<String>,
    #[doc = "Expression that defines the filter to apply across create/update\nevents of findings. The\nexpression is a list of zero or more restrictions combined via\nlogical operators AND and OR. Parentheses are supported, and OR\nhas higher precedence than AND.\n\nRestrictions have the form <field> <operator> <value> and may have\na - character in front of them to indicate negation. The fields\nmap to those defined in the corresponding resource.\n\nThe supported operators are:\n\n* = for all value types.\n* >, <, >=, <= for integer values.\n* :, meaning substring matching, for strings.\n\nThe supported value types are:\n\n* string literals in quotes.\n* integer literals without quotes.\n* boolean literals true and false without quotes.\n\nSee\n[Filtering notifications](https://cloud.google.com/security-command-center/docs/how-to-api-filter-notifications)\nfor information on how to write a filter."]
    pub filter: PrimField<String>,
    #[doc = "The folder where Cloud Security Command Center Big Query Export\nConfig lives in."]
    pub folder: PrimField<String>,
}
impl BuildSccFolderSccBigQueryExport {
    pub fn build(self, stack: &mut Stack) -> SccFolderSccBigQueryExport {
        let out = SccFolderSccBigQueryExport(Rc::new(SccFolderSccBigQueryExport_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SccFolderSccBigQueryExportData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                big_query_export_id: self.big_query_export_id,
                dataset: self.dataset,
                deletion_policy: core::default::Default::default(),
                description: self.description,
                filter: self.filter,
                folder: self.folder,
                id: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct SccFolderSccBigQueryExportRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccFolderSccBigQueryExportRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl SccFolderSccBigQueryExportRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `big_query_export_id` after provisioning.\nThis must be unique within the organization."]
    pub fn big_query_export_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.big_query_export_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the BigQuery export was created.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nThe dataset to write findings' updates to.\nIts format is \"projects/[projectId]/datasets/[bigquery_dataset_id]\".\nBigQuery Dataset unique ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_)."]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.dataset", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the export (max of 1024 characters)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `filter` after provisioning.\nExpression that defines the filter to apply across create/update\nevents of findings. The\nexpression is a list of zero or more restrictions combined via\nlogical operators AND and OR. Parentheses are supported, and OR\nhas higher precedence than AND.\n\nRestrictions have the form <field> <operator> <value> and may have\na - character in front of them to indicate negation. The fields\nmap to those defined in the corresponding resource.\n\nThe supported operators are:\n\n* = for all value types.\n* >, <, >=, <= for integer values.\n* :, meaning substring matching, for strings.\n\nThe supported value types are:\n\n* string literals in quotes.\n* integer literals without quotes.\n* boolean literals true and false without quotes.\n\nSee\n[Filtering notifications](https://cloud.google.com/security-command-center/docs/how-to-api-filter-notifications)\nfor information on how to write a filter."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.filter", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `folder` after provisioning.\nThe folder where Cloud Security Command Center Big Query Export\nConfig lives in."]
    pub fn folder(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.folder", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `most_recent_editor` after provisioning.\nEmail address of the user who last edited the BigQuery export."]
    pub fn most_recent_editor(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.most_recent_editor", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of this export, in the format\n'projects/{{project}}/bigQueryExports/{{big_query_export_id}}'.\nThis field is provided in responses, and is ignored when provided in create requests."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `principal` after provisioning.\nThe service account that needs permission to create table and upload data to the BigQuery dataset."]
    pub fn principal(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.principal", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nThe most recent time at which the BigQuery export was updated.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits.\nExamples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SccFolderSccBigQueryExportTimeoutsElRef {
        SccFolderSccBigQueryExportTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct SccFolderSccBigQueryExportTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl SccFolderSccBigQueryExportTimeoutsEl {
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
impl ToListMappable for SccFolderSccBigQueryExportTimeoutsEl {
    type O = BlockAssignable<SccFolderSccBigQueryExportTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildSccFolderSccBigQueryExportTimeoutsEl {}
impl BuildSccFolderSccBigQueryExportTimeoutsEl {
    pub fn build(self) -> SccFolderSccBigQueryExportTimeoutsEl {
        SccFolderSccBigQueryExportTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct SccFolderSccBigQueryExportTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for SccFolderSccBigQueryExportTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> SccFolderSccBigQueryExportTimeoutsElRef {
        SccFolderSccBigQueryExportTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl SccFolderSccBigQueryExportTimeoutsElRef {
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
