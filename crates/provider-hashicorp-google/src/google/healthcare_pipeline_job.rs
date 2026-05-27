use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct HealthcarePipelineJobData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    dataset: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_lineage: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backfill_pipeline_job: Option<Vec<HealthcarePipelineJobBackfillPipelineJobEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping_pipeline_job: Option<Vec<HealthcarePipelineJobMappingPipelineJobEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reconciliation_pipeline_job: Option<Vec<HealthcarePipelineJobReconciliationPipelineJobEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<HealthcarePipelineJobTimeoutsEl>,
    dynamic: HealthcarePipelineJobDynamic,
}
struct HealthcarePipelineJob_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<HealthcarePipelineJobData>,
}
#[derive(Clone)]
pub struct HealthcarePipelineJob(Rc<HealthcarePipelineJob_>);
impl HealthcarePipelineJob {
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
    #[doc = "Set the field `disable_lineage`.\nIf true, disables writing lineage for the pipeline."]
    pub fn set_disable_lineage(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_lineage = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nUser-supplied key-value pairs used to organize Pipeline Jobs.\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, and must conform to the following PCRE regular expression:\n[\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\nLabel values are optional, must be between 1 and 63 characters long, have a\nUTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE\nregular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\nNo more than 64 labels can be associated with a given pipeline.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `backfill_pipeline_job`.\n"]
    pub fn set_backfill_pipeline_job(
        self,
        v: impl Into<BlockAssignable<HealthcarePipelineJobBackfillPipelineJobEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backfill_pipeline_job = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backfill_pipeline_job = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mapping_pipeline_job`.\n"]
    pub fn set_mapping_pipeline_job(
        self,
        v: impl Into<BlockAssignable<HealthcarePipelineJobMappingPipelineJobEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mapping_pipeline_job = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mapping_pipeline_job = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `reconciliation_pipeline_job`.\n"]
    pub fn set_reconciliation_pipeline_job(
        self,
        v: impl Into<BlockAssignable<HealthcarePipelineJobReconciliationPipelineJobEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reconciliation_pipeline_job = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reconciliation_pipeline_job = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<HealthcarePipelineJobTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nHealthcare Dataset under which the Pipeline Job is to run"]
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
    #[doc = "Get a reference to the value of field `disable_lineage` after provisioning.\nIf true, disables writing lineage for the pipeline."]
    pub fn disable_lineage(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_lineage", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize Pipeline Jobs.\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, and must conform to the following PCRE regular expression:\n[\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\nLabel values are optional, must be between 1 and 63 characters long, have a\nUTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE\nregular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\nNo more than 64 labels can be associated with a given pipeline.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation where the Pipeline Job is to run"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecifies the name of the pipeline job. This field is user-assigned."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backfill_pipeline_job` after provisioning.\n"]
    pub fn backfill_pipeline_job(&self) -> ListRef<HealthcarePipelineJobBackfillPipelineJobElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backfill_pipeline_job", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mapping_pipeline_job` after provisioning.\n"]
    pub fn mapping_pipeline_job(&self) -> ListRef<HealthcarePipelineJobMappingPipelineJobElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mapping_pipeline_job", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciliation_pipeline_job` after provisioning.\n"]
    pub fn reconciliation_pipeline_job(
        &self,
    ) -> ListRef<HealthcarePipelineJobReconciliationPipelineJobElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reconciliation_pipeline_job", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcarePipelineJobTimeoutsElRef {
        HealthcarePipelineJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for HealthcarePipelineJob {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for HealthcarePipelineJob {}
impl ToListMappable for HealthcarePipelineJob {
    type O = ListRef<HealthcarePipelineJobRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for HealthcarePipelineJob_ {
    fn extract_resource_type(&self) -> String {
        "google_healthcare_pipeline_job".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildHealthcarePipelineJob {
    pub tf_id: String,
    #[doc = "Healthcare Dataset under which the Pipeline Job is to run"]
    pub dataset: PrimField<String>,
    #[doc = "Location where the Pipeline Job is to run"]
    pub location: PrimField<String>,
    #[doc = "Specifies the name of the pipeline job. This field is user-assigned."]
    pub name: PrimField<String>,
}
impl BuildHealthcarePipelineJob {
    pub fn build(self, stack: &mut Stack) -> HealthcarePipelineJob {
        let out = HealthcarePipelineJob(Rc::new(HealthcarePipelineJob_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(HealthcarePipelineJobData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                dataset: self.dataset,
                deletion_policy: core::default::Default::default(),
                disable_lineage: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                backfill_pipeline_job: core::default::Default::default(),
                mapping_pipeline_job: core::default::Default::default(),
                reconciliation_pipeline_job: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct HealthcarePipelineJobRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl HealthcarePipelineJobRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dataset` after provisioning.\nHealthcare Dataset under which the Pipeline Job is to run"]
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
    #[doc = "Get a reference to the value of field `disable_lineage` after provisioning.\nIf true, disables writing lineage for the pipeline."]
    pub fn disable_lineage(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_lineage", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize Pipeline Jobs.\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, and must conform to the following PCRE regular expression:\n[\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\nLabel values are optional, must be between 1 and 63 characters long, have a\nUTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE\nregular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\nNo more than 64 labels can be associated with a given pipeline.\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nLocation where the Pipeline Job is to run"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nSpecifies the name of the pipeline job. This field is user-assigned."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `backfill_pipeline_job` after provisioning.\n"]
    pub fn backfill_pipeline_job(&self) -> ListRef<HealthcarePipelineJobBackfillPipelineJobElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.backfill_pipeline_job", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `mapping_pipeline_job` after provisioning.\n"]
    pub fn mapping_pipeline_job(&self) -> ListRef<HealthcarePipelineJobMappingPipelineJobElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mapping_pipeline_job", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `reconciliation_pipeline_job` after provisioning.\n"]
    pub fn reconciliation_pipeline_job(
        &self,
    ) -> ListRef<HealthcarePipelineJobReconciliationPipelineJobElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.reconciliation_pipeline_job", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcarePipelineJobTimeoutsElRef {
        HealthcarePipelineJobTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct HealthcarePipelineJobBackfillPipelineJobEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping_pipeline_job: Option<PrimField<String>>,
}
impl HealthcarePipelineJobBackfillPipelineJobEl {
    #[doc = "Set the field `mapping_pipeline_job`.\nSpecifies the mapping pipeline job to backfill, the name format\nshould follow: projects/{projectId}/locations/{locationId}/datasets/{datasetId}/pipelineJobs/{pipelineJobId}."]
    pub fn set_mapping_pipeline_job(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mapping_pipeline_job = Some(v.into());
        self
    }
}
impl ToListMappable for HealthcarePipelineJobBackfillPipelineJobEl {
    type O = BlockAssignable<HealthcarePipelineJobBackfillPipelineJobEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcarePipelineJobBackfillPipelineJobEl {}
impl BuildHealthcarePipelineJobBackfillPipelineJobEl {
    pub fn build(self) -> HealthcarePipelineJobBackfillPipelineJobEl {
        HealthcarePipelineJobBackfillPipelineJobEl {
            mapping_pipeline_job: core::default::Default::default(),
        }
    }
}
pub struct HealthcarePipelineJobBackfillPipelineJobElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobBackfillPipelineJobElRef {
    fn new(shared: StackShared, base: String) -> HealthcarePipelineJobBackfillPipelineJobElRef {
        HealthcarePipelineJobBackfillPipelineJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcarePipelineJobBackfillPipelineJobElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `mapping_pipeline_job` after provisioning.\nSpecifies the mapping pipeline job to backfill, the name format\nshould follow: projects/{projectId}/locations/{locationId}/datasets/{datasetId}/pipelineJobs/{pipelineJobId}."]
    pub fn mapping_pipeline_job(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mapping_pipeline_job", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    fhir_store: PrimField<String>,
}
impl HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl {
    #[doc = "Set the field `description`.\nDescribes the streaming FHIR data source."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
}
impl ToListMappable for HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl {
    type O = BlockAssignable<HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl {
    #[doc = "The path to the FHIR store in the format projects/{projectId}/locations/{locationId}/datasets/{datasetId}/fhirStores/{fhirStoreId}."]
    pub fhir_store: PrimField<String>,
}
impl BuildHealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl {
    pub fn build(self) -> HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl {
        HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl {
            description: core::default::Default::default(),
            fhir_store: self.fhir_store,
        }
    }
}
pub struct HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceElRef {
        HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescribes the streaming FHIR data source."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `fhir_store` after provisioning.\nThe path to the FHIR store in the format projects/{projectId}/locations/{locationId}/datasets/{datasetId}/fhirStores/{fhirStoreId}."]
    pub fn fhir_store(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fhir_store", self.base))
    }
}
#[derive(Serialize)]
pub struct HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl {
    import_uri_prefix: PrimField<String>,
    uri: PrimField<String>,
}
impl HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl {}
impl ToListMappable
    for HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl
{
    type O = BlockAssignable<
        HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl {
    #[doc = "Directory path where all the Whistle files are located.\nExample: gs://{bucket-id}/{path/to/import-root/dir}"]
    pub import_uri_prefix: PrimField<String>,
    #[doc = "Main configuration file which has the entrypoint or the root function.\nExample: gs://{bucket-id}/{path/to/import-root/dir}/entrypoint-file-name.wstl."]
    pub uri: PrimField<String>,
}
impl BuildHealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl {
    pub fn build(
        self,
    ) -> HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl {
        HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl {
            import_uri_prefix: self.import_uri_prefix,
            uri: self.uri,
        }
    }
}
pub struct HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceElRef {
        HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `import_uri_prefix` after provisioning.\nDirectory path where all the Whistle files are located.\nExample: gs://{bucket-id}/{path/to/import-root/dir}"]
    pub fn import_uri_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_uri_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nMain configuration file which has the entrypoint or the root function.\nExample: gs://{bucket-id}/{path/to/import-root/dir}/entrypoint-file-name.wstl."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct HealthcarePipelineJobMappingPipelineJobElMappingConfigElDynamic {
    whistle_config_source: Option<
        DynamicBlock<HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl>,
    >,
}
#[derive(Serialize)]
pub struct HealthcarePipelineJobMappingPipelineJobElMappingConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    whistle_config_source:
        Option<Vec<HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl>>,
    dynamic: HealthcarePipelineJobMappingPipelineJobElMappingConfigElDynamic,
}
impl HealthcarePipelineJobMappingPipelineJobElMappingConfigEl {
    #[doc = "Set the field `description`.\nDescribes the mapping configuration."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `whistle_config_source`.\n"]
    pub fn set_whistle_config_source(
        mut self,
        v: impl Into<
            BlockAssignable<
                HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.whistle_config_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.whistle_config_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HealthcarePipelineJobMappingPipelineJobElMappingConfigEl {
    type O = BlockAssignable<HealthcarePipelineJobMappingPipelineJobElMappingConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcarePipelineJobMappingPipelineJobElMappingConfigEl {}
impl BuildHealthcarePipelineJobMappingPipelineJobElMappingConfigEl {
    pub fn build(self) -> HealthcarePipelineJobMappingPipelineJobElMappingConfigEl {
        HealthcarePipelineJobMappingPipelineJobElMappingConfigEl {
            description: core::default::Default::default(),
            whistle_config_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HealthcarePipelineJobMappingPipelineJobElMappingConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobMappingPipelineJobElMappingConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HealthcarePipelineJobMappingPipelineJobElMappingConfigElRef {
        HealthcarePipelineJobMappingPipelineJobElMappingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcarePipelineJobMappingPipelineJobElMappingConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescribes the mapping configuration."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `whistle_config_source` after provisioning.\n"]
    pub fn whistle_config_source(
        &self,
    ) -> ListRef<HealthcarePipelineJobMappingPipelineJobElMappingConfigElWhistleConfigSourceElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.whistle_config_source", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct HealthcarePipelineJobMappingPipelineJobElDynamic {
    fhir_streaming_source:
        Option<DynamicBlock<HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl>>,
    mapping_config: Option<DynamicBlock<HealthcarePipelineJobMappingPipelineJobElMappingConfigEl>>,
}
#[derive(Serialize)]
pub struct HealthcarePipelineJobMappingPipelineJobEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fhir_store_destination: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reconciliation_destination: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fhir_streaming_source:
        Option<Vec<HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mapping_config: Option<Vec<HealthcarePipelineJobMappingPipelineJobElMappingConfigEl>>,
    dynamic: HealthcarePipelineJobMappingPipelineJobElDynamic,
}
impl HealthcarePipelineJobMappingPipelineJobEl {
    #[doc = "Set the field `fhir_store_destination`.\nIf set, the mapping pipeline will write snapshots to this\nFHIR store without assigning stable IDs. You must\ngrant your pipeline project's Cloud Healthcare Service\nAgent serviceaccount healthcare.fhirResources.executeBundle\nand healthcare.fhirResources.create permissions on the\ndestination store. The destination store must set\n[disableReferentialIntegrity][FhirStore.disable_referential_integrity]\nto true. The destination store must use FHIR version R4.\nFormat: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{fhirStoreID}."]
    pub fn set_fhir_store_destination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fhir_store_destination = Some(v.into());
        self
    }
    #[doc = "Set the field `reconciliation_destination`.\nIf set to true, a mapping pipeline will send output snapshots\nto the reconciliation pipeline in its dataset. A reconciliation\npipeline must exist in this dataset before a mapping pipeline\nwith a reconciliation destination can be created."]
    pub fn set_reconciliation_destination(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.reconciliation_destination = Some(v.into());
        self
    }
    #[doc = "Set the field `fhir_streaming_source`.\n"]
    pub fn set_fhir_streaming_source(
        mut self,
        v: impl Into<BlockAssignable<HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.fhir_streaming_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.fhir_streaming_source = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `mapping_config`.\n"]
    pub fn set_mapping_config(
        mut self,
        v: impl Into<BlockAssignable<HealthcarePipelineJobMappingPipelineJobElMappingConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.mapping_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HealthcarePipelineJobMappingPipelineJobEl {
    type O = BlockAssignable<HealthcarePipelineJobMappingPipelineJobEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcarePipelineJobMappingPipelineJobEl {}
impl BuildHealthcarePipelineJobMappingPipelineJobEl {
    pub fn build(self) -> HealthcarePipelineJobMappingPipelineJobEl {
        HealthcarePipelineJobMappingPipelineJobEl {
            fhir_store_destination: core::default::Default::default(),
            reconciliation_destination: core::default::Default::default(),
            fhir_streaming_source: core::default::Default::default(),
            mapping_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HealthcarePipelineJobMappingPipelineJobElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobMappingPipelineJobElRef {
    fn new(shared: StackShared, base: String) -> HealthcarePipelineJobMappingPipelineJobElRef {
        HealthcarePipelineJobMappingPipelineJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcarePipelineJobMappingPipelineJobElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fhir_store_destination` after provisioning.\nIf set, the mapping pipeline will write snapshots to this\nFHIR store without assigning stable IDs. You must\ngrant your pipeline project's Cloud Healthcare Service\nAgent serviceaccount healthcare.fhirResources.executeBundle\nand healthcare.fhirResources.create permissions on the\ndestination store. The destination store must set\n[disableReferentialIntegrity][FhirStore.disable_referential_integrity]\nto true. The destination store must use FHIR version R4.\nFormat: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{fhirStoreID}."]
    pub fn fhir_store_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fhir_store_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `reconciliation_destination` after provisioning.\nIf set to true, a mapping pipeline will send output snapshots\nto the reconciliation pipeline in its dataset. A reconciliation\npipeline must exist in this dataset before a mapping pipeline\nwith a reconciliation destination can be created."]
    pub fn reconciliation_destination(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.reconciliation_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `fhir_streaming_source` after provisioning.\n"]
    pub fn fhir_streaming_source(
        &self,
    ) -> ListRef<HealthcarePipelineJobMappingPipelineJobElFhirStreamingSourceElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.fhir_streaming_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `mapping_config` after provisioning.\n"]
    pub fn mapping_config(
        &self,
    ) -> ListRef<HealthcarePipelineJobMappingPipelineJobElMappingConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.mapping_config", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl {
    import_uri_prefix: PrimField<String>,
    uri: PrimField<String>,
}
impl HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl {}
impl ToListMappable
    for HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl
{
    type O = BlockAssignable<
        HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl {
    #[doc = "Directory path where all the Whistle files are located.\nExample: gs://{bucket-id}/{path/to/import-root/dir}"]
    pub import_uri_prefix: PrimField<String>,
    #[doc = "Main configuration file which has the entrypoint or the root function.\nExample: gs://{bucket-id}/{path/to/import-root/dir}/entrypoint-file-name.wstl."]
    pub uri: PrimField<String>,
}
impl BuildHealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl {
    pub fn build(
        self,
    ) -> HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl {
        HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl {
            import_uri_prefix: self.import_uri_prefix,
            uri: self.uri,
        }
    }
}
pub struct HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceElRef {
        HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `import_uri_prefix` after provisioning.\nDirectory path where all the Whistle files are located.\nExample: gs://{bucket-id}/{path/to/import-root/dir}"]
    pub fn import_uri_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.import_uri_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uri` after provisioning.\nMain configuration file which has the entrypoint or the root function.\nExample: gs://{bucket-id}/{path/to/import-root/dir}/entrypoint-file-name.wstl."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}
#[derive(Serialize, Default)]
struct HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElDynamic {
    whistle_config_source: Option<
        DynamicBlock<
            HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct HealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    whistle_config_source: Option<
        Vec<HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl>,
    >,
    dynamic: HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElDynamic,
}
impl HealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl {
    #[doc = "Set the field `description`.\nDescribes the mapping configuration."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `whistle_config_source`.\n"]
    pub fn set_whistle_config_source(
        mut self,
        v: impl Into<
            BlockAssignable<
                HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.whistle_config_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.whistle_config_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl {
    type O = BlockAssignable<HealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl {}
impl BuildHealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl {
    pub fn build(self) -> HealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl {
        HealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl {
            description: core::default::Default::default(),
            whistle_config_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElRef {
        HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescribes the mapping configuration."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `whistle_config_source` after provisioning.\n"]
    pub fn whistle_config_source(
        &self,
    ) -> ListRef<
        HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElWhistleConfigSourceElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.whistle_config_source", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct HealthcarePipelineJobReconciliationPipelineJobElDynamic {
    merge_config:
        Option<DynamicBlock<HealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl>>,
}
#[derive(Serialize)]
pub struct HealthcarePipelineJobReconciliationPipelineJobEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fhir_store_destination: Option<PrimField<String>>,
    matching_uri_prefix: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merge_config: Option<Vec<HealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl>>,
    dynamic: HealthcarePipelineJobReconciliationPipelineJobElDynamic,
}
impl HealthcarePipelineJobReconciliationPipelineJobEl {
    #[doc = "Set the field `fhir_store_destination`.\nThe harmonized FHIR store to write harmonized FHIR resources to,\nin the format of: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{id}"]
    pub fn set_fhir_store_destination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fhir_store_destination = Some(v.into());
        self
    }
    #[doc = "Set the field `merge_config`.\n"]
    pub fn set_merge_config(
        mut self,
        v: impl Into<BlockAssignable<HealthcarePipelineJobReconciliationPipelineJobElMergeConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.merge_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.merge_config = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for HealthcarePipelineJobReconciliationPipelineJobEl {
    type O = BlockAssignable<HealthcarePipelineJobReconciliationPipelineJobEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcarePipelineJobReconciliationPipelineJobEl {
    #[doc = "Specifies the top level directory of the matching configs used\nin all mapping pipelines, which extract properties for resources\nto be matched on.\nExample: gs://{bucket-id}/{path/to/matching/configs}"]
    pub matching_uri_prefix: PrimField<String>,
}
impl BuildHealthcarePipelineJobReconciliationPipelineJobEl {
    pub fn build(self) -> HealthcarePipelineJobReconciliationPipelineJobEl {
        HealthcarePipelineJobReconciliationPipelineJobEl {
            fhir_store_destination: core::default::Default::default(),
            matching_uri_prefix: self.matching_uri_prefix,
            merge_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct HealthcarePipelineJobReconciliationPipelineJobElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobReconciliationPipelineJobElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> HealthcarePipelineJobReconciliationPipelineJobElRef {
        HealthcarePipelineJobReconciliationPipelineJobElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcarePipelineJobReconciliationPipelineJobElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `fhir_store_destination` after provisioning.\nThe harmonized FHIR store to write harmonized FHIR resources to,\nin the format of: project/{projectID}/locations/{locationID}/datasets/{datasetName}/fhirStores/{id}"]
    pub fn fhir_store_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.fhir_store_destination", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `matching_uri_prefix` after provisioning.\nSpecifies the top level directory of the matching configs used\nin all mapping pipelines, which extract properties for resources\nto be matched on.\nExample: gs://{bucket-id}/{path/to/matching/configs}"]
    pub fn matching_uri_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.matching_uri_prefix", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `merge_config` after provisioning.\n"]
    pub fn merge_config(
        &self,
    ) -> ListRef<HealthcarePipelineJobReconciliationPipelineJobElMergeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.merge_config", self.base))
    }
}
#[derive(Serialize)]
pub struct HealthcarePipelineJobTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl HealthcarePipelineJobTimeoutsEl {
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
impl ToListMappable for HealthcarePipelineJobTimeoutsEl {
    type O = BlockAssignable<HealthcarePipelineJobTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildHealthcarePipelineJobTimeoutsEl {}
impl BuildHealthcarePipelineJobTimeoutsEl {
    pub fn build(self) -> HealthcarePipelineJobTimeoutsEl {
        HealthcarePipelineJobTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct HealthcarePipelineJobTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for HealthcarePipelineJobTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> HealthcarePipelineJobTimeoutsElRef {
        HealthcarePipelineJobTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl HealthcarePipelineJobTimeoutsElRef {
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
struct HealthcarePipelineJobDynamic {
    backfill_pipeline_job: Option<DynamicBlock<HealthcarePipelineJobBackfillPipelineJobEl>>,
    mapping_pipeline_job: Option<DynamicBlock<HealthcarePipelineJobMappingPipelineJobEl>>,
    reconciliation_pipeline_job:
        Option<DynamicBlock<HealthcarePipelineJobReconciliationPipelineJobEl>>,
}
