use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccessContextManagerServicePerimeterData {
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
    name: PrimField<String>,
    parent: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    perimeter_type: Option<PrimField<String>>,
    title: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_explicit_dry_run_spec: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<AccessContextManagerServicePerimeterSpecEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Vec<AccessContextManagerServicePerimeterStatusEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerServicePerimeterTimeoutsEl>,
    dynamic: AccessContextManagerServicePerimeterDynamic,
}
struct AccessContextManagerServicePerimeter_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerServicePerimeterData>,
}
#[derive(Clone)]
pub struct AccessContextManagerServicePerimeter(Rc<AccessContextManagerServicePerimeter_>);
impl AccessContextManagerServicePerimeter {
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
    #[doc = "Set the field `description`.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `perimeter_type`.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn set_perimeter_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().perimeter_type = Some(v.into());
        self
    }
    #[doc = "Set the field `use_explicit_dry_run_spec`.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn set_use_explicit_dry_run_spec(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_explicit_dry_run_spec = Some(v.into());
        self
    }
    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterSpecEl>>,
    ) -> Self {
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
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterStatusEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().status = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.status = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<AccessContextManagerServicePerimeterTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the AccessPolicy was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `perimeter_type` after provisioning.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn perimeter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.perimeter_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the AccessPolicy was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `use_explicit_dry_run_spec` after provisioning.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn use_explicit_dry_run_spec(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_explicit_dry_run_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AccessContextManagerServicePerimeterSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> ListRef<AccessContextManagerServicePerimeterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterTimeoutsElRef {
        AccessContextManagerServicePerimeterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AccessContextManagerServicePerimeter {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccessContextManagerServicePerimeter {}
impl ToListMappable for AccessContextManagerServicePerimeter {
    type O = ListRef<AccessContextManagerServicePerimeterRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccessContextManagerServicePerimeter_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_service_perimeter".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccessContextManagerServicePerimeter {
    pub tf_id: String,
    #[doc = "Resource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub name: PrimField<String>,
    #[doc = "The AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub parent: PrimField<String>,
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: PrimField<String>,
}
impl BuildAccessContextManagerServicePerimeter {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerServicePerimeter {
        let out =
            AccessContextManagerServicePerimeter(Rc::new(AccessContextManagerServicePerimeter_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(AccessContextManagerServicePerimeterData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    description: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: self.name,
                    parent: self.parent,
                    perimeter_type: core::default::Default::default(),
                    title: self.title,
                    use_explicit_dry_run_spec: core::default::Default::default(),
                    spec: core::default::Default::default(),
                    status: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccessContextManagerServicePerimeterRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccessContextManagerServicePerimeterRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nTime the AccessPolicy was created in UTC."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the ServicePerimeter and its use. Does not affect\nbehavior."]
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name for the ServicePerimeter. The short_name component must\nbegin with a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/servicePerimeters/{short_name}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this ServicePerimeter lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `perimeter_type` after provisioning.\nSpecifies the type of the Perimeter. There are two types: regular and\nbridge. Regular Service Perimeter contains resources, access levels,\nand restricted services. Every resource can be in at most\nONE regular Service Perimeter.\n\nIn addition to being in a regular service perimeter, a resource can also\nbe in zero or more perimeter bridges. A perimeter bridge only contains\nresources. Cross project operations are permitted if all effected\nresources share some perimeter (whether bridge or regular). Perimeter\nBridge does not contain access levels or services: those are governed\nentirely by the regular perimeter that resource is in.\n\nPerimeter Bridges are typically useful when building more complex\ntopologies with many independent perimeters that need to share some data\nwith a common perimeter, but should not be able to share data among\nthemselves. Default value: \"PERIMETER_TYPE_REGULAR\" Possible values: [\"PERIMETER_TYPE_REGULAR\", \"PERIMETER_TYPE_BRIDGE\"]"]
    pub fn perimeter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.perimeter_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nTime the AccessPolicy was updated in UTC."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `use_explicit_dry_run_spec` after provisioning.\nUse explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists\nfor all Service Perimeters, and that spec is identical to the status for those\nService Perimeters. When this flag is set, it inhibits the generation of the\nimplicit spec, thereby allowing the user to explicitly provide a\nconfiguration (\"spec\") to use in a dry-run version of the Service Perimeter.\nThis allows the user to test changes to the enforced config (\"status\") without\nactually enforcing them. This testing is done through analyzing the differences\nbetween currently enforced and suggested restrictions. useExplicitDryRunSpec must\nbet set to True if any of the fields in the spec are set to non-default values."]
    pub fn use_explicit_dry_run_spec(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.use_explicit_dry_run_spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AccessContextManagerServicePerimeterSpecElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.spec", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> ListRef<AccessContextManagerServicePerimeterStatusElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerServicePerimeterTimeoutsElRef {
        AccessContextManagerServicePerimeterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
    #[doc = "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }
    #[doc = "Set the field `resource`.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nCurrently only projects are allowed. Project format: 'projects/{project_number}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {}
impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nCurrently only projects are allowed. Project format: 'projects/{project_number}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElDynamic {
    sources: Option<
        DynamicBlock<
            AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_restriction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<
        Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElDynamic,
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
    #[doc = "Set the field `identities`.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn set_identities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_restriction = Some(v.into());
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {}
impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this 'EgressPolicy'.\nShould be in the format of email address. The email address should\nrepresent individual user or service account only."]
    pub fn identities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_restriction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElSourcesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { # [doc = "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."] pub fn set_method (mut self , v : impl Into < PrimField < String > >) -> Self { self . method = Some (v . into ()) ; self } # [doc = "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn set_permission (mut self , v : impl Into < PrimField < String > >) -> Self { self . permission = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { type O = BlockAssignable < AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl
{}
impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { pub fn build (self) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { method : core :: default :: Default :: default () , permission : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."] pub fn method (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.method" , self . base)) } # [doc = "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn permission (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permission" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElDynamic { method_selectors : Option < DynamicBlock < AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl { # [serde (skip_serializing_if = "Option::is_none")] service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] method_selectors : Option < Vec < AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl > > , dynamic : AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElDynamic , }
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
    #[doc = "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.method_selectors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
}
impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]    pub fn method_selectors (& self) -> ListRef < AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.method_selectors", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElDynamic {
    operations: Option<
        DynamicBlock<
            AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operations: Option<
        Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElDynamic,
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
    #[doc = "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.external_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn set_roles(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {}
impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.external_resources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElOperationsElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElDynamic {
    egress_from: Option<
        DynamicBlock<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl>,
    >,
    egress_to:
        Option<DynamicBlock<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl>>,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_from:
        Option<Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_to: Option<Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElEgressPoliciesElDynamic,
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
    #[doc = "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v: impl Into<
            BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v: impl Into<
            BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesEl {}
impl BuildAccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesEl {
            title: core::default::Default::default(),
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
        AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressFromElRef> {
        ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }
    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElEgressToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
    #[doc = "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }
    #[doc = "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
}
impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects are allowed. Format 'projects/{project_number}'\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElDynamic {
    sources: Option<
        DynamicBlock<
            AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<
        Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElDynamic,
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
    #[doc = "Set the field `identities`.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn set_identities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {}
impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\nA list of identities that are allowed access through this ingress policy.\nShould be in the format of email address. The email address should represent\nindividual user or service account only."]
    pub fn identities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElSourcesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { # [doc = "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."] pub fn set_method (mut self , v : impl Into < PrimField < String > >) -> Self { self . method = Some (v . into ()) ; self } # [doc = "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn set_permission (mut self , v : impl Into < PrimField < String > >) -> Self { self . permission = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { type O = BlockAssignable < AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl
{}
impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { pub fn build (self) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { method : core :: default :: Default :: default () , permission : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."] pub fn method (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.method" , self . base)) } # [doc = "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn permission (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permission" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElDynamic { method_selectors : Option < DynamicBlock < AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl { # [serde (skip_serializing_if = "Option::is_none")] service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] method_selectors : Option < Vec < AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl > > , dynamic : AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElDynamic , }
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
    #[doc = "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.method_selectors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl
{}
impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]    pub fn method_selectors (& self) -> ListRef < AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.method_selectors", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElDynamic {
    operations: Option<
        DynamicBlock<
            AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operations: Option<
        Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElDynamic,
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
    #[doc = "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn set_roles(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {}
impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl {
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> ListRef<
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElOperationsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElDynamic {
    ingress_from: Option<
        DynamicBlock<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl>,
    >,
    ingress_to: Option<
        DynamicBlock<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl>,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_from:
        Option<Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_to: Option<Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElIngressPoliciesElDynamic,
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
    #[doc = "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        mut self,
        v: impl Into<
            BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_to = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesEl {}
impl BuildAccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesEl {
            title: core::default::Default::default(),
            ingress_from: core::default::Default::default(),
            ingress_to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
        AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressFromElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElIngressToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_restriction: Option<PrimField<bool>>,
}
impl AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
    #[doc = "Set the field `allowed_services`.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn set_allowed_services(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_services = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_restriction`.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn set_enable_restriction(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_restriction = Some(v.into());
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {}
impl BuildAccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
        AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl {
            allowed_services: core::default::Default::default(),
            enable_restriction: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
        AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_services` after provisioning.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn allowed_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_services", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_restriction` after provisioning.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn enable_restriction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_restriction", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterSpecElDynamic {
    egress_policies:
        Option<DynamicBlock<AccessContextManagerServicePerimeterSpecElEgressPoliciesEl>>,
    ingress_policies:
        Option<DynamicBlock<AccessContextManagerServicePerimeterSpecElIngressPoliciesEl>>,
    vpc_accessible_services:
        Option<DynamicBlock<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl>>,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_levels: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_policies: Option<Vec<AccessContextManagerServicePerimeterSpecElEgressPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_policies: Option<Vec<AccessContextManagerServicePerimeterSpecElIngressPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_accessible_services:
        Option<Vec<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl>>,
    dynamic: AccessContextManagerServicePerimeterSpecElDynamic,
}
impl AccessContextManagerServicePerimeterSpecEl {
    #[doc = "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.access_levels = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.restricted_services = Some(v.into());
        self
    }
    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterSpecElEgressPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterSpecElIngressPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterSpecEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterSpecEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterSpecEl {}
impl BuildAccessContextManagerServicePerimeterSpecEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterSpecEl {
        AccessContextManagerServicePerimeterSpecEl {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterSpecElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterSpecElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerServicePerimeterSpecElRef {
        AccessContextManagerServicePerimeterSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.access_levels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.restricted_services", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElEgressPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.egress_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_policies` after provisioning.\n"]
    pub fn ingress_policies(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElIngressPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingress_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterSpecElVpcAccessibleServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_accessible_services", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
    #[doc = "Set the field `access_level`.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }
    #[doc = "Set the field `resource`.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nCurrently only projects are allowed. Project format: 'projects/{project_number}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
}
impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_level` after provisioning.\nAn AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to egress the perimeter.\nRequests from these resources are allowed to access data outside the perimeter.\nCurrently only projects are allowed. Project format: 'projects/{project_number}'.\nThe resource may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the\ncase of allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElDynamic {
    sources: Option<
        DynamicBlock<
            AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_restriction: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<
        Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElDynamic,
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
    #[doc = "Set the field `identities`.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn set_identities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `source_restriction`.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn set_source_restriction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_restriction = Some(v.into());
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {}
impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            source_restriction: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn identities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access to outside the\nperimeter. If left unspecified, then members of 'identities' field will\nbe allowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_restriction` after provisioning.\nWhether to enforce traffic restrictions based on 'sources' field. If the 'sources' field is non-empty, then this field must be set to 'SOURCE_RESTRICTION_ENABLED'. Possible values: [\"SOURCE_RESTRICTION_UNSPECIFIED\", \"SOURCE_RESTRICTION_ENABLED\", \"SOURCE_RESTRICTION_DISABLED\"]"]
    pub fn source_restriction(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_restriction", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElSourcesElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { # [doc = "Set the field `method`.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."] pub fn set_method (mut self , v : impl Into < PrimField < String > >) -> Self { self . method = Some (v . into ()) ; self } # [doc = "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn set_permission (mut self , v : impl Into < PrimField < String > >) -> Self { self . permission = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { type O = BlockAssignable < AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl
{}
impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { pub fn build (self) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl { method : core :: default :: Default :: default () , permission : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `method` after provisioning.\nValue for 'method' should be a valid method name for the corresponding\n'serviceName' in 'ApiOperation'. If '*' used as value for method,\nthen ALL methods and permissions are allowed."] pub fn method (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.method" , self . base)) } # [doc = "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn permission (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permission" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElDynamic { method_selectors : Option < DynamicBlock < AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl { # [serde (skip_serializing_if = "Option::is_none")] service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] method_selectors : Option < Vec < AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl > > , dynamic : AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElDynamic , }
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
    #[doc = "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.method_selectors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl
{}
impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with serviceName\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]    pub fn method_selectors (& self) -> ListRef < AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElMethodSelectorsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.method_selectors", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElDynamic {
    operations: Option<
        DynamicBlock<
            AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operations: Option<
        Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElDynamic,
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
    #[doc = "Set the field `external_resources`.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn set_external_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.external_resources = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn set_roles(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {}
impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl {
            external_resources: core::default::Default::default(),
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `external_resources` after provisioning.\nA list of external resources that are allowed to be accessed. A request\nmatches if it contains an external resource in this list (Example:\ns3://bucket/path). Currently '*' is not allowed."]
    pub fn external_resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.external_resources", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', that match this to stanza. A request matches\nif it contains a resource in this list. If * is specified for resources,\nthen this 'EgressTo' rule will authorize access to all resources outside\nthe perimeter."]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'EgressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> ListRef<
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElOperationsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElDynamic {
    egress_from: Option<
        DynamicBlock<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl>,
    >,
    egress_to: Option<
        DynamicBlock<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl>,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_from:
        Option<Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_to: Option<Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElEgressPoliciesElDynamic,
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
    #[doc = "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `egress_from`.\n"]
    pub fn set_egress_from(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `egress_to`.\n"]
    pub fn set_egress_to(
        mut self,
        v: impl Into<
            BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_to = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesEl {}
impl BuildAccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesEl {
            title: core::default::Default::default(),
            egress_from: core::default::Default::default(),
            egress_to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
        AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `egress_from` after provisioning.\n"]
    pub fn egress_from(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressFromElRef> {
        ListRef::new(self.shared().clone(), format!("{}.egress_from", self.base))
    }
    #[doc = "Get a reference to the value of field `egress_to` after provisioning.\n"]
    pub fn egress_to(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElEgressToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.egress_to", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
    #[doc = "Set the field `access_level`.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn set_access_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_level = Some(v.into());
        self
    }
    #[doc = "Set the field `resource`.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects and VPCs are allowed.\nProject format: 'projects/{projectNumber}'\nVPC network format:\n'//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'.\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn set_resource(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl
{}
impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl {
            access_level: core::default::Default::default(),
            resource: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef
    {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_level` after provisioning.\nAn 'AccessLevel' resource name that allow resources within the\n'ServicePerimeters' to be accessed from the internet. 'AccessLevels' listed\nmust be in the same policy as this 'ServicePerimeter'. Referencing a nonexistent\n'AccessLevel' will cause an error. If no 'AccessLevel' names are listed,\nresources within the perimeter can only be accessed via Google Cloud calls\nwith request origins within the perimeter.\nExample 'accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.'\nIf * is specified, then all IngressSources will be allowed."]
    pub fn access_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_level", self.base))
    }
    #[doc = "Get a reference to the value of field `resource` after provisioning.\nA Google Cloud resource that is allowed to ingress the perimeter.\nRequests from these resources will be allowed to access perimeter data.\nCurrently only projects and VPCs are allowed.\nProject format: 'projects/{projectNumber}'\nVPC network format:\n'//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}'.\nThe project may be in any Google Cloud organization, not just the\norganization that the perimeter is defined in. '*' is not allowed, the case\nof allowing all Google Cloud resources only is not supported."]
    pub fn resource(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElDynamic {
    sources: Option<
        DynamicBlock<
            AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    identities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<
        Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElDynamic,
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
    #[doc = "Set the field `identities`.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn set_identities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.identities = Some(v.into());
        self
    }
    #[doc = "Set the field `identity_type`.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn set_identity_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.identity_type = Some(v.into());
        self
    }
    #[doc = "Set the field `sources`.\n"]
    pub fn set_sources(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {}
impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl {
            identities: core::default::Default::default(),
            identity_type: core::default::Default::default(),
            sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `identities` after provisioning.\nIdentities can be an individual user, service account, Google group,\nor third-party identity. For third-party identity, only single identities\nare supported and other identity types are not supported.The v1 identities\nthat have the prefix user, group and serviceAccount in\nhttps://cloud.google.com/iam/docs/principal-identifiers#v1 are supported."]
    pub fn identities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.identities", self.base))
    }
    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\nSpecifies the type of identities that are allowed access from outside the\nperimeter. If left unspecified, then members of 'identities' field will be\nallowed access. Possible values: [\"IDENTITY_TYPE_UNSPECIFIED\", \"ANY_IDENTITY\", \"ANY_USER_ACCOUNT\", \"ANY_SERVICE_ACCOUNT\"]"]
    pub fn identity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(
        &self,
    ) -> ListRef<
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElSourcesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.sources", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { # [doc = "Set the field `method`.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."] pub fn set_method (mut self , v : impl Into < PrimField < String > >) -> Self { self . method = Some (v . into ()) ; self } # [doc = "Set the field `permission`.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn set_permission (mut self , v : impl Into < PrimField < String > >) -> Self { self . permission = Some (v . into ()) ; self } }
impl ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { type O = BlockAssignable < AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl
{}
impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { pub fn build (self) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl { method : core :: default :: Default :: default () , permission : core :: default :: Default :: default () , } } }
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { fn new (shared : StackShared , base : String) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { shared : shared , base : base . to_string () , } } }
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `method` after provisioning.\nValue for method should be a valid method name for the corresponding\nserviceName in 'ApiOperation'. If '*' used as value for 'method', then\nALL methods and permissions are allowed."] pub fn method (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.method" , self . base)) } # [doc = "Get a reference to the value of field `permission` after provisioning.\nValue for permission should be a valid Cloud IAM permission for the\ncorresponding 'serviceName' in 'ApiOperation'."] pub fn permission (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.permission" , self . base)) } }
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElDynamic { method_selectors : Option < DynamicBlock < AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl >> , }
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl { # [serde (skip_serializing_if = "Option::is_none")] service_name : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] method_selectors : Option < Vec < AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl > > , dynamic : AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElDynamic , }
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
    #[doc = "Set the field `service_name`.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn set_service_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `method_selectors`.\n"]
    pub fn set_method_selectors(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.method_selectors = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.method_selectors = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl
{
    type O = BlockAssignable<
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl
{}
impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl {
            service_name: core::default::Default::default(),
            method_selectors: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef
    {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\nThe name of the API whose methods or permissions the 'IngressPolicy' or\n'EgressPolicy' want to allow. A single 'ApiOperation' with 'serviceName'\nfield set to '*' will allow all methods AND permissions for all services."]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
    #[doc = "Get a reference to the value of field `method_selectors` after provisioning.\n"]    pub fn method_selectors (& self) -> ListRef < AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElMethodSelectorsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.method_selectors", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElDynamic {
    operations: Option<
        DynamicBlock<
            AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operations: Option<
        Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl>,
    >,
    dynamic: AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElDynamic,
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
    #[doc = "Set the field `resources`.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `roles`.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn set_roles(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.roles = Some(v.into());
        self
    }
    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.operations = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.operations = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
    type O =
        BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {}
impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl {
            resources: core::default::Default::default(),
            roles: core::default::Default::default(),
            operations: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of resources, currently only projects in the form\n'projects/<projectnumber>', protected by this 'ServicePerimeter'\nthat are allowed to be accessed by sources defined in the\ncorresponding 'IngressFrom'. A request matches if it contains\na resource in this list. If '*' is specified for resources,\nthen this 'IngressTo' rule will authorize access to all\nresources inside the perimeter, provided that the request\nalso matches the 'operations' field."]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `roles` after provisioning.\nA list of IAM roles that represent the set of operations that the sources\nspecified in the corresponding 'IngressFrom'\nare allowed to perform."]
    pub fn roles(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }
    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(
        &self,
    ) -> ListRef<
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElOperationsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.operations", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElDynamic {
    ingress_from: Option<
        DynamicBlock<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl>,
    >,
    ingress_to: Option<
        DynamicBlock<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl>,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_from:
        Option<Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_to:
        Option<Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElIngressPoliciesElDynamic,
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
    #[doc = "Set the field `title`.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
    #[doc = "Set the field `ingress_from`.\n"]
    pub fn set_ingress_from(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_from = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_from = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_to`.\n"]
    pub fn set_ingress_to(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_to = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_to = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesEl {}
impl BuildAccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesEl {
            title: core::default::Default::default(),
            ingress_from: core::default::Default::default(),
            ingress_to: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
        AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the perimeter. Does not affect behavior."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_from` after provisioning.\n"]
    pub fn ingress_from(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressFromElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.ingress_from", self.base))
    }
    #[doc = "Get a reference to the value of field `ingress_to` after provisioning.\n"]
    pub fn ingress_to(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterStatusElIngressPoliciesElIngressToElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ingress_to", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_restriction: Option<PrimField<bool>>,
}
impl AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
    #[doc = "Set the field `allowed_services`.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn set_allowed_services(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.allowed_services = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_restriction`.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn set_enable_restriction(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_restriction = Some(v.into());
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {}
impl BuildAccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
        AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl {
            allowed_services: core::default::Default::default(),
            enable_restriction: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
        AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_services` after provisioning.\nThe list of APIs usable within the Service Perimeter.\nMust be empty unless 'enableRestriction' is True."]
    pub fn allowed_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.allowed_services", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `enable_restriction` after provisioning.\nWhether to restrict API calls within the Service Perimeter to the\nlist of APIs specified in 'allowedServices'."]
    pub fn enable_restriction(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_restriction", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerServicePerimeterStatusElDynamic {
    egress_policies:
        Option<DynamicBlock<AccessContextManagerServicePerimeterStatusElEgressPoliciesEl>>,
    ingress_policies:
        Option<DynamicBlock<AccessContextManagerServicePerimeterStatusElIngressPoliciesEl>>,
    vpc_accessible_services:
        Option<DynamicBlock<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl>>,
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_levels: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restricted_services: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_policies: Option<Vec<AccessContextManagerServicePerimeterStatusElEgressPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_policies: Option<Vec<AccessContextManagerServicePerimeterStatusElIngressPoliciesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_accessible_services:
        Option<Vec<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl>>,
    dynamic: AccessContextManagerServicePerimeterStatusElDynamic,
}
impl AccessContextManagerServicePerimeterStatusEl {
    #[doc = "Set the field `access_levels`.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn set_access_levels(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.access_levels = Some(v.into());
        self
    }
    #[doc = "Set the field `resources`.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }
    #[doc = "Set the field `restricted_services`.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn set_restricted_services(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.restricted_services = Some(v.into());
        self
    }
    #[doc = "Set the field `egress_policies`.\n"]
    pub fn set_egress_policies(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterStatusElEgressPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_policies`.\n"]
    pub fn set_ingress_policies(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerServicePerimeterStatusElIngressPoliciesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_policies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_policies = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_accessible_services`.\n"]
    pub fn set_vpc_accessible_services(
        mut self,
        v: impl Into<
            BlockAssignable<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_accessible_services = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_accessible_services = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerServicePerimeterStatusEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterStatusEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterStatusEl {}
impl BuildAccessContextManagerServicePerimeterStatusEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterStatusEl {
        AccessContextManagerServicePerimeterStatusEl {
            access_levels: core::default::Default::default(),
            resources: core::default::Default::default(),
            restricted_services: core::default::Default::default(),
            egress_policies: core::default::Default::default(),
            ingress_policies: core::default::Default::default(),
            vpc_accessible_services: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterStatusElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterStatusElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerServicePerimeterStatusElRef {
        AccessContextManagerServicePerimeterStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_levels` after provisioning.\nA list of AccessLevel resource names that allow resources within\nthe ServicePerimeter to be accessed from the internet.\nAccessLevels listed must be in the same policy as this\nServicePerimeter. Referencing a nonexistent AccessLevel is a\nsyntax error. If no AccessLevel names are listed, resources within\nthe perimeter can only be accessed via GCP calls with request\norigins within the perimeter. For Service Perimeter Bridge, must\nbe empty.\n\nFormat: accessPolicies/{policy_id}/accessLevels/{access_level_name}"]
    pub fn access_levels(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.access_levels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `resources` after provisioning.\nA list of GCP resources that are inside of the service perimeter.\nCurrently only projects are allowed.\nFormat: projects/{project_number}"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }
    #[doc = "Get a reference to the value of field `restricted_services` after provisioning.\nGCP services that are subject to the Service Perimeter\nrestrictions. Must contain a list of services. For example, if\n'storage.googleapis.com' is specified, access to the storage\nbuckets inside the perimeter must meet the perimeter's access\nrestrictions."]
    pub fn restricted_services(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.restricted_services", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `egress_policies` after provisioning.\n"]
    pub fn egress_policies(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterStatusElEgressPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.egress_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_policies` after provisioning.\n"]
    pub fn ingress_policies(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterStatusElIngressPoliciesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingress_policies", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_accessible_services` after provisioning.\n"]
    pub fn vpc_accessible_services(
        &self,
    ) -> ListRef<AccessContextManagerServicePerimeterStatusElVpcAccessibleServicesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_accessible_services", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerServicePerimeterTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AccessContextManagerServicePerimeterTimeoutsEl {
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
impl ToListMappable for AccessContextManagerServicePerimeterTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerServicePerimeterTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerServicePerimeterTimeoutsEl {}
impl BuildAccessContextManagerServicePerimeterTimeoutsEl {
    pub fn build(self) -> AccessContextManagerServicePerimeterTimeoutsEl {
        AccessContextManagerServicePerimeterTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerServicePerimeterTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerServicePerimeterTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerServicePerimeterTimeoutsElRef {
        AccessContextManagerServicePerimeterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerServicePerimeterTimeoutsElRef {
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
struct AccessContextManagerServicePerimeterDynamic {
    spec: Option<DynamicBlock<AccessContextManagerServicePerimeterSpecEl>>,
    status: Option<DynamicBlock<AccessContextManagerServicePerimeterStatusEl>>,
}
