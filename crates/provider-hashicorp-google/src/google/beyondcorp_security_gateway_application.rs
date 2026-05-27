use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BeyondcorpSecurityGatewayApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    application_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
    security_gateway_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_matchers: Option<Vec<BeyondcorpSecurityGatewayApplicationEndpointMatchersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BeyondcorpSecurityGatewayApplicationTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upstreams: Option<Vec<BeyondcorpSecurityGatewayApplicationUpstreamsEl>>,
    dynamic: BeyondcorpSecurityGatewayApplicationDynamic,
}
struct BeyondcorpSecurityGatewayApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BeyondcorpSecurityGatewayApplicationData>,
}
#[derive(Clone)]
pub struct BeyondcorpSecurityGatewayApplication(Rc<BeyondcorpSecurityGatewayApplication_>);
impl BeyondcorpSecurityGatewayApplication {
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
    #[doc = "Set the field `display_name`.\nOptional. An arbitrary user-provided name for the Application resource.\nCannot exceed 64 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `schema`.\nType of the external application. Possible values: [\"PROXY_GATEWAY\", \"API_GATEWAY\"]"]
    pub fn set_schema(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().schema = Some(v.into());
        self
    }
    #[doc = "Set the field `endpoint_matchers`.\n"]
    pub fn set_endpoint_matchers(
        self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayApplicationEndpointMatchersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint_matchers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint_matchers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<BeyondcorpSecurityGatewayApplicationTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `upstreams`.\n"]
    pub fn set_upstreams(
        self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().upstreams = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.upstreams = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nUser-settable Application resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the Application resource.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
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
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nType of the external application. Possible values: [\"PROXY_GATEWAY\", \"API_GATEWAY\"]"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_gateway_id` after provisioning.\nID of the Security Gateway resource this belongs to."]
    pub fn security_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_matchers` after provisioning.\n"]
    pub fn endpoint_matchers(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayApplicationEndpointMatchersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_matchers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpSecurityGatewayApplicationTimeoutsElRef {
        BeyondcorpSecurityGatewayApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upstreams` after provisioning.\n"]
    pub fn upstreams(&self) -> ListRef<BeyondcorpSecurityGatewayApplicationUpstreamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upstreams", self.extract_ref()),
        )
    }
}
impl Referable for BeyondcorpSecurityGatewayApplication {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BeyondcorpSecurityGatewayApplication {}
impl ToListMappable for BeyondcorpSecurityGatewayApplication {
    type O = ListRef<BeyondcorpSecurityGatewayApplicationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BeyondcorpSecurityGatewayApplication_ {
    fn extract_resource_type(&self) -> String {
        "google_beyondcorp_security_gateway_application".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplication {
    pub tf_id: String,
    #[doc = "User-settable Application resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub application_id: PrimField<String>,
    #[doc = "ID of the Security Gateway resource this belongs to."]
    pub security_gateway_id: PrimField<String>,
}
impl BuildBeyondcorpSecurityGatewayApplication {
    pub fn build(self, stack: &mut Stack) -> BeyondcorpSecurityGatewayApplication {
        let out =
            BeyondcorpSecurityGatewayApplication(Rc::new(BeyondcorpSecurityGatewayApplication_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(BeyondcorpSecurityGatewayApplicationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    application_id: self.application_id,
                    deletion_policy: core::default::Default::default(),
                    display_name: core::default::Default::default(),
                    id: core::default::Default::default(),
                    project: core::default::Default::default(),
                    schema: core::default::Default::default(),
                    security_gateway_id: self.security_gateway_id,
                    endpoint_matchers: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    upstreams: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BeyondcorpSecurityGatewayApplicationRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BeyondcorpSecurityGatewayApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `application_id` after provisioning.\nUser-settable Application resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn application_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
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
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the Application resource.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. Name of the resource."]
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
    #[doc = "Get a reference to the value of field `schema` after provisioning.\nType of the external application. Possible values: [\"PROXY_GATEWAY\", \"API_GATEWAY\"]"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.schema", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_gateway_id` after provisioning.\nID of the Security Gateway resource this belongs to."]
    pub fn security_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `endpoint_matchers` after provisioning.\n"]
    pub fn endpoint_matchers(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayApplicationEndpointMatchersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.endpoint_matchers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpSecurityGatewayApplicationTimeoutsElRef {
        BeyondcorpSecurityGatewayApplicationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `upstreams` after provisioning.\n"]
    pub fn upstreams(&self) -> ListRef<BeyondcorpSecurityGatewayApplicationUpstreamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.upstreams", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationEndpointMatchersEl {
    hostname: PrimField<String>,
    ports: ListField<PrimField<f64>>,
}
impl BeyondcorpSecurityGatewayApplicationEndpointMatchersEl {}
impl ToListMappable for BeyondcorpSecurityGatewayApplicationEndpointMatchersEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayApplicationEndpointMatchersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationEndpointMatchersEl {
    #[doc = "Required. Hostname of the application."]
    pub hostname: PrimField<String>,
    #[doc = "Optional. Ports of the application."]
    pub ports: ListField<PrimField<f64>>,
}
impl BuildBeyondcorpSecurityGatewayApplicationEndpointMatchersEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplicationEndpointMatchersEl {
        BeyondcorpSecurityGatewayApplicationEndpointMatchersEl {
            hostname: self.hostname,
            ports: self.ports,
        }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationEndpointMatchersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationEndpointMatchersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayApplicationEndpointMatchersElRef {
        BeyondcorpSecurityGatewayApplicationEndpointMatchersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayApplicationEndpointMatchersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nRequired. Hostname of the application."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\nOptional. Ports of the application."]
    pub fn ports(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayApplicationTimeoutsEl {
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
impl ToListMappable for BeyondcorpSecurityGatewayApplicationTimeoutsEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayApplicationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationTimeoutsEl {}
impl BuildBeyondcorpSecurityGatewayApplicationTimeoutsEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplicationTimeoutsEl {
        BeyondcorpSecurityGatewayApplicationTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BeyondcorpSecurityGatewayApplicationTimeoutsElRef {
        BeyondcorpSecurityGatewayApplicationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayApplicationTimeoutsElRef {
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
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl {
    regions: ListField<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl {}
impl ToListMappable for BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl {
    #[doc = "Required. List of regions where the application sends traffic to."]
    pub regions: ListField<PrimField<String>>,
}
impl BuildBeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl {
        BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl {
            regions: self.regions,
        }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyElRef {
        BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\nRequired. List of regions where the application sends traffic to."]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl {
    hostname: PrimField<String>,
    port: PrimField<f64>,
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl {}
impl ToListMappable for BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl {
    #[doc = "Hostname of the endpoint."]
    pub hostname: PrimField<String>,
    #[doc = "Port of the endpoint."]
    pub port: PrimField<f64>,
}
impl BuildBeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl {
        BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl {
            hostname: self.hostname,
            port: self.port,
        }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsElRef {
        BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname of the endpoint."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\nPort of the endpoint."]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElDynamic {
    endpoints:
        Option<DynamicBlock<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl>>,
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<Vec<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl>>,
    dynamic: BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElDynamic,
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl {
    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl Into<
            BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.endpoints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.endpoints = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl {}
impl BuildBeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl {
        BeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl {
            endpoints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElRef {
        BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl {
    name: PrimField<String>,
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl {}
impl ToListMappable for BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl {
    #[doc = "Required. Network name is of the format:\n'projects/{project}/global/networks/{network}'"]
    pub name: PrimField<String>,
}
impl BuildBeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl {
        BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl { name: self.name }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkElRef {
        BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nRequired. Network name is of the format:\n'projects/{project}/global/networks/{network}'"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl {
    #[doc = "Set the field `output_type`.\nThe output type of the delegated device info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
}
impl ToListMappable for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl { type O = BlockAssignable < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl
{}
impl BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl { pub fn build (self) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl { BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl { output_type : core :: default :: Default :: default () , } } }
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoElRef { fn new (shared : StackShared , base : String) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoElRef { BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoElRef { shared : shared , base : base . to_string () , } } }
impl
    BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated device info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl {
    #[doc = "Set the field `output_type`.\nThe output type of the delegated group info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl
{
    type O = BlockAssignable < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl > ;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl
{}
impl BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl { pub fn build (self) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl { BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl { output_type : core :: default :: Default :: default () , } } }
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoElRef { fn new (shared : StackShared , base : String) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoElRef { BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoElRef { shared : shared , base : base . to_string () , } } }
impl
    BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated group info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl {
    #[doc = "Set the field `output_type`.\nThe output type of the delegated user info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl
{
    type O = BlockAssignable<
        BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl
{}
impl
    BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl
{
    pub fn build(
        self,
    ) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl
    {
        BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl { output_type : core :: default :: Default :: default () , }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoElRef { fn new (shared : StackShared , base : String) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoElRef { BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoElRef { shared : shared , base : base . to_string () , } } }
impl
    BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated user info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDynamic { device_info : Option < DynamicBlock < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl >> , group_info : Option < DynamicBlock < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl >> , user_info : Option < DynamicBlock < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl >> , }
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl { # [serde (skip_serializing_if = "Option::is_none")] output_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] device_info : Option < Vec < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl > > , # [serde (skip_serializing_if = "Option::is_none")] group_info : Option < Vec < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl > > , # [serde (skip_serializing_if = "Option::is_none")] user_info : Option < Vec < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl > > , dynamic : BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDynamic , }
impl BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl {
    #[doc = "Set the field `output_type`.\nDefault output type for all enabled headers. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
    #[doc = "Set the field `device_info`.\n"]
    pub fn set_device_info(
        mut self,
        v : impl Into < BlockAssignable < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.device_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.device_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `group_info`.\n"]
    pub fn set_group_info(
        mut self,
        v : impl Into < BlockAssignable < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.group_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.group_info = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `user_info`.\n"]
    pub fn set_user_info(
        mut self,
        v : impl Into < BlockAssignable < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.user_info = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.user_info = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl
{
    type O = BlockAssignable<
        BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl {
}
impl BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl {
    pub fn build(
        self,
    ) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl {
        BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl {
            output_type: core::default::Default::default(),
            device_info: core::default::Default::default(),
            group_info: core::default::Default::default(),
            user_info: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElRef {
        BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\nDefault output type for all enabled headers. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
    #[doc = "Get a reference to the value of field `device_info` after provisioning.\n"]    pub fn device_info (& self) -> ListRef < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElDeviceInfoElRef >{
        ListRef::new(self.shared().clone(), format!("{}.device_info", self.base))
    }
    #[doc = "Get a reference to the value of field `group_info` after provisioning.\n"]    pub fn group_info (& self) -> ListRef < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElGroupInfoElRef >{
        ListRef::new(self.shared().clone(), format!("{}.group_info", self.base))
    }
    #[doc = "Get a reference to the value of field `user_info` after provisioning.\n"]    pub fn user_info (& self) -> ListRef < BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElUserInfoElRef >{
        ListRef::new(self.shared().clone(), format!("{}.user_info", self.base))
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElDynamic {
    contextual_headers: Option<
        DynamicBlock<
            BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_client_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_identity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contextual_headers: Option<
        Vec<BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl>,
    >,
    dynamic: BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElDynamic,
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl {
    #[doc = "Set the field `allowed_client_headers`.\nThe configuration for the proxy."]
    pub fn set_allowed_client_headers(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_client_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `client_ip`.\nClient IP configuration. The client IP address is included if true."]
    pub fn set_client_ip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.client_ip = Some(v.into());
        self
    }
    #[doc = "Set the field `gateway_identity`.\nGateway identity configuration. Possible values: [\"RESOURCE_NAME\"]"]
    pub fn set_gateway_identity(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gateway_identity = Some(v.into());
        self
    }
    #[doc = "Set the field `metadata_headers`.\nCustom resource specific headers along with the values.\nThe names should conform to RFC 9110:\n> Field names SHOULD constrain themselves to alphanumeric characters, \"-\",\n  and \".\", and SHOULD begin with a letter.\n> Field values SHOULD contain only ASCII printable characters and tab."]
    pub fn set_metadata_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.metadata_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `contextual_headers`.\n"]
    pub fn set_contextual_headers(
        mut self,
        v: impl Into<
            BlockAssignable<
                BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.contextual_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.contextual_headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl {}
impl BuildBeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl {
        BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl {
            allowed_client_headers: core::default::Default::default(),
            client_ip: core::default::Default::default(),
            gateway_identity: core::default::Default::default(),
            metadata_headers: core::default::Default::default(),
            contextual_headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElRef {
        BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_client_headers` after provisioning.\nThe configuration for the proxy."]
    pub fn allowed_client_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_client_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `client_ip` after provisioning.\nClient IP configuration. The client IP address is included if true."]
    pub fn client_ip(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_ip", self.base))
    }
    #[doc = "Get a reference to the value of field `gateway_identity` after provisioning.\nGateway identity configuration. Possible values: [\"RESOURCE_NAME\"]"]
    pub fn gateway_identity(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_identity", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `metadata_headers` after provisioning.\nCustom resource specific headers along with the values.\nThe names should conform to RFC 9110:\n> Field names SHOULD constrain themselves to alphanumeric characters, \"-\",\n  and \".\", and SHOULD begin with a letter.\n> Field values SHOULD contain only ASCII printable characters and tab."]
    pub fn metadata_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.metadata_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `contextual_headers` after provisioning.\n"]
    pub fn contextual_headers(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElContextualHeadersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contextual_headers", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplicationUpstreamsElDynamic {
    egress_policy:
        Option<DynamicBlock<BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl>>,
    external: Option<DynamicBlock<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl>>,
    network: Option<DynamicBlock<BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl>>,
    proxy_protocol:
        Option<DynamicBlock<BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl>>,
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_policy: Option<Vec<BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external: Option<Vec<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<Vec<BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_protocol: Option<Vec<BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl>>,
    dynamic: BeyondcorpSecurityGatewayApplicationUpstreamsElDynamic,
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsEl {
    #[doc = "Set the field `egress_policy`.\n"]
    pub fn set_egress_policy(
        mut self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `external`.\n"]
    pub fn set_external(
        mut self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.external = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.external = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(
        mut self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.network = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.network = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proxy_protocol`.\n"]
    pub fn set_proxy_protocol(
        mut self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.proxy_protocol = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.proxy_protocol = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BeyondcorpSecurityGatewayApplicationUpstreamsEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayApplicationUpstreamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayApplicationUpstreamsEl {}
impl BuildBeyondcorpSecurityGatewayApplicationUpstreamsEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayApplicationUpstreamsEl {
        BeyondcorpSecurityGatewayApplicationUpstreamsEl {
            egress_policy: core::default::Default::default(),
            external: core::default::Default::default(),
            network: core::default::Default::default(),
            proxy_protocol: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayApplicationUpstreamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayApplicationUpstreamsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayApplicationUpstreamsElRef {
        BeyondcorpSecurityGatewayApplicationUpstreamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayApplicationUpstreamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `egress_policy` after provisioning.\n"]
    pub fn egress_policy(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayApplicationUpstreamsElEgressPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.egress_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `external` after provisioning.\n"]
    pub fn external(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayApplicationUpstreamsElExternalElRef> {
        ListRef::new(self.shared().clone(), format!("{}.external", self.base))
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> ListRef<BeyondcorpSecurityGatewayApplicationUpstreamsElNetworkElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `proxy_protocol` after provisioning.\n"]
    pub fn proxy_protocol(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayApplicationUpstreamsElProxyProtocolElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_protocol", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayApplicationDynamic {
    endpoint_matchers: Option<DynamicBlock<BeyondcorpSecurityGatewayApplicationEndpointMatchersEl>>,
    upstreams: Option<DynamicBlock<BeyondcorpSecurityGatewayApplicationUpstreamsEl>>,
}
