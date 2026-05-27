use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeResizeRequestData {
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
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_group_manager: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    resize_by: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requested_run_duration: Option<Vec<ComputeResizeRequestRequestedRunDurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeResizeRequestTimeoutsEl>,
    dynamic: ComputeResizeRequestDynamic,
}
struct ComputeResizeRequest_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeResizeRequestData>,
}
#[derive(Clone)]
pub struct ComputeResizeRequest(Rc<ComputeResizeRequest_>);
impl ComputeResizeRequest {
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
    #[doc = "Set the field `description`.\nAn optional description of this resize-request."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
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
    #[doc = "Set the field `zone`.\nThe reference of the compute zone scoping this request. If it is not provided, the provider zone is used."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }
    #[doc = "Set the field `requested_run_duration`.\n"]
    pub fn set_requested_run_duration(
        self,
        v: impl Into<BlockAssignable<ComputeResizeRequestRequestedRunDurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().requested_run_duration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.requested_run_duration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeResizeRequestTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nThe creation timestamp for this resize request in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resize-request."]
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
    #[doc = "Get a reference to the value of field `instance_group_manager` after provisioning.\nThe reference of the instance group manager this ResizeRequest is a part of."]
    pub fn instance_group_manager(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_group_manager", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resize request. The name must be 1-63 characters long, and comply with RFC1035."]
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
    #[doc = "Get a reference to the value of field `resize_by` after provisioning.\nThe number of instances to be created by this resize request. The group's target size will be increased by this number."]
    pub fn resize_by(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resize_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the request."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the request."]
    pub fn status(&self) -> ListRef<ComputeResizeRequestStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe reference of the compute zone scoping this request. If it is not provided, the provider zone is used."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requested_run_duration` after provisioning.\n"]
    pub fn requested_run_duration(&self) -> ListRef<ComputeResizeRequestRequestedRunDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requested_run_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResizeRequestTimeoutsElRef {
        ComputeResizeRequestTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeResizeRequest {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeResizeRequest {}
impl ToListMappable for ComputeResizeRequest {
    type O = ListRef<ComputeResizeRequestRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeResizeRequest_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_resize_request".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeResizeRequest {
    pub tf_id: String,
    #[doc = "The reference of the instance group manager this ResizeRequest is a part of."]
    pub instance_group_manager: PrimField<String>,
    #[doc = "The name of this resize request. The name must be 1-63 characters long, and comply with RFC1035."]
    pub name: PrimField<String>,
    #[doc = "The number of instances to be created by this resize request. The group's target size will be increased by this number."]
    pub resize_by: PrimField<f64>,
}
impl BuildComputeResizeRequest {
    pub fn build(self, stack: &mut Stack) -> ComputeResizeRequest {
        let out = ComputeResizeRequest(Rc::new(ComputeResizeRequest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeResizeRequestData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_group_manager: self.instance_group_manager,
                name: self.name,
                project: core::default::Default::default(),
                resize_by: self.resize_by,
                zone: core::default::Default::default(),
                requested_run_duration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeResizeRequestRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeResizeRequestRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nThe creation timestamp for this resize request in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resize-request."]
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
    #[doc = "Get a reference to the value of field `instance_group_manager` after provisioning.\nThe reference of the instance group manager this ResizeRequest is a part of."]
    pub fn instance_group_manager(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_group_manager", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of this resize request. The name must be 1-63 characters long, and comply with RFC1035."]
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
    #[doc = "Get a reference to the value of field `resize_by` after provisioning.\nThe number of instances to be created by this resize request. The group's target size will be increased by this number."]
    pub fn resize_by(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resize_by", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the request."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the request."]
    pub fn status(&self) -> ListRef<ComputeResizeRequestStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `zone` after provisioning.\nThe reference of the compute zone scoping this request. If it is not provided, the provider zone is used."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.zone", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `requested_run_duration` after provisioning.\n"]
    pub fn requested_run_duration(&self) -> ListRef<ComputeResizeRequestRequestedRunDurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.requested_run_duration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeResizeRequestTimeoutsElRef {
        ComputeResizeRequestTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadatas: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `metadatas`.\n"]
    pub fn set_metadatas(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadatas = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl {
    type O = BlockAssignable<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl {}
impl BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl {
    pub fn build(self) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl {
            domain: core::default::Default::default(),
            metadatas: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoElRef {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `metadatas` after provisioning.\n"]
    pub fn metadatas(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadatas", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl {
    type O =
        BlockAssignable<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl {}
impl BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl {
    pub fn build(self) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl {
            description: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksElRef {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    links:
        Option<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl>>,
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl {
    #[doc = "Set the field `links`.\n"]
    pub fn set_links(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksEl>>,
    ) -> Self {
        self.links = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl {
    type O = BlockAssignable<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl {}
impl BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl {
    pub fn build(self) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl {
            links: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElRef {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `links` after provisioning.\n"]
    pub fn links(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElLinksElRef> {
        ListRef::new(self.shared().clone(), format!("{}.links", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl {
    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locale = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl
{
    type O = BlockAssignable<
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl {}
impl BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl {
    pub fn build(
        self,
    ) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl {
            locale: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageElRef {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    future_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout_status: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }
    #[doc = "Set the field `future_limit`.\n"]
    pub fn set_future_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.future_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `limit`.\n"]
    pub fn set_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.limit = Some(v.into());
        self
    }
    #[doc = "Set the field `limit_name`.\n"]
    pub fn set_limit_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.limit_name = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_name = Some(v.into());
        self
    }
    #[doc = "Set the field `rollout_status`.\n"]
    pub fn set_rollout_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rollout_status = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl {
    type O = BlockAssignable<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl {}
impl BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl {
    pub fn build(self) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl {
            dimensions: core::default::Default::default(),
            future_limit: core::default::Default::default(),
            limit: core::default::Default::default(),
            limit_name: core::default::Default::default(),
            metric_name: core::default::Default::default(),
            rollout_status: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
    #[doc = "Get a reference to the value of field `future_limit` after provisioning.\n"]
    pub fn future_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.future_limit", self.base))
    }
    #[doc = "Get a reference to the value of field `limit` after provisioning.\n"]
    pub fn limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }
    #[doc = "Get a reference to the value of field `limit_name` after provisioning.\n"]
    pub fn limit_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit_name", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }
    #[doc = "Get a reference to the value of field `rollout_status` after provisioning.\n"]
    pub fn rollout_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollout_status", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error_info:
        Option<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    help: Option<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    localized_message: Option<
        ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota_info:
        Option<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl>>,
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl {
    #[doc = "Set the field `error_info`.\n"]
    pub fn set_error_info(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoEl>>,
    ) -> Self {
        self.error_info = Some(v.into());
        self
    }
    #[doc = "Set the field `help`.\n"]
    pub fn set_help(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpEl>>,
    ) -> Self {
        self.help = Some(v.into());
        self
    }
    #[doc = "Set the field `localized_message`.\n"]
    pub fn set_localized_message(
        mut self,
        v: impl Into<
            ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageEl>,
        >,
    ) -> Self {
        self.localized_message = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_info`.\n"]
    pub fn set_quota_info(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoEl>>,
    ) -> Self {
        self.quota_info = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl {
    type O = BlockAssignable<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl {}
impl BuildComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl {
    pub fn build(self) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl {
            error_info: core::default::Default::default(),
            help: core::default::Default::default(),
            localized_message: core::default::Default::default(),
            quota_info: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElRef {
        ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error_info` after provisioning.\n"]
    pub fn error_info(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElErrorInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error_info", self.base))
    }
    #[doc = "Get a reference to the value of field `help` after provisioning.\n"]
    pub fn help(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElHelpElRef> {
        ListRef::new(self.shared().clone(), format!("{}.help", self.base))
    }
    #[doc = "Get a reference to the value of field `localized_message` after provisioning.\n"]
    pub fn localized_message(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElLocalizedMessageElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.localized_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `quota_info` after provisioning.\n"]
    pub fn quota_info(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElQuotaInfoElRef> {
        ListRef::new(self.shared().clone(), format!("{}.quota_info", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElErrorElErrorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_details: Option<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElErrorElErrorsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `error_details`.\n"]
    pub fn set_error_details(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsEl>>,
    ) -> Self {
        self.error_details = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElErrorElErrorsEl {
    type O = BlockAssignable<ComputeResizeRequestStatusElErrorElErrorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElErrorElErrorsEl {}
impl BuildComputeResizeRequestStatusElErrorElErrorsEl {
    pub fn build(self) -> ComputeResizeRequestStatusElErrorElErrorsEl {
        ComputeResizeRequestStatusElErrorElErrorsEl {
            code: core::default::Default::default(),
            error_details: core::default::Default::default(),
            location: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElErrorElErrorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElErrorElErrorsElRef {
    fn new(shared: StackShared, base: String) -> ComputeResizeRequestStatusElErrorElErrorsElRef {
        ComputeResizeRequestStatusElErrorElErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElErrorElErrorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElErrorElErrorsElErrorDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<ListField<ComputeResizeRequestStatusElErrorElErrorsEl>>,
}
impl ComputeResizeRequestStatusElErrorEl {
    #[doc = "Set the field `errors`.\n"]
    pub fn set_errors(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElErrorElErrorsEl>>,
    ) -> Self {
        self.errors = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElErrorEl {
    type O = BlockAssignable<ComputeResizeRequestStatusElErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElErrorEl {}
impl BuildComputeResizeRequestStatusElErrorEl {
    pub fn build(self) -> ComputeResizeRequestStatusElErrorEl {
        ComputeResizeRequestStatusElErrorEl {
            errors: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElErrorElRef {
    fn new(shared: StackShared, base: String) -> ComputeResizeRequestStatusElErrorElRef {
        ComputeResizeRequestStatusElErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\n"]
    pub fn errors(&self) -> ListRef<ComputeResizeRequestStatusElErrorElErrorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.errors", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadatas: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.domain = Some(v.into());
        self
    }
    #[doc = "Set the field `metadatas`.\n"]
    pub fn set_metadatas(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadatas = Some(v.into());
        self
    }
    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.reason = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl
{
    type O = BlockAssignable<
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl {
}
impl BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl {
    pub fn build(
        self,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl {
            domain: core::default::Default::default(),
            metadatas: core::default::Default::default(),
            reason: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoElRef {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
    #[doc = "Get a reference to the value of field `metadatas` after provisioning.\n"]
    pub fn metadatas(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadatas", self.base))
    }
    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.url = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl
{
    type O = BlockAssignable<
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl
{}
impl BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl {
    pub fn build(
        self,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl {
            description: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksElRef
    {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<
        ListField<
            ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl,
        >,
    >,
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl {
    #[doc = "Set the field `links`.\n"]
    pub fn set_links(
        mut self,
        v: impl Into<
            ListField<
                ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksEl,
            >,
        >,
    ) -> Self {
        self.links = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl
{
    type O = BlockAssignable<
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl {}
impl BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl {
    pub fn build(
        self,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl {
            links: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElRef {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `links` after provisioning.\n"]
    pub fn links(
        &self,
    ) -> ListRef<
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElLinksElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.links", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl {
    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locale = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl
{
    type O = BlockAssignable<
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl
{}
impl BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl {
    pub fn build(
        self,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl
    {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl {
            locale: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageElRef
    {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageElRef { shared : shared , base : base . to_string () , }
    }
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dimensions: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    future_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rollout_status: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.dimensions = Some(v.into());
        self
    }
    #[doc = "Set the field `future_limit`.\n"]
    pub fn set_future_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.future_limit = Some(v.into());
        self
    }
    #[doc = "Set the field `limit`.\n"]
    pub fn set_limit(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.limit = Some(v.into());
        self
    }
    #[doc = "Set the field `limit_name`.\n"]
    pub fn set_limit_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.limit_name = Some(v.into());
        self
    }
    #[doc = "Set the field `metric_name`.\n"]
    pub fn set_metric_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metric_name = Some(v.into());
        self
    }
    #[doc = "Set the field `rollout_status`.\n"]
    pub fn set_rollout_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rollout_status = Some(v.into());
        self
    }
}
impl ToListMappable
    for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl
{
    type O = BlockAssignable<
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl {
}
impl BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl {
    pub fn build(
        self,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl {
            dimensions: core::default::Default::default(),
            future_limit: core::default::Default::default(),
            limit: core::default::Default::default(),
            limit_name: core::default::Default::default(),
            metric_name: core::default::Default::default(),
            rollout_status: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }
    #[doc = "Get a reference to the value of field `future_limit` after provisioning.\n"]
    pub fn future_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.future_limit", self.base))
    }
    #[doc = "Get a reference to the value of field `limit` after provisioning.\n"]
    pub fn limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }
    #[doc = "Get a reference to the value of field `limit_name` after provisioning.\n"]
    pub fn limit_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.limit_name", self.base))
    }
    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }
    #[doc = "Get a reference to the value of field `rollout_status` after provisioning.\n"]
    pub fn rollout_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rollout_status", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl { # [serde (skip_serializing_if = "Option::is_none")] error_info : Option < ListField < ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl > > , # [serde (skip_serializing_if = "Option::is_none")] help : Option < ListField < ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl > > , # [serde (skip_serializing_if = "Option::is_none")] localized_message : Option < ListField < ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl > > , # [serde (skip_serializing_if = "Option::is_none")] quota_info : Option < ListField < ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl > > , }
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl {
    #[doc = "Set the field `error_info`.\n"]
    pub fn set_error_info(
        mut self,
        v: impl Into<
            ListField<
                ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoEl,
            >,
        >,
    ) -> Self {
        self.error_info = Some(v.into());
        self
    }
    #[doc = "Set the field `help`.\n"]
    pub fn set_help(
        mut self,
        v: impl Into<
            ListField<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpEl>,
        >,
    ) -> Self {
        self.help = Some(v.into());
        self
    }
    #[doc = "Set the field `localized_message`.\n"]
    pub fn set_localized_message(
        mut self,
        v : impl Into < ListField < ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageEl > >,
    ) -> Self {
        self.localized_message = Some(v.into());
        self
    }
    #[doc = "Set the field `quota_info`.\n"]
    pub fn set_quota_info(
        mut self,
        v: impl Into<
            ListField<
                ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoEl,
            >,
        >,
    ) -> Self {
        self.quota_info = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl {
    type O =
        BlockAssignable<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl {}
impl BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl {
    pub fn build(self) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl {
            error_info: core::default::Default::default(),
            help: core::default::Default::default(),
            localized_message: core::default::Default::default(),
            quota_info: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElRef {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error_info` after provisioning.\n"]
    pub fn error_info(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElErrorInfoElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.error_info", self.base))
    }
    #[doc = "Get a reference to the value of field `help` after provisioning.\n"]
    pub fn help(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElHelpElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.help", self.base))
    }
    #[doc = "Get a reference to the value of field `localized_message` after provisioning.\n"]
    pub fn localized_message(
        &self,
    ) -> ListRef<
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElLocalizedMessageElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.localized_message", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `quota_info` after provisioning.\n"]
    pub fn quota_info(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElQuotaInfoElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.quota_info", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_details:
        Option<ListField<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<PrimField<String>>,
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `error_details`.\n"]
    pub fn set_error_details(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsEl>>,
    ) -> Self {
        self.error_details = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.message = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl {
    type O = BlockAssignable<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl {}
impl BuildComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl {
    pub fn build(self) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl {
            code: core::default::Default::default(),
            error_details: core::default::Default::default(),
            location: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElRef {
        ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `error_details` after provisioning.\n"]
    pub fn error_details(
        &self,
    ) -> ListRef<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElErrorDetailsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.error_details", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElLastAttemptElErrorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<ListField<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl>>,
}
impl ComputeResizeRequestStatusElLastAttemptElErrorEl {
    #[doc = "Set the field `errors`.\n"]
    pub fn set_errors(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsEl>>,
    ) -> Self {
        self.errors = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElLastAttemptElErrorEl {
    type O = BlockAssignable<ComputeResizeRequestStatusElLastAttemptElErrorEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElLastAttemptElErrorEl {}
impl BuildComputeResizeRequestStatusElLastAttemptElErrorEl {
    pub fn build(self) -> ComputeResizeRequestStatusElLastAttemptElErrorEl {
        ComputeResizeRequestStatusElLastAttemptElErrorEl {
            errors: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElLastAttemptElErrorElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElLastAttemptElErrorElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeResizeRequestStatusElLastAttemptElErrorElRef {
        ComputeResizeRequestStatusElLastAttemptElErrorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElLastAttemptElErrorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `errors` after provisioning.\n"]
    pub fn errors(&self) -> ListRef<ComputeResizeRequestStatusElLastAttemptElErrorElErrorsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.errors", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusElLastAttemptEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ListField<ComputeResizeRequestStatusElLastAttemptElErrorEl>>,
}
impl ComputeResizeRequestStatusElLastAttemptEl {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElLastAttemptElErrorEl>>,
    ) -> Self {
        self.error = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusElLastAttemptEl {
    type O = BlockAssignable<ComputeResizeRequestStatusElLastAttemptEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusElLastAttemptEl {}
impl BuildComputeResizeRequestStatusElLastAttemptEl {
    pub fn build(self) -> ComputeResizeRequestStatusElLastAttemptEl {
        ComputeResizeRequestStatusElLastAttemptEl {
            error: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElLastAttemptElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElLastAttemptElRef {
    fn new(shared: StackShared, base: String) -> ComputeResizeRequestStatusElLastAttemptElRef {
        ComputeResizeRequestStatusElLastAttemptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElLastAttemptElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> ListRef<ComputeResizeRequestStatusElLastAttemptElErrorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ListField<ComputeResizeRequestStatusElErrorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_attempt: Option<ListField<ComputeResizeRequestStatusElLastAttemptEl>>,
}
impl ComputeResizeRequestStatusEl {
    #[doc = "Set the field `error`.\n"]
    pub fn set_error(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElErrorEl>>,
    ) -> Self {
        self.error = Some(v.into());
        self
    }
    #[doc = "Set the field `last_attempt`.\n"]
    pub fn set_last_attempt(
        mut self,
        v: impl Into<ListField<ComputeResizeRequestStatusElLastAttemptEl>>,
    ) -> Self {
        self.last_attempt = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestStatusEl {
    type O = BlockAssignable<ComputeResizeRequestStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestStatusEl {}
impl BuildComputeResizeRequestStatusEl {
    pub fn build(self) -> ComputeResizeRequestStatusEl {
        ComputeResizeRequestStatusEl {
            error: core::default::Default::default(),
            last_attempt: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestStatusElRef {
    fn new(shared: StackShared, base: String) -> ComputeResizeRequestStatusElRef {
        ComputeResizeRequestStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `error` after provisioning.\n"]
    pub fn error(&self) -> ListRef<ComputeResizeRequestStatusElErrorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.error", self.base))
    }
    #[doc = "Get a reference to the value of field `last_attempt` after provisioning.\n"]
    pub fn last_attempt(&self) -> ListRef<ComputeResizeRequestStatusElLastAttemptElRef> {
        ListRef::new(self.shared().clone(), format!("{}.last_attempt", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestRequestedRunDurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<String>,
}
impl ComputeResizeRequestRequestedRunDurationEl {
    #[doc = "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeResizeRequestRequestedRunDurationEl {
    type O = BlockAssignable<ComputeResizeRequestRequestedRunDurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestRequestedRunDurationEl {
    #[doc = "Span of time at a resolution of a second. Must be from 600 to 604800 inclusive. Note: minimum and maximum allowed range for requestedRunDuration is 10 minutes (600 seconds) and 7 days(604800 seconds) correspondingly."]
    pub seconds: PrimField<String>,
}
impl BuildComputeResizeRequestRequestedRunDurationEl {
    pub fn build(self) -> ComputeResizeRequestRequestedRunDurationEl {
        ComputeResizeRequestRequestedRunDurationEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}
pub struct ComputeResizeRequestRequestedRunDurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestRequestedRunDurationElRef {
    fn new(shared: StackShared, base: String) -> ComputeResizeRequestRequestedRunDurationElRef {
        ComputeResizeRequestRequestedRunDurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestRequestedRunDurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }
    #[doc = "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 600 to 604800 inclusive. Note: minimum and maximum allowed range for requestedRunDuration is 10 minutes (600 seconds) and 7 days(604800 seconds) correspondingly."]
    pub fn seconds(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeResizeRequestTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}
impl ComputeResizeRequestTimeoutsEl {
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
impl ToListMappable for ComputeResizeRequestTimeoutsEl {
    type O = BlockAssignable<ComputeResizeRequestTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeResizeRequestTimeoutsEl {}
impl BuildComputeResizeRequestTimeoutsEl {
    pub fn build(self) -> ComputeResizeRequestTimeoutsEl {
        ComputeResizeRequestTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}
pub struct ComputeResizeRequestTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeResizeRequestTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeResizeRequestTimeoutsElRef {
        ComputeResizeRequestTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeResizeRequestTimeoutsElRef {
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
struct ComputeResizeRequestDynamic {
    requested_run_duration: Option<DynamicBlock<ComputeResizeRequestRequestedRunDurationEl>>,
}
