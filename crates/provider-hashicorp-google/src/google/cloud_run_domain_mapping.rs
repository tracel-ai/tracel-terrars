use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudRunDomainMappingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Vec<CloudRunDomainMappingMetadataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<CloudRunDomainMappingSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<CloudRunDomainMappingTimeoutsEl>,
    dynamic: CloudRunDomainMappingDynamic,
}
struct CloudRunDomainMapping_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudRunDomainMappingData>,
}
#[derive(Clone)]
pub struct CloudRunDomainMapping(Rc<CloudRunDomainMapping_>);
impl CloudRunDomainMapping {
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
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        self,
        v: impl Into<BlockAssignable<CloudRunDomainMappingMetadataEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<CloudRunDomainMappingSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<CloudRunDomainMappingTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run instance. eg us-central1"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName should be a [verified](https://support.google.com/webmasters/answer/9008080) domain"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe current status of the DomainMapping."]
    pub fn status(&self) -> ListRef<CloudRunDomainMappingStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<CloudRunDomainMappingMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<CloudRunDomainMappingSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunDomainMappingTimeoutsElRef {
        CloudRunDomainMappingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for CloudRunDomainMapping {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudRunDomainMapping {}
impl ToListMappable for CloudRunDomainMapping {
    type O = ListRef<CloudRunDomainMappingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudRunDomainMapping_ {
    fn extract_resource_type(&self) -> String {
        "google_cloud_run_domain_mapping".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudRunDomainMapping {
    pub tf_id: String,
    #[doc = "The location of the cloud run instance. eg us-central1"]
    pub location: PrimField<String>,
    #[doc = "Name should be a [verified](https://support.google.com/webmasters/answer/9008080) domain"]
    pub name: PrimField<String>,
}
impl BuildCloudRunDomainMapping {
    pub fn build(self, stack: &mut Stack) -> CloudRunDomainMapping {
        let out = CloudRunDomainMapping(Rc::new(CloudRunDomainMapping_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudRunDomainMappingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                metadata: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudRunDomainMappingRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunDomainMappingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudRunDomainMappingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run instance. eg us-central1"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName should be a [verified](https://support.google.com/webmasters/answer/9008080) domain"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe current status of the DomainMapping."]
    pub fn status(&self) -> ListRef<CloudRunDomainMappingStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> ListRef<CloudRunDomainMappingMetadataElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.metadata", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<CloudRunDomainMappingSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunDomainMappingTimeoutsElRef {
        CloudRunDomainMappingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunDomainMappingStatusElConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CloudRunDomainMappingStatusElConditionsEl {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunDomainMappingStatusElConditionsEl {
    type O = BlockAssignable<CloudRunDomainMappingStatusElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunDomainMappingStatusElConditionsEl {}
impl BuildCloudRunDomainMappingStatusElConditionsEl {
    pub fn build(self) -> CloudRunDomainMappingStatusElConditionsEl {
        CloudRunDomainMappingStatusElConditionsEl {
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CloudRunDomainMappingStatusElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunDomainMappingStatusElConditionsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunDomainMappingStatusElConditionsElRef {
        CloudRunDomainMappingStatusElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunDomainMappingStatusElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunDomainMappingStatusElResourceRecordsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rrdata: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl CloudRunDomainMappingStatusElResourceRecordsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `rrdata`.\n"]
    pub fn set_rrdata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rrdata = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunDomainMappingStatusElResourceRecordsEl {
    type O = BlockAssignable<CloudRunDomainMappingStatusElResourceRecordsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunDomainMappingStatusElResourceRecordsEl {}
impl BuildCloudRunDomainMappingStatusElResourceRecordsEl {
    pub fn build(self) -> CloudRunDomainMappingStatusElResourceRecordsEl {
        CloudRunDomainMappingStatusElResourceRecordsEl {
            name: core::default::Default::default(),
            rrdata: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct CloudRunDomainMappingStatusElResourceRecordsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunDomainMappingStatusElResourceRecordsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunDomainMappingStatusElResourceRecordsElRef {
        CloudRunDomainMappingStatusElResourceRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunDomainMappingStatusElResourceRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `rrdata` after provisioning.\n"]
    pub fn rrdata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rrdata", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunDomainMappingStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<ListField<CloudRunDomainMappingStatusElConditionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mapped_route_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observed_generation: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_records: Option<ListField<CloudRunDomainMappingStatusElResourceRecordsEl>>,
}
impl CloudRunDomainMappingStatusEl {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<ListField<CloudRunDomainMappingStatusElConditionsEl>>,
    ) -> Self {
        self.conditions = Some(v.into());
        self
    }
    #[doc = "Set the field `mapped_route_name`.\n"]
    pub fn set_mapped_route_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mapped_route_name = Some(v.into());
        self
    }
    #[doc = "Set the field `observed_generation`.\n"]
    pub fn set_observed_generation(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.observed_generation = Some(v.into());
        self
    }
    #[doc = "Set the field `resource_records`.\n"]
    pub fn set_resource_records(
        mut self,
        v: impl Into<ListField<CloudRunDomainMappingStatusElResourceRecordsEl>>,
    ) -> Self {
        self.resource_records = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunDomainMappingStatusEl {
    type O = BlockAssignable<CloudRunDomainMappingStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunDomainMappingStatusEl {}
impl BuildCloudRunDomainMappingStatusEl {
    pub fn build(self) -> CloudRunDomainMappingStatusEl {
        CloudRunDomainMappingStatusEl {
            conditions: core::default::Default::default(),
            mapped_route_name: core::default::Default::default(),
            observed_generation: core::default::Default::default(),
            resource_records: core::default::Default::default(),
        }
    }
}
pub struct CloudRunDomainMappingStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunDomainMappingStatusElRef {
    fn new(shared: StackShared, base: String) -> CloudRunDomainMappingStatusElRef {
        CloudRunDomainMappingStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunDomainMappingStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<CloudRunDomainMappingStatusElConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
    #[doc = "Get a reference to the value of field `mapped_route_name` after provisioning.\n"]
    pub fn mapped_route_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.mapped_route_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\n"]
    pub fn observed_generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.observed_generation", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resource_records` after provisioning.\n"]
    pub fn resource_records(&self) -> ListRef<CloudRunDomainMappingStatusElResourceRecordsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_records", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct CloudRunDomainMappingMetadataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    namespace: PrimField<String>,
}
impl CloudRunDomainMappingMetadataEl {
    #[doc = "Set the field `annotations`.\nAnnotations is a key value map stored with a resource that\nmay be set by external tools to store and retrieve arbitrary metadata.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations\n\n**Note**: The Cloud Run API may add additional annotations that were not provided in your config.\nIf terraform plan shows a diff where a server-side annotation is added, you can add it to your config\nor apply the lifecycle.ignore_changes rule to the metadata.0.annotations field.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.annotations = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nMap of string keys and values that can be used to organize and categorize\n(scope and select) objects. May match selectors of replication controllers\nand routes.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunDomainMappingMetadataEl {
    type O = BlockAssignable<CloudRunDomainMappingMetadataEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunDomainMappingMetadataEl {
    #[doc = "In Cloud Run the namespace must be equal to either the\nproject ID or project number."]
    pub namespace: PrimField<String>,
}
impl BuildCloudRunDomainMappingMetadataEl {
    pub fn build(self) -> CloudRunDomainMappingMetadataEl {
        CloudRunDomainMappingMetadataEl {
            annotations: core::default::Default::default(),
            labels: core::default::Default::default(),
            namespace: self.namespace,
        }
    }
}
pub struct CloudRunDomainMappingMetadataElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunDomainMappingMetadataElRef {
    fn new(shared: StackShared, base: String) -> CloudRunDomainMappingMetadataElRef {
        CloudRunDomainMappingMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunDomainMappingMetadataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `annotations` after provisioning.\nAnnotations is a key value map stored with a resource that\nmay be set by external tools to store and retrieve arbitrary metadata.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations\n\n**Note**: The Cloud Run API may add additional annotations that were not provided in your config.\nIf terraform plan shows a diff where a server-side annotation is added, you can add it to your config\nor apply the lifecycle.ignore_changes rule to the metadata.0.annotations field.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }
    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_annotations", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `generation` after provisioning.\nA sequence number representing a specific generation of the desired state."]
    pub fn generation(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nMap of string keys and values that can be used to organize and categorize\n(scope and select) objects. May match selectors of replication controllers\nand routes.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }
    #[doc = "Get a reference to the value of field `namespace` after provisioning.\nIn Cloud Run the namespace must be equal to either the\nproject ID or project number."]
    pub fn namespace(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
    #[doc = "Get a reference to the value of field `resource_version` after provisioning.\nAn opaque value that represents the internal version of this object that\ncan be used by clients to determine when objects have changed. May be used\nfor optimistic concurrency, change detection, and the watch operation on a\nresource or set of resources. They may only be valid for a\nparticular resource or set of resources.\n\nMore info:\nhttps://git.k8s.io/community/contributors/devel/api-conventions.md#concurrency-control-and-consistency"]
    pub fn resource_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nSelfLink is a URL representing this object."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUID is a unique id generated by the server on successful creation of a resource and is not\nallowed to change on PUT operations.\n\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names#uids"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunDomainMappingSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_override: Option<PrimField<bool>>,
    route_name: PrimField<String>,
}
impl CloudRunDomainMappingSpecEl {
    #[doc = "Set the field `certificate_mode`.\nThe mode of the certificate. Default value: \"AUTOMATIC\" Possible values: [\"NONE\", \"AUTOMATIC\"]"]
    pub fn set_certificate_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `force_override`.\nIf set, the mapping will override any mapping set before this spec was set.\nIt is recommended that the user leaves this empty to receive an error\nwarning about a potential conflict and only set it once the respective UI\nhas given such a warning."]
    pub fn set_force_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.force_override = Some(v.into());
        self
    }
}
impl ToListMappable for CloudRunDomainMappingSpecEl {
    type O = BlockAssignable<CloudRunDomainMappingSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunDomainMappingSpecEl {
    #[doc = "The name of the Cloud Run Service that this DomainMapping applies to.\nThe route must exist."]
    pub route_name: PrimField<String>,
}
impl BuildCloudRunDomainMappingSpecEl {
    pub fn build(self) -> CloudRunDomainMappingSpecEl {
        CloudRunDomainMappingSpecEl {
            certificate_mode: core::default::Default::default(),
            force_override: core::default::Default::default(),
            route_name: self.route_name,
        }
    }
}
pub struct CloudRunDomainMappingSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunDomainMappingSpecElRef {
    fn new(shared: StackShared, base: String) -> CloudRunDomainMappingSpecElRef {
        CloudRunDomainMappingSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunDomainMappingSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_mode` after provisioning.\nThe mode of the certificate. Default value: \"AUTOMATIC\" Possible values: [\"NONE\", \"AUTOMATIC\"]"]
    pub fn certificate_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_mode", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `force_override` after provisioning.\nIf set, the mapping will override any mapping set before this spec was set.\nIt is recommended that the user leaves this empty to receive an error\nwarning about a potential conflict and only set it once the respective UI\nhas given such a warning."]
    pub fn force_override(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_override", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `route_name` after provisioning.\nThe name of the Cloud Run Service that this DomainMapping applies to.\nThe route must exist."]
    pub fn route_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.route_name", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudRunDomainMappingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl CloudRunDomainMappingTimeoutsEl {
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
}
impl ToListMappable for CloudRunDomainMappingTimeoutsEl {
    type O = BlockAssignable<CloudRunDomainMappingTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudRunDomainMappingTimeoutsEl {}
impl BuildCloudRunDomainMappingTimeoutsEl {
    pub fn build(self) -> CloudRunDomainMappingTimeoutsEl {
        CloudRunDomainMappingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct CloudRunDomainMappingTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudRunDomainMappingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> CloudRunDomainMappingTimeoutsElRef {
        CloudRunDomainMappingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudRunDomainMappingTimeoutsElRef {
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
}
#[derive(Serialize, Default)]
struct CloudRunDomainMappingDynamic {
    metadata: Option<DynamicBlock<CloudRunDomainMappingMetadataEl>>,
    spec: Option<DynamicBlock<CloudRunDomainMappingSpecEl>>,
}
